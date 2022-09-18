use ::libc;
extern "C" {
    pub type archive_string_conv;
    /*-
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
     *
     * $FreeBSD$
     */
    pub type archive_entry;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    /* The 'clone' function does a deep copy; all of the strings are copied too. */
    #[no_mangle]
    fn archive_entry_clone(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    /*
     * This form of archive_entry_new2() will pull character-set
     * conversion information from the specified archive handle.  The
     * older archive_entry_new(void) form is equivalent to calling
     * archive_entry_new2(NULL) and will result in the use of an internal
     * default character-set conversion.
     */
    #[no_mangle]
    fn archive_entry_new2(_: *mut archive) -> *mut archive_entry;
    /*
     * Retrieve fields from an archive_entry.
     *
     * There are a number of implicit conversions among these fields.  For
     * example, if a regular string field is set and you read the _w wide
     * character field, the entry will implicitly convert narrow-to-wide
     * using the current locale.  Similarly, dev values are automatically
     * updated when you write devmajor or devminor and vice versa.
     *
     * In addition, fields can be "set" or "unset."  Unset string fields
     * return NULL, non-string fields have _is_set() functions to test
     * whether they've been set.  You can "unset" a string field by
     * assigning NULL; non-string fields have _unset() functions to
     * unset them.
     *
     * Note: There is one ambiguity in the above; string fields will
     * also return NULL when implicit character set conversions fail.
     * This is usually what you want.
     */
    #[no_mangle]
    fn archive_entry_atime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_atime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_birthtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_birthtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_birthtime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_ctime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_ctime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_fflags_text(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_gname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_perm(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_rdevmajor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_rdevminor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_symlink_type(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uname(_: *mut archive_entry) -> *const libc::c_char;
    /*
     * Set fields in an archive_entry.
     *
     * Note: Before libarchive 2.4, there were 'set' and 'copy' versions
     * of the string setters.  'copy' copied the actual string, 'set' just
     * stored the pointer.  In libarchive 2.4 and later, strings are
     * always copied.
     */
    #[no_mangle]
    fn archive_entry_set_atime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_ctime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_filetype(_: *mut archive_entry, _: libc::c_uint);
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_hardlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_mode(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_mtime(_: *mut archive_entry, _: time_t, _: libc::c_long);
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_perm(_: *mut archive_entry, _: mode_t);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_symlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uname(_: *mut archive_entry, _: *const libc::c_char);
    /*
     * Storage for Mac OS-specific AppleDouble metadata information.
     * Apple-format tar files store a separate binary blob containing
     * encoded metadata with ACL, extended attributes, etc.
     * This provides a place to store that blob.
     */
    #[no_mangle]
    fn archive_entry_mac_metadata(_: *mut archive_entry, _: *mut size_t) -> *const libc::c_void;
    /* Return bitmask of ACL types in an archive entry */
    #[no_mangle]
    fn archive_entry_acl_types(_: *mut archive_entry) -> libc::c_int;
    /*
     * To retrieve the xattr list, first "reset", then repeatedly ask for the
     * "next" entry.
     */
    #[no_mangle]
    fn archive_entry_xattr_count(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_xattr_reset(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_xattr_next(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut *const libc::c_void,
        _: *mut size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_sparse_add_entry(_: *mut archive_entry, _: la_int64_t, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_sparse_reset(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_sparse_next(
        _: *mut archive_entry,
        _: *mut la_int64_t,
        _: *mut la_int64_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_gname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_hardlink_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_pathname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_symlink_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_uname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_acl_to_text_l(
        _: *mut archive_entry,
        _: *mut ssize_t,
        _: libc::c_int,
        _: *mut archive_string_conv,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_strncat(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_strncpy_l(
        _: *mut archive_string,
        _: *const libc::c_void,
        _: size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_conversion_to_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_ensure(_: *mut archive_string, _: size_t) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn archive_strcat(_: *mut archive_string, _: *const libc::c_void) -> *mut archive_string;
    #[no_mangle]
    fn archive_array_append(
        _: *mut archive_string,
        _: *const libc::c_char,
        _: size_t,
    ) -> *mut archive_string;
    #[no_mangle]
    fn archive_strappend_char(_: *mut archive_string, _: libc::c_char) -> *mut archive_string;
    #[no_mangle]
    fn archive_string_free(_: *mut archive_string);
    #[no_mangle]
    fn archive_string_sprintf(_: *mut archive_string, _: *const libc::c_char, _: ...);
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_nulls(_: *mut archive_write, _: size_t) -> libc::c_int;
    /*
     * Utility function to format a USTAR header into a buffer.  If
     * "strict" is set, this tries to create the absolutely most portable
     * version of a ustar header.  If "strict" is set to 0, then it will
     * relax certain requirements.
     *
     * Generally, format-specific declarations don't belong in this
     * header; this is a rare example of a function that is shared by
     * two very similar formats (ustar and pax).
     */
    #[no_mangle]
    fn __archive_write_format_header_ustar(
        _: *mut archive_write,
        buff: *mut libc::c_char,
        _: *mut archive_entry,
        tartype: libc::c_int,
        strict: libc::c_int,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
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
    fn __archive_write_entry_filetype_unsupported(
        a: *mut archive,
        entry: *mut archive_entry,
        format: *const libc::c_char,
    );
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type size_t = libc::c_ulong;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type la_int64_t = int64_t;
pub type la_ssize_t = ssize_t;
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
pub type archive_write_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *const libc::c_void,
    _: size_t,
) -> la_ssize_t;
pub type archive_open_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write {
    pub archive: archive,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub nulls: *const libc::c_uchar,
    pub null_length: size_t,
    pub client_opener: Option<archive_open_callback>,
    pub client_writer: Option<archive_write_callback>,
    pub client_closer: Option<archive_close_callback>,
    pub client_data: *mut libc::c_void,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub filter_first: *mut archive_write_filter,
    pub filter_last: *mut archive_write_filter,
    pub format_data: *mut libc::c_void,
    pub format_name: *const libc::c_char,
    pub format_init: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub format_finish_entry: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_write_header:
        Option<unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int>,
    pub format_write_data: Option<
        unsafe extern "C" fn(_: *mut archive_write, _: *const libc::c_void, _: size_t) -> ssize_t,
    >,
    pub format_close: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_free: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub passphrase: *mut libc::c_char,
    pub passphrase_callback: Option<archive_passphrase_callback>,
    pub passphrase_client_data: *mut libc::c_void,
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
 * $FreeBSD: head/lib/libarchive/archive_write_private.h 201155 2009-12-29 05:20:12Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write_filter {
    pub bytes_written: int64_t,
    pub archive: *mut archive,
    pub next_filter: *mut archive_write_filter,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub open: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub write: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub code: libc::c_int,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pax {
    pub entry_bytes_remaining: uint64_t,
    pub entry_padding: uint64_t,
    pub l_url_encoded_name: archive_string,
    pub pax_header: archive_string,
    pub sparse_map: archive_string,
    pub sparse_map_padding: size_t,
    pub sparse_list: *mut sparse_block,
    pub sparse_tail: *mut sparse_block,
    pub sconv_utf8: *mut archive_string_conv,
    pub opt_binary: libc::c_int,
    pub flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse_block {
    pub next: *mut sparse_block,
    pub is_hole: libc::c_int,
    pub offset: uint64_t,
    pub remaining: uint64_t,
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
pub const INT64_MIN: libc::c_long =
    -(9223372036854775807 as libc::c_long) - 1 as libc::c_int as libc::c_long;
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const S_ISUID: libc::c_int = __S_ISUID;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR: libc::c_int = 0x30000 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE: libc::c_int = ARCHIVE_FORMAT_TAR | 2 as libc::c_int;
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const __S_ISVTX: libc::c_int = 0o1000 as libc::c_int;
pub const __S_ISGID: libc::c_int = 0o2000 as libc::c_int;
pub const __S_ISUID: libc::c_int = 0o4000 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_FORMAT_TAR_PAX_RESTRICTED: libc::c_int = ARCHIVE_FORMAT_TAR | 3 as libc::c_int;
pub const S_ISVTX: libc::c_int = __S_ISVTX;
pub const S_ISGID: libc::c_int = __S_ISGID;
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
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFIFO: libc::c_int = 0o10000 as libc::c_int;
/*
 * Symlink types
 */
pub const AE_SYMLINK_TYPE_FILE: libc::c_int = 1 as libc::c_int;
pub const AE_SYMLINK_TYPE_DIRECTORY: libc::c_int = 2 as libc::c_int;
/*
 * ACL routines.  This used to simply store and return text-format ACL
 * strings, but that proved insufficient for a number of reasons:
 *   = clients need control over uname/uid and gname/gid mappings
 *   = there are many different ACL text formats
 *   = would like to be able to read/convert archives containing ACLs
 *     on platforms that lack ACL libraries
 *
 *  This last point, in particular, forces me to implement a reasonably
 *  complete set of ACL support routines.
 */
/*
 * Permission bits.
 */
/*
 * Inheritance values (NFS4 ACLs only); included in permset.
 */
/* We need to be able to specify combinations of these. */
pub const ARCHIVE_ENTRY_ACL_TYPE_ACCESS: libc::c_int = 0x100 as libc::c_int;
/* POSIX.1e only */
pub const ARCHIVE_ENTRY_ACL_TYPE_DEFAULT: libc::c_int = 0x200 as libc::c_int;
/* POSIX.1e only */
pub const ARCHIVE_ENTRY_ACL_TYPE_ALLOW: libc::c_int = 0x400 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_DENY: libc::c_int = 0x800 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_AUDIT: libc::c_int = 0x1000 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_ALARM: libc::c_int = 0x2000 as libc::c_int;
/* NFS4 only */
pub const ARCHIVE_ENTRY_ACL_TYPE_NFS4: libc::c_int = ARCHIVE_ENTRY_ACL_TYPE_ALLOW
    | ARCHIVE_ENTRY_ACL_TYPE_DENY
    | ARCHIVE_ENTRY_ACL_TYPE_AUDIT
    | ARCHIVE_ENTRY_ACL_TYPE_ALARM;
/*
 * Construct a text-format ACL.  The flags argument is a bitmask that
 * can include any of the following:
 *
 * Flags only for archive entries with POSIX.1e ACL:
 * ARCHIVE_ENTRY_ACL_TYPE_ACCESS - Include POSIX.1e "access" entries.
 * ARCHIVE_ENTRY_ACL_TYPE_DEFAULT - Include POSIX.1e "default" entries.
 * ARCHIVE_ENTRY_ACL_STYLE_MARK_DEFAULT - Include "default:" before each
 *    default ACL entry.
 * ARCHIVE_ENTRY_ACL_STYLE_SOLARIS - Output only one colon after "other" and
 *    "mask" entries.
 *
 * Flags only for archive entries with NFSv4 ACL:
 * ARCHIVE_ENTRY_ACL_STYLE_COMPACT - Do not output the minus character for
 *    unset permissions and flags in NFSv4 ACL permission and flag fields
 *
 * Flags for for archive entries with POSIX.1e ACL or NFSv4 ACL:
 * ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID - Include extra numeric ID field in
 *    each ACL entry.
 * ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA - Separate entries with comma
 *    instead of newline.
 */
pub const ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID: libc::c_int = 0x1 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA: libc::c_int = 0x8 as libc::c_int;
pub const ARCHIVE_ENTRY_ACL_STYLE_COMPACT: libc::c_int = 0x10 as libc::c_int;
/*
 * Utility functions to set and get entry attributes by translating
 * character-set. These are designed for use in format readers and writers.
 *
 * The return code and interface of these are quite different from other
 * functions for archive_entry defined in archive_entry.h.
 * Common return code are:
 *   Return 0 if the string conversion succeeded.
 *   Return -1 if the string conversion failed.
 */
pub const archive_entry_gname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_gname_l;
pub const archive_entry_hardlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_hardlink_l;
pub const archive_entry_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_pathname_l;
pub const archive_entry_symlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_symlink_l;
pub const archive_entry_uname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_uname_l;
pub const archive_entry_acl_to_text_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut ssize_t,
    _: libc::c_int,
    _: *mut archive_string_conv,
) -> *mut libc::c_char = _archive_entry_acl_to_text_l;
pub const WRITE_SCHILY_XATTR: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const WRITE_LIBARCHIVE_XATTR: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
/*
 * Set output format to 'restricted pax' format.
 *
 * This is the same as normal 'pax', but tries to suppress
 * the pax header whenever possible.  This is the default for
 * bsdtar, for instance.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_pax_restricted(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut r: libc::c_int = 0;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_pax_restricted\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    r = archive_write_set_format_pax(&mut (*a).archive);
    (*a).archive.archive_format = ARCHIVE_FORMAT_TAR_PAX_RESTRICTED;
    (*a).archive.archive_format_name =
        b"restricted POSIX pax interchange\x00" as *const u8 as *const libc::c_char;
    return r;
}
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
 * $FreeBSD: src/lib/libarchive/archive.h.in,v 1.50 2008/05/26 17:00:22 kientzle Exp $
 */
/*
 * The version number is expressed as a single integer that makes it
 * easy to compare versions at build time: for version a.b.c, the
 * version number is printf("%d%03d%03d",a,b,c).  For example, if you
 * know your application requires version 2.12.108 or later, you can
 * assert that ARCHIVE_VERSION_NUMBER >= 2012108.
 */
/* Note: Compiler will complain if this does not match archive_entry.h! */
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
/* The la_ssize_t should match the type used in 'struct stat' */
/* Older code relied on the __LA_SSIZE_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
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
/*
 * Textual name/version of the library, useful for version displays.
 */
/*
 * Detailed textual name/version of the library and its dependencies.
 * This has the form:
 *    "libarchive x.y.z zlib/a.b.c liblzma/d.e.f ... etc ..."
 * the list of libraries described here will vary depending on how
 * libarchive was compiled.
 */
/*
 * Returns NULL if libarchive was compiled without the associated library.
 * Otherwise, returns the version number that libarchive was compiled
 * against.
 */
/* Declare our basic types. */
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
/* Operation was successful. */
/* Retry might succeed. */
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
/* No more operations are possible. */
/*
 * As far as possible, archive_errno returns standard platform errno codes.
 * Of course, the details vary by platform, so the actual definitions
 * here are stored in "archive_platform.h".  The symbols are listed here
 * for reference; as a rule, clients should not need to know the exact
 * platform-dependent error code.
 */
/* Unrecognized or invalid file format. */
/* #define	ARCHIVE_ERRNO_FILE_FORMAT */
/* Illegal usage of the library. */
/* #define	ARCHIVE_ERRNO_PROGRAMMER_ERROR */
/* Unknown or unclassified error. */
/* #define	ARCHIVE_ERRNO_MISC */
/*
 * Callbacks are invoked to automatically read/skip/write/open/close the
 * archive. You can provide your own for complex tasks (like breaking
 * archives across multiple tapes) or use standard ones built into the
 * library.
 */
/* Returns pointer and size of next block of data from archive. */
/* Skips at most request bytes from archive and returns the skipped amount.
 * This may skip fewer bytes than requested; it may even skip zero bytes.
 * If you do skip fewer bytes than requested, libarchive will invoke your
 * read callback and discard data as necessary to make up the full skip.
 */
/* Seeks to specified location in the file and returns the position.
 * Whence values are SEEK_SET, SEEK_CUR, SEEK_END from stdio.h.
 * Return ARCHIVE_FATAL if the seek fails for any reason.
 */
/* Returns size actually written, zero on EOF, -1 on error. */
/* Switches from one client data object to the next/prev client data object.
 * This is useful for reading from different data blocks such as a set of files
 * that make up one large file.
 */
/*
 * Returns a passphrase used for encryption or decryption, NULL on nothing
 * to do and give it up.
 */
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
/*
 * Codes returned by archive_read_format_capabilities().
 *
 * This list can be extended with values between 0 and 0xffff.
 * The original purpose of this list was to let different archive
 * format readers expose their general capabilities in terms of
 * encryption.
 */
/* no special capabilities */
/* reader can detect encrypted data */
/* reader can detect encryptable metadata (pathname, mtime, etc.) */
/*
 * Codes returned by archive_read_has_encrypted_entries().
 *
 * In case the archive does not support encryption detection at all
 * ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED is returned. If the reader
 * for some other reason (e.g. not enough bytes read) cannot say if
 * there are encrypted entries, ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW
 * is returned.
 */
/*-
 * Basic outline for reading an archive:
 *   1) Ask archive_read_new for an archive reader object.
 *   2) Update any global properties as appropriate.
 *      In particular, you'll certainly want to call appropriate
 *      archive_read_support_XXX functions.
 *   3) Call archive_read_open_XXX to open the archive
 *   4) Repeatedly call archive_read_next_header to get information about
 *      successive archive entries.  Call archive_read_data to extract
 *      data for entries of interest.
 *   5) Call archive_read_free to end processing.
 */
/*
 * The archive_read_support_XXX calls enable auto-detect for this
 * archive handle.  They also link in the necessary support code.
 * For example, if you don't want bzlib linked in, don't invoke
 * support_compression_bzip2().  The "all" functions provide the
 * obvious shorthand.
 */
/* match */
/* cmd */
/* match */
/* archive_read_support_format_zip() enables both streamable and seekable
 * zip readers. */
/* Reads Zip archives as stream from beginning to end.  Doesn't
 * correctly handle SFX ZIP files or ZIP archives that have been modified
 * in-place. */
/* Reads starting from central directory; requires seekable input. */
/* Functions to manually set the format and filters to be used. This is
 * useful to bypass the bidding process when the format and filters to use
 * is known in advance.
 */
/* match */
/* Set various callbacks. */
/* Callback used to switch between one data object to the next */
/* This sets the first data object. */
/* This sets data object at specified index */
/* This adds a data object at the specified index. */
/* This appends a data object to the end of list */
/* This prepends a data object to the beginning of list */
/* Opening freezes the callbacks. */
/* Convenience wrappers around the above. */
/*
 * A variety of shortcuts that invoke archive_read_open() with
 * canned callbacks suitable for common situations.  The ones that
 * accept a block size handle tape blocking correctly.
 */
/* Use this if you know the filename.  Note: NULL indicates stdin. */
/* Use this for reading multivolume files by filenames.
 * NOTE: Must be NULL terminated. Sorting is NOT done. */
/* archive_read_open_file() is a deprecated synonym for ..._open_filename(). */
/* Read an archive that's stored in memory. */
/* A more involved version that is only used for internal testing. */
/* Read an archive that's already open, using the file descriptor. */
/* Read an archive that's already open, using a FILE *. */
/* Note: DO NOT use this with tape drives. */
/* Parses and returns next entry header. */
/* Parses and returns next entry header using the archive_entry passed in */
/*
 * Retrieve the byte offset in UNCOMPRESSED data where last-read
 * header started.
 */
/*
 * Returns 1 if the archive contains at least one encrypted entry.
 * If the archive format not support encryption at all
 * ARCHIVE_READ_FORMAT_ENCRYPTION_UNSUPPORTED is returned.
 * If for any other reason (e.g. not enough data read so far)
 * we cannot say whether there are encrypted entries, then
 * ARCHIVE_READ_FORMAT_ENCRYPTION_DONT_KNOW is returned.
 * In general, this function will return values below zero when the
 * reader is uncertain or totally incapable of encryption support.
 * When this function returns 0 you can be sure that the reader
 * supports encryption detection but no encrypted entries have
 * been found yet.
 *
 * NOTE: If the metadata/header of an archive is also encrypted, you
 * cannot rely on the number of encrypted entries. That is why this
 * function does not return the number of encrypted entries but#
 * just shows that there are some.
 */
/*
 * Returns a bitmask of capabilities that are supported by the archive format reader.
 * If the reader has no special capabilities, ARCHIVE_READ_FORMAT_CAPS_NONE is returned.
 */
/* Read data from the body of an entry.  Similar to read(2). */
/* Seek within the body of an entry.  Similar to lseek(2). */
/*
 * A zero-copy version of archive_read_data that also exposes the file offset
 * of each returned block.  Note that the client has no way to specify
 * the desired size of the block.  The API does guarantee that offsets will
 * be strictly increasing and that returned blocks will not overlap.
 */
/*-
 * Some convenience functions that are built on archive_read_data:
 *  'skip': skips entire entry
 *  'into_buffer': writes data into memory buffer that you provide
 *  'into_fd': writes data to specified filedes
 */
/*
 * Set read options.
 */
/* Apply option to the format only. */
/* Apply option to the filter only. */
/* Apply option to both the format and the filter. */
/* Apply option string to both the format and the filter. */
/*
 * Add a decryption passphrase.
 */
/*-
 * Convenience function to recreate the current entry (whose header
 * has just been read) on disk.
 *
 * This does quite a bit more than just copy data to disk. It also:
 *  - Creates intermediate directories as required.
 *  - Manages directory permissions:  non-writable directories will
 *    be initially created with write permission enabled; when the
 *    archive is closed, dir permissions are edited to the values specified
 *    in the archive.
 *  - Checks hardlinks:  hardlinks will not be extracted unless the
 *    linked-to file was also extracted within the same session. (TODO)
 */
/* The "flags" argument selects optional behavior, 'OR' the flags you want. */
/* Default: Do not try to set owner/group. */
/* Default: Do obey umask, do not restore SUID/SGID/SVTX bits. */
/* Default: Do not restore mtime/atime. */
/* Default: Replace existing files. */
/* Default: Try create first, unlink only if create fails with EEXIST. */
/* Default: Do not restore ACLs. */
/* Default: Do not restore fflags. */
/* Default: Do not restore xattrs. */
/* Default: Do not try to guard against extracts redirected by symlinks. */
/* Note: With ARCHIVE_EXTRACT_UNLINK, will remove any intermediate symlink. */
/* Default: Do not reject entries with '..' as path elements. */
/* Default: Create parent directories as needed. */
/* Default: Overwrite files, even if one on disk is newer. */
/* Detect blocks of 0 and write holes instead. */
/* Default: Do not restore Mac extended metadata. */
/* This has no effect except on Mac OS. */
/* Default: Use HFS+ compression if it was compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not use HFS+ compression if it was not compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not reject entries with absolute paths */
/* Default: Do not clear no-change flags when unlinking object */
/* Default: Do not extract atomically (using rename) */
/* dest */
/* Record the dev/ino of a file that will not be written.  This is
 * generally set to the dev/ino of the archive being read. */
/* Close the file and release most resources. */
/* Release all resources and destroy the object. */
/* Note that archive_read_free will call archive_read_close for you. */
/* Synonym for archive_read_free() for backwards compatibility. */
/*-
 * To create an archive:
 *   1) Ask archive_write_new for an archive writer object.
 *   2) Set any global properties.  In particular, you should set
 *      the compression and format to use.
 *   3) Call archive_write_open to open the file (most people
 *       will use archive_write_open_file or archive_write_open_fd,
 *       which provide convenient canned I/O callbacks for you).
 *   4) For each entry:
 *      - construct an appropriate struct archive_entry structure
 *      - archive_write_header to write the header
 *      - archive_write_data to write the entry data
 *   5) archive_write_close to close the output
 *   6) archive_write_free to cleanup the writer and release resources
 */
/* XXX This is badly misnamed; suggestions appreciated. XXX */
/* The dev/ino of a file that won't be archived.  This is used
 * to avoid recursively adding an archive to itself. */
/* A convenience function to set the filter based on the code. */
/* A convenience function to set the format based on the code or name. */
/* To minimize link pollution, use one or more of the following. */
/* TODO: int archive_write_set_format_old_tar(struct archive *); */
/*
 * Set output format to 'pax' format.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_pax(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut pax: *mut pax = 0 as *mut pax;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_pax\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    pax = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<pax>() as libc::c_ulong,
    ) as *mut pax;
    if pax.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate pax data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*pax).flags = (WRITE_LIBARCHIVE_XATTR | WRITE_SCHILY_XATTR) as libc::c_uint;
    (*a).format_data = pax as *mut libc::c_void;
    (*a).format_name = b"pax\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        archive_write_pax_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        archive_write_pax_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        archive_write_pax_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_close =
        Some(archive_write_pax_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_free =
        Some(archive_write_pax_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_finish_entry = Some(
        archive_write_pax_finish_entry
            as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).archive.archive_format = ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE;
    (*a).archive.archive_format_name =
        b"POSIX pax interchange\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_pax_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut pax: *mut pax = (*a).format_data as *mut pax;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /*
         * The character-set we can use are defined in
         * IEEE Std 1003.1-2001
         */
        if val.is_null()
            || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"pax: hdrcharset option needs a character-set name\x00" as *const u8
                    as *const libc::c_char,
            );
        } else if strcmp(val, b"BINARY\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(val, b"binary\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            /*
             * Specify binary mode. We will not convert
             * filenames, uname and gname to any charsets.
             */
            (*pax).opt_binary = 1 as libc::c_int;
            ret = ARCHIVE_OK
        } else if strcmp(val, b"UTF-8\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            /*
             * Specify UTF-8 character-set to be used for
             * filenames. This is almost the test that
             * running platform supports the string conversion.
             * Especially libarchive_test needs this trick for
             * its test.
             */
            (*pax).sconv_utf8 = archive_string_conversion_to_charset(
                &mut (*a).archive,
                b"UTF-8\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            if (*pax).sconv_utf8.is_null() {
                ret = ARCHIVE_FATAL
            } else {
                ret = ARCHIVE_OK
            }
        } else {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"pax: invalid charset name\x00" as *const u8 as *const libc::c_char,
            );
        }
        return ret;
    } else {
        if strcmp(key, b"xattrheader\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            if val.is_null()
                || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"pax: xattrheader requires a value\x00" as *const u8 as *const libc::c_char,
                );
            } else if strcmp(val, b"ALL\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(val, b"all\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            {
                (*pax).flags |= (WRITE_LIBARCHIVE_XATTR | WRITE_SCHILY_XATTR) as libc::c_uint;
                ret = ARCHIVE_OK
            } else if strcmp(val, b"SCHILY\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(val, b"schily\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                (*pax).flags |= WRITE_SCHILY_XATTR as libc::c_uint;
                (*pax).flags &= !WRITE_LIBARCHIVE_XATTR as libc::c_uint;
                ret = ARCHIVE_OK
            } else if strcmp(val, b"LIBARCHIVE\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
                || strcmp(val, b"libarchive\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
            {
                (*pax).flags |= WRITE_LIBARCHIVE_XATTR as libc::c_uint;
                (*pax).flags &= !WRITE_SCHILY_XATTR as libc::c_uint;
                ret = ARCHIVE_OK
            } else {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_MISC,
                    b"pax: invalid xattr header name\x00" as *const u8 as *const libc::c_char,
                );
            }
            return ret;
        }
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
/*
 * Note: This code assumes that 'nanos' has the same sign as 'sec',
 * which implies that sec=-1, nanos=200000000 represents -1.2 seconds
 * and not -0.8 seconds.  This is a pretty pedantic point, as we're
 * unlikely to encounter many real files created before Jan 1, 1970,
 * much less ones with timestamps recorded to sub-second resolution.
 */
unsafe extern "C" fn add_pax_attr_time(
    mut as_0: *mut archive_string,
    mut key: *const libc::c_char,
    mut sec: int64_t,
    mut nanos: libc::c_ulong,
) {
    let mut digit: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    /*
     * Note that each byte contributes fewer than 3 base-10
     * digits, so this will always be big enough.
     */
    let mut tmp: [libc::c_char; 50] = [0; 50];
    tmp[(::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
        0 as libc::c_int as libc::c_char;
    t = tmp
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 50]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    /* Skip trailing zeros in the fractional part. */
    digit = 0 as libc::c_int;
    i = 10 as libc::c_int;
    while i > 0 as libc::c_int && digit == 0 as libc::c_int {
        digit = nanos.wrapping_rem(10 as libc::c_int as libc::c_ulong) as libc::c_int;
        nanos = nanos.wrapping_div(10 as libc::c_int as libc::c_ulong);
        i -= 1
    }
    /* Only format the fraction if it's non-zero. */
    if i > 0 as libc::c_int {
        while i > 0 as libc::c_int {
            t = t.offset(-1);
            *t = (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"0123456789\x00"))
                [digit as usize];
            digit = nanos.wrapping_rem(10 as libc::c_int as libc::c_ulong) as libc::c_int;
            nanos = nanos.wrapping_div(10 as libc::c_int as libc::c_ulong);
            i -= 1
        }
        t = t.offset(-1);
        *t = '.' as i32 as libc::c_char
    }
    t = format_int(t, sec);
    add_pax_attr(as_0, key, t);
}
unsafe extern "C" fn format_int(mut t: *mut libc::c_char, mut i: int64_t) -> *mut libc::c_char {
    let mut ui: uint64_t = 0;
    if i < 0 as libc::c_int as libc::c_long {
        ui = if i == INT64_MIN {
            (9223372036854775807 as libc::c_long as uint64_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            -i as uint64_t
        }
    } else {
        ui = i as uint64_t
    }
    loop {
        t = t.offset(-1);
        *t = (*::std::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"0123456789\x00"))
            [ui.wrapping_rem(10 as libc::c_int as libc::c_ulong) as usize];
        ui = (ui as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong) as uint64_t
            as uint64_t;
        if !(ui != 0) {
            break;
        }
    }
    if i < 0 as libc::c_int as libc::c_long {
        t = t.offset(-1);
        *t = '-' as i32 as libc::c_char
    }
    return t;
}
unsafe extern "C" fn add_pax_attr_int(
    mut as_0: *mut archive_string,
    mut key: *const libc::c_char,
    mut value: int64_t,
) {
    let mut tmp: [libc::c_char; 25] = [0; 25];
    tmp[(::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
        0 as libc::c_int as libc::c_char;
    add_pax_attr(
        as_0,
        key,
        format_int(
            tmp.as_mut_ptr()
                .offset(::std::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as isize)
                .offset(-(1 as libc::c_int as isize)),
            value,
        ),
    );
}
/*
 * Add a key/value attribute to the pax header.  This function handles
 * the length field and various other syntactic requirements.
 */
unsafe extern "C" fn add_pax_attr(
    mut as_0: *mut archive_string,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    add_pax_attr_binary(as_0, key, value, strlen(value));
}
/*
 * Add a key/value attribute to the pax header.  This function handles
 * binary values.
 */
unsafe extern "C" fn add_pax_attr_binary(
    mut as_0: *mut archive_string,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
    mut value_len: size_t,
) {
    let mut digits: libc::c_int = 0; /* < 3 base-10 digits per byte */
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut next_ten: libc::c_int = 0;
    let mut tmp: [libc::c_char; 13] = [0; 13];
    /*-
     * PAX attributes have the following layout:
     *     <len> <space> <key> <=> <value> <nl>
     */
    len = 1 as libc::c_int
        + strlen(key) as libc::c_int
        + 1 as libc::c_int
        + value_len as libc::c_int
        + 1 as libc::c_int;
    /*
     * The <len> field includes the length of the <len> field, so
     * computing the correct length is tricky.  I start by
     * counting the number of base-10 digits in 'len' and
     * computing the next higher power of 10.
     */
    next_ten = 1 as libc::c_int;
    digits = 0 as libc::c_int;
    i = len;
    while i > 0 as libc::c_int {
        i = i / 10 as libc::c_int;
        digits += 1;
        next_ten = next_ten * 10 as libc::c_int
    }
    /*
     * For example, if string without the length field is 99
     * chars, then adding the 2 digit length "99" will force the
     * total length past 100, requiring an extra digit.  The next
     * statement adjusts for this effect.
     */
    if len + digits >= next_ten {
        digits += 1
    }
    /* Now, we have the right length so we can build the line. */
    tmp[(::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] =
        0 as libc::c_int as libc::c_char; /* Null-terminate the work area. */
    archive_strcat(
        as_0,
        format_int(
            tmp.as_mut_ptr()
                .offset(::std::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as isize)
                .offset(-(1 as libc::c_int as isize)),
            (len + digits) as int64_t,
        ) as *const libc::c_void,
    );
    archive_strappend_char(as_0, ' ' as i32 as libc::c_char);
    archive_strcat(as_0, key as *const libc::c_void);
    archive_strappend_char(as_0, '=' as i32 as libc::c_char);
    archive_array_append(as_0, value, value_len);
    archive_strappend_char(as_0, '\n' as i32 as libc::c_char);
}
unsafe extern "C" fn archive_write_pax_header_xattr(
    mut pax: *mut pax,
    mut encoded_name: *const libc::c_char,
    mut value: *const libc::c_void,
    mut value_len: size_t,
) {
    let mut s: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    let mut encoded_value: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*pax).flags & WRITE_LIBARCHIVE_XATTR as libc::c_uint != 0 {
        encoded_value = base64_encode(value as *const libc::c_char, value_len);
        if !encoded_name.is_null() && !encoded_value.is_null() {
            s.s = NULL as *mut libc::c_char;
            s.length = 0 as libc::c_int as size_t;
            s.buffer_length = 0 as libc::c_int as size_t;
            s.length = 0 as libc::c_int as size_t;
            archive_strncat(
                &mut s,
                b"LIBARCHIVE.xattr.\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                (if (b"LIBARCHIVE.xattr.\x00" as *const u8 as *const libc::c_char).is_null() {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    strlen(b"LIBARCHIVE.xattr.\x00" as *const u8 as *const libc::c_char)
                }),
            );
            archive_strcat(&mut s, encoded_name as *const libc::c_void);
            add_pax_attr(&mut (*pax).pax_header, s.s, encoded_value);
            archive_string_free(&mut s);
        }
        free(encoded_value as *mut libc::c_void);
    }
    if (*pax).flags & WRITE_SCHILY_XATTR as libc::c_uint != 0 {
        s.s = NULL as *mut libc::c_char;
        s.length = 0 as libc::c_int as size_t;
        s.buffer_length = 0 as libc::c_int as size_t;
        s.length = 0 as libc::c_int as size_t;
        archive_strncat(
            &mut s,
            b"SCHILY.xattr.\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            (if (b"SCHILY.xattr.\x00" as *const u8 as *const libc::c_char).is_null() {
                0 as libc::c_int as libc::c_ulong
            } else {
                strlen(b"SCHILY.xattr.\x00" as *const u8 as *const libc::c_char)
            }),
        );
        archive_strcat(&mut s, encoded_name as *const libc::c_void);
        add_pax_attr_binary(
            &mut (*pax).pax_header,
            s.s,
            value as *const libc::c_char,
            value_len,
        );
        archive_string_free(&mut s);
    };
}
unsafe extern "C" fn archive_write_pax_header_xattrs(
    mut a: *mut archive_write,
    mut pax: *mut pax,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut i: libc::c_int = archive_entry_xattr_reset(entry);
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *const libc::c_void = 0 as *const libc::c_void;
        let mut url_encoded_name: *mut libc::c_char = NULL as *mut libc::c_char;
        let mut encoded_name: *mut libc::c_char = NULL as *mut libc::c_char;
        let mut size: size_t = 0;
        let mut r: libc::c_int = 0;
        archive_entry_xattr_next(entry, &mut name, &mut value, &mut size);
        url_encoded_name = url_encode(name);
        if !url_encoded_name.is_null() {
            /* Convert narrow-character to UTF-8. */
            r = archive_strncpy_l(
                &mut (*pax).l_url_encoded_name,
                url_encoded_name as *const libc::c_void,
                if url_encoded_name.is_null() {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    strlen(url_encoded_name)
                },
                (*pax).sconv_utf8,
            ); /* Done with this. */
            free(url_encoded_name as *mut libc::c_void);
            if r == 0 as libc::c_int {
                encoded_name = (*pax).l_url_encoded_name.s
            } else if errno == ENOMEM {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Can\'t allocate memory for Linkname\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
        }
        archive_write_pax_header_xattr(pax, encoded_name, value, size);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_entry_hardlink(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
    mut name: *mut *const libc::c_char,
    mut length: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = _archive_entry_hardlink_l(entry, name, length, sc);
    if r != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Linkname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        return -(20 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_entry_pathname(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
    mut name: *mut *const libc::c_char,
    mut length: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = _archive_entry_pathname_l(entry, name, length, sc);
    if r != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Pathname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        return -(20 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_entry_uname(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
    mut name: *mut *const libc::c_char,
    mut length: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = _archive_entry_uname_l(entry, name, length, sc);
    if r != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Uname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        return -(20 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_entry_gname(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
    mut name: *mut *const libc::c_char,
    mut length: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = _archive_entry_gname_l(entry, name, length, sc);
    if r != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Gname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        return -(20 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_entry_symlink(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
    mut name: *mut *const libc::c_char,
    mut length: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = _archive_entry_symlink_l(entry, name, length, sc);
    if r != 0 as libc::c_int {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate memory for Linkname\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        return -(20 as libc::c_int);
    }
    return 0 as libc::c_int;
}
/* Add ACL to pax header */
unsafe extern "C" fn add_pax_acl(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
    mut pax: *mut pax,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut attr: *const libc::c_char = 0 as *const libc::c_char;
    let mut acl_types: libc::c_int = 0;
    acl_types = archive_entry_acl_types(entry);
    if acl_types & ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 as libc::c_int {
        attr = b"SCHILY.acl.ace\x00" as *const u8 as *const libc::c_char
    } else if flags & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 as libc::c_int {
        attr = b"SCHILY.acl.access\x00" as *const u8 as *const libc::c_char
    } else if flags & ARCHIVE_ENTRY_ACL_TYPE_DEFAULT != 0 as libc::c_int {
        attr = b"SCHILY.acl.default\x00" as *const u8 as *const libc::c_char
    } else {
        return -(30 as libc::c_int);
    }
    p = _archive_entry_acl_to_text_l(entry, NULL as *mut ssize_t, flags, (*pax).sconv_utf8);
    if p.is_null() {
        if errno == ENOMEM {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"%s %s\x00" as *const u8 as *const libc::c_char,
                b"Can\'t allocate memory for \x00" as *const u8 as *const libc::c_char,
                attr,
            );
            return -(30 as libc::c_int);
        }
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_FILE_FORMAT,
            b"%s %s %s\x00" as *const u8 as *const libc::c_char,
            b"Can\'t translate \x00" as *const u8 as *const libc::c_char,
            attr,
            b" to UTF-8\x00" as *const u8 as *const libc::c_char,
        );
        return -(20 as libc::c_int);
    }
    if *p as libc::c_int != '\u{0}' as i32 {
        add_pax_attr(&mut (*pax).pax_header, attr, p);
    }
    free(p as *mut libc::c_void);
    return 0 as libc::c_int;
}
/*
 * TODO: Consider adding 'comment' and 'charset' fields to
 * archive_entry so that clients can specify them.  Also, consider
 * adding generic key/value tags so clients can add arbitrary
 * key/value data.
 *
 * TODO: Break up this 700-line function!!!!  Yowza!
 */
unsafe extern "C" fn archive_write_pax_header(
    mut a: *mut archive_write,
    mut entry_original: *mut archive_entry,
) -> libc::c_int {
    let mut entry_main: *mut archive_entry = 0 as *mut archive_entry;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut need_extension: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut acl_types: libc::c_int = 0;
    let mut sparse_count: libc::c_int = 0;
    let mut sparse_total: uint64_t = 0;
    let mut real_size: uint64_t = 0;
    let mut pax: *mut pax = 0 as *mut pax;
    let mut hardlink: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = NULL as *const libc::c_char;
    let mut linkpath: *const libc::c_char = NULL as *const libc::c_char;
    let mut uname: *const libc::c_char = NULL as *const libc::c_char;
    let mut gname: *const libc::c_char = NULL as *const libc::c_char;
    let mut mac_metadata: *const libc::c_void = 0 as *const libc::c_void;
    let mut mac_metadata_size: size_t = 0;
    let mut sconv: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut hardlink_length: size_t = 0;
    let mut path_length: size_t = 0;
    let mut linkpath_length: size_t = 0;
    let mut uname_length: size_t = 0;
    let mut gname_length: size_t = 0;
    let mut paxbuff: [libc::c_char; 512] = [0; 512];
    let mut ustarbuff: [libc::c_char; 512] = [0; 512];
    let mut ustar_entry_name: [libc::c_char; 256] = [0; 256];
    let mut pax_entry_name: [libc::c_char; 256] = [0; 256];
    let mut gnu_sparse_name: [libc::c_char; 256] = [0; 256];
    let mut entry_name: archive_string = archive_string {
        s: 0 as *mut libc::c_char,
        length: 0,
        buffer_length: 0,
    };
    ret = ARCHIVE_OK;
    need_extension = 0 as libc::c_int;
    pax = (*a).format_data as *mut pax;
    /* Sanity check. */
    if archive_entry_pathname(entry_original).is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Can\'t record entry in tar file without pathname\x00" as *const u8
                as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /*
     * Choose a header encoding.
     */
    if (*pax).opt_binary != 0 {
        sconv = NULL as *mut archive_string_conv
    } else {
        /* Binary mode. */
        /* Header encoding is UTF-8. */
        if (*pax).sconv_utf8.is_null() {
            /* Initialize the string conversion object
             * we must need */
            (*pax).sconv_utf8 = archive_string_conversion_to_charset(
                &mut (*a).archive,
                b"UTF-8\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*pax).sconv_utf8.is_null() {
                /* Couldn't allocate memory */
                return -(25 as libc::c_int);
            }
        }
        sconv = (*pax).sconv_utf8
    }
    r = get_entry_hardlink(
        a,
        entry_original,
        &mut hardlink,
        &mut hardlink_length,
        sconv,
    );
    if r == ARCHIVE_FATAL {
        return r;
    } else {
        if r != ARCHIVE_OK {
            r = get_entry_hardlink(
                a,
                entry_original,
                &mut hardlink,
                &mut hardlink_length,
                NULL as *mut archive_string_conv,
            );
            if r == ARCHIVE_FATAL {
                return r;
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Can\'t translate linkname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
                hardlink,
                archive_string_conversion_charset_name(sconv),
            );
            ret = ARCHIVE_WARN;
            sconv = NULL as *mut archive_string_conv
            /* The header charset switches to binary mode. */
        }
    }
    /* Make sure this is a type of entry that we can handle here */
    if hardlink.is_null() {
        match archive_entry_filetype(entry_original) {
            24576 | 8192 | 4096 | 40960 | 32768 => {}
            16384 => {
                /*
                 * Ensure a trailing '/'.  Modify the original
                 * entry so the client sees the change.
                 */
                p = archive_entry_pathname(entry_original);
                /*
                 * On Windows, this is a backup operation just in
                 * case getting WCS failed. On POSIX, this is a
                 * normal operation.
                 */
                if !p.is_null()
                    && *p.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
                    && *p.offset(strlen(p).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        != '/' as i32
                {
                    let mut as_0: archive_string = archive_string {
                        s: 0 as *mut libc::c_char,
                        length: 0,
                        buffer_length: 0,
                    };
                    as_0.s = NULL as *mut libc::c_char;
                    as_0.length = 0 as libc::c_int as size_t;
                    as_0.buffer_length = 0 as libc::c_int as size_t;
                    path_length = strlen(p);
                    if archive_string_ensure(
                        &mut as_0,
                        path_length.wrapping_add(2 as libc::c_int as libc::c_ulong),
                    )
                    .is_null()
                    {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ENOMEM,
                            b"Can\'t allocate pax data\x00" as *const u8 as *const libc::c_char,
                        );
                        archive_string_free(&mut as_0);
                        return -(30 as libc::c_int);
                    }
                    as_0.length = 0 as libc::c_int as size_t;
                    archive_strncat(&mut as_0, p as *const libc::c_void, path_length);
                    archive_strappend_char(&mut as_0, '/' as i32 as libc::c_char);
                    archive_entry_copy_pathname(entry_original, as_0.s);
                    archive_string_free(&mut as_0);
                }
            }
            _ => {
                /* AE_IFSOCK and unknown */
                __archive_write_entry_filetype_unsupported(
                    &mut (*a).archive,
                    entry_original,
                    b"pax\x00" as *const u8 as *const libc::c_char,
                );
                return -(25 as libc::c_int);
            }
        }
    }
    /*
     * If Mac OS metadata blob is here, recurse to write that
     * as a separate entry.  This is really a pretty poor design:
     * In particular, it doubles the overhead for long filenames.
     * TODO: Help Apple folks design something better and figure
     * out how to transition from this legacy format.
     *
     * Note that this code is present on every platform; clients
     * on non-Mac are unlikely to ever provide this data, but
     * applications that copy entries from one archive to another
     * should not lose data just because the local filesystem
     * can't store it.
     */
    mac_metadata = archive_entry_mac_metadata(entry_original, &mut mac_metadata_size);
    if mac_metadata != NULL as *const libc::c_void {
        let mut oname: *const libc::c_char = 0 as *const libc::c_char;
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut bname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut name_length: size_t = 0;
        let mut extra: *mut archive_entry = archive_entry_new2(&mut (*a).archive);
        oname = archive_entry_pathname(entry_original);
        name_length = strlen(oname);
        name = malloc(name_length.wrapping_add(3 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if name.is_null() || extra.is_null() {
            /* XXX error message */
            archive_entry_free(extra);
            free(name as *mut libc::c_void);
            return -(25 as libc::c_int);
        }
        strcpy(name, oname);
        /* Find last '/'; strip trailing '/' characters */
        bname = strrchr(name, '/' as i32);
        while !bname.is_null()
            && *bname.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        {
            *bname = '\u{0}' as i32 as libc::c_char;
            bname = strrchr(name, '/' as i32)
        }
        if bname.is_null() {
            memmove(
                name.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                name as *const libc::c_void,
                name_length.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            memmove(
                name as *mut libc::c_void,
                b"._\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
        } else {
            bname = bname.offset(1 as libc::c_int as isize);
            memmove(
                bname.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                bname as *const libc::c_void,
                strlen(bname).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            memmove(
                bname as *mut libc::c_void,
                b"._\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            );
        }
        archive_entry_copy_pathname(extra, name);
        free(name as *mut libc::c_void);
        archive_entry_set_size(extra, mac_metadata_size as la_int64_t);
        archive_entry_set_filetype(extra, AE_IFREG as mode_t);
        archive_entry_set_perm(extra, archive_entry_perm(entry_original));
        archive_entry_set_mtime(
            extra,
            archive_entry_mtime(entry_original),
            archive_entry_mtime_nsec(entry_original),
        );
        archive_entry_set_gid(extra, archive_entry_gid(entry_original));
        archive_entry_set_gname(extra, archive_entry_gname(entry_original));
        archive_entry_set_uid(extra, archive_entry_uid(entry_original));
        archive_entry_set_uname(extra, archive_entry_uname(entry_original));
        /* Recurse to write the special copyfile entry. */
        r = archive_write_pax_header(a, extra);
        archive_entry_free(extra);
        if r < ARCHIVE_WARN {
            return r;
        }
        if r < ret {
            ret = r
        }
        r = archive_write_pax_data(a, mac_metadata, mac_metadata_size) as libc::c_int;
        if r < ARCHIVE_WARN {
            return r;
        }
        if r < ret {
            ret = r
        }
        r = archive_write_pax_finish_entry(a);
        if r < ARCHIVE_WARN {
            return r;
        }
        if r < ret {
            ret = r
        }
    }
    /* Copy entry so we can modify it as needed. */
    entry_main = archive_entry_clone(entry_original); /* Blank our work area. */
    if entry_main.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate pax data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*pax).pax_header.length = 0 as libc::c_int as size_t;
    (*pax).sparse_map.length = 0 as libc::c_int as size_t;
    sparse_total = 0 as libc::c_int as uint64_t;
    sparse_list_clear(pax);
    if hardlink.is_null() && archive_entry_filetype(entry_main) == AE_IFREG as mode_t {
        sparse_count = archive_entry_sparse_reset(entry_main)
    } else {
        sparse_count = 0 as libc::c_int
    }
    if sparse_count != 0 {
        let mut offset: int64_t = 0;
        let mut length: int64_t = 0;
        let mut last_offset: int64_t = 0 as libc::c_int as int64_t;
        /* Get the last entry of sparse block. */
        while archive_entry_sparse_next(entry_main, &mut offset, &mut length) == ARCHIVE_OK {
            last_offset = offset + length
        }
        /* If the last sparse block does not reach the end of file,
         * We have to add a empty sparse block as the last entry to
         * manage storing file data. */
        if last_offset < archive_entry_size(entry_main) {
            archive_entry_sparse_add_entry(
                entry_main,
                archive_entry_size(entry_main),
                0 as libc::c_int as la_int64_t,
            );
        }
        sparse_count = archive_entry_sparse_reset(entry_main)
    }
    /*
     * First, check the name fields and see if any of them
     * require binary coding.  If any of them does, then all of
     * them do.
     */
    r = get_entry_pathname(a, entry_main, &mut path, &mut path_length, sconv);
    if r == ARCHIVE_FATAL {
        archive_entry_free(entry_main);
        return r;
    } else {
        if r != ARCHIVE_OK {
            r = get_entry_pathname(
                a,
                entry_main,
                &mut path,
                &mut path_length,
                NULL as *mut archive_string_conv,
            );
            if r == ARCHIVE_FATAL {
                archive_entry_free(entry_main);
                return r;
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Can\'t translate pathname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
                path,
                archive_string_conversion_charset_name(sconv),
            );
            ret = ARCHIVE_WARN;
            sconv = NULL as *mut archive_string_conv
            /* The header charset switches to binary mode. */
        }
    }
    r = get_entry_uname(a, entry_main, &mut uname, &mut uname_length, sconv);
    if r == ARCHIVE_FATAL {
        archive_entry_free(entry_main);
        return r;
    } else {
        if r != ARCHIVE_OK {
            r = get_entry_uname(
                a,
                entry_main,
                &mut uname,
                &mut uname_length,
                NULL as *mut archive_string_conv,
            );
            if r == ARCHIVE_FATAL {
                archive_entry_free(entry_main);
                return r;
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Can\'t translate uname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
                uname,
                archive_string_conversion_charset_name(sconv),
            );
            ret = ARCHIVE_WARN;
            sconv = NULL as *mut archive_string_conv
            /* The header charset switches to binary mode. */
        }
    }
    r = get_entry_gname(a, entry_main, &mut gname, &mut gname_length, sconv);
    if r == ARCHIVE_FATAL {
        archive_entry_free(entry_main);
        return r;
    } else {
        if r != ARCHIVE_OK {
            r = get_entry_gname(
                a,
                entry_main,
                &mut gname,
                &mut gname_length,
                NULL as *mut archive_string_conv,
            );
            if r == ARCHIVE_FATAL {
                archive_entry_free(entry_main);
                return r;
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Can\'t translate gname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
                gname,
                archive_string_conversion_charset_name(sconv),
            );
            ret = ARCHIVE_WARN;
            sconv = NULL as *mut archive_string_conv
            /* The header charset switches to binary mode. */
        }
    }
    linkpath = hardlink;
    linkpath_length = hardlink_length;
    if linkpath.is_null() {
        r = get_entry_symlink(a, entry_main, &mut linkpath, &mut linkpath_length, sconv);
        if r == ARCHIVE_FATAL {
            archive_entry_free(entry_main);
            return r;
        } else {
            if r != ARCHIVE_OK {
                r = get_entry_symlink(
                    a,
                    entry_main,
                    &mut linkpath,
                    &mut linkpath_length,
                    NULL as *mut archive_string_conv,
                );
                if r == ARCHIVE_FATAL {
                    archive_entry_free(entry_main);
                    return r;
                }
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ARCHIVE_ERRNO_FILE_FORMAT,
                    b"Can\'t translate linkname \'%s\' to %s\x00" as *const u8
                        as *const libc::c_char,
                    linkpath,
                    archive_string_conversion_charset_name(sconv),
                );
                ret = ARCHIVE_WARN;
                sconv = NULL as *mut archive_string_conv
            }
        }
    }
    /* If any string conversions failed, get all attributes
     * in binary-mode. */
    if sconv.is_null() && (*pax).opt_binary == 0 {
        if !hardlink.is_null() {
            r = get_entry_hardlink(
                a,
                entry_main,
                &mut hardlink,
                &mut hardlink_length,
                NULL as *mut archive_string_conv,
            );
            if r == ARCHIVE_FATAL {
                archive_entry_free(entry_main);
                return r;
            }
            linkpath = hardlink;
            linkpath_length = hardlink_length
        }
        r = get_entry_pathname(
            a,
            entry_main,
            &mut path,
            &mut path_length,
            NULL as *mut archive_string_conv,
        );
        if r == ARCHIVE_FATAL {
            archive_entry_free(entry_main);
            return r;
        }
        r = get_entry_uname(
            a,
            entry_main,
            &mut uname,
            &mut uname_length,
            NULL as *mut archive_string_conv,
        );
        if r == ARCHIVE_FATAL {
            archive_entry_free(entry_main);
            return r;
        }
        r = get_entry_gname(
            a,
            entry_main,
            &mut gname,
            &mut gname_length,
            NULL as *mut archive_string_conv,
        );
        if r == ARCHIVE_FATAL {
            archive_entry_free(entry_main);
            return r;
        }
    }
    /* Store the header encoding first, to be nice to readers. */
    if sconv.is_null() {
        add_pax_attr(
            &mut (*pax).pax_header,
            b"hdrcharset\x00" as *const u8 as *const libc::c_char,
            b"BINARY\x00" as *const u8 as *const libc::c_char,
        );
    }
    /*
     * If name is too long, or has non-ASCII characters, add
     * 'path' to pax extended attrs.  (Note that an unconvertible
     * name must have non-ASCII characters.)
     */
    if has_non_ASCII(path) != 0 {
        /* We have non-ASCII characters. */
        add_pax_attr(
            &mut (*pax).pax_header,
            b"path\x00" as *const u8 as *const libc::c_char,
            path,
        );
        archive_entry_set_pathname(
            entry_main,
            build_ustar_entry_name(
                ustar_entry_name.as_mut_ptr(),
                path,
                path_length,
                NULL as *const libc::c_char,
            ),
        );
        need_extension = 1 as libc::c_int
    } else if !(path_length <= 100 as libc::c_int as libc::c_ulong) {
        /* We have an all-ASCII path; we'd like to just store
         * it in the ustar header if it will fit.  Yes, this
         * duplicates some of the logic in
         * archive_write_set_format_ustar.c
         */
        /* Find largest suffix that will fit. */
        /* Note: strlen() > 100, so strlen() - 100 - 1 >= 0 */
        suffix = strchr(
            path.offset(path_length as isize)
                .offset(-(100 as libc::c_int as isize))
                .offset(-(1 as libc::c_int as isize)),
            '/' as i32,
        );
        /* Don't attempt an empty prefix. */
        if suffix == path {
            suffix = strchr(suffix.offset(1 as libc::c_int as isize), '/' as i32)
        }
        /* We can put it in the ustar header if it's
         * all ASCII and it's either <= 100 characters
         * or can be split at a '/' into a prefix <=
         * 155 chars and a suffix <= 100 chars.  (Note
         * the strchr() above will return NULL exactly
         * when the path can't be split.)
         */
        if suffix.is_null()
            || *suffix.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || suffix.offset_from(path) as libc::c_long
                > 155 as libc::c_int as libc::c_long
        {
            /* Prefix > 155 chars */
            add_pax_attr(
                &mut (*pax).pax_header,
                b"path\x00" as *const u8 as *const libc::c_char,
                path,
            );
            archive_entry_set_pathname(
                entry_main,
                build_ustar_entry_name(
                    ustar_entry_name.as_mut_ptr(),
                    path,
                    path_length,
                    NULL as *const libc::c_char,
                ),
            );
            need_extension = 1 as libc::c_int
        }
    }
    if !linkpath.is_null() {
        /* If link name is too long or has non-ASCII characters, add
         * 'linkpath' to pax extended attrs. */
        if linkpath_length > 100 as libc::c_int as libc::c_ulong || has_non_ASCII(linkpath) != 0 {
            add_pax_attr(
                &mut (*pax).pax_header,
                b"linkpath\x00" as *const u8 as *const libc::c_char,
                linkpath,
            );
            if linkpath_length > 100 as libc::c_int as libc::c_ulong {
                if !hardlink.is_null() {
                    archive_entry_set_hardlink(
                        entry_main,
                        b"././@LongHardLink\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                    archive_entry_set_symlink(
                        entry_main,
                        b"././@LongSymLink\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
            need_extension = 1 as libc::c_int
        }
    }
    /* Save a pathname since it will be renamed if `entry_main` has
     * sparse blocks. */
    entry_name.s = NULL as *mut libc::c_char;
    entry_name.length = 0 as libc::c_int as size_t;
    entry_name.buffer_length = 0 as libc::c_int as size_t;
    entry_name.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut entry_name,
        archive_entry_pathname(entry_main) as *const libc::c_void,
        (if archive_entry_pathname(entry_main).is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(archive_entry_pathname(entry_main))
        }),
    );
    /* If file size is too large, add 'size' to pax extended attrs. */
    if archive_entry_size(entry_main) >= (1 as libc::c_int as int64_t) << 33 as libc::c_int {
        add_pax_attr_int(
            &mut (*pax).pax_header,
            b"size\x00" as *const u8 as *const libc::c_char,
            archive_entry_size(entry_main),
        );
        need_extension = 1 as libc::c_int
    }
    /* If numeric GID is too large, add 'gid' to pax extended attrs. */
    if archive_entry_gid(entry_main) as libc::c_uint
        >= ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_uint
    {
        add_pax_attr_int(
            &mut (*pax).pax_header,
            b"gid\x00" as *const u8 as *const libc::c_char,
            archive_entry_gid(entry_main),
        );
        need_extension = 1 as libc::c_int
    }
    /* If group name is too large or has non-ASCII characters, add
     * 'gname' to pax extended attrs. */
    if !gname.is_null() {
        if gname_length > 31 as libc::c_int as libc::c_ulong || has_non_ASCII(gname) != 0 {
            add_pax_attr(
                &mut (*pax).pax_header,
                b"gname\x00" as *const u8 as *const libc::c_char,
                gname,
            );
            need_extension = 1 as libc::c_int
        }
    }
    /* If numeric UID is too large, add 'uid' to pax extended attrs. */
    if archive_entry_uid(entry_main) as libc::c_uint
        >= ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_uint
    {
        add_pax_attr_int(
            &mut (*pax).pax_header,
            b"uid\x00" as *const u8 as *const libc::c_char,
            archive_entry_uid(entry_main),
        );
        need_extension = 1 as libc::c_int
    }
    /* Add 'uname' to pax extended attrs if necessary. */
    if !uname.is_null() {
        if uname_length > 31 as libc::c_int as libc::c_ulong || has_non_ASCII(uname) != 0 {
            add_pax_attr(
                &mut (*pax).pax_header,
                b"uname\x00" as *const u8 as *const libc::c_char,
                uname,
            );
            need_extension = 1 as libc::c_int
        }
    }
    /*
     * POSIX/SUSv3 doesn't provide a standard key for large device
     * numbers.  I use the same keys here that Joerg Schilling
     * used for 'star.'  (Which, somewhat confusingly, are called
     * "devXXX" even though they code "rdev" values.)  No doubt,
     * other implementations use other keys.  Note that there's no
     * reason we can't write the same information into a number of
     * different keys.
     *
     * Of course, this is only needed for block or char device entries.
     */
    if archive_entry_filetype(entry_main) == AE_IFBLK as mode_t
        || archive_entry_filetype(entry_main) == AE_IFCHR as mode_t
    {
        /*
         * If rdevmajor is too large, add 'SCHILY.devmajor' to
         * extended attributes.
         */
        let mut rdevmajor: libc::c_int = 0;
        let mut rdevminor: libc::c_int = 0;
        rdevmajor = archive_entry_rdevmajor(entry_main) as libc::c_int;
        rdevminor = archive_entry_rdevminor(entry_main) as libc::c_int;
        if rdevmajor >= (1 as libc::c_int) << 18 as libc::c_int {
            add_pax_attr_int(
                &mut (*pax).pax_header,
                b"SCHILY.devmajor\x00" as *const u8 as *const libc::c_char,
                rdevmajor as int64_t,
            );
            /*
             * Non-strict formatting below means we don't
             * have to truncate here.  Not truncating improves
             * the chance that some more modern tar archivers
             * (such as GNU tar 1.13) can restore the full
             * value even if they don't understand the pax
             * extended attributes.  See my rant below about
             * file size fields for additional details.
             */
            /* archive_entry_set_rdevmajor(entry_main,
            rdevmajor & ((1 << 18) - 1)); */
            need_extension = 1 as libc::c_int
        }
        /*
         * If devminor is too large, add 'SCHILY.devminor' to
         * extended attributes.
         */
        if rdevminor >= (1 as libc::c_int) << 18 as libc::c_int {
            add_pax_attr_int(
                &mut (*pax).pax_header,
                b"SCHILY.devminor\x00" as *const u8 as *const libc::c_char,
                rdevminor as int64_t,
            );
            /* Truncation is not necessary here, either. */
            /* archive_entry_set_rdevminor(entry_main,
            rdevminor & ((1 << 18) - 1)); */
            need_extension = 1 as libc::c_int
        }
    }
    /*
     * Technically, the mtime field in the ustar header can
     * support 33 bits, but many platforms use signed 32-bit time
     * values.  The cutoff of 0x7fffffff here is a compromise.
     * Yes, this check is duplicated just below; this helps to
     * avoid writing an mtime attribute just to handle a
     * high-resolution timestamp in "restricted pax" mode.
     */
    if need_extension == 0
        && (archive_entry_mtime(entry_main) < 0 as libc::c_int as libc::c_long
            || archive_entry_mtime(entry_main) >= 0x7fffffff as libc::c_int as libc::c_long)
    {
        need_extension = 1 as libc::c_int
    }
    /* I use a star-compatible file flag attribute. */
    p = archive_entry_fflags_text(entry_main);
    if need_extension == 0 && !p.is_null() && *p as libc::c_int != '\u{0}' as i32 {
        need_extension = 1 as libc::c_int
    }
    /* If there are extended attributes, we need an extension */
    if need_extension == 0 && archive_entry_xattr_count(entry_original) > 0 as libc::c_int {
        need_extension = 1 as libc::c_int
    }
    /* If there are sparse info, we need an extension */
    if need_extension == 0 && sparse_count > 0 as libc::c_int {
        need_extension = 1 as libc::c_int
    }
    acl_types = archive_entry_acl_types(entry_original);
    /* If there are any ACL entries, we need an extension */
    if need_extension == 0 && acl_types != 0 as libc::c_int {
        need_extension = 1 as libc::c_int
    }
    /* If the symlink type is defined, we need an extension */
    if need_extension == 0 && archive_entry_symlink_type(entry_main) > 0 as libc::c_int {
        need_extension = 1 as libc::c_int
    }
    /*
     * Libarchive used to include these in extended headers for
     * restricted pax format, but that confused people who
     * expected ustar-like time semantics.  So now we only include
     * them in full pax format.
     */
    if (*a).archive.archive_format != ARCHIVE_FORMAT_TAR_PAX_RESTRICTED {
        if archive_entry_ctime(entry_main) != 0 as libc::c_int as libc::c_long
            || archive_entry_ctime_nsec(entry_main) != 0 as libc::c_int as libc::c_long
        {
            add_pax_attr_time(
                &mut (*pax).pax_header,
                b"ctime\x00" as *const u8 as *const libc::c_char,
                archive_entry_ctime(entry_main),
                archive_entry_ctime_nsec(entry_main) as libc::c_ulong,
            );
        }
        if archive_entry_atime(entry_main) != 0 as libc::c_int as libc::c_long
            || archive_entry_atime_nsec(entry_main) != 0 as libc::c_int as libc::c_long
        {
            add_pax_attr_time(
                &mut (*pax).pax_header,
                b"atime\x00" as *const u8 as *const libc::c_char,
                archive_entry_atime(entry_main),
                archive_entry_atime_nsec(entry_main) as libc::c_ulong,
            );
        }
        /* Store birth/creationtime only if it's earlier than mtime */
        if archive_entry_birthtime_is_set(entry_main) != 0
            && archive_entry_birthtime(entry_main) < archive_entry_mtime(entry_main)
        {
            add_pax_attr_time(
                &mut (*pax).pax_header,
                b"LIBARCHIVE.creationtime\x00" as *const u8 as *const libc::c_char,
                archive_entry_birthtime(entry_main),
                archive_entry_birthtime_nsec(entry_main) as libc::c_ulong,
            );
        }
    }
    /*
     * The following items are handled differently in "pax
     * restricted" format.  In particular, in "pax restricted"
     * format they won't be added unless need_extension is
     * already set (we're already generating an extended header, so
     * may as well include these).
     */
    if (*a).archive.archive_format != ARCHIVE_FORMAT_TAR_PAX_RESTRICTED || need_extension != 0 {
        if archive_entry_mtime(entry_main) < 0 as libc::c_int as libc::c_long
            || archive_entry_mtime(entry_main) >= 0x7fffffff as libc::c_int as libc::c_long
            || archive_entry_mtime_nsec(entry_main) != 0 as libc::c_int as libc::c_long
        {
            add_pax_attr_time(
                &mut (*pax).pax_header,
                b"mtime\x00" as *const u8 as *const libc::c_char,
                archive_entry_mtime(entry_main),
                archive_entry_mtime_nsec(entry_main) as libc::c_ulong,
            );
        }
        /* I use a star-compatible file flag attribute. */
        p = archive_entry_fflags_text(entry_main);
        if !p.is_null() && *p as libc::c_int != '\u{0}' as i32 {
            add_pax_attr(
                &mut (*pax).pax_header,
                b"SCHILY.fflags\x00" as *const u8 as *const libc::c_char,
                p,
            );
        }
        /* I use star-compatible ACL attributes. */
        if acl_types & ARCHIVE_ENTRY_ACL_TYPE_NFS4 != 0 as libc::c_int {
            ret = add_pax_acl(
                a,
                entry_original,
                pax,
                ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID
                    | ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA
                    | ARCHIVE_ENTRY_ACL_STYLE_COMPACT,
            );
            if ret == ARCHIVE_FATAL {
                archive_entry_free(entry_main);
                archive_string_free(&mut entry_name);
                return -(30 as libc::c_int);
            }
        }
        if acl_types & ARCHIVE_ENTRY_ACL_TYPE_ACCESS != 0 {
            ret = add_pax_acl(
                a,
                entry_original,
                pax,
                ARCHIVE_ENTRY_ACL_TYPE_ACCESS
                    | ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID
                    | ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA,
            );
            if ret == ARCHIVE_FATAL {
                archive_entry_free(entry_main);
                archive_string_free(&mut entry_name);
                return -(30 as libc::c_int);
            }
        }
        if acl_types & ARCHIVE_ENTRY_ACL_TYPE_DEFAULT != 0 {
            ret = add_pax_acl(
                a,
                entry_original,
                pax,
                ARCHIVE_ENTRY_ACL_TYPE_DEFAULT
                    | ARCHIVE_ENTRY_ACL_STYLE_EXTRA_ID
                    | ARCHIVE_ENTRY_ACL_STYLE_SEPARATOR_COMMA,
            );
            if ret == ARCHIVE_FATAL {
                archive_entry_free(entry_main);
                archive_string_free(&mut entry_name);
                return -(30 as libc::c_int);
            }
        }
        /* We use GNU-tar-compatible sparse attributes. */
        if sparse_count > 0 as libc::c_int {
            let mut soffset: int64_t = 0;
            let mut slength: int64_t = 0;
            add_pax_attr_int(
                &mut (*pax).pax_header,
                b"GNU.sparse.major\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int as int64_t,
            );
            add_pax_attr_int(
                &mut (*pax).pax_header,
                b"GNU.sparse.minor\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int as int64_t,
            );
            /*
             * Make sure to store the original path, since
             * truncation to ustar limit happened already.
             */
            add_pax_attr(
                &mut (*pax).pax_header,
                b"GNU.sparse.name\x00" as *const u8 as *const libc::c_char,
                path,
            );
            add_pax_attr_int(
                &mut (*pax).pax_header,
                b"GNU.sparse.realsize\x00" as *const u8 as *const libc::c_char,
                archive_entry_size(entry_main),
            );
            /* Rename the file name which will be used for
             * ustar header to a special name, which GNU
             * PAX Format 1.0 requires */
            archive_entry_set_pathname(
                entry_main,
                build_gnu_sparse_name(gnu_sparse_name.as_mut_ptr(), entry_name.s),
            );
            /*
             * - Make a sparse map, which will precede a file data.
             * - Get the total size of available data of sparse.
             */
            archive_string_sprintf(
                &mut (*pax).sparse_map as *mut archive_string,
                b"%d\n\x00" as *const u8 as *const libc::c_char,
                sparse_count,
            );
            while archive_entry_sparse_next(entry_main, &mut soffset, &mut slength) == ARCHIVE_OK {
                archive_string_sprintf(
                    &mut (*pax).sparse_map as *mut archive_string,
                    b"%jd\n%jd\n\x00" as *const u8 as *const libc::c_char,
                    soffset,
                    slength,
                );
                sparse_total = (sparse_total as libc::c_ulong)
                    .wrapping_add(slength as libc::c_ulong)
                    as uint64_t as uint64_t;
                if sparse_list_add(pax, soffset, slength) != ARCHIVE_OK {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"Can\'t allocate memory\x00" as *const u8 as *const libc::c_char,
                    );
                    archive_entry_free(entry_main);
                    archive_string_free(&mut entry_name);
                    return -(30 as libc::c_int);
                }
            }
        }
        /* Store extended attributes */
        if archive_write_pax_header_xattrs(a, pax, entry_original) == ARCHIVE_FATAL {
            archive_entry_free(entry_main);
            archive_string_free(&mut entry_name);
            return -(30 as libc::c_int);
        }
        /* Store extended symlink information */
        if archive_entry_symlink_type(entry_main) == AE_SYMLINK_TYPE_FILE {
            add_pax_attr(
                &mut (*pax).pax_header,
                b"LIBARCHIVE.symlinktype\x00" as *const u8 as *const libc::c_char,
                b"file\x00" as *const u8 as *const libc::c_char,
            );
        } else if archive_entry_symlink_type(entry_main) == AE_SYMLINK_TYPE_DIRECTORY {
            add_pax_attr(
                &mut (*pax).pax_header,
                b"LIBARCHIVE.symlinktype\x00" as *const u8 as *const libc::c_char,
                b"dir\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    /* Only regular files have data. */
    if archive_entry_filetype(entry_main) != AE_IFREG as mode_t {
        archive_entry_set_size(entry_main, 0 as libc::c_int as la_int64_t);
    }
    /*
     * Pax-restricted does not store data for hardlinks, in order
     * to improve compatibility with ustar.
     */
    if (*a).archive.archive_format != ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE && !hardlink.is_null() {
        archive_entry_set_size(entry_main, 0 as libc::c_int as la_int64_t);
    }
    /*
     * XXX Full pax interchange format does permit a hardlink
     * entry to have data associated with it.  I'm not supporting
     * that here because the client expects me to tell them whether
     * or not this format expects data for hardlinks.  If I
     * don't check here, then every pax archive will end up with
     * duplicated data for hardlinks.  Someday, there may be
     * need to select this behavior, in which case the following
     * will need to be revisited. XXX
     */
    if !hardlink.is_null() {
        archive_entry_set_size(entry_main, 0 as libc::c_int as la_int64_t);
    }
    /* Save a real file size. */
    real_size = archive_entry_size(entry_main) as uint64_t;
    /*
     * Overwrite a file size by the total size of sparse blocks and
     * the size of sparse map info. That file size is the length of
     * the data, which we will exactly store into an archive file.
     */
    if (*pax).sparse_map.length != 0 {
        let mut mapsize: size_t = (*pax).sparse_map.length;
        (*pax).sparse_map_padding =
            (0x1ff as libc::c_int as libc::c_long & -(mapsize as ssize_t)) as size_t;
        archive_entry_set_size(
            entry_main,
            mapsize
                .wrapping_add((*pax).sparse_map_padding)
                .wrapping_add(sparse_total) as la_int64_t,
        );
    }
    /* Format 'ustar' header for main entry.
     *
     * The trouble with file size: If the reader can't understand
     * the file size, they may not be able to locate the next
     * entry and the rest of the archive is toast.  Pax-compliant
     * readers are supposed to ignore the file size in the main
     * header, so the question becomes how to maximize portability
     * for readers that don't support pax attribute extensions.
     * For maximum compatibility, I permit numeric extensions in
     * the main header so that the file size stored will always be
     * correct, even if it's in a format that only some
     * implementations understand.  The technique used here is:
     *
     *  a) If possible, follow the standard exactly.  This handles
     *  files up to 8 gigabytes minus 1.
     *
     *  b) If that fails, try octal but omit the field terminator.
     *  That handles files up to 64 gigabytes minus 1.
     *
     *  c) Otherwise, use base-256 extensions.  That handles files
     *  up to 2^63 in this implementation, with the potential to
     *  go up to 2^94.  That should hold us for a while. ;-)
     *
     * The non-strict formatter uses similar logic for other
     * numeric fields, though they're less critical.
     */
    if __archive_write_format_header_ustar(
        a,
        ustarbuff.as_mut_ptr(),
        entry_main,
        -(1 as libc::c_int),
        0 as libc::c_int,
        NULL as *mut archive_string_conv,
    ) == ARCHIVE_FATAL
    {
        archive_entry_free(entry_main);
        archive_string_free(&mut entry_name);
        return -(30 as libc::c_int);
    }
    /* If we built any extended attributes, write that entry first. */
    if (*pax).pax_header.length > 0 as libc::c_int as libc::c_ulong {
        let mut pax_attr_entry: *mut archive_entry = 0 as *mut archive_entry;
        let mut s: time_t = 0;
        let mut uid: int64_t = 0;
        let mut gid: int64_t = 0;
        let mut mode: libc::c_int = 0;
        pax_attr_entry = archive_entry_new2(&mut (*a).archive);
        p = entry_name.s;
        archive_entry_set_pathname(
            pax_attr_entry,
            build_pax_attribute_name(pax_entry_name.as_mut_ptr(), p),
        );
        archive_entry_set_size(pax_attr_entry, (*pax).pax_header.length as la_int64_t);
        /* Copy uid/gid (but clip to ustar limits). */
        uid = archive_entry_uid(entry_main);
        if uid >= ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_long {
            uid = (((1 as libc::c_int) << 18 as libc::c_int) - 1 as libc::c_int) as int64_t
        }
        archive_entry_set_uid(pax_attr_entry, uid);
        gid = archive_entry_gid(entry_main);
        if gid >= ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_long {
            gid = (((1 as libc::c_int) << 18 as libc::c_int) - 1 as libc::c_int) as int64_t
        }
        archive_entry_set_gid(pax_attr_entry, gid);
        /* Copy mode over (but not setuid/setgid bits) */
        mode = archive_entry_mode(entry_main) as libc::c_int;
        mode &= !S_ISUID;
        mode &= !S_ISGID;
        mode &= !S_ISVTX;
        archive_entry_set_mode(pax_attr_entry, mode as mode_t);
        /* Copy uname/gname. */
        archive_entry_set_uname(pax_attr_entry, archive_entry_uname(entry_main));
        archive_entry_set_gname(pax_attr_entry, archive_entry_gname(entry_main));
        /* Copy mtime, but clip to ustar limits. */
        s = archive_entry_mtime(entry_main);
        if s < 0 as libc::c_int as libc::c_long {
            s = 0 as libc::c_int as time_t
        }
        if s >= 0x7fffffff as libc::c_int as libc::c_long {
            s = 0x7fffffff as libc::c_int as time_t
        }
        archive_entry_set_mtime(pax_attr_entry, s, 0 as libc::c_int as libc::c_long);
        /* Standard ustar doesn't support atime. */
        archive_entry_set_atime(
            pax_attr_entry,
            0 as libc::c_int as time_t,
            0 as libc::c_int as libc::c_long,
        );
        /* Standard ustar doesn't support ctime. */
        archive_entry_set_ctime(
            pax_attr_entry,
            0 as libc::c_int as time_t,
            0 as libc::c_int as libc::c_long,
        );
        r = __archive_write_format_header_ustar(
            a,
            paxbuff.as_mut_ptr(),
            pax_attr_entry,
            'x' as i32,
            1 as libc::c_int,
            NULL as *mut archive_string_conv,
        );
        archive_entry_free(pax_attr_entry);
        /* Note that the 'x' header shouldn't ever fail to format */
        if r < ARCHIVE_WARN {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"archive_write_pax_header: \'x\' header failed?!  This can\'t happen.\n\x00"
                    as *const u8 as *const libc::c_char,
            );
            archive_entry_free(entry_main);
            archive_string_free(&mut entry_name);
            return -(30 as libc::c_int);
        } else {
            if r < ret {
                ret = r
            }
        }
        r = __archive_write_output(
            a,
            paxbuff.as_mut_ptr() as *const libc::c_void,
            512 as libc::c_int as size_t,
        );
        if r != ARCHIVE_OK {
            sparse_list_clear(pax);
            (*pax).entry_bytes_remaining = 0 as libc::c_int as uint64_t;
            (*pax).entry_padding = 0 as libc::c_int as uint64_t;
            archive_entry_free(entry_main);
            archive_string_free(&mut entry_name);
            return -(30 as libc::c_int);
        }
        (*pax).entry_bytes_remaining = (*pax).pax_header.length;
        (*pax).entry_padding = (0x1ff as libc::c_int as libc::c_long
            & -((*pax).entry_bytes_remaining as int64_t))
            as uint64_t;
        r = __archive_write_output(
            a,
            (*pax).pax_header.s as *const libc::c_void,
            (*pax).pax_header.length,
        );
        if r != ARCHIVE_OK {
            /* If a write fails, we're pretty much toast. */
            archive_entry_free(entry_main);
            archive_string_free(&mut entry_name);
            return -(30 as libc::c_int);
        }
        /* Pad out the end of the entry. */
        r = __archive_write_nulls(a, (*pax).entry_padding);
        if r != ARCHIVE_OK {
            /* If a write fails, we're pretty much toast. */
            archive_entry_free(entry_main);
            archive_string_free(&mut entry_name);
            return -(30 as libc::c_int);
        }
        (*pax).entry_padding = 0 as libc::c_int as uint64_t;
        (*pax).entry_bytes_remaining = (*pax).entry_padding
    }
    /* Write the header for main entry. */
    r = __archive_write_output(
        a,
        ustarbuff.as_mut_ptr() as *const libc::c_void,
        512 as libc::c_int as size_t,
    );
    if r != ARCHIVE_OK {
        archive_entry_free(entry_main);
        archive_string_free(&mut entry_name);
        return r;
    }
    /*
     * Inform the client of the on-disk size we're using, so
     * they can avoid unnecessarily writing a body for something
     * that we're just going to ignore.
     */
    archive_entry_set_size(entry_original, real_size as la_int64_t);
    if (*pax).sparse_list.is_null() && real_size > 0 as libc::c_int as libc::c_ulong {
        /* This is not a sparse file but we handle its data as
         * a sparse block. */
        sparse_list_add(pax, 0 as libc::c_int as int64_t, real_size as int64_t);
        sparse_total = real_size
    }
    (*pax).entry_padding =
        (0x1ff as libc::c_int as libc::c_long & -(sparse_total as int64_t)) as uint64_t;
    archive_entry_free(entry_main);
    archive_string_free(&mut entry_name);
    return ret;
}
/*
 * We need a valid name for the regular 'ustar' entry.  This routine
 * tries to hack something more-or-less reasonable.
 *
 * The approach here tries to preserve leading dir names.  We do so by
 * working with four sections:
 *   1) "prefix" directory names,
 *   2) "suffix" directory names,
 *   3) inserted dir name (optional),
 *   4) filename.
 *
 * These sections must satisfy the following requirements:
 *   * Parts 1 & 2 together form an initial portion of the dir name.
 *   * Part 3 is specified by the caller.  (It should not contain a leading
 *     or trailing '/'.)
 *   * Part 4 forms an initial portion of the base filename.
 *   * The filename must be <= 99 chars to fit the ustar 'name' field.
 *   * Parts 2, 3, 4 together must be <= 99 chars to fit the ustar 'name' fld.
 *   * Part 1 must be <= 155 chars to fit the ustar 'prefix' field.
 *   * If the original name ends in a '/', the new name must also end in a '/'
 *   * Trailing '/.' sequences may be stripped.
 *
 * Note: Recall that the ustar format does not store the '/' separating
 * parts 1 & 2, but does store the '/' separating parts 2 & 3.
 */
unsafe extern "C" fn build_ustar_entry_name(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut src_length: size_t,
    mut insert: *const libc::c_char,
) -> *mut libc::c_char {
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char; /* Was there a trailing slash? */
    let mut prefix_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut suffix: *const libc::c_char = 0 as *const libc::c_char;
    let mut suffix_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut filename_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut need_slash: libc::c_int = 0 as libc::c_int;
    let mut suffix_length: size_t = 99 as libc::c_int as size_t;
    let mut insert_length: size_t = 0;
    /* Length of additional dir element to be added. */
    if insert.is_null() {
        insert_length = 0 as libc::c_int as size_t
    } else {
        /* +2 here allows for '/' before and after the insert. */
        insert_length = strlen(insert).wrapping_add(2 as libc::c_int as libc::c_ulong)
    }
    /* Step 0: Quick bailout in a common case. */
    if src_length < 100 as libc::c_int as libc::c_ulong && insert.is_null() {
        strncpy(dest, src, src_length);
        *dest.offset(src_length as isize) = '\u{0}' as i32 as libc::c_char;
        return dest;
    }
    /* Step 1: Locate filename and enforce the length restriction. */
    filename_end = src.offset(src_length as isize);
    loop
    /* Remove trailing '/' chars and '/.' pairs. */
    {
        if filename_end > src
            && *filename_end.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32
        {
            filename_end = filename_end.offset(-1); /* Remember to restore trailing '/'. */
            need_slash = 1 as libc::c_int
        } else {
            if !(filename_end > src.offset(1 as libc::c_int as isize)
                && *filename_end.offset(-(1 as libc::c_int) as isize) as libc::c_int == '.' as i32
                && *filename_end.offset(-(2 as libc::c_int) as isize) as libc::c_int == '/' as i32)
            {
                break; /* "foo/." will become "foo/" */
            }
            filename_end = filename_end.offset(-(2 as libc::c_int as isize));
            need_slash = 1 as libc::c_int
        }
    }
    if need_slash != 0 {
        suffix_length = suffix_length.wrapping_sub(1)
    }
    /* Find start of filename. */
    filename = filename_end.offset(-(1 as libc::c_int as isize));
    while filename > src && *filename as libc::c_int != '/' as i32 {
        filename = filename.offset(-1)
    }
    if *filename as libc::c_int == '/' as i32
        && filename < filename_end.offset(-(1 as libc::c_int as isize))
    {
        filename = filename.offset(1)
    }
    /* Adjust filename_end so that filename + insert fits in 99 chars. */
    suffix_length =
        (suffix_length as libc::c_ulong).wrapping_sub(insert_length) as size_t as size_t;
    if filename_end > filename.offset(suffix_length as isize) {
        filename_end = filename.offset(suffix_length as isize)
    }
    /* Calculate max size for "suffix" section (#3 above). */
    suffix_length = (suffix_length as libc::c_ulong)
        .wrapping_sub(filename_end.offset_from(filename) as libc::c_long as libc::c_ulong)
        as size_t as size_t;
    /* Step 2: Locate the "prefix" section of the dirname, including
     * trailing '/'. */
    prefix = src;
    prefix_end = prefix.offset(155 as libc::c_int as isize);
    if prefix_end > filename {
        prefix_end = filename
    }
    while prefix_end > prefix && *prefix_end as libc::c_int != '/' as i32 {
        prefix_end = prefix_end.offset(-1)
    }
    if prefix_end < filename && *prefix_end as libc::c_int == '/' as i32 {
        prefix_end = prefix_end.offset(1)
    }
    /* Step 3: Locate the "suffix" section of the dirname,
     * including trailing '/'. */
    suffix = prefix_end; /* Enforce limit. */
    suffix_end = suffix.offset(suffix_length as isize);
    if suffix_end > filename {
        suffix_end = filename
    }
    if suffix_end < suffix {
        suffix_end = suffix
    }
    while suffix_end > suffix && *suffix_end as libc::c_int != '/' as i32 {
        suffix_end = suffix_end.offset(-1)
    }
    if suffix_end < filename && *suffix_end as libc::c_int == '/' as i32 {
        suffix_end = suffix_end.offset(1)
    }
    /* Step 4: Build the new name. */
    /* The OpenBSD strlcpy function is safer, but less portable. */
    /* Rather than maintain two versions, just use the strncpy version. */
    p = dest;
    if prefix_end > prefix {
        strncpy(
            p,
            prefix,
            prefix_end.offset_from(prefix) as libc::c_long as libc::c_ulong,
        );
        p = p.offset(prefix_end.offset_from(prefix) as libc::c_long as isize)
    }
    if suffix_end > suffix {
        strncpy(
            p,
            suffix,
            suffix_end.offset_from(suffix) as libc::c_long as libc::c_ulong,
        );
        p = p.offset(suffix_end.offset_from(suffix) as libc::c_long as isize)
    }
    if !insert.is_null() {
        /* Note: assume insert does not have leading or trailing '/' */
        strcpy(p, insert);
        p = p.offset(strlen(insert) as isize);
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = '/' as i32 as libc::c_char
    }
    strncpy(
        p,
        filename,
        filename_end.offset_from(filename) as libc::c_long as libc::c_ulong,
    );
    p = p.offset(filename_end.offset_from(filename) as libc::c_long as isize);
    if need_slash != 0 {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = '/' as i32 as libc::c_char
    }
    *p = '\u{0}' as i32 as libc::c_char;
    return dest;
}
/*
 * The ustar header for the pax extended attributes must have a
 * reasonable name:  SUSv3 requires 'dirname'/PaxHeader.'pid'/'filename'
 * where 'pid' is the PID of the archiving process.  Unfortunately,
 * that makes testing a pain since the output varies for each run,
 * so I'm sticking with the simpler 'dirname'/PaxHeader/'filename'
 * for now.  (Someday, I'll make this settable.  Then I can use the
 * SUS recommendation as default and test harnesses can override it
 * to get predictable results.)
 *
 * Joerg Schilling has argued that this is unnecessary because, in
 * practice, if the pax extended attributes get extracted as regular
 * files, no one is going to bother reading those attributes to
 * manually restore them.  Based on this, 'star' uses
 * /tmp/PaxHeader/'basename' as the ustar header name.  This is a
 * tempting argument, in part because it's simpler than the SUSv3
 * recommendation, but I'm not entirely convinced.  I'm also
 * uncomfortable with the fact that "/tmp" is a Unix-ism.
 *
 * The following routine leverages build_ustar_entry_name() above and
 * so is simpler than you might think.  It just needs to provide the
 * additional path element and handle a few pathological cases).
 */
unsafe extern "C" fn build_pax_attribute_name(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buff: [libc::c_char; 64] = [0; 64];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* Handle the null filename case. */
    if src.is_null() || *src as libc::c_int == '\u{0}' as i32 {
        strcpy(
            dest,
            b"PaxHeader/blank\x00" as *const u8 as *const libc::c_char,
        );
        return dest;
    }
    /* Prune final '/' and other unwanted final elements. */
    p = src.offset(strlen(src) as isize);
    loop
    /* Ends in "/", remove the '/' */
    {
        if p > src && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32 {
            p = p.offset(-1)
        } else {
            /* Ends in "/.", remove the '.' */
            if !(p > src.offset(1 as libc::c_int as isize)
                && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '.' as i32
                && *p.offset(-(2 as libc::c_int) as isize) as libc::c_int == '/' as i32)
            {
                break;
            }
            p = p.offset(-1)
        }
    }
    /* Pathological case: After above, there was nothing left.
     * This includes "/." "/./." "/.//./." etc. */
    if p == src {
        strcpy(
            dest,
            b"/PaxHeader/rootdir\x00" as *const u8 as *const libc::c_char,
        );
        return dest;
    }
    /* Convert unadorned "." into a suitable filename. */
    if *src as libc::c_int == '.' as i32 && p == src.offset(1 as libc::c_int as isize) {
        strcpy(
            dest,
            b"PaxHeader/currentdir\x00" as *const u8 as *const libc::c_char,
        );
        return dest;
    }
    /*
     * TODO: Push this string into the 'pax' structure to avoid
     * recomputing it every time.  That will also open the door
     * to having clients override it.
     */
    /* Disable this for now; see above comment. */
    /* If the platform can't fetch the pid, don't include it. */
    strcpy(
        buff.as_mut_ptr(),
        b"PaxHeader\x00" as *const u8 as *const libc::c_char,
    );
    /* General case: build a ustar-compatible name adding
     * "/PaxHeader/". */
    build_ustar_entry_name(
        dest,
        src,
        p.offset_from(src) as libc::c_long as size_t,
        buff.as_mut_ptr(),
    );
    return dest;
}
/*
 * GNU PAX Format 1.0 requires the special name, which pattern is:
 * <dir>/GNUSparseFile.<pid>/<original file name>
 *
 * Since reproducible archives are more important, use 0 as pid.
 *
 * This function is used for only Sparse file, a file type of which
 * is regular file.
 */
unsafe extern "C" fn build_gnu_sparse_name(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    /* Handle the null filename case. */
    if src.is_null() || *src as libc::c_int == '\u{0}' as i32 {
        strcpy(
            dest,
            b"GNUSparseFile/blank\x00" as *const u8 as *const libc::c_char,
        );
        return dest;
    }
    /* Prune final '/' and other unwanted final elements. */
    p = src.offset(strlen(src) as isize);
    loop
    /* Ends in "/", remove the '/' */
    {
        if p > src && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32 {
            p = p.offset(-1)
        } else {
            /* Ends in "/.", remove the '.' */
            if !(p > src.offset(1 as libc::c_int as isize)
                && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '.' as i32
                && *p.offset(-(2 as libc::c_int) as isize) as libc::c_int == '/' as i32)
            {
                break;
            }
            p = p.offset(-1)
        }
    }
    /* General case: build a ustar-compatible name adding
     * "/GNUSparseFile/". */
    build_ustar_entry_name(
        dest,
        src,
        p.offset_from(src) as libc::c_long as size_t,
        b"GNUSparseFile.0\x00" as *const u8 as *const libc::c_char,
    );
    return dest;
}
/* Write two null blocks for the end of archive */
unsafe extern "C" fn archive_write_pax_close(mut a: *mut archive_write) -> libc::c_int {
    return __archive_write_nulls(a, (512 as libc::c_int * 2 as libc::c_int) as size_t);
}
unsafe extern "C" fn archive_write_pax_free(mut a: *mut archive_write) -> libc::c_int {
    let mut pax: *mut pax = 0 as *mut pax;
    pax = (*a).format_data as *mut pax;
    if pax.is_null() {
        return 0 as libc::c_int;
    }
    archive_string_free(&mut (*pax).pax_header);
    archive_string_free(&mut (*pax).sparse_map);
    archive_string_free(&mut (*pax).l_url_encoded_name);
    sparse_list_clear(pax);
    free(pax as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_pax_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut pax: *mut pax = 0 as *mut pax;
    let mut remaining: uint64_t = 0;
    let mut ret: libc::c_int = 0;
    pax = (*a).format_data as *mut pax;
    remaining = (*pax).entry_bytes_remaining;
    if remaining == 0 as libc::c_int as libc::c_ulong {
        while !(*pax).sparse_list.is_null() {
            let mut sb: *mut sparse_block = 0 as *mut sparse_block;
            if (*(*pax).sparse_list).is_hole == 0 {
                remaining = (remaining as libc::c_ulong)
                    .wrapping_add((*(*pax).sparse_list).remaining)
                    as uint64_t as uint64_t
            }
            sb = (*(*pax).sparse_list).next;
            free((*pax).sparse_list as *mut libc::c_void);
            (*pax).sparse_list = sb
        }
    }
    ret = __archive_write_nulls(a, remaining.wrapping_add((*pax).entry_padding));
    (*pax).entry_padding = 0 as libc::c_int as uint64_t;
    (*pax).entry_bytes_remaining = (*pax).entry_padding;
    return ret;
}
unsafe extern "C" fn archive_write_pax_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut pax: *mut pax = 0 as *mut pax;
    let mut ws: size_t = 0;
    let mut total: size_t = 0;
    let mut ret: libc::c_int = 0;
    pax = (*a).format_data as *mut pax;
    /*
     * According to GNU PAX format 1.0, write a sparse map
     * before the body.
     */
    if (*pax).sparse_map.length != 0 {
        ret = __archive_write_output(
            a,
            (*pax).sparse_map.s as *const libc::c_void,
            (*pax).sparse_map.length,
        );
        if ret != ARCHIVE_OK {
            return ret as ssize_t;
        }
        ret = __archive_write_nulls(a, (*pax).sparse_map_padding);
        if ret != ARCHIVE_OK {
            return ret as ssize_t;
        }
        (*pax).sparse_map.length = 0 as libc::c_int as size_t
    }
    total = 0 as libc::c_int as size_t;
    while total < s {
        let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
        while !(*pax).sparse_list.is_null()
            && (*(*pax).sparse_list).remaining == 0 as libc::c_int as libc::c_ulong
        {
            let mut sb: *mut sparse_block = (*(*pax).sparse_list).next;
            free((*pax).sparse_list as *mut libc::c_void);
            (*pax).sparse_list = sb
        }
        if (*pax).sparse_list.is_null() {
            return total as ssize_t;
        }
        p = (buff as *const libc::c_uchar).offset(total as isize);
        ws = s.wrapping_sub(total);
        if ws > (*(*pax).sparse_list).remaining {
            ws = (*(*pax).sparse_list).remaining
        }
        if (*(*pax).sparse_list).is_hole != 0 {
            /* Current block is hole thus we do not write
             * the body. */
            (*(*pax).sparse_list).remaining = ((*(*pax).sparse_list).remaining as libc::c_ulong)
                .wrapping_sub(ws) as uint64_t
                as uint64_t;
            total = (total as libc::c_ulong).wrapping_add(ws) as size_t as size_t
        } else {
            ret = __archive_write_output(a, p as *const libc::c_void, ws);
            (*(*pax).sparse_list).remaining = ((*(*pax).sparse_list).remaining as libc::c_ulong)
                .wrapping_sub(ws) as uint64_t
                as uint64_t;
            total = (total as libc::c_ulong).wrapping_add(ws) as size_t as size_t;
            if ret != ARCHIVE_OK {
                return ret as ssize_t;
            }
        }
    }
    return total as ssize_t;
}
unsafe extern "C" fn has_non_ASCII(mut _p: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_uchar = _p as *const libc::c_uchar;
    if p.is_null() {
        return 1 as libc::c_int;
    }
    while *p as libc::c_int != '\u{0}' as i32 && (*p as libc::c_int) < 128 as libc::c_int {
        p = p.offset(1)
    }
    return (*p as libc::c_int != '\u{0}' as i32) as libc::c_int;
}
/*
 * Used by extended attribute support; encodes the name
 * so that there will be no '=' characters in the result.
 */
unsafe extern "C" fn url_encode(mut in_0: *const libc::c_char) -> *mut libc::c_char {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out_len: libc::c_int = 0 as libc::c_int;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    s = in_0;
    while *s as libc::c_int != '\u{0}' as i32 {
        if (*s as libc::c_int) < 33 as libc::c_int
            || *s as libc::c_int > 126 as libc::c_int
            || *s as libc::c_int == '%' as i32
            || *s as libc::c_int == '=' as i32
        {
            out_len += 3 as libc::c_int
        } else {
            out_len += 1
        }
        s = s.offset(1)
    }
    out = malloc((out_len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    if out.is_null() {
        return 0 as *mut libc::c_char;
    }
    s = in_0;
    d = out;
    while *s as libc::c_int != '\u{0}' as i32 {
        /* encode any non-printable ASCII character or '%' or '=' */
        if (*s as libc::c_int) < 33 as libc::c_int
            || *s as libc::c_int > 126 as libc::c_int
            || *s as libc::c_int == '%' as i32
            || *s as libc::c_int == '=' as i32
        {
            /* URL encoding is '%' followed by two hex digits */
            let fresh3 = d;
            d = d.offset(1);
            *fresh3 = '%' as i32 as libc::c_char;
            let fresh4 = d;
            d = d.offset(1);
            *fresh4 =
                (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789ABCDEF\x00"))
                    [(0xf as libc::c_int & *s as libc::c_int >> 4 as libc::c_int) as usize];
            let fresh5 = d;
            d = d.offset(1);
            *fresh5 =
                (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"0123456789ABCDEF\x00"))
                    [(0xf as libc::c_int & *s as libc::c_int) as usize]
        } else {
            let fresh6 = d;
            d = d.offset(1);
            *fresh6 = *s
        }
        s = s.offset(1)
    }
    *d = '\u{0}' as i32 as libc::c_char;
    return out;
}
/*
 * Encode a sequence of bytes into a C string using base-64 encoding.
 *
 * Returns a null-terminated C string allocated with malloc(); caller
 * is responsible for freeing the result.
 */
unsafe extern "C" fn base64_encode(
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    static mut digits: [libc::c_char; 64] = [
        'A' as i32 as libc::c_char,
        'B' as i32 as libc::c_char,
        'C' as i32 as libc::c_char,
        'D' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'F' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'H' as i32 as libc::c_char,
        'I' as i32 as libc::c_char,
        'J' as i32 as libc::c_char,
        'K' as i32 as libc::c_char,
        'L' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        'N' as i32 as libc::c_char,
        'O' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        'Q' as i32 as libc::c_char,
        'R' as i32 as libc::c_char,
        'S' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'U' as i32 as libc::c_char,
        'V' as i32 as libc::c_char,
        'W' as i32 as libc::c_char,
        'X' as i32 as libc::c_char,
        'Y' as i32 as libc::c_char,
        'Z' as i32 as libc::c_char,
        'a' as i32 as libc::c_char,
        'b' as i32 as libc::c_char,
        'c' as i32 as libc::c_char,
        'd' as i32 as libc::c_char,
        'e' as i32 as libc::c_char,
        'f' as i32 as libc::c_char,
        'g' as i32 as libc::c_char,
        'h' as i32 as libc::c_char,
        'i' as i32 as libc::c_char,
        'j' as i32 as libc::c_char,
        'k' as i32 as libc::c_char,
        'l' as i32 as libc::c_char,
        'm' as i32 as libc::c_char,
        'n' as i32 as libc::c_char,
        'o' as i32 as libc::c_char,
        'p' as i32 as libc::c_char,
        'q' as i32 as libc::c_char,
        'r' as i32 as libc::c_char,
        's' as i32 as libc::c_char,
        't' as i32 as libc::c_char,
        'u' as i32 as libc::c_char,
        'v' as i32 as libc::c_char,
        'w' as i32 as libc::c_char,
        'x' as i32 as libc::c_char,
        'y' as i32 as libc::c_char,
        'z' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
        '2' as i32 as libc::c_char,
        '3' as i32 as libc::c_char,
        '4' as i32 as libc::c_char,
        '5' as i32 as libc::c_char,
        '6' as i32 as libc::c_char,
        '7' as i32 as libc::c_char,
        '8' as i32 as libc::c_char,
        '9' as i32 as libc::c_char,
        '+' as i32 as libc::c_char,
        '/' as i32 as libc::c_char,
    ];
    let mut v: libc::c_int = 0;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    /* 3 bytes becomes 4 chars, but round up and allow for trailing NUL */
    out = malloc(
        len.wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if out.is_null() {
        return 0 as *mut libc::c_char;
    }
    d = out;
    /* Convert each group of 3 bytes into 4 characters. */
    while len >= 3 as libc::c_int as libc::c_ulong {
        v = (*s.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
            & 0xff0000 as libc::c_int
            | (*s.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
                & 0xff00 as libc::c_int
            | *s.offset(2 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int;
        s = s.offset(3 as libc::c_int as isize);
        len = (len as libc::c_ulong).wrapping_sub(3 as libc::c_int as libc::c_ulong) as size_t
            as size_t;
        let fresh7 = d;
        d = d.offset(1);
        *fresh7 = digits[(v >> 18 as libc::c_int & 0x3f as libc::c_int) as usize];
        let fresh8 = d;
        d = d.offset(1);
        *fresh8 = digits[(v >> 12 as libc::c_int & 0x3f as libc::c_int) as usize];
        let fresh9 = d;
        d = d.offset(1);
        *fresh9 = digits[(v >> 6 as libc::c_int & 0x3f as libc::c_int) as usize];
        let fresh10 = d;
        d = d.offset(1);
        *fresh10 = digits[(v & 0x3f as libc::c_int) as usize]
    }
    /* Handle final group of 1 byte (2 chars) or 2 bytes (3 chars). */
    match len {
        1 => {
            v = (*s.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
                & 0xff0000 as libc::c_int;
            let fresh11 = d;
            d = d.offset(1);
            *fresh11 = digits[(v >> 18 as libc::c_int & 0x3f as libc::c_int) as usize];
            let fresh12 = d;
            d = d.offset(1);
            *fresh12 = digits[(v >> 12 as libc::c_int & 0x3f as libc::c_int) as usize]
        }
        2 => {
            v = (*s.offset(0 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
                & 0xff0000 as libc::c_int
                | (*s.offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
                    & 0xff00 as libc::c_int;
            let fresh13 = d;
            d = d.offset(1);
            *fresh13 = digits[(v >> 18 as libc::c_int & 0x3f as libc::c_int) as usize];
            let fresh14 = d;
            d = d.offset(1);
            *fresh14 = digits[(v >> 12 as libc::c_int & 0x3f as libc::c_int) as usize];
            let fresh15 = d;
            d = d.offset(1);
            *fresh15 = digits[(v >> 6 as libc::c_int & 0x3f as libc::c_int) as usize]
        }
        0 | _ => {}
    }
    /* Add trailing NUL character so output is a valid C string. */
    *d = '\u{0}' as i32 as libc::c_char;
    return out;
}
unsafe extern "C" fn sparse_list_clear(mut pax: *mut pax) {
    while !(*pax).sparse_list.is_null() {
        let mut sb: *mut sparse_block = (*pax).sparse_list;
        (*pax).sparse_list = (*sb).next;
        free(sb as *mut libc::c_void);
    }
    (*pax).sparse_tail = NULL as *mut sparse_block;
}
unsafe extern "C" fn _sparse_list_add_block(
    mut pax: *mut pax,
    mut offset: int64_t,
    mut length: int64_t,
    mut is_hole: libc::c_int,
) -> libc::c_int {
    let mut sb: *mut sparse_block = 0 as *mut sparse_block;
    sb = malloc(::std::mem::size_of::<sparse_block>() as libc::c_ulong) as *mut sparse_block;
    if sb.is_null() {
        return -(30 as libc::c_int);
    }
    (*sb).next = NULL as *mut sparse_block;
    (*sb).is_hole = is_hole;
    (*sb).offset = offset as uint64_t;
    (*sb).remaining = length as uint64_t;
    if (*pax).sparse_list.is_null() || (*pax).sparse_tail.is_null() {
        (*pax).sparse_tail = sb;
        (*pax).sparse_list = (*pax).sparse_tail
    } else {
        (*(*pax).sparse_tail).next = sb;
        (*pax).sparse_tail = sb
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sparse_list_add(
    mut pax: *mut pax,
    mut offset: int64_t,
    mut length: int64_t,
) -> libc::c_int {
    let mut last_offset: int64_t = 0;
    let mut r: libc::c_int = 0;
    if (*pax).sparse_tail.is_null() {
        last_offset = 0 as libc::c_int as int64_t
    } else {
        last_offset = (*(*pax).sparse_tail)
            .offset
            .wrapping_add((*(*pax).sparse_tail).remaining) as int64_t
    }
    if last_offset < offset {
        /* Add a hole block. */
        r = _sparse_list_add_block(pax, last_offset, offset - last_offset, 1 as libc::c_int);
        if r != ARCHIVE_OK {
            return r;
        }
    }
    /* Add data block. */
    return _sparse_list_add_block(pax, offset, length, 0 as libc::c_int);
}
