use ::c2rust_bitfields;
use ::libc;
extern "C" {
    /* Declare our basic types. */
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
    pub type archive;
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
    pub type siginfo_data;
    pub type name_cache;
    pub type security;
    pub type archive_dir;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regexec(
        __preg: *const regex_t,
        __string: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn regerror(
        __errcode: libc::c_int,
        __preg: *const regex_t,
        __errbuf: *mut libc::c_char,
        __errbuf_size: size_t,
    ) -> size_t;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lafe_errc(eval: libc::c_int, code: libc::c_int, fmt: *const libc::c_char, _: ...) -> !;
}
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uid_t = __uid_t;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct substitution {
    pub first_rule: *mut subst_rule,
    pub last_rule: *mut subst_rule,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct subst_rule {
    pub next: *mut subst_rule,
    pub re: regex_t,
    pub result: *mut libc::c_char,
    #[bitfield(name = "global", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "print", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "regular", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "symlink", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "hardlink", ty = "libc::c_uint", bits = "4..=4")]
    pub global_print_regular_symlink_hardlink: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut libc::c_uchar,
    pub allocated: libc::c_ulong,
    pub used: libc::c_ulong,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type reg_syntax_t = libc::c_ulong;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const REG_BASIC: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn init_substitution(mut bsdtar: *mut bsdtar) {
    let mut subst: *mut substitution = 0 as *mut substitution;
    subst = malloc(::std::mem::size_of::<substitution>() as libc::c_ulong) as *mut substitution;
    (*bsdtar).substitution = subst;
    if subst.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*subst).last_rule = NULL as *mut subst_rule;
    (*subst).first_rule = (*subst).last_rule;
}
#[no_mangle]
pub unsafe extern "C" fn add_substitution(
    mut bsdtar: *mut bsdtar,
    mut rule_text: *const libc::c_char,
) {
    let mut rule: *mut subst_rule = 0 as *mut subst_rule;
    let mut subst: *mut substitution = 0 as *mut substitution;
    let mut end_pattern: *const libc::c_char = 0 as *const libc::c_char;
    let mut start_subst: *const libc::c_char = 0 as *const libc::c_char;
    let mut pattern: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: libc::c_int = 0;
    subst = (*bsdtar).substitution;
    if subst.is_null() {
        init_substitution(bsdtar);
        subst = (*bsdtar).substitution
    }
    rule = malloc(::std::mem::size_of::<subst_rule>() as libc::c_ulong) as *mut subst_rule;
    if rule.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*rule).next = NULL as *mut subst_rule;
    (*rule).result = NULL as *mut libc::c_char;
    if (*subst).last_rule.is_null() {
        (*subst).first_rule = rule
    } else {
        (*(*subst).last_rule).next = rule
    }
    (*subst).last_rule = rule;
    if *rule_text as libc::c_int == '\u{0}' as i32 {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Empty replacement string\x00" as *const u8 as *const libc::c_char,
        );
    }
    end_pattern = strchr(
        rule_text.offset(1 as libc::c_int as isize),
        *rule_text as libc::c_int,
    );
    if end_pattern.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Invalid replacement string\x00" as *const u8 as *const libc::c_char,
        );
    }
    pattern = malloc(end_pattern.offset_from(rule_text) as libc::c_long as libc::c_ulong)
        as *mut libc::c_char;
    if pattern.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        pattern as *mut libc::c_void,
        rule_text.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (end_pattern.offset_from(rule_text) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
    );
    *pattern.offset(
        (end_pattern.offset_from(rule_text) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as isize,
    ) = '\u{0}' as i32 as libc::c_char;
    r = regcomp(&mut (*rule).re, pattern, REG_BASIC);
    if r != 0 as libc::c_int {
        let mut buf: [libc::c_char; 80] = [0; 80];
        regerror(
            r,
            &mut (*rule).re,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong,
        );
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Invalid regular expression: %s\x00" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    free(pattern as *mut libc::c_void);
    start_subst = end_pattern.offset(1 as libc::c_int as isize);
    end_pattern = strchr(start_subst, *rule_text as libc::c_int);
    if end_pattern.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Invalid replacement string\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*rule).result = malloc(
        (end_pattern.offset_from(start_subst) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
    ) as *mut libc::c_char;
    if (*rule).result.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        (*rule).result as *mut libc::c_void,
        start_subst as *const libc::c_void,
        end_pattern.offset_from(start_subst) as libc::c_long as libc::c_ulong,
    );
    *(*rule)
        .result
        .offset(end_pattern.offset_from(start_subst) as libc::c_long as isize) =
        '\u{0}' as i32 as libc::c_char;
    /* Defaults */
    (*rule).set_global(0 as libc::c_int as libc::c_uint); /* Don't do multiple replacements. */
    (*rule).set_print(0 as libc::c_int as libc::c_uint); /* Don't print. */
    (*rule).set_regular(1 as libc::c_int as libc::c_uint); /* Rewrite regular filenames. */
    (*rule).set_symlink(1 as libc::c_int as libc::c_uint); /* Rewrite symlink targets. */
    (*rule).set_hardlink(1 as libc::c_int as libc::c_uint); /* Rewrite hardlink targets. */
    loop {
        end_pattern = end_pattern.offset(1); /* Regular filename. */
        if !(*end_pattern != 0) {
            break;
        }
        match *end_pattern as libc::c_int {
            103 | 71 => (*rule).set_global(1 as libc::c_int as libc::c_uint),
            104 => (*rule).set_hardlink(1 as libc::c_int as libc::c_uint),
            72 => (*rule).set_hardlink(0 as libc::c_int as libc::c_uint),
            112 | 80 => (*rule).set_print(1 as libc::c_int as libc::c_uint),
            114 => (*rule).set_regular(1 as libc::c_int as libc::c_uint),
            82 => (*rule).set_regular(0 as libc::c_int as libc::c_uint),
            115 => (*rule).set_symlink(1 as libc::c_int as libc::c_uint),
            83 => (*rule).set_symlink(0 as libc::c_int as libc::c_uint),
            _ => {
                lafe_errc(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"Invalid replacement flag %c\x00" as *const u8 as *const libc::c_char,
                    *end_pattern as libc::c_int,
                );
            }
        }
    }
}
unsafe extern "C" fn realloc_strncat(
    mut str: *mut *mut libc::c_char,
    mut append: *const libc::c_char,
    mut len: size_t,
) {
    let mut new_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_len: size_t = 0;
    if (*str).is_null() {
        old_len = 0 as libc::c_int as size_t
    } else {
        old_len = strlen(*str)
    }
    new_str = malloc(
        old_len
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if new_str.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !(*str).is_null() {
        memcpy(
            new_str as *mut libc::c_void,
            *str as *const libc::c_void,
            old_len,
        );
    }
    memcpy(
        new_str.offset(old_len as isize) as *mut libc::c_void,
        append as *const libc::c_void,
        len,
    );
    *new_str.offset(old_len.wrapping_add(len) as isize) = '\u{0}' as i32 as libc::c_char;
    free(*str as *mut libc::c_void);
    *str = new_str;
}
unsafe extern "C" fn realloc_strcat(
    mut str: *mut *mut libc::c_char,
    mut append: *const libc::c_char,
) {
    let mut new_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_len: size_t = 0;
    if (*str).is_null() {
        old_len = 0 as libc::c_int as size_t
    } else {
        old_len = strlen(*str)
    }
    new_str = malloc(
        old_len
            .wrapping_add(strlen(append))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if new_str.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !(*str).is_null() {
        memcpy(
            new_str as *mut libc::c_void,
            *str as *const libc::c_void,
            old_len,
        );
    }
    strcpy(new_str.offset(old_len as isize), append);
    free(*str as *mut libc::c_void);
    *str = new_str;
}
#[no_mangle]
pub unsafe extern "C" fn apply_substitution(
    mut bsdtar: *mut bsdtar,
    mut name: *const libc::c_char,
    mut result: *mut *mut libc::c_char,
    mut symlink_target: libc::c_int,
    mut hardlink_target: libc::c_int,
) -> libc::c_int {
    let mut path: *const libc::c_char = name;
    let mut matches: [regmatch_t; 10] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 10];
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut rule: *mut subst_rule = 0 as *mut subst_rule;
    let mut subst: *mut substitution = 0 as *mut substitution;
    let mut c: libc::c_int = 0;
    let mut got_match: libc::c_int = 0;
    let mut print_match: libc::c_int = 0;
    *result = NULL as *mut libc::c_char;
    subst = (*bsdtar).substitution;
    if subst.is_null() {
        return 0 as libc::c_int;
    }
    got_match = 0 as libc::c_int;
    print_match = 0 as libc::c_int;
    let mut current_block_29: u64;
    rule = (*subst).first_rule;
    while !rule.is_null() {
        if symlink_target != 0 {
            if (*rule).symlink() == 0 {
                current_block_29 = 17216689946888361452;
            } else {
                current_block_29 = 15976848397966268834;
            }
        } else if hardlink_target != 0 {
            if (*rule).hardlink() == 0 {
                current_block_29 = 17216689946888361452;
            } else {
                current_block_29 = 15976848397966268834;
            }
        } else if (*rule).regular() == 0 {
            current_block_29 = 17216689946888361452;
        } else {
            current_block_29 = 15976848397966268834;
        }
        match current_block_29 {
            15976848397966268834 => {
                while !(regexec(
                    &mut (*rule).re,
                    name,
                    10 as libc::c_int as size_t,
                    matches.as_mut_ptr(),
                    0 as libc::c_int,
                ) != 0)
                {
                    got_match = 1 as libc::c_int;
                    print_match |= (*rule).print() as libc::c_int;
                    realloc_strncat(
                        result,
                        name,
                        matches[0 as libc::c_int as usize].rm_so as size_t,
                    );
                    i = 0 as libc::c_int as size_t;
                    j = 0 as libc::c_int as size_t;
                    while *(*rule).result.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
                        if *(*rule).result.offset(i as isize) as libc::c_int == '~' as i32 {
                            realloc_strncat(
                                result,
                                (*rule).result.offset(j as isize),
                                i.wrapping_sub(j),
                            );
                            realloc_strncat(
                                result,
                                name.offset(matches[0 as libc::c_int as usize].rm_so as isize),
                                (matches[0 as libc::c_int as usize].rm_eo
                                    - matches[0 as libc::c_int as usize].rm_so)
                                    as size_t,
                            );
                            j = i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else if !(*(*rule).result.offset(i as isize) as libc::c_int
                            != '\\' as i32)
                        {
                            i = i.wrapping_add(1);
                            c = *(*rule).result.offset(i as isize) as libc::c_int;
                            match c {
                                126 | 92 => {
                                    realloc_strncat(
                                        result,
                                        (*rule).result.offset(j as isize),
                                        i.wrapping_sub(j)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    );
                                    j = i
                                }
                                49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                                    realloc_strncat(
                                        result,
                                        (*rule).result.offset(j as isize),
                                        i.wrapping_sub(j)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    );
                                    if (c - '0' as i32) as size_t > (*rule).re.re_nsub {
                                        free(*result as *mut libc::c_void);
                                        *result = NULL as *mut libc::c_char;
                                        return -(1 as libc::c_int);
                                    }
                                    realloc_strncat(
                                        result,
                                        name.offset(
                                            matches[(c - '0' as i32) as usize].rm_so as isize,
                                        ),
                                        (matches[(c - '0' as i32) as usize].rm_eo
                                            - matches[(c - '0' as i32) as usize].rm_so)
                                            as size_t,
                                    );
                                    j = i.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                }
                                _ => {}
                            }
                        }
                        i = i.wrapping_add(1)
                    }
                    realloc_strcat(result, (*rule).result.offset(j as isize));
                    name = name.offset(matches[0 as libc::c_int as usize].rm_eo as isize);
                    if (*rule).global() == 0 {
                        break;
                    }
                }
            }
            _ => {}
        }
        rule = (*rule).next
    }
    if got_match != 0 {
        realloc_strcat(result, name);
    }
    if print_match != 0 {
        fprintf(
            stderr,
            b"%s >> %s\n\x00" as *const u8 as *const libc::c_char,
            path,
            *result,
        );
    }
    return got_match;
}
#[no_mangle]
pub unsafe extern "C" fn cleanup_substitution(mut bsdtar: *mut bsdtar) {
    let mut rule: *mut subst_rule = 0 as *mut subst_rule;
    let mut subst: *mut substitution = 0 as *mut substitution;
    subst = (*bsdtar).substitution;
    if subst.is_null() {
        return;
    }
    loop {
        rule = (*subst).first_rule;
        if rule.is_null() {
            break;
        }
        (*subst).first_rule = (*rule).next;
        free((*rule).result as *mut libc::c_void);
        free(rule as *mut libc::c_void);
    }
    free(subst as *mut libc::c_void);
}
/* defined(HAVE_REGEX_H) || defined(HAVE_PCREPOSIX_H) */
