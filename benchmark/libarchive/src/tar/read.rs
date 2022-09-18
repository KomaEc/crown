use ::libc;
extern "C" {
    /* Declare our basic types. */
    pub type archive;
    pub type archive_entry;
    pub type archive_entry_linkresolver;
    pub type creation_set;
    pub type substitution;
    pub type siginfo_data;
    pub type name_cache;
    pub type security;
    pub type archive_dir;
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
    #[no_mangle]
    fn archive_read_new() -> *mut archive;
    #[no_mangle]
    fn archive_read_support_filter_all(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_format_all(_: *mut archive) -> libc::c_int;
    /*
     * A variety of shortcuts that invoke archive_read_open() with
     * canned callbacks suitable for common situations.  The ones that
     * accept a block size handle tape blocking correctly.
     */
    /* Use this if you know the filename.  Note: NULL indicates stdin. */
    #[no_mangle]
    fn archive_read_open_filename(
        _: *mut archive,
        _filename: *const libc::c_char,
        _block_size: size_t,
    ) -> libc::c_int;
    /* Parses and returns next entry header. */
    #[no_mangle]
    fn archive_read_next_header(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int;
    /*-
     * Some convenience functions that are built on archive_read_data:
     *  'skip': skips entire entry
     *  'into_buffer': writes data into memory buffer that you provide
     *  'into_fd': writes data to specified filedes
     */
    #[no_mangle]
    fn archive_read_data_skip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_data_into_fd(_: *mut archive, fd: libc::c_int) -> libc::c_int;
    /* Apply option string to both the format and the filter. */
    #[no_mangle]
    fn archive_read_set_options(_a: *mut archive, opts: *const libc::c_char) -> libc::c_int;
    /*
     * Add a decryption passphrase.
     */
    #[no_mangle]
    fn archive_read_add_passphrase(_: *mut archive, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_passphrase_callback(
        _: *mut archive,
        client_data: *mut libc::c_void,
        _: Option<archive_passphrase_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_extract2(
        _: *mut archive,
        _: *mut archive_entry,
        _: *mut archive,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_extract_set_progress_callback(
        _: *mut archive,
        _progress_func: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        _user_data: *mut libc::c_void,
    );
    /* Close the file and release most resources. */
    #[no_mangle]
    fn archive_read_close(_: *mut archive) -> libc::c_int;
    /* Release all resources and destroy the object. */
    /* Note that archive_read_free will call archive_read_close for you. */
    #[no_mangle]
    fn archive_read_free(_: *mut archive) -> libc::c_int;
    /* This can fail if the archive wasn't already closed, in which case
     * archive_write_free() will implicitly call archive_write_close(). */
    #[no_mangle]
    fn archive_write_free(_: *mut archive) -> libc::c_int;
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
    #[no_mangle]
    fn archive_write_disk_new() -> *mut archive;
    /* Set flags to control how the next item gets created.
     * This accepts a bitmask of ARCHIVE_EXTRACT_XXX flags defined above. */
    #[no_mangle]
    fn archive_write_disk_set_options(_: *mut archive, flags: libc::c_int) -> libc::c_int;
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
    #[no_mangle]
    fn archive_write_disk_set_standard_lookup(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_filter_bytes(_: *mut archive, _: libc::c_int) -> la_int64_t;
    #[no_mangle]
    fn archive_filter_name(_: *mut archive, _: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn chroot(__path: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    fn archive_error_string(_: *mut archive) -> *const libc::c_char;
    #[no_mangle]
    fn archive_format_name(_: *mut archive) -> *const libc::c_char;
    #[no_mangle]
    fn archive_clear_error(_: *mut archive);
    #[no_mangle]
    fn archive_file_count(_: *mut archive) -> libc::c_int;
    /*
     * Test if archive_entry is excluded.
     * This is a convenience function. This is the same as calling all
     * archive_match_path_excluded, archive_match_time_excluded
     * and archive_match_owner_excluded.
     */
    #[no_mangle]
    fn archive_match_excluded(_: *mut archive, _: *mut archive_entry) -> libc::c_int;
    /* Add inclusion pathname pattern. */
    #[no_mangle]
    fn archive_match_include_pattern(_: *mut archive, _: *const libc::c_char) -> libc::c_int;
    /* Add inclusion pathname pattern from file. */
    #[no_mangle]
    fn archive_match_include_pattern_from_file(
        _: *mut archive,
        _: *const libc::c_char,
        _nullSeparator: libc::c_int,
    ) -> libc::c_int;
    /*
     * How to get statistic information for inclusion patterns.
     */
    /* Return the amount number of unmatched inclusion patterns. */
    #[no_mangle]
    fn archive_match_path_unmatched_inclusions(_: *mut archive) -> libc::c_int;
    /* Return the pattern of unmatched inclusion with ARCHIVE_OK.
     * Return ARCHIVE_EOF if there is no inclusion pattern. */
    #[no_mangle]
    fn archive_match_path_unmatched_inclusions_next(
        _: *mut archive,
        _: *mut *const libc::c_char,
    ) -> libc::c_int;
    /* set */
    /* clear */
    /*
     * Set fields in an archive_entry.
     *
     * Note: Before libarchive 2.4, there were 'set' and 'copy' versions
     * of the string setters.  'copy' copied the actual string, 'set' just
     * stored the pointer.  In libarchive 2.4 and later, strings are
     * always copied.
     */
    /* set */
    /* clear */
    /* Returns pointer to start of first invalid token, or NULL if none. */
    /* Note that all recognized tokens are processed, regardless. */
    #[no_mangle]
    fn archive_entry_set_uname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn do_chdir(_: *mut bsdtar);
    #[no_mangle]
    fn edit_pathname(_: *mut bsdtar, _: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn need_report() -> libc::c_int;
    #[no_mangle]
    fn safe_fprintf(_: *mut FILE, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn tar_i64toa(_: int64_t) -> *const libc::c_char;
    #[no_mangle]
    fn yes(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn cset_read_support_filter_program(_: *mut creation_set, _: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn passphrase_callback(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
    #[no_mangle]
    fn list_item_verbose(_: *mut bsdtar, _: *mut FILE, _: *mut archive_entry);
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
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
/*
 * Returns a passphrase used for encryption or decryption, NULL on nothing
 * to do and give it up.
 */
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
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
pub struct progress_data {
    pub bsdtar: *mut bsdtar,
    pub archive: *mut archive,
    pub entry: *mut archive_entry,
}
/*
 * Error codes: Use archive_errno() and archive_error_string()
 * to retrieve details.  Unless specified otherwise, all functions
 * that return 'int' use these codes.
 */
pub const ARCHIVE_EOF: libc::c_int = 1 as libc::c_int;
/* Found end of archive. */
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
/* Operation was successful. */
pub const ARCHIVE_RETRY: libc::c_int = -(10 as libc::c_int);
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
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
pub const ENV_READER_OPTIONS: [libc::c_char; 19] =
    unsafe { *::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"TAR_READER_OPTIONS\x00") };
pub const IGNORE_WRONG_MODULE_NAME: [libc::c_char; 30] = unsafe {
    *::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(b"__ignore_wrong_module_name__,\x00")
};
/* for util.c */
/* Options for flags bitfield */
/* -a */
/* -P */
pub const OPTFLAG_CHROOT: libc::c_int = 0x4 as libc::c_int;
/* --chroot */
pub const OPTFLAG_FAST_READ: libc::c_int = 0x8 as libc::c_int;
/* --fast-read */
pub const OPTFLAG_IGNORE_ZEROS: libc::c_int = 0x10 as libc::c_int;
/* --ignore-zeros */
pub const OPTFLAG_INTERACTIVE: libc::c_int = 0x20 as libc::c_int;
/* -w */
/* -o */
/* -n */
pub const OPTFLAG_NULL: libc::c_int = 0x100 as libc::c_int;
/* --null */
pub const OPTFLAG_NUMERIC_OWNER: libc::c_int = 0x200 as libc::c_int;
/* --numeric-owner */
/* -o */
pub const OPTFLAG_STDOUT: libc::c_int = 0x800 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn tar_mode_t(mut bsdtar: *mut bsdtar) {
    read_archive(bsdtar, 't' as i32 as libc::c_char, NULL as *mut archive);
    if unmatched_inclusions_warn(
        (*bsdtar).matching,
        b"Not found in archive\x00" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        (*bsdtar).return_value = 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn tar_mode_x(mut bsdtar: *mut bsdtar) {
    let mut writer: *mut archive = 0 as *mut archive;
    writer = archive_write_disk_new();
    if writer.is_null() {
        lafe_errc(
            1 as libc::c_int,
            ENOMEM,
            b"Cannot allocate disk writer object\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).flags & OPTFLAG_NUMERIC_OWNER as libc::c_uint == 0 as libc::c_int as libc::c_uint {
        archive_write_disk_set_standard_lookup(writer);
    }
    archive_write_disk_set_options(writer, (*bsdtar).extract_flags);
    read_archive(bsdtar, 'x' as i32 as libc::c_char, writer);
    if unmatched_inclusions_warn(
        (*bsdtar).matching,
        b"Not found in archive\x00" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        (*bsdtar).return_value = 1 as libc::c_int
    }
    archive_write_free(writer);
}
unsafe extern "C" fn progress_func(mut cookie: *mut libc::c_void) {
    let mut progress_data: *mut progress_data = cookie as *mut progress_data;
    let mut bsdtar: *mut bsdtar = (*progress_data).bsdtar;
    let mut a: *mut archive = (*progress_data).archive;
    let mut entry: *mut archive_entry = (*progress_data).entry;
    let mut comp: uint64_t = 0;
    let mut uncomp: uint64_t = 0;
    let mut compression: libc::c_int = 0;
    if need_report() == 0 {
        return;
    }
    if (*bsdtar).verbose != 0 {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if !a.is_null() {
        comp = archive_filter_bytes(a, -(1 as libc::c_int)) as uint64_t;
        uncomp = archive_filter_bytes(a, 0 as libc::c_int) as uint64_t;
        if comp > uncomp {
            compression = 0 as libc::c_int
        } else {
            compression = uncomp
                .wrapping_sub(comp)
                .wrapping_mul(100 as libc::c_int as libc::c_ulong)
                .wrapping_div(uncomp) as libc::c_int
        }
        fprintf(
            stderr,
            b"In: %s bytes, compression %d%%;\x00" as *const u8 as *const libc::c_char,
            tar_i64toa(comp as int64_t),
            compression,
        );
        fprintf(
            stderr,
            b"  Out: %d files, %s bytes\n\x00" as *const u8 as *const libc::c_char,
            archive_file_count(a),
            tar_i64toa(uncomp as int64_t),
        );
    }
    if !entry.is_null() {
        safe_fprintf(
            stderr,
            b"Current: %s\x00" as *const u8 as *const libc::c_char,
            archive_entry_pathname(entry),
        );
        fprintf(
            stderr,
            b" (%s bytes)\n\x00" as *const u8 as *const libc::c_char,
            tar_i64toa(archive_entry_size(entry)),
        );
    };
}
/*
 * Handle 'x' and 't' modes.
 */
unsafe extern "C" fn read_archive(
    mut bsdtar: *mut bsdtar,
    mut mode: libc::c_char,
    mut writer: *mut archive,
) {
    let mut progress_data: progress_data = progress_data {
        bsdtar: 0 as *mut bsdtar,
        archive: 0 as *mut archive,
        entry: 0 as *mut archive_entry,
    };
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut a: *mut archive = 0 as *mut archive;
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut reader_options: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    while !(*(*bsdtar).argv).is_null() {
        if archive_match_include_pattern((*bsdtar).matching, *(*bsdtar).argv) != ARCHIVE_OK {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"Error inclusion pattern: %s\x00" as *const u8 as *const libc::c_char,
                archive_error_string((*bsdtar).matching),
            );
        }
        (*bsdtar).argv = (*bsdtar).argv.offset(1)
    }
    if !(*bsdtar).names_from_file.is_null() {
        if archive_match_include_pattern_from_file(
            (*bsdtar).matching,
            (*bsdtar).names_from_file,
            ((*bsdtar).flags & OPTFLAG_NULL as libc::c_uint) as libc::c_int,
        ) != ARCHIVE_OK
        {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"Error inclusion pattern: %s\x00" as *const u8 as *const libc::c_char,
                archive_error_string((*bsdtar).matching),
            );
        }
    }
    a = archive_read_new();
    if cset_read_support_filter_program((*bsdtar).cset, a) == 0 as libc::c_int {
        archive_read_support_filter_all(a);
    }
    archive_read_support_format_all(a);
    reader_options = getenv(ENV_READER_OPTIONS.as_ptr());
    if !reader_options.is_null() {
        let mut module_len: size_t = (::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut opt_len: size_t =
            strlen(reader_options).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        /* Set default read options. */
        p = malloc(module_len.wrapping_add(opt_len)) as *mut libc::c_char;
        if p.is_null() {
            lafe_errc(
                1 as libc::c_int,
                errno,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
        }
        /* Prepend magic code to ignore options for
         * a format or  modules which are not added to
         *  the archive read object. */
        memcpy(
            p as *mut libc::c_void,
            IGNORE_WRONG_MODULE_NAME.as_ptr(),
            module_len,
        );
        memcpy(
            p.offset(module_len as isize) as *mut libc::c_void,
            reader_options as *const libc::c_void,
            opt_len,
        );
        r = archive_read_set_options(a, p);
        free(p as *mut libc::c_void);
        if r == ARCHIVE_FATAL {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
        } else {
            archive_clear_error(a);
        }
    }
    if ARCHIVE_OK != archive_read_set_options(a, (*bsdtar).option_options) {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    if (*bsdtar).flags & OPTFLAG_IGNORE_ZEROS as libc::c_uint != 0 {
        if archive_read_set_options(
            a,
            b"read_concatenated_archives\x00" as *const u8 as *const libc::c_char,
        ) != ARCHIVE_OK
        {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
        }
    }
    if !(*bsdtar).passphrase.is_null() {
        r = archive_read_add_passphrase(a, (*bsdtar).passphrase)
    } else {
        r = archive_read_set_passphrase_callback(
            a,
            bsdtar as *mut libc::c_void,
            Some(
                passphrase_callback
                    as unsafe extern "C" fn(
                        _: *mut archive,
                        _: *mut libc::c_void,
                    ) -> *const libc::c_char,
            ),
        )
    }
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    if archive_read_open_filename(a, (*bsdtar).filename, (*bsdtar).bytes_per_block as size_t) != 0 {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Error opening archive: %s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    do_chdir(bsdtar);
    if mode as libc::c_int == 'x' as i32 {
        /* Set an extract callback so that we can handle SIGINFO. */
        progress_data.bsdtar = bsdtar;
        progress_data.archive = a;
        archive_read_extract_set_progress_callback(
            a,
            Some(progress_func as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
            &mut progress_data as *mut progress_data as *mut libc::c_void,
        );
    }
    if mode as libc::c_int == 'x' as i32 && (*bsdtar).flags & OPTFLAG_CHROOT as libc::c_uint != 0 {
        if chroot(b".\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
            lafe_errc(
                1 as libc::c_int,
                errno,
                b"Can\'t chroot to \".\"\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    loop
    /* Support --fast-read option */
    {
        let mut p_0: *const libc::c_char = 0 as *const libc::c_char;
        if (*bsdtar).flags & OPTFLAG_FAST_READ as libc::c_uint != 0
            && archive_match_path_unmatched_inclusions((*bsdtar).matching) == 0 as libc::c_int
        {
            break;
        }
        r = archive_read_next_header(a, &mut entry);
        progress_data.entry = entry;
        if r == ARCHIVE_EOF {
            break;
        }
        if r < ARCHIVE_OK {
            lafe_warnc(
                0 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
        }
        if r <= ARCHIVE_WARN {
            (*bsdtar).return_value = 1 as libc::c_int
        }
        if r == ARCHIVE_RETRY {
            /* Retryable error: try again */
            lafe_warnc(
                0 as libc::c_int,
                b"Retrying...\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            if r == ARCHIVE_FATAL {
                break;
            }
            p_0 = archive_entry_pathname(entry);
            if p_0.is_null()
                || *p_0.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            {
                lafe_warnc(
                    0 as libc::c_int,
                    b"Archive entry has empty or unreadable filename ... skipping.\x00" as *const u8
                        as *const libc::c_char,
                );
                (*bsdtar).return_value = 1 as libc::c_int
            } else {
                if (*bsdtar).uid >= 0 as libc::c_int {
                    archive_entry_set_uid(entry, (*bsdtar).uid as la_int64_t);
                    archive_entry_set_uname(entry, NULL as *const libc::c_char);
                }
                if (*bsdtar).gid >= 0 as libc::c_int {
                    archive_entry_set_gid(entry, (*bsdtar).gid as la_int64_t);
                    archive_entry_set_gname(entry, NULL as *const libc::c_char);
                }
                if !(*bsdtar).uname.is_null() {
                    archive_entry_set_uname(entry, (*bsdtar).uname);
                }
                if !(*bsdtar).gname.is_null() {
                    archive_entry_set_gname(entry, (*bsdtar).gname);
                }
                /*
                 * Note that pattern exclusions are checked before
                 * pathname rewrites are handled.  This gives more
                 * control over exclusions, since rewrites always lose
                 * information.  (For example, consider a rewrite
                 * s/foo[0-9]/foo/.  If we check exclusions after the
                 * rewrite, there would be no way to exclude foo1/bar
                 * while allowing foo2/bar.)
                 */
                if archive_match_excluded((*bsdtar).matching, entry) != 0 {
                    continue; /* Excluded by a pattern test. */
                }
                if mode as libc::c_int == 't' as i32 {
                    /* Perversely, gtar uses -O to mean "send to stderr"
                     * when used with -t. */
                    out = if (*bsdtar).flags & OPTFLAG_STDOUT as libc::c_uint != 0 {
                        stderr
                    } else {
                        stdout
                    };
                    /*
                     * TODO: Provide some reasonable way to
                     * preview rewrites.  gtar always displays
                     * the unedited path in -t output, which means
                     * you cannot easily preview rewrites.
                     */
                    if (*bsdtar).verbose < 2 as libc::c_int {
                        safe_fprintf(
                            out,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            archive_entry_pathname(entry),
                        );
                    } else {
                        list_item_verbose(bsdtar, out, entry);
                    }
                    fflush(out);
                    r = archive_read_data_skip(a);
                    if r == ARCHIVE_WARN {
                        fprintf(out, b"\n\x00" as *const u8 as *const libc::c_char);
                        lafe_warnc(
                            0 as libc::c_int,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            archive_error_string(a),
                        );
                    }
                    if r == ARCHIVE_RETRY {
                        fprintf(out, b"\n\x00" as *const u8 as *const libc::c_char);
                        lafe_warnc(
                            0 as libc::c_int,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            archive_error_string(a),
                        );
                    }
                    if r == ARCHIVE_FATAL {
                        fprintf(out, b"\n\x00" as *const u8 as *const libc::c_char);
                        lafe_warnc(
                            0 as libc::c_int,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            archive_error_string(a),
                        );
                        (*bsdtar).return_value = 1 as libc::c_int;
                        break;
                    } else {
                        fprintf(out, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                } else {
                    /* Note: some rewrite failures prevent extraction. */
                    if edit_pathname(bsdtar, entry) != 0 {
                        continue; /* Excluded by a rewrite failure. */
                    }
                    if (*bsdtar).flags & OPTFLAG_INTERACTIVE as libc::c_uint != 0
                        && yes(
                            b"extract \'%s\'\x00" as *const u8 as *const libc::c_char,
                            archive_entry_pathname(entry),
                        ) == 0
                    {
                        continue;
                    }
                    if (*bsdtar).verbose > 1 as libc::c_int {
                        /* GNU tar uses -tv format with -xvv */
                        safe_fprintf(stderr, b"x \x00" as *const u8 as *const libc::c_char);
                        list_item_verbose(bsdtar, stderr, entry);
                        fflush(stderr);
                    } else if (*bsdtar).verbose > 0 as libc::c_int {
                        /* Format follows SUSv2, including the
                         * deferred '\n'. */
                        safe_fprintf(
                            stderr,
                            b"x %s\x00" as *const u8 as *const libc::c_char,
                            archive_entry_pathname(entry),
                        );
                        fflush(stderr);
                    }
                    /* TODO siginfo_printinfo(bsdtar, 0); */
                    if (*bsdtar).flags & OPTFLAG_STDOUT as libc::c_uint != 0 {
                        r = archive_read_data_into_fd(a, 1 as libc::c_int)
                    } else {
                        r = archive_read_extract2(a, entry, writer)
                    }
                    if r != ARCHIVE_OK {
                        if (*bsdtar).verbose == 0 {
                            safe_fprintf(
                                stderr,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                archive_entry_pathname(entry),
                            );
                        }
                        safe_fprintf(
                            stderr,
                            b": %s\x00" as *const u8 as *const libc::c_char,
                            archive_error_string(a),
                        );
                        if (*bsdtar).verbose == 0 {
                            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
                        }
                        (*bsdtar).return_value = 1 as libc::c_int
                    }
                    if (*bsdtar).verbose != 0 {
                        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
                    }
                    if r == ARCHIVE_FATAL {
                        break;
                    }
                }
            }
        }
    }
    r = archive_read_close(a);
    if r != ARCHIVE_OK {
        lafe_warnc(
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    if r <= ARCHIVE_WARN {
        (*bsdtar).return_value = 1 as libc::c_int
    }
    if (*bsdtar).verbose > 2 as libc::c_int {
        fprintf(
            stdout,
            b"Archive Format: %s,  Compression: %s\n\x00" as *const u8 as *const libc::c_char,
            archive_format_name(a),
            archive_filter_name(a, 0 as libc::c_int),
        );
    }
    archive_read_free(a);
}
unsafe extern "C" fn unmatched_inclusions_warn(
    mut matching: *mut archive,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    if matching.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        r = archive_match_path_unmatched_inclusions_next(matching, &mut p);
        if !(r == ARCHIVE_OK) {
            break;
        }
        lafe_warnc(
            0 as libc::c_int,
            b"%s: %s\x00" as *const u8 as *const libc::c_char,
            p,
            msg,
        );
    }
    if r == ARCHIVE_FATAL {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return archive_match_path_unmatched_inclusions(matching);
}
