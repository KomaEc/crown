use ::libc;
extern "C" {
    pub type archive;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn getgrgid_r(
        __gid: __gid_t,
        __resultbuf: *mut group,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut group,
    ) -> libc::c_int;
    #[no_mangle]
    fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn archive_read_disk_set_uname_lookup(
        _: *mut archive,
        _: *mut libc::c_void,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: la_int64_t) -> *const libc::c_char>,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_disk_set_gname_lookup(
        _: *mut archive,
        _: *mut libc::c_void,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: la_int64_t) -> *const libc::c_char>,
        _: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __id_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type uintptr_t = libc::c_ulong;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type id_t = __id_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut libc::c_char,
    pub gr_passwd: *mut libc::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type la_int64_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_cache {
    pub archive: *mut archive,
    pub buff: *mut libc::c_char,
    pub buff_size: size_t,
    pub probes: libc::c_int,
    pub hits: libc::c_int,
    pub size: size_t,
    pub cache: [C2RustUnnamed; 127],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub id: id_t,
    pub name: *const libc::c_char,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const ERANGE: libc::c_int = 34 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
/* ! (_WIN32 && !__CYGWIN__) */
pub const name_cache_size: libc::c_int = 127 as libc::c_int;
static mut NO_NAME: *const libc::c_char = b"(noname)\x00" as *const u8 as *const libc::c_char;
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
/*
 * Installs functions that use getpwuid()/getgrgid()---along with
 * a simple cache to accelerate such lookups---into the archive_read_disk
 * object.  This is in a separate file because getpwuid()/getgrgid()
 * can pull in a LOT of library code (including NIS/LDAP functions, which
 * pull in DNS resolvers, etc).  This can easily top 500kB, which makes
 * it inappropriate for some space-constrained applications.
 *
 * Applications that are size-sensitive may want to just use the
 * real default functions (defined in archive_read_disk.c) that just
 * use the uid/gid without the lookup.  Or define your own custom functions
 * if you prefer.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_disk_set_standard_lookup(mut a: *mut archive) -> libc::c_int {
    let mut ucache: *mut name_cache =
        malloc(::std::mem::size_of::<name_cache>() as libc::c_ulong) as *mut name_cache;
    let mut gcache: *mut name_cache =
        malloc(::std::mem::size_of::<name_cache>() as libc::c_ulong) as *mut name_cache;
    if ucache.is_null() || gcache.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"Can\'t allocate uname/gname lookup cache\x00" as *const u8 as *const libc::c_char,
        );
        free(ucache as *mut libc::c_void);
        free(gcache as *mut libc::c_void);
        return -(30 as libc::c_int);
    }
    memset(
        ucache as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<name_cache>() as libc::c_ulong,
    );
    (*ucache).archive = a;
    (*ucache).size = name_cache_size as size_t;
    memset(
        gcache as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<name_cache>() as libc::c_ulong,
    );
    (*gcache).archive = a;
    (*gcache).size = name_cache_size as size_t;
    archive_read_disk_set_gname_lookup(
        a,
        gcache as *mut libc::c_void,
        Some(
            lookup_gname
                as unsafe extern "C" fn(_: *mut libc::c_void, _: int64_t) -> *const libc::c_char,
        ),
        Some(cleanup as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
    archive_read_disk_set_uname_lookup(
        a,
        ucache as *mut libc::c_void,
        Some(
            lookup_uname
                as unsafe extern "C" fn(_: *mut libc::c_void, _: int64_t) -> *const libc::c_char,
        ),
        Some(cleanup as unsafe extern "C" fn(_: *mut libc::c_void) -> ()),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn cleanup(mut data: *mut libc::c_void) {
    let mut cache: *mut name_cache = data as *mut name_cache;
    let mut i: size_t = 0;
    if !cache.is_null() {
        i = 0 as libc::c_int as size_t;
        while i < (*cache).size {
            if !(*cache).cache[i as usize].name.is_null()
                && (*cache).cache[i as usize].name != NO_NAME
            {
                free((*cache).cache[i as usize].name as uintptr_t as *mut libc::c_void);
            }
            i = i.wrapping_add(1)
        }
        free((*cache).buff as *mut libc::c_void);
        free(cache as *mut libc::c_void);
    };
}
/*
 * Lookup uid/gid from uname/gname, return NULL if no match.
 */
unsafe extern "C" fn lookup_name(
    mut cache: *mut name_cache,
    mut lookup_fn: Option<unsafe extern "C" fn(_: *mut name_cache, _: id_t) -> *const libc::c_char>,
    mut id: id_t,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut slot: libc::c_int = 0;
    (*cache).probes += 1;
    slot = (id as libc::c_ulong).wrapping_rem((*cache).size) as libc::c_int;
    if !(*cache).cache[slot as usize].name.is_null() {
        if (*cache).cache[slot as usize].id == id {
            (*cache).hits += 1;
            if (*cache).cache[slot as usize].name == NO_NAME {
                return 0 as *const libc::c_char;
            }
            return (*cache).cache[slot as usize].name;
        }
        if (*cache).cache[slot as usize].name != NO_NAME {
            free((*cache).cache[slot as usize].name as uintptr_t as *mut libc::c_void);
        }
        (*cache).cache[slot as usize].name = NULL as *const libc::c_char
    }
    name = lookup_fn.expect("non-null function pointer")(cache, id);
    if name.is_null() {
        /* Cache and return the negative response. */
        (*cache).cache[slot as usize].name = NO_NAME; /* Old getpwuid_r ignores last arg. */
        (*cache).cache[slot as usize].id = id;
        return 0 as *const libc::c_char;
    }
    (*cache).cache[slot as usize].name = name;
    (*cache).cache[slot as usize].id = id;
    return (*cache).cache[slot as usize].name;
}
unsafe extern "C" fn lookup_uname(
    mut data: *mut libc::c_void,
    mut uid: int64_t,
) -> *const libc::c_char {
    let mut uname_cache: *mut name_cache = data as *mut name_cache;
    return lookup_name(
        uname_cache,
        Some(
            lookup_uname_helper
                as unsafe extern "C" fn(_: *mut name_cache, _: id_t) -> *const libc::c_char,
        ),
        uid as id_t,
    );
}
unsafe extern "C" fn lookup_uname_helper(
    mut cache: *mut name_cache,
    mut id: id_t,
) -> *const libc::c_char {
    let mut pwent: passwd = passwd {
        pw_name: 0 as *mut libc::c_char,
        pw_passwd: 0 as *mut libc::c_char,
        pw_uid: 0,
        pw_gid: 0,
        pw_gecos: 0 as *mut libc::c_char,
        pw_dir: 0 as *mut libc::c_char,
        pw_shell: 0 as *mut libc::c_char,
    };
    let mut result: *mut passwd = 0 as *mut passwd;
    let mut nbuff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nbuff_size: size_t = 0;
    let mut r: libc::c_int = 0;
    if (*cache).buff_size == 0 as libc::c_int as libc::c_ulong {
        (*cache).buff_size = 256 as libc::c_int as size_t;
        (*cache).buff = malloc((*cache).buff_size) as *mut libc::c_char
    }
    if (*cache).buff.is_null() {
        return 0 as *const libc::c_char;
    }
    loop {
        result = &mut pwent;
        r = getpwuid_r(
            id,
            &mut pwent,
            (*cache).buff,
            (*cache).buff_size,
            &mut result,
        );
        if r == 0 as libc::c_int {
            break;
        }
        if r != ERANGE {
            break;
        }
        /* ERANGE means our buffer was too small, but POSIX
         * doesn't tell us how big the buffer should be, so
         * we just double it and try again.  Because the buffer
         * is kept around in the cache object, we shouldn't
         * have to do this very often. */
        nbuff_size = (*cache)
            .buff_size
            .wrapping_mul(2 as libc::c_int as libc::c_ulong); /* Old getgrgid_r ignores last arg. */
        nbuff = realloc((*cache).buff as *mut libc::c_void, nbuff_size) as *mut libc::c_char;
        if nbuff.is_null() {
            break;
        }
        (*cache).buff = nbuff;
        (*cache).buff_size = nbuff_size
    }
    if r != 0 as libc::c_int {
        archive_set_error(
            (*cache).archive,
            errno,
            b"Can\'t lookup user for id %d\x00" as *const u8 as *const libc::c_char,
            id as libc::c_int,
        );
        return 0 as *const libc::c_char;
    }
    if result.is_null() {
        return 0 as *const libc::c_char;
    }
    return strdup((*result).pw_name);
}
unsafe extern "C" fn lookup_gname(
    mut data: *mut libc::c_void,
    mut gid: int64_t,
) -> *const libc::c_char {
    let mut gname_cache: *mut name_cache = data as *mut name_cache;
    return lookup_name(
        gname_cache,
        Some(
            lookup_gname_helper
                as unsafe extern "C" fn(_: *mut name_cache, _: id_t) -> *const libc::c_char,
        ),
        gid as id_t,
    );
}
unsafe extern "C" fn lookup_gname_helper(
    mut cache: *mut name_cache,
    mut id: id_t,
) -> *const libc::c_char {
    let mut grent: group = group {
        gr_name: 0 as *mut libc::c_char,
        gr_passwd: 0 as *mut libc::c_char,
        gr_gid: 0,
        gr_mem: 0 as *mut *mut libc::c_char,
    };
    let mut result: *mut group = 0 as *mut group;
    let mut nbuff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nbuff_size: size_t = 0;
    let mut r: libc::c_int = 0;
    if (*cache).buff_size == 0 as libc::c_int as libc::c_ulong {
        (*cache).buff_size = 256 as libc::c_int as size_t;
        (*cache).buff = malloc((*cache).buff_size) as *mut libc::c_char
    }
    if (*cache).buff.is_null() {
        return 0 as *const libc::c_char;
    }
    loop {
        result = &mut grent;
        r = getgrgid_r(
            id,
            &mut grent,
            (*cache).buff,
            (*cache).buff_size,
            &mut result,
        );
        if r == 0 as libc::c_int {
            break;
        }
        if r != ERANGE {
            break;
        }
        /* ERANGE means our buffer was too small, but POSIX
         * doesn't tell us how big the buffer should be, so
         * we just double it and try again. */
        nbuff_size = (*cache)
            .buff_size
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        nbuff = realloc((*cache).buff as *mut libc::c_void, nbuff_size) as *mut libc::c_char;
        if nbuff.is_null() {
            break;
        }
        (*cache).buff = nbuff;
        (*cache).buff_size = nbuff_size
    }
    if r != 0 as libc::c_int {
        archive_set_error(
            (*cache).archive,
            errno,
            b"Can\'t lookup group for id %d\x00" as *const u8 as *const libc::c_char,
            id as libc::c_int,
        );
        return 0 as *const libc::c_char;
    }
    if result.is_null() {
        return 0 as *const libc::c_char;
    }
    return strdup((*result).gr_name);
}
/* ! (_WIN32 && !__CYGWIN__) */
