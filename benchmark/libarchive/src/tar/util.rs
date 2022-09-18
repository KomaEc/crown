use ::libc;
extern "C" {
    /* Declare our basic types. */
    pub type archive;
    pub type archive_entry;
    /*
     * Utility to match up hardlinks.
     *
     * The 'struct archive_entry_linkresolver' is a cache of archive entries
     * for files with multiple links.  Here's how to use it:
     *   1. Create a lookup object with archive_entry_linkresolver_new()
     *   2. Tell it the archive format you're using.
     *   3. Hand each archive_entry to archive_entry_linkify().
     *      That function will return 0, 1, or 2 entries that should
     *      be written.
     *   4. Call archive_entry_linkify(resolver, NULL) until
     *      no more entries are returned.
     *   5. Call archive_entry_linkresolver_free(resolver) to free resources.
     *
     * The entries returned have their hardlink and size fields updated
     * appropriately.  If an entry is passed in that does not refer to
     * a file with multiple links, it is returned unchanged.  The intention
     * is that you should be able to simply filter all entries through
     * this machine.
     *
     * To make things more efficient, be sure that each entry has a valid
     * nlinks value.  The hardlink cache uses this to track when all links
     * have been found.  If the nlinks value is zero, it will keep every
     * name in the cache indefinitely, which can use a lot of memory.
     *
     * Note that archive_entry_size() is reset to zero if the file
     * body should not be written to the archive.  Pay attention!
     */
    pub type archive_entry_linkresolver;
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
     * $FreeBSD: src/usr.bin/tar/bsdtar.h,v 1.37 2008/12/06 07:37:14 kientzle Exp $
     */
    pub type creation_set;
    pub type substitution;
    pub type siginfo_data;
    pub type name_cache;
    pub type security;
    pub type archive_dir;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_gname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_hardlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_rdevmajor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_rdevminor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_strmode(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_copy_hardlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_symlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn mbtowc(__pwc: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn iswprint(__wc: wint_t) -> libc::c_int;
    #[no_mangle]
    fn apply_substitution(
        _: *mut bsdtar,
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    /*-
     * Copyright (c) 2009 Joerg Sonnenberger
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
    #[no_mangle]
    fn lafe_warnc(code: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn lafe_errc(eval: libc::c_int, code: libc::c_int, fmt: *const libc::c_char, _: ...) -> !;
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
    #[no_mangle]
    fn lafe_readpassphrase(
        prompt: *const libc::c_char,
        buf: *mut libc::c_char,
        bufsiz: size_t,
    ) -> *mut libc::c_char;
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
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type time_t = __time_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type va_list = __builtin_va_list;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
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
pub type la_int64_t = int64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type wint_t = libc::c_uint;
/*
 * The internal state for the "bsdtar" program.
 *
 * Keeping all of the state in a structure like this simplifies memory
 * leak testing (at exit, anything left on the heap is suspect).  A
 * pointer to this structure is passed to most bsdtar internal
 * functions.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bsdtar {
    pub filename: *const libc::c_char,
    pub pending_chdir: *mut libc::c_char,
    pub names_from_file: *const libc::c_char,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub verbose: libc::c_int,
    pub flags: libc::c_uint,
    pub extract_flags: libc::c_int,
    pub readdisk_flags: libc::c_int,
    pub strip_components: libc::c_int,
    pub gid: libc::c_int,
    pub gname: *const libc::c_char,
    pub uid: libc::c_int,
    pub uname: *const libc::c_char,
    pub passphrase: *const libc::c_char,
    pub mode: libc::c_char,
    pub symlink_mode: libc::c_char,
    pub option_options: *const libc::c_char,
    pub day_first: libc::c_char,
    pub cset: *mut creation_set,
    pub getopt_state: libc::c_int,
    pub getopt_word: *mut libc::c_char,
    pub fd: libc::c_int,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub argument: *const libc::c_char,
    pub gs_width: size_t,
    pub u_width: size_t,
    pub user_uid: uid_t,
    pub return_value: libc::c_int,
    pub warned_lead_slash: libc::c_char,
    pub next_line_is_dir: libc::c_char,
    pub diskreader: *mut archive,
    pub resolver: *mut archive_entry_linkresolver,
    pub archive_dir: *mut archive_dir,
    pub gname_cache: *mut name_cache,
    pub buff: *mut libc::c_char,
    pub buff_size: size_t,
    pub first_fs: libc::c_int,
    pub matching: *mut archive,
    pub security: *mut security,
    pub uname_cache: *mut name_cache,
    pub siginfo: *mut siginfo_data,
    pub substitution: *mut substitution,
    pub ppbuff: *mut libc::c_char,
}
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
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
/* for util.c */
/* Options for flags bitfield */
/* -a */
pub const OPTFLAG_ABSOLUTE_PATHS: libc::c_int = 0x2 as libc::c_int;
/* TODO:  Hack up a version of mbtowc for platforms with no wide
 * character support at all.  I think the following might suffice,
 * but it needs careful testing.
 * #if !HAVE_MBTOWC
 * #define	mbtowc(wcp, p, n) ((*wcp = *p), 1)
 * #endif
 */
/*
 * Print a string, taking care with any non-printable characters.
 *
 * Note that we use a stack-allocated buffer to receive the formatted
 * string if we can.  This is partly performance (avoiding a call to
 * malloc()), partly out of expedience (we have to call vsnprintf()
 * before malloc() anyway to find out how big a buffer we need; we may
 * as well point that first call at a small local buffer in case it
 * works), but mostly for safety (so we can use this to print messages
 * about out-of-memory conditions).
 */
#[no_mangle]
pub unsafe extern "C" fn safe_fprintf(
    mut f: *mut FILE,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut fmtbuff_stack: [libc::c_char; 256] = [0; 256]; /* Place to format the printf() string. */
    let mut outbuff: [libc::c_char; 256] = [0; 256]; /* Buffer for outgoing characters. */
    let mut fmtbuff_heap: *mut libc::c_char = 0 as *mut libc::c_char; /* If fmtbuff_stack is too small, we use malloc */
    let mut fmtbuff: *mut libc::c_char = 0 as *mut libc::c_char; /* Pointer to fmtbuff_stack or fmtbuff_heap. */
    let mut fmtbuff_length: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ap: ::std::ffi::VaListImpl;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_uint = 0;
    let mut wc: wchar_t = 0;
    let mut try_wc: libc::c_char = 0;
    /* Use a stack-allocated buffer if we can, for speed and safety. */
    fmtbuff_heap = NULL as *mut libc::c_char;
    fmtbuff_length = ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int;
    fmtbuff = fmtbuff_stack.as_mut_ptr();
    /* Try formatting into the stack buffer. */
    ap = args.clone();
    length = vsnprintf(
        fmtbuff,
        fmtbuff_length as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    /* If the result was too large, allocate a buffer on the heap. */
    while length < 0 as libc::c_int || length >= fmtbuff_length {
        if length >= fmtbuff_length {
            fmtbuff_length = length + 1 as libc::c_int
        } else if fmtbuff_length < 8192 as libc::c_int {
            fmtbuff_length *= 2 as libc::c_int
        } else if fmtbuff_length < 1000000 as libc::c_int {
            fmtbuff_length += fmtbuff_length / 4 as libc::c_int
        } else {
            length = fmtbuff_length;
            *fmtbuff_heap.offset((length - 1 as libc::c_int) as isize) =
                '\u{0}' as i32 as libc::c_char;
            break;
        }
        free(fmtbuff_heap as *mut libc::c_void);
        fmtbuff_heap = malloc(fmtbuff_length as libc::c_ulong) as *mut libc::c_char;
        /* Reformat the result into the heap buffer if we can. */
        if !fmtbuff_heap.is_null() {
            fmtbuff = fmtbuff_heap;
            ap = args.clone();
            length = vsnprintf(
                fmtbuff,
                fmtbuff_length as libc::c_ulong,
                fmt,
                ap.as_va_list(),
            )
        } else {
            /* Leave fmtbuff pointing to the truncated
             * string in fmtbuff_stack. */
            fmtbuff = fmtbuff_stack.as_mut_ptr();
            length = (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as libc::c_int;
            break;
        }
    }
    /* Note: mbrtowc() has a cleaner API, but mbtowc() seems a bit
     * more portable, so we use that here instead. */
    if mbtowc(
        NULL as *mut wchar_t,
        NULL as *const libc::c_char,
        1 as libc::c_int as size_t,
    ) == -(1 as libc::c_int)
    {
        /* Reset the shift state. */
        /* mbtowc() should never fail in practice, but
         * handle the theoretical error anyway. */
        free(fmtbuff_heap as *mut libc::c_void);
        return;
    }
    /* Write data, expanding unprintable characters. */
    p = fmtbuff;
    i = 0 as libc::c_int as libc::c_uint;
    try_wc = 1 as libc::c_int as libc::c_char;
    while *p as libc::c_int != '\u{0}' as i32 {
        /* Convert to wide char, test if the wide
         * char is printable in the current locale. */
        if try_wc as libc::c_int != 0 && {
            n = mbtowc(&mut wc, p, length as size_t);
            (n) != -(1 as libc::c_int)
        } {
            length -= n;
            if iswprint(wc as wint_t) != 0 && wc != '\\' as i32 {
                loop
                /* Printable, copy the bytes through. */
                {
                    let fresh0 = n;
                    n = n - 1;
                    if !(fresh0 > 0 as libc::c_int) {
                        break;
                    }
                    let fresh1 = p;
                    p = p.offset(1);
                    let fresh2 = i;
                    i = i.wrapping_add(1);
                    outbuff[fresh2 as usize] = *fresh1
                }
            } else {
                loop
                /* Not printable, format the bytes. */
                {
                    let fresh3 = n;
                    n = n - 1;
                    if !(fresh3 > 0 as libc::c_int) {
                        break;
                    }
                    let fresh4 = p;
                    p = p.offset(1);
                    i = i.wrapping_add(bsdtar_expand_char(
                        outbuff.as_mut_ptr(),
                        i as size_t,
                        *fresh4,
                    ) as libc::c_uint)
                }
            }
        } else {
            /* After any conversion failure, don't bother
             * trying to convert the rest. */
            let fresh5 = p;
            p = p.offset(1);
            i = i.wrapping_add(
                bsdtar_expand_char(outbuff.as_mut_ptr(), i as size_t, *fresh5) as libc::c_uint,
            );
            try_wc = 0 as libc::c_int as libc::c_char
        }
        /* If our output buffer is full, dump it and keep going. */
        if i as libc::c_ulong
            > (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(128 as libc::c_int as libc::c_ulong)
        {
            outbuff[i as usize] = '\u{0}' as i32 as libc::c_char;
            fprintf(
                f,
                b"%s\x00" as *const u8 as *const libc::c_char,
                outbuff.as_mut_ptr(),
            );
            i = 0 as libc::c_int as libc::c_uint
        }
    }
    outbuff[i as usize] = '\u{0}' as i32 as libc::c_char;
    fprintf(
        f,
        b"%s\x00" as *const u8 as *const libc::c_char,
        outbuff.as_mut_ptr(),
    );
    /* If we allocated a heap-based formatting buffer, free it now. */
    free(fmtbuff_heap as *mut libc::c_void);
}
/* Linux doesn't define mode_t, etc. in sys/stat.h. */
/*
 * Render an arbitrary sequence of bytes into printable ASCII characters.
 */
unsafe extern "C" fn bsdtar_expand_char(
    mut buff: *mut libc::c_char,
    mut offset: size_t,
    mut c: libc::c_char,
) -> size_t {
    let mut i: size_t = offset;
    if *(*__ctype_b_loc()).offset(c as libc::c_uchar as libc::c_int as isize) as libc::c_int
        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
        != 0
        && c as libc::c_int != '\\' as i32
    {
        let fresh6 = i;
        i = i.wrapping_add(1);
        *buff.offset(fresh6 as isize) = c
    } else {
        let fresh7 = i;
        i = i.wrapping_add(1);
        *buff.offset(fresh7 as isize) = '\\' as i32 as libc::c_char;
        match c as libc::c_int {
            7 => {
                let fresh8 = i;
                i = i.wrapping_add(1);
                *buff.offset(fresh8 as isize) = 'a' as i32 as libc::c_char
            }
            8 => {
                let fresh9 = i;
                i = i.wrapping_add(1);
                *buff.offset(fresh9 as isize) = 'b' as i32 as libc::c_char
            }
            12 => {
                let fresh10 = i;
                i = i.wrapping_add(1);
                *buff.offset(fresh10 as isize) = 'f' as i32 as libc::c_char
            }
            10 => {
                let fresh11 = i;
                i = i.wrapping_add(1);
                *buff.offset(fresh11 as isize) = 'n' as i32 as libc::c_char
            }
            13 => {
                /* On some platforms, \n and \r are the same. */
                let fresh12 = i;
                i = i.wrapping_add(1);
                *buff.offset(fresh12 as isize) = 'r' as i32 as libc::c_char
            }
            9 => {
                let fresh13 = i;
                i = i.wrapping_add(1);
                *buff.offset(fresh13 as isize) = 't' as i32 as libc::c_char
            }
            11 => {
                let fresh14 = i;
                i = i.wrapping_add(1);
                *buff.offset(fresh14 as isize) = 'v' as i32 as libc::c_char
            }
            92 => {
                let fresh15 = i;
                i = i.wrapping_add(1);
                *buff.offset(fresh15 as isize) = '\\' as i32 as libc::c_char
            }
            _ => {
                sprintf(
                    buff.offset(i as isize),
                    b"%03o\x00" as *const u8 as *const libc::c_char,
                    0xff as libc::c_int & c as libc::c_int,
                );
                i = (i as libc::c_ulong).wrapping_add(3 as libc::c_int as libc::c_ulong) as size_t
                    as size_t
            }
        }
    }
    return i.wrapping_sub(offset);
}
#[no_mangle]
pub unsafe extern "C" fn yes(mut fmt: *const libc::c_char, mut args: ...) -> libc::c_int {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: ssize_t = 0;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(stderr, b" (y/N)? \x00" as *const u8 as *const libc::c_char);
    fflush(stderr);
    l = read(
        2 as libc::c_int,
        buff.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if l < 0 as libc::c_int as libc::c_long {
        fprintf(
            stderr,
            b"Keyboard read failed\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if l == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    buff[l as usize] = 0 as libc::c_int as libc::c_char;
    p = buff.as_mut_ptr();
    while *p as libc::c_int != '\u{0}' as i32 {
        if *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            p = p.offset(1)
        } else {
            match *p as libc::c_int {
                121 | 89 => return 1 as libc::c_int,
                110 | 78 => return 0 as libc::c_int,
                _ => return 0 as libc::c_int,
            }
        }
    }
    return 0 as libc::c_int;
}
/*-
 * The logic here for -C <dir> attempts to avoid
 * chdir() as long as possible.  For example:
 * "-C /foo -C /bar file"          needs chdir("/bar") but not chdir("/foo")
 * "-C /foo -C bar file"           needs chdir("/foo/bar")
 * "-C /foo -C bar /file1"         does not need chdir()
 * "-C /foo -C bar /file1 file2"   needs chdir("/foo/bar") before file2
 *
 * The only correct way to handle this is to record a "pending" chdir
 * request and combine multiple requests intelligently until we
 * need to process a non-absolute file.  set_chdir() adds the new dir
 * to the pending list; do_chdir() actually executes any pending chdir.
 *
 * This way, programs that build tar command lines don't have to worry
 * about -C with non-existent directories; such requests will only
 * fail if the directory must be accessed.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn set_chdir(mut bsdtar: *mut bsdtar, mut newdir: *const libc::c_char) {
    if *newdir.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        /* The -C /foo -C /bar case; dump first one. */
        free((*bsdtar).pending_chdir as *mut libc::c_void);
        (*bsdtar).pending_chdir = NULL as *mut libc::c_char
    }
    if (*bsdtar).pending_chdir.is_null() {
        /* Easy case: no previously-saved dir. */
        (*bsdtar).pending_chdir = strdup(newdir)
    } else {
        /* The -C /foo -C bar case; concatenate */
        let mut old_pending: *mut libc::c_char = (*bsdtar).pending_chdir;
        let mut old_len: size_t = strlen(old_pending);
        (*bsdtar).pending_chdir = malloc(
            old_len
                .wrapping_add(strlen(newdir))
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if *old_pending.offset(old_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            == '/' as i32
        {
            *old_pending.offset(old_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) =
                '\u{0}' as i32 as libc::c_char
        }
        if !(*bsdtar).pending_chdir.is_null() {
            sprintf(
                (*bsdtar).pending_chdir,
                b"%s/%s\x00" as *const u8 as *const libc::c_char,
                old_pending,
                newdir,
            );
        }
        free(old_pending as *mut libc::c_void);
    }
    if (*bsdtar).pending_chdir.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_chdir(mut bsdtar: *mut bsdtar) {
    if (*bsdtar).pending_chdir.is_null() {
        return;
    }
    if chdir((*bsdtar).pending_chdir) != 0 as libc::c_int {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"could not chdir to \'%s\'\n\x00" as *const u8 as *const libc::c_char,
            (*bsdtar).pending_chdir,
        );
    }
    free((*bsdtar).pending_chdir as *mut libc::c_void);
    (*bsdtar).pending_chdir = NULL as *mut libc::c_char;
}
unsafe extern "C" fn strip_components(
    mut p: *const libc::c_char,
    mut elements: libc::c_int,
) -> *const libc::c_char {
    /* Skip as many elements as necessary. */
    while elements > 0 as libc::c_int {
        let fresh16 = p;
        p = p.offset(1);
        match *fresh16 as libc::c_int {
            47 => elements -= 1,
            0 => {
                /* Path is too short, skip it. */
                return 0 as *const libc::c_char;
            }
            _ => {}
        }
    }
    loop
    /* Skip any / characters.  This handles short paths that have
     * additional / termination.  This also handles the case where
     * the logic above stops in the middle of a duplicate //
     * sequence (which would otherwise get converted to an
     * absolute path). */
    {
        match *p as libc::c_int {
            47 => p = p.offset(1),
            0 => return 0 as *const libc::c_char,
            _ => return p,
        }
    }
}
unsafe extern "C" fn warn_strip_leading_char(mut bsdtar: *mut bsdtar, mut c: *const libc::c_char) {
    if (*bsdtar).warned_lead_slash == 0 {
        lafe_warnc(
            0 as libc::c_int,
            b"Removing leading \'%c\' from member names\x00" as *const u8 as *const libc::c_char,
            *c.offset(0 as libc::c_int as isize) as libc::c_int,
        );
        (*bsdtar).warned_lead_slash = 1 as libc::c_int as libc::c_char
    };
}
unsafe extern "C" fn warn_strip_drive_letter(mut bsdtar: *mut bsdtar) {
    if (*bsdtar).warned_lead_slash == 0 {
        lafe_warnc(
            0 as libc::c_int,
            b"Removing leading drive letter from member names\x00" as *const u8
                as *const libc::c_char,
        );
        (*bsdtar).warned_lead_slash = 1 as libc::c_int as libc::c_char
    };
}
/*
 * Convert absolute path to non-absolute path by skipping leading
 * absolute path prefixes.
 */
unsafe extern "C" fn strip_absolute_path(
    mut bsdtar: *mut bsdtar,
    mut p: *const libc::c_char,
) -> *const libc::c_char {
    let mut rp: *const libc::c_char = 0 as *const libc::c_char;
    /* Remove leading "//./" or "//?/" or "//?/UNC/"
     * (absolute path prefixes used by Windows API) */
    if (*p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        && (*p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *p.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        && (*p.offset(2 as libc::c_int as isize) as libc::c_int == '.' as i32
            || *p.offset(2 as libc::c_int as isize) as libc::c_int == '?' as i32)
        && (*p.offset(3 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *p.offset(3 as libc::c_int as isize) as libc::c_int == '\\' as i32)
    {
        if *p.offset(2 as libc::c_int as isize) as libc::c_int == '?' as i32
            && (*p.offset(4 as libc::c_int as isize) as libc::c_int == 'U' as i32
                || *p.offset(4 as libc::c_int as isize) as libc::c_int == 'u' as i32)
            && (*p.offset(5 as libc::c_int as isize) as libc::c_int == 'N' as i32
                || *p.offset(5 as libc::c_int as isize) as libc::c_int == 'n' as i32)
            && (*p.offset(6 as libc::c_int as isize) as libc::c_int == 'C' as i32
                || *p.offset(6 as libc::c_int as isize) as libc::c_int == 'c' as i32)
            && (*p.offset(7 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *p.offset(7 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        {
            p = p.offset(8 as libc::c_int as isize)
        } else {
            p = p.offset(4 as libc::c_int as isize)
        }
        warn_strip_drive_letter(bsdtar);
    }
    loop
    /* Remove multiple leading slashes and Windows drive letters. */
    {
        rp = p;
        if (*p.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
            && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
            || *p.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32
                && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32)
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
        {
            p = p.offset(2 as libc::c_int as isize);
            warn_strip_drive_letter(bsdtar);
        }
        /* Remove leading "/../", "/./", "//", etc. */
        while *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
        {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '.' as i32
                && (*p.offset(3 as libc::c_int as isize) as libc::c_int == '/' as i32
                    || *p.offset(3 as libc::c_int as isize) as libc::c_int == '\\' as i32)
            {
                p = p.offset(3 as libc::c_int as isize) /* Remove "/". */
            /* Remove "/..", leave "/" for next pass. */
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && (*p.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                    || *p.offset(2 as libc::c_int as isize) as libc::c_int == '\\' as i32)
            {
                p = p.offset(2 as libc::c_int as isize)
            /* Remove "/.", leave "/" for next pass. */
            } else {
                p = p.offset(1 as libc::c_int as isize)
            }
            warn_strip_leading_char(bsdtar, rp);
        }
        if !(rp != p) {
            break;
        }
    }
    return p;
}
/*
 * Handle --strip-components and any future path-rewriting options.
 * Returns non-zero if the pathname should not be extracted.
 *
 * Note: The rewrites are applied uniformly to pathnames and hardlink
 * names but not to symlink bodies.  This is deliberate: Symlink
 * bodies are not necessarily filenames.  Even when they are, they
 * need to be interpreted relative to the directory containing them,
 * so simple rewrites like this are rarely appropriate.
 *
 * TODO: Support pax-style regex path rewrites.
 */
#[no_mangle]
pub unsafe extern "C" fn edit_pathname(
    mut bsdtar: *mut bsdtar,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut name: *const libc::c_char = archive_entry_pathname(entry);
    let mut original_name: *const libc::c_char = name;
    let mut hardlinkname: *const libc::c_char = archive_entry_hardlink(entry);
    let mut original_hardlinkname: *const libc::c_char = hardlinkname;
    let mut subst_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    /* Apply user-specified substitution to pathname. */
    r = apply_substitution(
        bsdtar,
        name,
        &mut subst_name,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if r == -(1 as libc::c_int) {
        lafe_warnc(
            0 as libc::c_int,
            b"Invalid substitution, skipping entry\x00" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if r == 1 as libc::c_int {
        archive_entry_copy_pathname(entry, subst_name);
        if *subst_name as libc::c_int == '\u{0}' as i32 {
            free(subst_name as *mut libc::c_void);
            return -(1 as libc::c_int);
        } else {
            free(subst_name as *mut libc::c_void);
        }
        name = archive_entry_pathname(entry);
        original_name = name
    }
    /* Apply user-specified substitution to hardlink target. */
    if !hardlinkname.is_null() {
        r = apply_substitution(
            bsdtar,
            hardlinkname,
            &mut subst_name,
            0 as libc::c_int,
            1 as libc::c_int,
        );
        if r == -(1 as libc::c_int) {
            lafe_warnc(
                0 as libc::c_int,
                b"Invalid substitution, skipping entry\x00" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        if r == 1 as libc::c_int {
            archive_entry_copy_hardlink(entry, subst_name);
            free(subst_name as *mut libc::c_void);
        }
        hardlinkname = archive_entry_hardlink(entry);
        original_hardlinkname = hardlinkname
    }
    /* Apply user-specified substitution to symlink body. */
    if !archive_entry_symlink(entry).is_null() {
        r = apply_substitution(
            bsdtar,
            archive_entry_symlink(entry),
            &mut subst_name,
            1 as libc::c_int,
            0 as libc::c_int,
        );
        if r == -(1 as libc::c_int) {
            lafe_warnc(
                0 as libc::c_int,
                b"Invalid substitution, skipping entry\x00" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        if r == 1 as libc::c_int {
            archive_entry_copy_symlink(entry, subst_name);
            free(subst_name as *mut libc::c_void);
        }
    }
    /* Strip leading dir names as per --strip-components option. */
    if (*bsdtar).strip_components > 0 as libc::c_int {
        name = strip_components(name, (*bsdtar).strip_components);
        if name.is_null() {
            return 1 as libc::c_int;
        }
        if !hardlinkname.is_null() {
            hardlinkname = strip_components(hardlinkname, (*bsdtar).strip_components);
            if hardlinkname.is_null() {
                return 1 as libc::c_int;
            }
        }
    }
    if (*bsdtar).flags & OPTFLAG_ABSOLUTE_PATHS as libc::c_uint == 0 as libc::c_int as libc::c_uint
    {
        /* By default, don't write or restore absolute pathnames. */
        name = strip_absolute_path(bsdtar, name);
        if *name as libc::c_int == '\u{0}' as i32 {
            name = b".\x00" as *const u8 as *const libc::c_char
        }
        if !hardlinkname.is_null() {
            hardlinkname = strip_absolute_path(bsdtar, hardlinkname);
            if *hardlinkname as libc::c_int == '\u{0}' as i32 {
                return 1 as libc::c_int;
            }
        }
    } else {
        /* Strip redundant leading '/' characters. */
        while *name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            && *name.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        {
            name = name.offset(1)
        }
    }
    /* Replace name in archive_entry. */
    if name != original_name {
        archive_entry_copy_pathname(entry, name);
    }
    if hardlinkname != original_hardlinkname {
        archive_entry_copy_hardlink(entry, hardlinkname);
    }
    return 0 as libc::c_int;
}
/*
 * It would be nice to just use printf() for formatting large numbers,
 * but the compatibility problems are quite a headache.  Hence the
 * following simple utility function.
 */
#[no_mangle]
pub unsafe extern "C" fn tar_i64toa(mut n0: int64_t) -> *const libc::c_char {
    static mut buff: [libc::c_char; 24] = [0; 24];
    let mut n: uint64_t = if n0 < 0 as libc::c_int as libc::c_long {
        -n0
    } else {
        n0
    } as uint64_t;
    let mut p: *mut libc::c_char = buff
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as isize);
    p = p.offset(-1);
    *p = '\u{0}' as i32 as libc::c_char;
    loop {
        p = p.offset(-1);
        *p = ('0' as i32 + n.wrapping_rem(10 as libc::c_int as libc::c_ulong) as libc::c_int)
            as libc::c_char;
        n = (n as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong) as uint64_t
            as uint64_t;
        if !(n != 0) {
            break;
        }
    }
    if n0 < 0 as libc::c_int as libc::c_long {
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char
    }
    return p;
}
/*
 * Like strcmp(), but try to be a little more aware of the fact that
 * we're comparing two paths.  Right now, it just handles leading
 * "./" and trailing '/' specially, so that "a/b/" == "./a/b"
 *
 * TODO: Make this better, so that "./a//b/./c/" == "a/b/c"
 * TODO: After this works, push it down into libarchive.
 * TODO: Publish the path normalization routines in libarchive so
 * that bsdtar can normalize paths and use fast strcmp() instead
 * of this.
 *
 * Note: This is currently only used within write.c, so should
 * not handle \ path separators.
 */
#[no_mangle]
pub unsafe extern "C" fn pathcmp(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    /* Skip leading './' */
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *a.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *a.offset(2 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        a = a.offset(2 as libc::c_int as isize)
    }
    if *b.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *b.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *b.offset(2 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        b = b.offset(2 as libc::c_int as isize)
    }
    /* Find the first difference, or return (0) if none. */
    while *a as libc::c_int == *b as libc::c_int {
        if *a as libc::c_int == '\u{0}' as i32 {
            return 0 as libc::c_int;
        }
        a = a.offset(1);
        b = b.offset(1)
    }
    /*
     * If one ends in '/' and the other one doesn't,
     * they're the same.
     */
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *a.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        && *b.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        return 0 as libc::c_int;
    }
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        && *b.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        && *b.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        return 0 as libc::c_int;
    }
    /* They're really different, return the correct sign. */
    return *(a as *const libc::c_uchar) as libc::c_int
        - *(b as *const libc::c_uchar) as libc::c_int;
}
pub const PPBUFF_SIZE: libc::c_int = 1024 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn passphrase_callback(
    mut a: *mut archive,
    mut _client_data: *mut libc::c_void,
) -> *const libc::c_char {
    let mut bsdtar: *mut bsdtar = _client_data as *mut bsdtar;
    /* UNUSED */
    if (*bsdtar).ppbuff.is_null() {
        (*bsdtar).ppbuff = malloc(PPBUFF_SIZE as libc::c_ulong) as *mut libc::c_char;
        if (*bsdtar).ppbuff.is_null() {
            lafe_errc(
                1 as libc::c_int,
                errno,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    return lafe_readpassphrase(
        b"Enter passphrase:\x00" as *const u8 as *const libc::c_char,
        (*bsdtar).ppbuff,
        PPBUFF_SIZE as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn passphrase_free(mut ppbuff: *mut libc::c_char) {
    if !ppbuff.is_null() {
        memset(
            ppbuff as *mut libc::c_void,
            0 as libc::c_int,
            PPBUFF_SIZE as libc::c_ulong,
        );
        free(ppbuff as *mut libc::c_void);
    };
}
/*
 * Display information about the current file.
 *
 * The format here roughly duplicates the output of 'ls -l'.
 * This is based on SUSv2, where 'tar tv' is documented as
 * listing additional information in an "unspecified format,"
 * and 'pax -l' is documented as using the same format as 'ls -l'.
 */
#[no_mangle]
pub unsafe extern "C" fn list_item_verbose(
    mut bsdtar: *mut bsdtar,
    mut out: *mut FILE,
    mut entry: *mut archive_entry,
) {
    let mut tmp: [libc::c_char; 100] = [0; 100];
    let mut w: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut tim: time_t = 0;
    static mut now: time_t = 0;
    let mut ltime: *mut tm = 0 as *mut tm;
    let mut tmbuf: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    /*
     * We avoid collecting the entire list in memory at once by
     * listing things as we see them.  However, that also means we can't
     * just pre-compute the field widths.  Instead, we start with guesses
     * and just widen them as necessary.  These numbers are completely
     * arbitrary.
     */
    if (*bsdtar).u_width == 0 {
        (*bsdtar).u_width = 6 as libc::c_int as size_t;
        (*bsdtar).gs_width = 13 as libc::c_int as size_t
    }
    if now == 0 {
        time(&mut now);
    }
    fprintf(
        out,
        b"%s %d \x00" as *const u8 as *const libc::c_char,
        archive_entry_strmode(entry),
        archive_entry_nlink(entry),
    );
    /* Use uname if it's present, else uid. */
    p = archive_entry_uname(entry);
    if p.is_null() || *p as libc::c_int == '\u{0}' as i32 {
        sprintf(
            tmp.as_mut_ptr(),
            b"%lu \x00" as *const u8 as *const libc::c_char,
            archive_entry_uid(entry) as libc::c_ulong,
        );
        p = tmp.as_mut_ptr()
    }
    w = strlen(p);
    if w > (*bsdtar).u_width {
        (*bsdtar).u_width = w
    }
    fprintf(
        out,
        b"%-*s \x00" as *const u8 as *const libc::c_char,
        (*bsdtar).u_width as libc::c_int,
        p,
    );
    /* Use gname if it's present, else gid. */
    p = archive_entry_gname(entry);
    if !p.is_null() && *p.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32 {
        fprintf(out, b"%s\x00" as *const u8 as *const libc::c_char, p);
        w = strlen(p)
    } else {
        sprintf(
            tmp.as_mut_ptr(),
            b"%lu\x00" as *const u8 as *const libc::c_char,
            archive_entry_gid(entry) as libc::c_ulong,
        );
        w = strlen(tmp.as_mut_ptr());
        fprintf(
            out,
            b"%s\x00" as *const u8 as *const libc::c_char,
            tmp.as_mut_ptr(),
        );
    }
    /*
     * Print device number or file size, right-aligned so as to make
     * total width of group and devnum/filesize fields be gs_width.
     * If gs_width is too small, grow it.
     */
    if archive_entry_filetype(entry) == AE_IFCHR as mode_t
        || archive_entry_filetype(entry) == AE_IFBLK as mode_t
    {
        sprintf(
            tmp.as_mut_ptr(),
            b"%lu,%lu\x00" as *const u8 as *const libc::c_char,
            archive_entry_rdevmajor(entry),
            archive_entry_rdevminor(entry),
        );
    } else {
        strcpy(tmp.as_mut_ptr(), tar_i64toa(archive_entry_size(entry)));
    }
    if w.wrapping_add(strlen(tmp.as_mut_ptr())) >= (*bsdtar).gs_width {
        (*bsdtar).gs_width = w
            .wrapping_add(strlen(tmp.as_mut_ptr()))
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    }
    fprintf(
        out,
        b"%*s\x00" as *const u8 as *const libc::c_char,
        (*bsdtar).gs_width.wrapping_sub(w) as libc::c_int,
        tmp.as_mut_ptr(),
    );
    /* Format the time using 'ls -l' conventions. */
    tim = archive_entry_mtime(entry);
    /* Day number without leading zeros */
    if tim < now - HALF_YEAR || tim > now + HALF_YEAR {
        fmt = if (*bsdtar).day_first as libc::c_int != 0 {
            b"%e %b  %Y\x00" as *const u8 as *const libc::c_char
        } else {
            b"%b %e  %Y\x00" as *const u8 as *const libc::c_char
        }
    } else {
        fmt = if (*bsdtar).day_first as libc::c_int != 0 {
            b"%e %b %H:%M\x00" as *const u8 as *const libc::c_char
        } else {
            b"%b %e %H:%M\x00" as *const u8 as *const libc::c_char
        }
    }
    ltime = localtime_r(&mut tim, &mut tmbuf);
    strftime(
        tmp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        fmt,
        ltime,
    );
    fprintf(
        out,
        b" %s \x00" as *const u8 as *const libc::c_char,
        tmp.as_mut_ptr(),
    );
    safe_fprintf(
        out,
        b"%s\x00" as *const u8 as *const libc::c_char,
        archive_entry_pathname(entry),
    );
    /* Extra information for links. */
    if !archive_entry_hardlink(entry).is_null() {
        /* Hard link */
        safe_fprintf(
            out,
            b" link to %s\x00" as *const u8 as *const libc::c_char,
            archive_entry_hardlink(entry),
        );
    } else if !archive_entry_symlink(entry).is_null() {
        /* Symbolic link */
        safe_fprintf(
            out,
            b" -> %s\x00" as *const u8 as *const libc::c_char,
            archive_entry_symlink(entry),
        );
    };
}
pub const HALF_YEAR: libc::c_long = 365 as libc::c_int as time_t
    * 86400 as libc::c_int as libc::c_long
    / 2 as libc::c_int as libc::c_long;
