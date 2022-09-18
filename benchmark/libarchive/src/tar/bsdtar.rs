#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(
    const_raw_ptr_to_usize_cast,
    const_transmute,
    extern_types,
    main,
    register_tool
)]
use ::rust::*;
extern "C" {
    /* Declare our basic types. */
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
    pub type creation_set;
    pub type substitution;
    pub type siginfo_data;
    pub type name_cache;
    pub type security;
    pub type archive_dir;
    /*
     * Detailed textual name/version of the library and its dependencies.
     * This has the form:
     *    "libarchive x.y.z zlib/a.b.c liblzma/d.e.f ... etc ..."
     * the list of libraries described here will vary depending on how
     * libarchive was compiled.
     */
    #[no_mangle]
    fn archive_version_details() -> *const libc::c_char;
    #[no_mangle]
    fn archive_error_string(_: *mut archive) -> *const libc::c_char;
    /*
     * ARCHIVE_MATCH API
     */
    #[no_mangle]
    fn archive_match_new() -> *mut archive;
    #[no_mangle]
    fn archive_match_free(_: *mut archive) -> libc::c_int;
    /* Control recursive inclusion of directory content when directory is included. Default on. */
    #[no_mangle]
    fn archive_match_set_inclusion_recursion(_: *mut archive, _: libc::c_int) -> libc::c_int;
    /* Add exclusion pathname pattern. */
    #[no_mangle]
    fn archive_match_exclude_pattern(_: *mut archive, _: *const libc::c_char) -> libc::c_int;
    /* Add exclusion pathname pattern from file. */
    #[no_mangle]
    fn archive_match_exclude_pattern_from_file(
        _: *mut archive,
        _: *const libc::c_char,
        _nullSeparator: libc::c_int,
    ) -> libc::c_int;
    /* Add inclusion pathname pattern. */
    #[no_mangle]
    fn archive_match_include_pattern(_: *mut archive, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn geteuid() -> __uid_t;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn putchar(__c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    /* Set inclusion time by a date string. */
    #[no_mangle]
    fn archive_match_include_date(
        _: *mut archive,
        _flag: libc::c_int,
        _datestr: *const libc::c_char,
    ) -> libc::c_int;
    /* Set inclusion time by a particular file. */
    #[no_mangle]
    fn archive_match_include_file_time(
        _: *mut archive,
        _flag: libc::c_int,
        _pathname: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn bsdtar_getopt(_: *mut bsdtar) -> libc::c_int;
    #[no_mangle]
    fn set_chdir(_: *mut bsdtar, newdir: *const libc::c_char);
    #[no_mangle]
    fn tar_mode_c(bsdtar: *mut bsdtar);
    #[no_mangle]
    fn tar_mode_r(bsdtar: *mut bsdtar);
    #[no_mangle]
    fn tar_mode_t(bsdtar: *mut bsdtar);
    #[no_mangle]
    fn tar_mode_u(bsdtar: *mut bsdtar);
    #[no_mangle]
    fn tar_mode_x(bsdtar: *mut bsdtar);
    #[no_mangle]
    fn add_substitution(_: *mut bsdtar, _: *const libc::c_char);
    #[no_mangle]
    fn cleanup_substitution(_: *mut bsdtar);
    #[no_mangle]
    fn cset_add_filter(_: *mut creation_set, _: *const libc::c_char);
    #[no_mangle]
    fn cset_add_filter_program(_: *mut creation_set, _: *const libc::c_char);
    #[no_mangle]
    fn cset_auto_compress(_: *mut creation_set, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn cset_free(_: *mut creation_set);
    #[no_mangle]
    fn cset_get_format(_: *mut creation_set) -> *const libc::c_char;
    #[no_mangle]
    fn cset_new() -> *mut creation_set;
    #[no_mangle]
    fn cset_set_format(_: *mut creation_set, _: *const libc::c_char);
    #[no_mangle]
    fn passphrase_free(_: *mut libc::c_char);
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
    #[no_mangle]
    fn lafe_getprogname() -> *const libc::c_char;
    #[no_mangle]
    fn lafe_setprogname(name: *const libc::c_char, defaultname: *const libc::c_char);
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(_: libc::c_int, _: *mut siginfo_t, _: *mut libc::c_void) -> ()>,
}
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
/* --mac-metadata */
/* Fake short equivalents for long options that otherwise lack them. */
pub type C2RustUnnamed_10 = libc::c_uint;
pub const OPTION_ZSTD: C2RustUnnamed_10 = 57;
pub const OPTION_XATTRS: C2RustUnnamed_10 = 56;
pub const OPTION_VERSION: C2RustUnnamed_10 = 55;
pub const OPTION_UUENCODE: C2RustUnnamed_10 = 54;
pub const OPTION_USE_COMPRESS_PROGRAM: C2RustUnnamed_10 = 53;
pub const OPTION_UNAME: C2RustUnnamed_10 = 52;
pub const OPTION_UID: C2RustUnnamed_10 = 51;
pub const OPTION_TOTALS: C2RustUnnamed_10 = 50;
pub const OPTION_STRIP_COMPONENTS: C2RustUnnamed_10 = 49;
pub const OPTION_SAME_OWNER: C2RustUnnamed_10 = 48;
pub const OPTION_SAFE_WRITES: C2RustUnnamed_10 = 47;
pub const OPTION_POSIX: C2RustUnnamed_10 = 46;
pub const OPTION_PASSPHRASE: C2RustUnnamed_10 = 45;
pub const OPTION_OPTIONS: C2RustUnnamed_10 = 44;
pub const OPTION_ONE_FILE_SYSTEM: C2RustUnnamed_10 = 43;
pub const OPTION_OLDER_MTIME_THAN: C2RustUnnamed_10 = 42;
pub const OPTION_OLDER_MTIME: C2RustUnnamed_10 = 41;
pub const OPTION_OLDER_CTIME_THAN: C2RustUnnamed_10 = 40;
pub const OPTION_OLDER_CTIME: C2RustUnnamed_10 = 39;
pub const OPTION_NUMERIC_OWNER: C2RustUnnamed_10 = 38;
pub const OPTION_NULL: C2RustUnnamed_10 = 37;
pub const OPTION_NO_XATTRS: C2RustUnnamed_10 = 36;
pub const OPTION_NO_SAME_PERMISSIONS: C2RustUnnamed_10 = 35;
pub const OPTION_NO_SAME_OWNER: C2RustUnnamed_10 = 34;
pub const OPTION_NO_SAFE_WRITES: C2RustUnnamed_10 = 33;
pub const OPTION_NO_MAC_METADATA: C2RustUnnamed_10 = 32;
pub const OPTION_NO_FFLAGS: C2RustUnnamed_10 = 31;
pub const OPTION_NO_ACLS: C2RustUnnamed_10 = 30;
pub const OPTION_NOPRESERVE_HFS_COMPRESSION: C2RustUnnamed_10 = 29;
pub const OPTION_NODUMP: C2RustUnnamed_10 = 28;
pub const OPTION_NEWER_MTIME_THAN: C2RustUnnamed_10 = 27;
pub const OPTION_NEWER_MTIME: C2RustUnnamed_10 = 26;
pub const OPTION_NEWER_CTIME_THAN: C2RustUnnamed_10 = 25;
pub const OPTION_NEWER_CTIME: C2RustUnnamed_10 = 24;
pub const OPTION_MAC_METADATA: C2RustUnnamed_10 = 23;
pub const OPTION_LZOP: C2RustUnnamed_10 = 22;
pub const OPTION_LZMA: C2RustUnnamed_10 = 21;
pub const OPTION_LZIP: C2RustUnnamed_10 = 20;
pub const OPTION_LZ4: C2RustUnnamed_10 = 19;
pub const OPTION_LRZIP: C2RustUnnamed_10 = 18;
pub const OPTION_KEEP_NEWER_FILES: C2RustUnnamed_10 = 17;
pub const OPTION_INCLUDE: C2RustUnnamed_10 = 16;
pub const OPTION_IGNORE_ZEROS: C2RustUnnamed_10 = 15;
pub const OPTION_HFS_COMPRESSION: C2RustUnnamed_10 = 14;
pub const OPTION_HELP: C2RustUnnamed_10 = 13;
pub const OPTION_GRZIP: C2RustUnnamed_10 = 12;
pub const OPTION_GNAME: C2RustUnnamed_10 = 11;
pub const OPTION_GID: C2RustUnnamed_10 = 10;
pub const OPTION_FORMAT: C2RustUnnamed_10 = 9;
pub const OPTION_FFLAGS: C2RustUnnamed_10 = 8;
pub const OPTION_EXCLUDE_VCS: C2RustUnnamed_10 = 7;
pub const OPTION_EXCLUDE: C2RustUnnamed_10 = 6;
pub const OPTION_CLEAR_NOCHANGE_FFLAGS: C2RustUnnamed_10 = 5;
pub const OPTION_CHROOT: C2RustUnnamed_10 = 4;
pub const OPTION_CHECK_LINKS: C2RustUnnamed_10 = 3;
pub const OPTION_B64ENCODE: C2RustUnnamed_10 = 2;
pub const OPTION_ACLS: C2RustUnnamed_10 = 1;
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
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
pub const ARCHIVE_EXTRACT_OWNER: libc::c_int = 0x1 as libc::c_int;
/* Default: Do obey umask, do not restore SUID/SGID/SVTX bits. */
pub const ARCHIVE_EXTRACT_PERM: libc::c_int = 0x2 as libc::c_int;
/* Default: Do not restore mtime/atime. */
pub const ARCHIVE_EXTRACT_TIME: libc::c_int = 0x4 as libc::c_int;
/* Default: Replace existing files. */
pub const ARCHIVE_EXTRACT_NO_OVERWRITE: libc::c_int = 0x8 as libc::c_int;
/* Default: Try create first, unlink only if create fails with EEXIST. */
pub const ARCHIVE_EXTRACT_UNLINK: libc::c_int = 0x10 as libc::c_int;
/* Default: Do not restore ACLs. */
pub const ARCHIVE_EXTRACT_ACL: libc::c_int = 0x20 as libc::c_int;
/* Default: Do not restore fflags. */
pub const ARCHIVE_EXTRACT_FFLAGS: libc::c_int = 0x40 as libc::c_int;
/* Default: Do not restore xattrs. */
pub const ARCHIVE_EXTRACT_XATTR: libc::c_int = 0x80 as libc::c_int;
/* Default: Do not try to guard against extracts redirected by symlinks. */
/* Note: With ARCHIVE_EXTRACT_UNLINK, will remove any intermediate symlink. */
pub const ARCHIVE_EXTRACT_SECURE_SYMLINKS: libc::c_int = 0x100 as libc::c_int;
/* Default: Do not reject entries with '..' as path elements. */
pub const ARCHIVE_EXTRACT_SECURE_NODOTDOT: libc::c_int = 0x200 as libc::c_int;
/* Default: Create parent directories as needed. */
/* Default: Overwrite files, even if one on disk is newer. */
pub const ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER: libc::c_int = 0x800 as libc::c_int;
/* Detect blocks of 0 and write holes instead. */
pub const ARCHIVE_EXTRACT_SPARSE: libc::c_int = 0x1000 as libc::c_int;
/* Default: Do not restore Mac extended metadata. */
/* This has no effect except on Mac OS. */
pub const ARCHIVE_EXTRACT_MAC_METADATA: libc::c_int = 0x2000 as libc::c_int;
/* Default: Use HFS+ compression if it was compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
pub const ARCHIVE_EXTRACT_NO_HFS_COMPRESSION: libc::c_int = 0x4000 as libc::c_int;
/* Default: Do not use HFS+ compression if it was not compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
pub const ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED: libc::c_int = 0x8000 as libc::c_int;
/* Default: Do not reject entries with absolute paths */
/* Default: Do not clear no-change flags when unlinking object */
pub const ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS: libc::c_int = 0x20000 as libc::c_int;
/* Default: Do not extract atomically (using rename) */
pub const ARCHIVE_EXTRACT_SAFE_WRITES: libc::c_int = 0x40000 as libc::c_int;
/*
 * Set behavior. The "flags" argument selects optional behavior.
 */
/* Request that the access time of the entry visited by traversal be restored.
 * This is the same as archive_read_disk_set_atime_restored. */
/* Default: Do not skip an entry which has nodump flags. */
pub const ARCHIVE_READDISK_HONOR_NODUMP: libc::c_int = 0x2 as libc::c_int;
/* Default: Skip a mac resource fork file whose prefix is "._" because of
 * using copyfile. */
pub const ARCHIVE_READDISK_MAC_COPYFILE: libc::c_int = 0x4 as libc::c_int;
/* Default: Traverse mount points. */
pub const ARCHIVE_READDISK_NO_TRAVERSE_MOUNTS: libc::c_int = 0x8 as libc::c_int;
/* Default: Xattrs are read from disk. */
pub const ARCHIVE_READDISK_NO_XATTR: libc::c_int = 0x10 as libc::c_int;
/* Default: ACLs are read from disk. */
pub const ARCHIVE_READDISK_NO_ACL: libc::c_int = 0x20 as libc::c_int;
/* Default: File flags are read from disk. */
pub const ARCHIVE_READDISK_NO_FFLAGS: libc::c_int = 0x40 as libc::c_int;
pub const ARCHIVE_MATCH_NEWER: libc::c_int = 0x1 as libc::c_int;
pub const ARCHIVE_MATCH_CTIME: libc::c_int = 0x200 as libc::c_int;
pub const ARCHIVE_MATCH_MTIME: libc::c_int = 0x100 as libc::c_int;
pub const ARCHIVE_MATCH_OLDER: libc::c_int = 0x2 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
/* config.h.  Generated from config.h.in by configure.  */
/* config.h.in.  Generated from configure.ac by autoheader.  */
/* Darwin ACL support */
/* #undef ARCHIVE_ACL_DARWIN */
/* FreeBSD ACL support */
/* #undef ARCHIVE_ACL_FREEBSD */
/* FreeBSD NFSv4 ACL support */
/* #undef ARCHIVE_ACL_FREEBSD_NFS4 */
/* Linux POSIX.1e ACL support via libacl */
/* #undef ARCHIVE_ACL_LIBACL */
/* Linux NFSv4 ACL support via librichacl */
/* #undef ARCHIVE_ACL_LIBRICHACL */
/* Solaris ACL support */
/* #undef ARCHIVE_ACL_SUNOS */
/* Solaris NFSv4 ACL support */
/* #undef ARCHIVE_ACL_SUNOS_NFS4 */
/* MD5 via ARCHIVE_CRYPTO_MD5_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_MD5_LIBC */
/* MD5 via ARCHIVE_CRYPTO_MD5_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_MD5_LIBMD */
/* MD5 via ARCHIVE_CRYPTO_MD5_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_MD5_LIBSYSTEM */
/* MD5 via ARCHIVE_CRYPTO_MD5_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_MD5_MBEDTLS */
/* MD5 via ARCHIVE_CRYPTO_MD5_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_MD5_NETTLE */
/* MD5 via ARCHIVE_CRYPTO_MD5_OPENSSL supported. */
/* MD5 via ARCHIVE_CRYPTO_MD5_WIN supported. */
/* #undef ARCHIVE_CRYPTO_MD5_WIN */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_RMD160_LIBC */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_RMD160_LIBMD */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_RMD160_MBEDTLS */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_RMD160_NETTLE */
/* RMD160 via ARCHIVE_CRYPTO_RMD160_OPENSSL supported. */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_LIBC */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_LIBMD */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_LIBSYSTEM */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_MBEDTLS */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_NETTLE */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_OPENSSL supported. */
/* SHA1 via ARCHIVE_CRYPTO_SHA1_WIN supported. */
/* #undef ARCHIVE_CRYPTO_SHA1_WIN */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBC */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBC2 supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBC2 */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBC3 supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBC3 */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBMD */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_LIBSYSTEM */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_MBEDTLS */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_NETTLE */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_OPENSSL supported. */
/* SHA256 via ARCHIVE_CRYPTO_SHA256_WIN supported. */
/* #undef ARCHIVE_CRYPTO_SHA256_WIN */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_LIBC */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_LIBC2 supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_LIBC2 */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_LIBC3 supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_LIBC3 */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_LIBSYSTEM */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_MBEDTLS */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_NETTLE */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_OPENSSL supported. */
/* SHA384 via ARCHIVE_CRYPTO_SHA384_WIN supported. */
/* #undef ARCHIVE_CRYPTO_SHA384_WIN */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBC supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBC */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBC2 supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBC2 */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBC3 supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBC3 */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBMD supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBMD */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_LIBSYSTEM supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_LIBSYSTEM */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_MBEDTLS supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_MBEDTLS */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_NETTLE supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_NETTLE */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_OPENSSL supported. */
/* SHA512 via ARCHIVE_CRYPTO_SHA512_WIN supported. */
/* #undef ARCHIVE_CRYPTO_SHA512_WIN */
/* AIX xattr support */
/* #undef ARCHIVE_XATTR_AIX */
/* Darwin xattr support */
/* #undef ARCHIVE_XATTR_DARWIN */
/* FreeBSD xattr support */
/* #undef ARCHIVE_XATTR_FREEBSD */
/* Linux xattr support */
/* Version number of bsdcat */
/* Version number of bsdcpio */
/* Version number of bsdtar */
pub const BSDTAR_VERSION_STRING: [libc::c_char; 6] =
    unsafe { *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"3.4.3\x00") };
pub const SIGPIPE: libc::c_int = 13 as libc::c_int;
pub const SIG_IGN: libc::c_int = 1 as libc::c_int;
pub const SIGUSR1: libc::c_int = 10 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const __LC_ALL: libc::c_int = 6 as libc::c_int;
pub const LC_ALL: libc::c_int = __LC_ALL;
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
pub const DEFAULT_BYTES_PER_BLOCK: libc::c_int = 20 as libc::c_int * 512 as libc::c_int;
/* for util.c */
/* Options for flags bitfield */
pub const OPTFLAG_AUTO_COMPRESS: libc::c_int = 0x1 as libc::c_int;
/* -a */
pub const OPTFLAG_ABSOLUTE_PATHS: libc::c_int = 0x2 as libc::c_int;
/* -P */
pub const OPTFLAG_CHROOT: libc::c_int = 0x4 as libc::c_int;
/* --chroot */
pub const OPTFLAG_FAST_READ: libc::c_int = 0x8 as libc::c_int;
/* --fast-read */
pub const OPTFLAG_IGNORE_ZEROS: libc::c_int = 0x10 as libc::c_int;
/* --ignore-zeros */
pub const OPTFLAG_INTERACTIVE: libc::c_int = 0x20 as libc::c_int;
/* -w */
pub const OPTFLAG_NO_OWNER: libc::c_int = 0x40 as libc::c_int;
/* -o */
pub const OPTFLAG_NO_SUBDIRS: libc::c_int = 0x80 as libc::c_int;
/* -n */
pub const OPTFLAG_NULL: libc::c_int = 0x100 as libc::c_int;
/* --null */
pub const OPTFLAG_NUMERIC_OWNER: libc::c_int = 0x200 as libc::c_int;
/* --numeric-owner */
pub const OPTFLAG_O: libc::c_int = 0x400 as libc::c_int;
/* -o */
pub const OPTFLAG_STDOUT: libc::c_int = 0x800 as libc::c_int;
/* -O */
pub const OPTFLAG_TOTALS: libc::c_int = 0x1000 as libc::c_int;
/* --totals */
pub const OPTFLAG_UNLINK_FIRST: libc::c_int = 0x2000 as libc::c_int;
/* -U */
pub const OPTFLAG_WARN_LINKS: libc::c_int = 0x4000 as libc::c_int;
/* --check-links */
pub const OPTFLAG_NO_XATTRS: libc::c_int = 0x8000 as libc::c_int;
/* --no-xattrs */
pub const OPTFLAG_XATTRS: libc::c_int = 0x10000 as libc::c_int;
/* --xattrs */
pub const OPTFLAG_NO_ACLS: libc::c_int = 0x20000 as libc::c_int;
/* --no-acls */
pub const OPTFLAG_ACLS: libc::c_int = 0x40000 as libc::c_int;
/* --acls */
pub const OPTFLAG_NO_FFLAGS: libc::c_int = 0x80000 as libc::c_int;
/* --no-fflags */
pub const OPTFLAG_FFLAGS: libc::c_int = 0x100000 as libc::c_int;
/* --fflags */
pub const OPTFLAG_NO_MAC_METADATA: libc::c_int = 0x200000 as libc::c_int;
/* --no-mac-metadata */
pub const OPTFLAG_MAC_METADATA: libc::c_int = 0x400000 as libc::c_int;
/*
 * Per POSIX.1-1988, tar defaults to reading/writing archives to/from
 * the default tape device for the system.  Pick something reasonable here.
 */
pub const _PATH_DEFTAPE: [libc::c_char; 9] =
    unsafe { *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"/dev/st0\x00") };
static mut siginfo_occurred: libc::c_int = 0;
unsafe extern "C" fn siginfo_handler(mut sig: libc::c_int) {
    /* UNUSED */
    ::std::ptr::write_volatile(&mut siginfo_occurred as *mut libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn need_report() -> libc::c_int {
    let mut r: libc::c_int = siginfo_occurred;
    ::std::ptr::write_volatile(&mut siginfo_occurred as *mut libc::c_int, 0 as libc::c_int);
    return r;
}
/* A basic set of security flags to request from libarchive. */
pub const SECURITY: libc::c_int = ARCHIVE_EXTRACT_SECURE_SYMLINKS | ARCHIVE_EXTRACT_SECURE_NODOTDOT;
static mut vcs_files: [*const libc::c_char; 22] = [
    b"CVS\x00" as *const u8 as *const libc::c_char,
    b".cvsignore\x00" as *const u8 as *const libc::c_char,
    b"RCS\x00" as *const u8 as *const libc::c_char,
    b"SCCS\x00" as *const u8 as *const libc::c_char,
    b".svn\x00" as *const u8 as *const libc::c_char,
    b".git\x00" as *const u8 as *const libc::c_char,
    b".gitignore\x00" as *const u8 as *const libc::c_char,
    b".gitattributes\x00" as *const u8 as *const libc::c_char,
    b".gitmodules\x00" as *const u8 as *const libc::c_char,
    b".arch-ids\x00" as *const u8 as *const libc::c_char,
    b"{arch}\x00" as *const u8 as *const libc::c_char,
    b"=RELEASE-ID\x00" as *const u8 as *const libc::c_char,
    b"=meta-update\x00" as *const u8 as *const libc::c_char,
    b"=update\x00" as *const u8 as *const libc::c_char,
    b".bzr\x00" as *const u8 as *const libc::c_char,
    b".bzrignore\x00" as *const u8 as *const libc::c_char,
    b".bzrtags\x00" as *const u8 as *const libc::c_char,
    b".hg\x00" as *const u8 as *const libc::c_char,
    b".hgignore\x00" as *const u8 as *const libc::c_char,
    b".hgtags\x00" as *const u8 as *const libc::c_char,
    b"_darcs\x00" as *const u8 as *const libc::c_char,
    NULL as *const libc::c_char,
];
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut bsdtar: *mut bsdtar = 0 as *mut bsdtar;
    let mut bsdtar_storage: bsdtar = bsdtar {
        filename: 0 as *const libc::c_char,
        pending_chdir: 0 as *mut libc::c_char,
        names_from_file: 0 as *const libc::c_char,
        bytes_per_block: 0,
        bytes_in_last_block: 0,
        verbose: 0,
        flags: 0,
        extract_flags: 0,
        readdisk_flags: 0,
        strip_components: 0,
        gid: 0,
        gname: 0 as *const libc::c_char,
        uid: 0,
        uname: 0 as *const libc::c_char,
        passphrase: 0 as *const libc::c_char,
        mode: 0,
        symlink_mode: 0,
        option_options: 0 as *const libc::c_char,
        day_first: 0,
        cset: 0 as *mut creation_set,
        getopt_state: 0,
        getopt_word: 0 as *mut libc::c_char,
        fd: 0,
        argc: 0,
        argv: 0 as *mut *mut libc::c_char,
        argument: 0 as *const libc::c_char,
        gs_width: 0,
        u_width: 0,
        user_uid: 0,
        return_value: 0,
        warned_lead_slash: 0,
        next_line_is_dir: 0,
        diskreader: 0 as *mut archive,
        resolver: 0 as *mut archive_entry_linkresolver,
        archive_dir: 0 as *mut archive_dir,
        gname_cache: 0 as *mut name_cache,
        buff: 0 as *mut libc::c_char,
        buff_size: 0,
        first_fs: 0,
        matching: 0 as *mut archive,
        security: 0 as *mut security,
        uname_cache: 0 as *mut name_cache,
        siginfo: 0 as *mut siginfo_data,
        substitution: 0 as *mut substitution,
        ppbuff: 0 as *mut libc::c_char,
    };
    let mut opt: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut compression: libc::c_char = 0;
    let mut compression2: libc::c_char = 0;
    let mut compression_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut compression2_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut compress_program: *const libc::c_char = 0 as *const libc::c_char;
    let mut tptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut possible_help_request: libc::c_char = 0;
    let mut buff: [libc::c_char; 16] = [0; 16];
    /*
     * Use a pointer for consistency, but stack-allocated storage
     * for ease of cleanup.
     */
    bsdtar = &mut bsdtar_storage; /* Mark as "unused" */
    memset(
        bsdtar as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<bsdtar>() as libc::c_ulong,
    );
    (*bsdtar).fd = -(1 as libc::c_int);
    (*bsdtar).gid = -(1 as libc::c_int);
    (*bsdtar).uid = -(1 as libc::c_int);
    (*bsdtar).flags = 0 as libc::c_int as libc::c_uint;
    compression2 = '\u{0}' as i32 as libc::c_char;
    compression = compression2;
    compression2_name = NULL as *const libc::c_char;
    compression_name = compression2_name;
    compress_program = NULL as *const libc::c_char;
    /* Set up signal handling. */
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sa.__sigaction_handler.sa_handler =
        Some(siginfo_handler as unsafe extern "C" fn(_: libc::c_int) -> ());
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0 as libc::c_int;
    /* ... and treat SIGUSR1 the same way as SIGINFO. */
    if sigaction(SIGUSR1, &mut sa, NULL as *mut sigaction) != 0 {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"sigaction(SIGUSR1) failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* Ignore SIGPIPE signals. */
    sa.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(SIG_IGN as libc::intptr_t);
    sigaction(SIGPIPE, &mut sa, NULL as *mut sigaction);
    /* Set lafe_progname before calling lafe_warnc. */
    lafe_setprogname(*argv, b"bsdtar\x00" as *const u8 as *const libc::c_char);
    if setlocale(LC_ALL, b"\x00" as *const u8 as *const libc::c_char).is_null() {
        lafe_warnc(
            0 as libc::c_int,
            b"Failed to set default locale\x00" as *const u8 as *const libc::c_char,
        );
    }
    possible_help_request = 0 as libc::c_int as libc::c_char;
    /* Look up uid of current user for future reference */
    (*bsdtar).user_uid = geteuid();
    /* Default: open tape drive. */
    (*bsdtar).filename = getenv(b"TAPE\x00" as *const u8 as *const libc::c_char);
    if (*bsdtar).filename.is_null() {
        (*bsdtar).filename = _PATH_DEFTAPE.as_ptr()
    }
    /* Default block size settings. */
    (*bsdtar).bytes_per_block = DEFAULT_BYTES_PER_BLOCK;
    /* Allow library to default this unless user specifies -b. */
    (*bsdtar).bytes_in_last_block = -(1 as libc::c_int);
    /* Default: preserve mod time on extract */
    (*bsdtar).extract_flags = ARCHIVE_EXTRACT_TIME;
    /* Default: Perform basic security checks. */
    (*bsdtar).extract_flags |= SECURITY;
    /* On POSIX systems, assume --same-owner and -p when run by
     * the root user.  This doesn't make any sense on Windows. */
    if (*bsdtar).user_uid == 0 as libc::c_int as libc::c_uint {
        /* --same-owner */
        (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_OWNER;
        /* -p */
        (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_PERM;
        (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_ACL;
        (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_XATTR;
        (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_FFLAGS;
        (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_MAC_METADATA
    }
    /*
     * Enable Mac OS "copyfile()" extension by default.
     * This has no effect on other platforms.
     */
    (*bsdtar).readdisk_flags |= ARCHIVE_READDISK_MAC_COPYFILE;
    (*bsdtar).matching = archive_match_new();
    if (*bsdtar).matching.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*bsdtar).cset = cset_new();
    if (*bsdtar).cset.is_null() {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*bsdtar).argv = argv;
    (*bsdtar).argc = argc;
    loop
    /*
     * Comments following each option indicate where that option
     * originated:  SUSv2, POSIX, GNU tar, star, etc.  If there's
     * no such comment, then I don't know of anyone else who
     * implements that option.
     */
    {
        opt = bsdtar_getopt(bsdtar);
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_238: u64;
        match opt {
            97 => {
                /* GNU tar */
                (*bsdtar).flags |= OPTFLAG_AUTO_COMPRESS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            1 => {
                /* GNU tar */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_ACL;
                (*bsdtar).readdisk_flags &= !ARCHIVE_READDISK_NO_ACL;
                (*bsdtar).flags |= OPTFLAG_ACLS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            66 => {
                current_block_238 = 16667653778084529680;
            }
            98 => {
                /* SUSv2 */
                errno = 0 as libc::c_int;
                tptr = NULL as *mut libc::c_char;
                t = strtol((*bsdtar).argument, &mut tptr, 10 as libc::c_int) as libc::c_int;
                if errno != 0
                    || t <= 0 as libc::c_int
                    || t > 8192 as libc::c_int
                    || *(*bsdtar).argument as libc::c_int == '\u{0}' as i32
                    || tptr.is_null()
                    || *tptr as libc::c_int != '\u{0}' as i32
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Invalid or out of range (1..8192) argument to -b\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*bsdtar).bytes_per_block = 512 as libc::c_int * t;
                /* Explicit -b forces last block size. */
                (*bsdtar).bytes_in_last_block = (*bsdtar).bytes_per_block;
                current_block_238 = 16667653778084529680;
            }
            2 => {
                if compression2 as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both --uuencode and --b64encode\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                compression2 = opt as libc::c_char;
                compression2_name = b"b64encode\x00" as *const u8 as *const libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            67 => {
                /* GNU tar */
                if strlen((*bsdtar).argument) == 0 as libc::c_int as libc::c_ulong {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Meaningless option: -C \'\'\x00" as *const u8 as *const libc::c_char,
                    );
                }
                set_chdir(bsdtar, (*bsdtar).argument);
                current_block_238 = 16667653778084529680;
            }
            99 => {
                /* SUSv2 */
                set_mode(bsdtar, opt as libc::c_char);
                current_block_238 = 16667653778084529680;
            }
            3 => {
                /* GNU tar */
                (*bsdtar).flags |= OPTFLAG_WARN_LINKS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            4 => {
                /* NetBSD */
                (*bsdtar).flags |= OPTFLAG_CHROOT as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            5 => {
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS;
                current_block_238 = 16667653778084529680;
            }
            6 => {
                /* GNU tar */
                if archive_match_exclude_pattern((*bsdtar).matching, (*bsdtar).argument)
                    != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Couldn\'t exclude %s\n\x00" as *const u8 as *const libc::c_char,
                        (*bsdtar).argument,
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            7 => {
                /* GNU tar */
                t = 0 as libc::c_int;
                while !vcs_files[t as usize].is_null() {
                    if archive_match_exclude_pattern((*bsdtar).matching, vcs_files[t as usize])
                        != ARCHIVE_OK
                    {
                        lafe_errc(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            b"Couldn\'t exclude %s\n\x00" as *const u8 as *const libc::c_char,
                            vcs_files[t as usize],
                        );
                    }
                    t += 1
                }
                current_block_238 = 16667653778084529680;
            }
            8 => {
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_FFLAGS;
                (*bsdtar).readdisk_flags &= !ARCHIVE_READDISK_NO_FFLAGS;
                (*bsdtar).flags |= OPTFLAG_FFLAGS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            9 => {
                /* GNU tar, others */
                cset_set_format((*bsdtar).cset, (*bsdtar).argument);
                current_block_238 = 16667653778084529680;
            }
            102 => {
                /* SUSv2 */
                (*bsdtar).filename = (*bsdtar).argument;
                current_block_238 = 16667653778084529680;
            }
            10 => {
                /* cpio */
                errno = 0 as libc::c_int;
                tptr = NULL as *mut libc::c_char;
                t = strtol((*bsdtar).argument, &mut tptr, 10 as libc::c_int) as libc::c_int;
                if errno != 0
                    || t < 0 as libc::c_int
                    || *(*bsdtar).argument as libc::c_int == '\u{0}' as i32
                    || tptr.is_null()
                    || *tptr as libc::c_int != '\u{0}' as i32
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Invalid argument to --gid\x00" as *const u8 as *const libc::c_char,
                    );
                }
                (*bsdtar).gid = t;
                current_block_238 = 16667653778084529680;
            }
            11 => {
                /* cpio */
                (*bsdtar).gname = (*bsdtar).argument;
                current_block_238 = 16667653778084529680;
            }
            12 => {
                if compression as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both -%c and -%c\x00" as *const u8 as *const libc::c_char,
                        opt,
                        compression as libc::c_int,
                    );
                }
                compression = opt as libc::c_char;
                compression_name = b"grzip\x00" as *const u8 as *const libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            72 => {
                /* BSD convention */
                (*bsdtar).symlink_mode = 'H' as i32 as libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            104 => {
                /* Linux Standards Base, gtar; synonym for -L */
                (*bsdtar).symlink_mode = 'L' as i32 as libc::c_char;
                /* Hack: -h by itself is the "help" command. */
                possible_help_request = 1 as libc::c_int as libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            13 => {
                /* GNU tar, others */
                long_help();
            }
            14 => {
                /* Mac OS X v10.6 or later */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED;
                current_block_238 = 16667653778084529680;
            }
            15 => {
                (*bsdtar).flags |= OPTFLAG_IGNORE_ZEROS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            73 => {
                /* GNU tar */
                /*
                 * TODO: Allow 'names' to come from an archive,
                 * not just a text file.  Design a good UI for
                 * allowing names and mode/owner to be read
                 * from an archive, with contents coming from
                 * disk.  This can be used to "refresh" an
                 * archive or to design archives with special
                 * permissions without having to create those
                 * permissions on disk.
                 */
                (*bsdtar).names_from_file = (*bsdtar).argument;
                current_block_238 = 16667653778084529680;
            }
            16 => {
                /*
                 * No one else has the @archive extension, so
                 * no one else needs this to filter entries
                 * when transforming archives.
                 */
                if archive_match_include_pattern((*bsdtar).matching, (*bsdtar).argument)
                    != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Failed to add %s to inclusion list\x00" as *const u8
                            as *const libc::c_char,
                        (*bsdtar).argument,
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            106 => {
                /* GNU tar */
                if compression as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both -%c and -%c\x00" as *const u8 as *const libc::c_char,
                        opt,
                        compression as libc::c_int,
                    );
                }
                compression = opt as libc::c_char;
                compression_name = b"bzip2\x00" as *const u8 as *const libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            74 => {
                /* GNU tar 1.21 and later */
                if compression as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both -%c and -%c\x00" as *const u8 as *const libc::c_char,
                        opt,
                        compression as libc::c_int,
                    );
                }
                compression = opt as libc::c_char;
                compression_name = b"xz\x00" as *const u8 as *const libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            107 => {
                /* GNU tar */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_NO_OVERWRITE;
                current_block_238 = 16667653778084529680;
            }
            17 => {
                /* GNU tar */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER;
                current_block_238 = 16667653778084529680;
            }
            76 => {
                /* BSD convention */
                (*bsdtar).symlink_mode = 'L' as i32 as libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            108 => {
                /* SUSv2 and GNU tar beginning with 1.16 */
                /* GNU tar 1.13  used -l for --one-file-system */
                (*bsdtar).flags |= OPTFLAG_WARN_LINKS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            18 | 19 | 20 => {
                /* GNU tar beginning with 1.23 */
                current_block_238 = 14452517466194687152;
            }
            21 => {
                current_block_238 = 14452517466194687152;
            }
            22 | 57 => {
                current_block_238 = 11708925467940708064;
            }
            109 => {
                /* SUSv2 */
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_TIME;
                current_block_238 = 16667653778084529680;
            }
            23 => {
                /* Mac OS X */
                (*bsdtar).readdisk_flags |= ARCHIVE_READDISK_MAC_COPYFILE;
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_MAC_METADATA;
                (*bsdtar).flags |= OPTFLAG_MAC_METADATA as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            110 => {
                /* GNU tar */
                (*bsdtar).flags |= OPTFLAG_NO_SUBDIRS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            24 => {
                /*
                 * Selecting files by time:
                 *    --newer-?time='date' Only files newer than 'date'
                 *    --newer-?time-than='file' Only files newer than time
                 *         on specified file (useful for incremental backups)
                 */
                /* GNU tar */
                if archive_match_include_date(
                    (*bsdtar).matching,
                    ARCHIVE_MATCH_CTIME | ARCHIVE_MATCH_NEWER,
                    (*bsdtar).argument,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            25 => {
                if archive_match_include_file_time(
                    (*bsdtar).matching,
                    ARCHIVE_MATCH_CTIME | ARCHIVE_MATCH_NEWER,
                    (*bsdtar).argument,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            26 => {
                /* GNU tar */
                if archive_match_include_date(
                    (*bsdtar).matching,
                    ARCHIVE_MATCH_MTIME | ARCHIVE_MATCH_NEWER,
                    (*bsdtar).argument,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            27 => {
                if archive_match_include_file_time(
                    (*bsdtar).matching,
                    ARCHIVE_MATCH_MTIME | ARCHIVE_MATCH_NEWER,
                    (*bsdtar).argument,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            28 => {
                /* star */
                (*bsdtar).readdisk_flags |= ARCHIVE_READDISK_HONOR_NODUMP;
                current_block_238 = 16667653778084529680;
            }
            29 => {
                /* Mac OS X v10.6 or later */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_NO_HFS_COMPRESSION;
                current_block_238 = 16667653778084529680;
            }
            30 => {
                /* GNU tar */
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_ACL;
                (*bsdtar).readdisk_flags |= ARCHIVE_READDISK_NO_ACL;
                (*bsdtar).flags |= OPTFLAG_NO_ACLS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            31 => {
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_FFLAGS;
                (*bsdtar).readdisk_flags |= ARCHIVE_READDISK_NO_FFLAGS;
                (*bsdtar).flags |= OPTFLAG_NO_FFLAGS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            32 => {
                /* Mac OS X */
                (*bsdtar).readdisk_flags &= !ARCHIVE_READDISK_MAC_COPYFILE;
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_MAC_METADATA;
                (*bsdtar).flags |= OPTFLAG_NO_MAC_METADATA as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            33 => {
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_SAFE_WRITES;
                current_block_238 = 16667653778084529680;
            }
            34 => {
                /* GNU tar */
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_OWNER;
                current_block_238 = 16667653778084529680;
            }
            35 => {
                /* GNU tar */
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_PERM;
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_ACL;
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_XATTR;
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_FFLAGS;
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_MAC_METADATA;
                current_block_238 = 16667653778084529680;
            }
            36 => {
                /* GNU tar */
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_XATTR;
                (*bsdtar).readdisk_flags |= ARCHIVE_READDISK_NO_XATTR;
                (*bsdtar).flags |= OPTFLAG_NO_XATTRS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            37 => {
                /* GNU tar */
                (*bsdtar).flags |= OPTFLAG_NULL as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            38 => {
                /* GNU tar */
                (*bsdtar).uname = b"\x00" as *const u8 as *const libc::c_char;
                (*bsdtar).gname = b"\x00" as *const u8 as *const libc::c_char;
                (*bsdtar).flags |= OPTFLAG_NUMERIC_OWNER as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            79 => {
                /* GNU tar */
                (*bsdtar).flags |= OPTFLAG_STDOUT as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            111 => {
                /* SUSv2 and GNU conflict here, but not fatally */
                (*bsdtar).flags |= OPTFLAG_O as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            39 => {
                /*
                 * Selecting files by time:
                 *    --older-?time='date' Only files older than 'date'
                 *    --older-?time-than='file' Only files older than time
                 *         on specified file
                 */
                if archive_match_include_date(
                    (*bsdtar).matching,
                    ARCHIVE_MATCH_CTIME | ARCHIVE_MATCH_OLDER,
                    (*bsdtar).argument,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            40 => {
                if archive_match_include_file_time(
                    (*bsdtar).matching,
                    ARCHIVE_MATCH_CTIME | ARCHIVE_MATCH_OLDER,
                    (*bsdtar).argument,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            41 => {
                if archive_match_include_date(
                    (*bsdtar).matching,
                    ARCHIVE_MATCH_MTIME | ARCHIVE_MATCH_OLDER,
                    (*bsdtar).argument,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            42 => {
                if archive_match_include_file_time(
                    (*bsdtar).matching,
                    ARCHIVE_MATCH_MTIME | ARCHIVE_MATCH_OLDER,
                    (*bsdtar).argument,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            43 => {
                /* GNU tar */
                (*bsdtar).readdisk_flags |= ARCHIVE_READDISK_NO_TRAVERSE_MOUNTS;
                current_block_238 = 16667653778084529680;
            }
            44 => {
                (*bsdtar).option_options = (*bsdtar).argument;
                current_block_238 = 16667653778084529680;
            }
            80 => {
                /* GNU tar */
                (*bsdtar).extract_flags &= !SECURITY;
                (*bsdtar).flags |= OPTFLAG_ABSOLUTE_PATHS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            112 => {
                /* GNU tar, star */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_PERM;
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_ACL;
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_XATTR;
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_FFLAGS;
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_MAC_METADATA;
                current_block_238 = 16667653778084529680;
            }
            45 => {
                (*bsdtar).passphrase = (*bsdtar).argument;
                current_block_238 = 16667653778084529680;
            }
            46 => {
                /* GNU tar */
                cset_set_format(
                    (*bsdtar).cset,
                    b"pax\x00" as *const u8 as *const libc::c_char,
                );
                current_block_238 = 16667653778084529680;
            }
            113 => {
                /* FreeBSD GNU tar --fast-read, NetBSD -q */
                (*bsdtar).flags |= OPTFLAG_FAST_READ as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            114 => {
                /* SUSv2 */
                set_mode(bsdtar, opt as libc::c_char);
                current_block_238 = 16667653778084529680;
            }
            83 => {
                /* NetBSD pax-as-tar */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_SPARSE;
                current_block_238 = 16667653778084529680;
            }
            115 => {
                /* NetBSD pax-as-tar */
                add_substitution(bsdtar, (*bsdtar).argument);
                current_block_238 = 16667653778084529680;
            }
            47 => {
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_SAFE_WRITES;
                current_block_238 = 16667653778084529680;
            }
            48 => {
                /* GNU tar */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_OWNER;
                current_block_238 = 16667653778084529680;
            }
            49 => {
                /* GNU tar 1.15 */
                errno = 0 as libc::c_int;
                tptr = NULL as *mut libc::c_char;
                t = strtol((*bsdtar).argument, &mut tptr, 10 as libc::c_int) as libc::c_int;
                if errno != 0
                    || t < 0 as libc::c_int
                    || *(*bsdtar).argument as libc::c_int == '\u{0}' as i32
                    || tptr.is_null()
                    || *tptr as libc::c_int != '\u{0}' as i32
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Invalid argument to --strip-components\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*bsdtar).strip_components = t;
                current_block_238 = 16667653778084529680;
            }
            84 => {
                /* GNU tar */
                (*bsdtar).names_from_file = (*bsdtar).argument;
                current_block_238 = 16667653778084529680;
            }
            116 => {
                /* SUSv2 */
                set_mode(bsdtar, opt as libc::c_char);
                (*bsdtar).verbose += 1;
                current_block_238 = 16667653778084529680;
            }
            50 => {
                /* GNU tar */
                (*bsdtar).flags |= OPTFLAG_TOTALS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            85 => {
                /* GNU tar */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_UNLINK;
                (*bsdtar).flags |= OPTFLAG_UNLINK_FIRST as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            117 => {
                /* SUSv2 */
                set_mode(bsdtar, opt as libc::c_char);
                current_block_238 = 16667653778084529680;
            }
            51 => {
                /* cpio */
                errno = 0 as libc::c_int;
                tptr = NULL as *mut libc::c_char;
                t = strtol((*bsdtar).argument, &mut tptr, 10 as libc::c_int) as libc::c_int;
                if errno != 0
                    || t < 0 as libc::c_int
                    || *(*bsdtar).argument as libc::c_int == '\u{0}' as i32
                    || tptr.is_null()
                    || *tptr as libc::c_int != '\u{0}' as i32
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Invalid argument to --uid\x00" as *const u8 as *const libc::c_char,
                    );
                }
                (*bsdtar).uid = t;
                current_block_238 = 16667653778084529680;
            }
            52 => {
                /* cpio */
                (*bsdtar).uname = (*bsdtar).argument;
                current_block_238 = 16667653778084529680;
            }
            54 => {
                if compression2 as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both --uuencode and --b64encode\x00" as *const u8
                            as *const libc::c_char,
                    );
                }
                compression2 = opt as libc::c_char;
                compression2_name = b"uuencode\x00" as *const u8 as *const libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            118 => {
                /* SUSv2 */
                (*bsdtar).verbose += 1;
                current_block_238 = 16667653778084529680;
            }
            55 => {
                /* GNU convention */
                version();
            }
            119 => {
                /* SUSv2 */
                (*bsdtar).flags |= OPTFLAG_INTERACTIVE as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            88 => {
                /* GNU tar */
                if archive_match_exclude_pattern_from_file(
                    (*bsdtar).matching,
                    (*bsdtar).argument,
                    0 as libc::c_int,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*bsdtar).matching),
                    );
                }
                current_block_238 = 16667653778084529680;
            }
            120 => {
                /* SUSv2 */
                set_mode(bsdtar, opt as libc::c_char);
                current_block_238 = 16667653778084529680;
            }
            56 => {
                /* GNU tar */
                (*bsdtar).extract_flags |= ARCHIVE_EXTRACT_XATTR;
                (*bsdtar).readdisk_flags &= !ARCHIVE_READDISK_NO_XATTR;
                (*bsdtar).flags |= OPTFLAG_XATTRS as libc::c_uint;
                current_block_238 = 16667653778084529680;
            }
            121 => {
                /* FreeBSD version of GNU tar */
                if compression as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both -%c and -%c\x00" as *const u8 as *const libc::c_char,
                        opt,
                        compression as libc::c_int,
                    );
                }
                compression = opt as libc::c_char;
                compression_name = b"bzip2\x00" as *const u8 as *const libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            90 => {
                /* GNU tar */
                if compression as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both -%c and -%c\x00" as *const u8 as *const libc::c_char,
                        opt,
                        compression as libc::c_int,
                    );
                }
                compression = opt as libc::c_char;
                compression_name = b"compress\x00" as *const u8 as *const libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            122 => {
                /* GNU tar, star, many others */
                if compression as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both -%c and -%c\x00" as *const u8 as *const libc::c_char,
                        opt,
                        compression as libc::c_int,
                    );
                }
                compression = opt as libc::c_char;
                compression_name = b"gzip\x00" as *const u8 as *const libc::c_char;
                current_block_238 = 16667653778084529680;
            }
            53 => {
                compress_program = (*bsdtar).argument;
                current_block_238 = 16667653778084529680;
            }
            _ => {
                usage();
            }
        }
        match current_block_238 {
            14452517466194687152 =>
            /* GNU tar beginning with 1.20 */
            {
                current_block_238 = 11708925467940708064;
            }
            _ => {}
        }
        match current_block_238 {
            11708925467940708064 =>
            /* GNU tar beginning with 1.21 */
            {
                if compression as libc::c_int != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Can\'t specify both -%c and -%c\x00" as *const u8 as *const libc::c_char,
                        opt,
                        compression as libc::c_int,
                    );
                }
                compression = opt as libc::c_char;
                match opt {
                    18 => compression_name = b"lrzip\x00" as *const u8 as *const libc::c_char,
                    19 => compression_name = b"lz4\x00" as *const u8 as *const libc::c_char,
                    20 => compression_name = b"lzip\x00" as *const u8 as *const libc::c_char,
                    21 => compression_name = b"lzma\x00" as *const u8 as *const libc::c_char,
                    22 => compression_name = b"lzop\x00" as *const u8 as *const libc::c_char,
                    57 => compression_name = b"zstd\x00" as *const u8 as *const libc::c_char,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    /*
     * Sanity-check options.
     */
    /* If no "real" mode was specified, treat -h as --help. */
    if (*bsdtar).mode as libc::c_int == '\u{0}' as i32 && possible_help_request as libc::c_int != 0
    {
        long_help();
    }
    /* Otherwise, a mode is required. */
    if (*bsdtar).mode as libc::c_int == '\u{0}' as i32 {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Must specify one of -c, -r, -t, -u, -x\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* Check boolean options only permitted in certain modes. */
    if (*bsdtar).flags & OPTFLAG_AUTO_COMPRESS as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"-a\x00" as *const u8 as *const libc::c_char,
            b"c\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).readdisk_flags & ARCHIVE_READDISK_NO_TRAVERSE_MOUNTS != 0 {
        only_mode(
            bsdtar,
            b"--one-file-system\x00" as *const u8 as *const libc::c_char,
            b"cru\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_FAST_READ as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--fast-read\x00" as *const u8 as *const libc::c_char,
            b"xt\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).extract_flags & ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED != 0 {
        only_mode(
            bsdtar,
            b"--hfsCompression\x00" as *const u8 as *const libc::c_char,
            b"x\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).extract_flags & ARCHIVE_EXTRACT_NO_HFS_COMPRESSION != 0 {
        only_mode(
            bsdtar,
            b"--nopreserveHFSCompression\x00" as *const u8 as *const libc::c_char,
            b"x\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).readdisk_flags & ARCHIVE_READDISK_HONOR_NODUMP != 0 {
        only_mode(
            bsdtar,
            b"--nodump\x00" as *const u8 as *const libc::c_char,
            b"cru\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_ACLS as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--acls\x00" as *const u8 as *const libc::c_char,
            b"crux\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_NO_ACLS as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--no-acls\x00" as *const u8 as *const libc::c_char,
            b"crux\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_XATTRS as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--xattrs\x00" as *const u8 as *const libc::c_char,
            b"crux\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_NO_XATTRS as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--no-xattrs\x00" as *const u8 as *const libc::c_char,
            b"crux\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_FFLAGS as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--fflags\x00" as *const u8 as *const libc::c_char,
            b"crux\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_NO_FFLAGS as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--no-fflags\x00" as *const u8 as *const libc::c_char,
            b"crux\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_MAC_METADATA as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--mac-metadata\x00" as *const u8 as *const libc::c_char,
            b"crux\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_NO_MAC_METADATA as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--no-mac-metadata\x00" as *const u8 as *const libc::c_char,
            b"crux\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_O as libc::c_uint != 0 {
        match (*bsdtar).mode as libc::c_int {
            99 => {
                /*
                 * In GNU tar, -o means "old format."  The
                 * "ustar" format is the closest thing
                 * supported by libarchive.
                 */
                cset_set_format(
                    (*bsdtar).cset,
                    b"ustar\x00" as *const u8 as *const libc::c_char,
                );
            }
            120 => {
                /* POSIX-compatible behavior. */
                (*bsdtar).flags |= OPTFLAG_NO_OWNER as libc::c_uint;
                (*bsdtar).extract_flags &= !ARCHIVE_EXTRACT_OWNER
            }
            _ => {
                only_mode(
                    bsdtar,
                    b"-o\x00" as *const u8 as *const libc::c_char,
                    b"xc\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    if (*bsdtar).flags & OPTFLAG_STDOUT as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"-O\x00" as *const u8 as *const libc::c_char,
            b"xt\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_UNLINK_FIRST as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"-U\x00" as *const u8 as *const libc::c_char,
            b"x\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_WARN_LINKS as libc::c_uint != 0 {
        only_mode(
            bsdtar,
            b"--check-links\x00" as *const u8 as *const libc::c_char,
            b"cr\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_AUTO_COMPRESS as libc::c_uint != 0
        && cset_auto_compress((*bsdtar).cset, (*bsdtar).filename) != 0
    {
        /* Ignore specified compressions if auto-compress works. */
        compression = '\u{0}' as i32 as libc::c_char;
        compression2 = '\u{0}' as i32 as libc::c_char
    }
    /* Check other parameters only permitted in certain modes. */
    if !compress_program.is_null() {
        only_mode(
            bsdtar,
            b"--use-compress-program\x00" as *const u8 as *const libc::c_char,
            b"cxt\x00" as *const u8 as *const libc::c_char,
        );
        cset_add_filter_program((*bsdtar).cset, compress_program);
        /* Ignore specified compressions. */
        compression = '\u{0}' as i32 as libc::c_char;
        compression2 = '\u{0}' as i32 as libc::c_char
    }
    if compression as libc::c_int != '\u{0}' as i32 {
        match compression as libc::c_int {
            74 | 106 | 121 | 90 | 122 => {
                strcpy(
                    buff.as_mut_ptr(),
                    b"-?\x00" as *const u8 as *const libc::c_char,
                );
                buff[1 as libc::c_int as usize] = compression
            }
            _ => {
                strcpy(
                    buff.as_mut_ptr(),
                    b"--\x00" as *const u8 as *const libc::c_char,
                );
                strcat(buff.as_mut_ptr(), compression_name);
            }
        }
        only_mode(
            bsdtar,
            buff.as_mut_ptr(),
            b"cxt\x00" as *const u8 as *const libc::c_char,
        );
        cset_add_filter((*bsdtar).cset, compression_name);
    }
    if compression2 as libc::c_int != '\u{0}' as i32 {
        strcpy(
            buff.as_mut_ptr(),
            b"--\x00" as *const u8 as *const libc::c_char,
        );
        strcat(buff.as_mut_ptr(), compression2_name);
        only_mode(
            bsdtar,
            buff.as_mut_ptr(),
            b"cxt\x00" as *const u8 as *const libc::c_char,
        );
        cset_add_filter((*bsdtar).cset, compression2_name);
    }
    if !cset_get_format((*bsdtar).cset).is_null() {
        only_mode(
            bsdtar,
            b"--format\x00" as *const u8 as *const libc::c_char,
            b"cru\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).symlink_mode as libc::c_int != '\u{0}' as i32 {
        strcpy(
            buff.as_mut_ptr(),
            b"-?\x00" as *const u8 as *const libc::c_char,
        );
        buff[1 as libc::c_int as usize] = (*bsdtar).symlink_mode;
        only_mode(
            bsdtar,
            buff.as_mut_ptr(),
            b"cru\x00" as *const u8 as *const libc::c_char,
        );
    }
    /*
     * When creating an archive from a directory tree, the directory
     * walking code will already avoid entering directories when
     * recursive inclusion of directory content is disabled, therefore
     * changing the matching behavior has no effect for creation modes.
     * It is relevant for extraction or listing.
     */
    archive_match_set_inclusion_recursion(
        (*bsdtar).matching,
        ((*bsdtar).flags & OPTFLAG_NO_SUBDIRS as libc::c_uint == 0) as libc::c_int,
    );
    /* Filename "-" implies stdio. */
    if strcmp(
        (*bsdtar).filename,
        b"-\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        (*bsdtar).filename = NULL as *const libc::c_char
    }
    match (*bsdtar).mode as libc::c_int {
        99 => {
            tar_mode_c(bsdtar);
        }
        114 => {
            tar_mode_r(bsdtar);
        }
        116 => {
            tar_mode_t(bsdtar);
        }
        117 => {
            tar_mode_u(bsdtar);
        }
        120 => {
            tar_mode_x(bsdtar);
        }
        _ => {}
    }
    archive_match_free((*bsdtar).matching);
    cleanup_substitution(bsdtar);
    cset_free((*bsdtar).cset);
    passphrase_free((*bsdtar).ppbuff);
    if (*bsdtar).return_value != 0 as libc::c_int {
        lafe_warnc(
            0 as libc::c_int,
            b"Error exit delayed from previous errors.\x00" as *const u8 as *const libc::c_char,
        );
    }
    return (*bsdtar).return_value;
}
unsafe extern "C" fn set_mode(mut bsdtar: *mut bsdtar, mut opt: libc::c_char) {
    if (*bsdtar).mode as libc::c_int != '\u{0}' as i32
        && (*bsdtar).mode as libc::c_int != opt as libc::c_int
    {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Can\'t specify both -%c and -%c\x00" as *const u8 as *const libc::c_char,
            opt as libc::c_int,
            (*bsdtar).mode as libc::c_int,
        );
    }
    (*bsdtar).mode = opt;
}
/*
 * Verify that the mode is correct.
 */
unsafe extern "C" fn only_mode(
    mut bsdtar: *mut bsdtar,
    mut opt: *const libc::c_char,
    mut valid_modes: *const libc::c_char,
) {
    if strchr(valid_modes, (*bsdtar).mode as libc::c_int).is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Option %s is not permitted in mode -%c\x00" as *const u8 as *const libc::c_char,
            opt,
            (*bsdtar).mode as libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn usage() -> ! {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = lafe_getprogname();
    fprintf(stderr, b"Usage:\n\x00" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"  List:    %s -tf <archive-filename>\n\x00" as *const u8 as *const libc::c_char,
        p,
    );
    fprintf(
        stderr,
        b"  Extract: %s -xf <archive-filename>\n\x00" as *const u8 as *const libc::c_char,
        p,
    );
    fprintf(
        stderr,
        b"  Create:  %s -cf <archive-filename> [filenames...]\n\x00" as *const u8
            as *const libc::c_char,
        p,
    );
    fprintf(
        stderr,
        b"  Help:    %s --help\n\x00" as *const u8 as *const libc::c_char,
        p,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn version() -> ! {
    printf(
        b"bsdtar %s - %s \n\x00" as *const u8 as *const libc::c_char,
        BSDTAR_VERSION_STRING.as_ptr(),
        archive_version_details(),
    );
    exit(0 as libc::c_int);
}
static mut long_help_msg: *const libc::c_char =
    b"First option must be a mode specifier:\n  -c Create  -r Add/Replace  -t List  -u Update  -x Extract\nCommon Options:\n  -b #  Use # 512-byte records per I/O block\n  -f <filename>  Location of archive (default /dev/st0)\n  -v    Verbose\n  -w    Interactive\nCreate: %p -c [options] [<file> | <dir> | @<archive> | -C <dir> ]\n  <file>, <dir>  add these items to archive\n  -z, -j, -J, --lzma  Compress archive with gzip/bzip2/xz/lzma\n  --format {ustar|pax|cpio|shar}  Select archive format\n  --exclude <pattern>  Skip files that match pattern\n  -C <dir>  Change to <dir> before processing remaining files\n  @<archive>  Add entries from <archive> to output\nList: %p -t [options] [<patterns>]\n  <patterns>  If specified, list only entries that match\nExtract: %p -x [options] [<patterns>]\n  <patterns>  If specified, extract only entries that match\n  -k    Keep (don\'t overwrite) existing files\n  -m    Don\'t restore modification times\n  -O    Write entries to stdout, don\'t restore to disk\n  -p    Restore permissions (including ACLs, owner, file flags)\n\x00"
        as *const u8 as *const libc::c_char;
/*
 * Note that the word 'bsdtar' will always appear in the first line
 * of output.
 *
 * In particular, /bin/sh scripts that need to test for the presence
 * of bsdtar can use the following template:
 *
 * if (tar --help 2>&1 | grep bsdtar >/dev/null 2>&1 ) then \
 *          echo bsdtar; else echo not bsdtar; fi
 */
unsafe extern "C" fn long_help() -> ! {
    let mut prog: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    prog = lafe_getprogname();
    fflush(stderr);
    p = if strcmp(prog, b"bsdtar\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
        b"(bsdtar)\x00" as *const u8 as *const libc::c_char
    } else {
        b"\x00" as *const u8 as *const libc::c_char
    };
    printf(
        b"%s%s: manipulate archive files\n\x00" as *const u8 as *const libc::c_char,
        prog,
        p,
    );
    p = long_help_msg;
    while *p as libc::c_int != '\u{0}' as i32 {
        if *p as libc::c_int == '%' as i32 {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'p' as i32 {
                fputs(prog, stdout);
                p = p.offset(1)
            } else {
                putchar('%' as i32);
            }
        } else {
            putchar(*p as libc::c_int);
        }
        p = p.offset(1)
    }
    version();
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
