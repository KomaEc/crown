use ::libc;
extern "C" {
    pub type archive;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn archive_read_set_open_callback(
        _: *mut archive,
        _: Option<archive_open_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_read_callback(
        _: *mut archive,
        _: Option<archive_read_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_seek_callback(
        _: *mut archive,
        _: Option<archive_seek_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_skip_callback(
        _: *mut archive,
        _: Option<archive_skip_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_close_callback(
        _: *mut archive,
        _: Option<archive_close_callback>,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_set_callback_data(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn archive_read_open1(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type la_int64_t = int64_t;
pub type la_ssize_t = ssize_t;
pub type archive_read_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *mut *const libc::c_void,
) -> la_ssize_t;
pub type archive_skip_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void, _: la_int64_t) -> la_int64_t;
pub type archive_seek_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: la_int64_t,
    _: libc::c_int,
) -> la_int64_t;
pub type archive_open_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
/*
 * Glue to read an archive from a block of memory.
 *
 * This is mostly a huge help in building test harnesses;
 * test programs can build archives in memory and read them
 * back again without having to mess with files on disk.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct read_memory_data {
    pub start: *const libc::c_uchar,
    pub p: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub read_size: ssize_t,
}
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const SEEK_CUR: libc::c_int = 1 as libc::c_int;
pub const SEEK_END: libc::c_int = 2 as libc::c_int;
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
#[no_mangle]
pub unsafe extern "C" fn archive_read_open_memory(
    mut a: *mut archive,
    mut buff: *const libc::c_void,
    mut size: size_t,
) -> libc::c_int {
    return archive_read_open_memory2(a, buff, size, size);
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
/*
 * Don't use _open_memory2() in production code; the archive_read_open_memory()
 * version is the one you really want.  This is just here so that
 * test harnesses can exercise block operations inside the library.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_read_open_memory2(
    mut a: *mut archive,
    mut buff: *const libc::c_void,
    mut size: size_t,
    mut read_size: size_t,
) -> libc::c_int {
    let mut mine: *mut read_memory_data = 0 as *mut read_memory_data;
    mine = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<read_memory_data>() as libc::c_ulong,
    ) as *mut read_memory_data;
    if mine.is_null() {
        archive_set_error(
            a,
            ENOMEM,
            b"No memory\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*mine).p = buff as *const libc::c_uchar;
    (*mine).start = (*mine).p;
    (*mine).end = (*mine).start.offset(size as isize);
    (*mine).read_size = read_size as ssize_t;
    archive_read_set_open_callback(
        a,
        Some(
            memory_read_open
                as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
    );
    archive_read_set_read_callback(
        a,
        Some(
            memory_read
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: *mut *const libc::c_void,
                ) -> ssize_t,
        ),
    );
    archive_read_set_seek_callback(
        a,
        Some(
            memory_read_seek
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: int64_t,
                    _: libc::c_int,
                ) -> int64_t,
        ),
    );
    archive_read_set_skip_callback(
        a,
        Some(
            memory_read_skip
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: int64_t,
                ) -> int64_t,
        ),
    );
    archive_read_set_close_callback(
        a,
        Some(
            memory_read_close
                as unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int,
        ),
    );
    archive_read_set_callback_data(a, mine as *mut libc::c_void);
    return archive_read_open1(a);
}
/*
 * There's nothing to open.
 */
unsafe extern "C" fn memory_read_open(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    /* UNUSED */
    return 0 as libc::c_int;
}
/*
 * This is scary simple:  Just advance a pointer.  Limiting
 * to read_size is not technically necessary, but it exercises
 * more of the internal logic when used with a small block size
 * in a test harness.  Production use should not specify a block
 * size; then this is much faster.
 */
unsafe extern "C" fn memory_read(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut buff: *mut *const libc::c_void,
) -> ssize_t {
    let mut mine: *mut read_memory_data = client_data as *mut read_memory_data;
    let mut size: ssize_t = 0;
    /* UNUSED */
    *buff = (*mine).p as *const libc::c_void;
    size = (*mine).end.offset_from((*mine).p) as libc::c_long;
    if size > (*mine).read_size {
        size = (*mine).read_size
    }
    (*mine).p = (*mine).p.offset(size as isize);
    return size;
}
/*
 * Advancing is just as simple.  Again, this is doing more than
 * necessary in order to better exercise internal code when used
 * as a test harness.
 */
unsafe extern "C" fn memory_read_skip(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut skip: int64_t,
) -> int64_t {
    let mut mine: *mut read_memory_data = client_data as *mut read_memory_data;
    /* UNUSED */
    if skip > (*mine).end.offset_from((*mine).p) as libc::c_long {
        skip = (*mine).end.offset_from((*mine).p) as libc::c_long
    }
    /* Round down to block size. */
    skip /= (*mine).read_size;
    skip *= (*mine).read_size;
    (*mine).p = (*mine).p.offset(skip as isize);
    return skip;
}
/*
 * Seeking.
 */
unsafe extern "C" fn memory_read_seek(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
    mut offset: int64_t,
    mut whence: libc::c_int,
) -> int64_t {
    let mut mine: *mut read_memory_data = client_data as *mut read_memory_data;
    /* UNUSED */
    match whence {
        SEEK_SET => (*mine).p = (*mine).start.offset(offset as isize),
        SEEK_CUR => (*mine).p = (*mine).p.offset(offset as isize),
        SEEK_END => (*mine).p = (*mine).end.offset(offset as isize),
        _ => return ARCHIVE_FATAL as int64_t,
    }
    if (*mine).p < (*mine).start {
        (*mine).p = (*mine).start;
        return ARCHIVE_FAILED as int64_t;
    }
    if (*mine).p > (*mine).end {
        (*mine).p = (*mine).end;
        return ARCHIVE_FAILED as int64_t;
    }
    return (*mine).p.offset_from((*mine).start) as libc::c_long;
}
/*
 * Close is just cleaning up our one small bit of data.
 */
unsafe extern "C" fn memory_read_close(
    mut a: *mut archive,
    mut client_data: *mut libc::c_void,
) -> libc::c_int {
    let mut mine: *mut read_memory_data = client_data as *mut read_memory_data;
    /* UNUSED */
    free(mine as *mut libc::c_void);
    return 0 as libc::c_int;
}
