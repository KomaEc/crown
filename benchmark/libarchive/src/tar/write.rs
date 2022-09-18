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
    pub type creation_set;
    pub type substitution;
    pub type siginfo_data;
    pub type name_cache;
    pub type security;
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
    pub type lafe_line_reader;
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
    #[no_mangle]
    fn archive_read_support_format_empty(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_format_gnutar(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_support_format_tar(_: *mut archive) -> libc::c_int;
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
    /* Read an archive that's already open, using the file descriptor. */
    #[no_mangle]
    fn archive_read_open_fd(_: *mut archive, _fd: libc::c_int, _block_size: size_t) -> libc::c_int;
    /* Parses and returns next entry header. */
    #[no_mangle]
    fn archive_read_next_header(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int;
    /* Parses and returns next entry header using the archive_entry passed in */
    #[no_mangle]
    fn archive_read_next_header2(_: *mut archive, _: *mut archive_entry) -> libc::c_int;
    /*
     * Retrieve the byte offset in UNCOMPRESSED data where last-read
     * header started.
     */
    #[no_mangle]
    fn archive_read_header_position(_: *mut archive) -> la_int64_t;
    /*
     * A zero-copy version of archive_read_data that also exposes the file offset
     * of each returned block.  Note that the client has no way to specify
     * the desired size of the block.  The API does guarantee that offsets will
     * be strictly increasing and that returned blocks will not overlap.
     */
    #[no_mangle]
    fn archive_read_data_block(
        a: *mut archive,
        buff: *mut *const libc::c_void,
        size: *mut size_t,
        offset: *mut la_int64_t,
    ) -> libc::c_int;
    /*-
     * Some convenience functions that are built on archive_read_data:
     *  'skip': skips entire entry
     *  'into_buffer': writes data into memory buffer that you provide
     *  'into_fd': writes data to specified filedes
     */
    #[no_mangle]
    fn archive_read_data_skip(_: *mut archive) -> libc::c_int;
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
    /* Close the file and release most resources. */
    #[no_mangle]
    fn archive_read_close(_: *mut archive) -> libc::c_int;
    /* Release all resources and destroy the object. */
    /* Note that archive_read_free will call archive_read_close for you. */
    #[no_mangle]
    fn archive_read_free(_: *mut archive) -> libc::c_int;
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
    #[no_mangle]
    fn archive_write_new() -> *mut archive;
    #[no_mangle]
    fn archive_write_set_bytes_per_block(
        _: *mut archive,
        bytes_per_block: libc::c_int,
    ) -> libc::c_int;
    /* XXX This is badly misnamed; suggestions appreciated. XXX */
    #[no_mangle]
    fn archive_write_set_bytes_in_last_block(
        _: *mut archive,
        bytes_in_last_block: libc::c_int,
    ) -> libc::c_int;
    /* A convenience function to set the format based on the code or name. */
    #[no_mangle]
    fn archive_write_set_format(_: *mut archive, format_code: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_by_name(_: *mut archive, name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_pax_restricted(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_open_fd(_: *mut archive, _fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn archive_write_open_filename(_: *mut archive, _file: *const libc::c_char) -> libc::c_int;
    /*
     * Note that the library will truncate writes beyond the size provided
     * to archive_write_header or pad if the provided data is short.
     */
    #[no_mangle]
    fn archive_write_header(_: *mut archive, _: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_write_data(_: *mut archive, _: *const libc::c_void, _: size_t) -> la_ssize_t;
    #[no_mangle]
    fn archive_write_close(_: *mut archive) -> libc::c_int;
    /* This can fail if the archive wasn't already closed, in which case
     * archive_write_free() will implicitly call archive_write_close(). */
    #[no_mangle]
    fn archive_write_free(_: *mut archive) -> libc::c_int;
    /* Apply option string to both the format and the filter. */
    #[no_mangle]
    fn archive_write_set_options(_a: *mut archive, opts: *const libc::c_char) -> libc::c_int;
    /*
     * Set a encryption passphrase.
     */
    #[no_mangle]
    fn archive_write_set_passphrase(_a: *mut archive, p: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_passphrase_callback(
        _: *mut archive,
        client_data: *mut libc::c_void,
        _: Option<archive_passphrase_callback>,
    ) -> libc::c_int;
    /*
     * ARCHIVE_READ_DISK API
     *
     * This is still evolving and somewhat experimental.
     */
    #[no_mangle]
    fn archive_read_disk_new() -> *mut archive;
    /* The names for symlink modes here correspond to an old BSD
     * command-line argument convention: -L, -P, -H */
    /* Follow all symlinks. */
    #[no_mangle]
    fn archive_read_disk_set_symlink_logical(_: *mut archive) -> libc::c_int;
    /* Follow no symlinks. */
    #[no_mangle]
    fn archive_read_disk_set_symlink_physical(_: *mut archive) -> libc::c_int;
    /* Follow symlink initially, then not. */
    #[no_mangle]
    fn archive_read_disk_set_symlink_hybrid(_: *mut archive) -> libc::c_int;
    /* Look up gname for gid or uname for uid. */
    /* Default implementations are very, very stupid. */
    #[no_mangle]
    fn archive_read_disk_gname(_: *mut archive, _: la_int64_t) -> *const libc::c_char;
    #[no_mangle]
    fn archive_read_disk_uname(_: *mut archive, _: la_int64_t) -> *const libc::c_char;
    /* "Standard" implementation uses getpwuid_r, getgrgid_r and caches the
     * results for performance. */
    #[no_mangle]
    fn archive_read_disk_set_standard_lookup(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    /* Start traversal. */
    #[no_mangle]
    fn archive_read_disk_open(_: *mut archive, _: *const libc::c_char) -> libc::c_int;
    /*
     * Request that current entry be visited.  If you invoke it on every
     * directory, you'll get a physical traversal.  This is ignored if the
     * current entry isn't a directory or a link to a directory.  So, if
     * you invoke this on every returned path, you'll get a full logical
     * traversal.
     */
    #[no_mangle]
    fn archive_read_disk_descend(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_read_disk_can_descend(_: *mut archive) -> libc::c_int;
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
    #[no_mangle]
    fn archive_read_disk_set_behavior(_: *mut archive, flags: libc::c_int) -> libc::c_int;
    /*
     * Set archive_match object that will be used in archive_read_disk to
     * know whether an entry should be skipped. The callback function
     * _excluded_func will be invoked when an entry is skipped by the result
     * of archive_match.
     */
    #[no_mangle]
    fn archive_read_disk_set_matching(
        _: *mut archive,
        _matching: *mut archive,
        _excluded_func: Option<
            unsafe extern "C" fn(
                _: *mut archive,
                _: *mut libc::c_void,
                _: *mut archive_entry,
            ) -> (),
        >,
        _client_data: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_read_disk_set_metadata_filter_callback(
        _: *mut archive,
        _metadata_filter_func: Option<
            unsafe extern "C" fn(
                _: *mut archive,
                _: *mut libc::c_void,
                _: *mut archive_entry,
            ) -> libc::c_int,
        >,
        _client_data: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_filter_bytes(_: *mut archive, _: libc::c_int) -> la_int64_t;
    #[no_mangle]
    fn archive_filter_code(_: *mut archive, _: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn archive_errno(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_error_string(_: *mut archive) -> *const libc::c_char;
    #[no_mangle]
    fn archive_format(_: *mut archive) -> libc::c_int;
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
    /* Add exclusion entry. */
    #[no_mangle]
    fn archive_match_exclude_entry(
        _: *mut archive,
        _flag: libc::c_int,
        _: *mut archive_entry,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_new() -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_sourcepath(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uname(_: *mut archive_entry, _: *const libc::c_char);
    /*
     * There are three different strategies for marking hardlinks.
     * The descriptions below name them after the best-known
     * formats that rely on each strategy:
     *
     * "Old cpio" is the simplest, it always returns any entry unmodified.
     *    As far as I know, only cpio formats use this.  Old cpio archives
     *    store every link with the full body; the onus is on the dearchiver
     *    to detect and properly link the files as they are restored.
     * "tar" is also pretty simple; it caches a copy the first time it sees
     *    any link.  Subsequent appearances are modified to be hardlink
     *    references to the first one without any body.  Used by all tar
     *    formats, although the newest tar formats permit the "old cpio" strategy
     *    as well.  This strategy is very simple for the dearchiver,
     *    and reasonably straightforward for the archiver.
     * "new cpio" is trickier.  It stores the body only with the last
     *    occurrence.  The complication is that we might not
     *    see every link to a particular file in a single session, so
     *    there's no easy way to know when we've seen the last occurrence.
     *    The solution here is to queue one link until we see the next.
     *    At the end of the session, you can enumerate any remaining
     *    entries by calling archive_entry_linkify(NULL) and store those
     *    bodies.  If you have a file with three links l1, l2, and l3,
     *    you'll get the following behavior if you see all three links:
     *           linkify(l1) => NULL   (the resolver stores l1 internally)
     *           linkify(l2) => l1     (resolver stores l2, you write l1)
     *           linkify(l3) => l2, l3 (all links seen, you can write both).
     *    If you only see l1 and l2, you'll get this behavior:
     *           linkify(l1) => NULL
     *           linkify(l2) => l1
     *           linkify(NULL) => l2   (at end, you retrieve remaining links)
     *    As the name suggests, this strategy is used by newer cpio variants.
     *    It's noticeably more complex for the archiver, slightly more complex
     *    for the dearchiver than the tar strategy, but makes it straightforward
     *    to restore a file using any link by simply continuing to scan until
     *    you see a link that is stored with a body.  In contrast, the tar
     *    strategy requires you to rescan the archive from the beginning to
     *    correctly extract an arbitrary link.
     */
    #[no_mangle]
    fn archive_entry_linkresolver_new() -> *mut archive_entry_linkresolver;
    #[no_mangle]
    fn archive_entry_linkresolver_set_strategy(_: *mut archive_entry_linkresolver, _: libc::c_int);
    #[no_mangle]
    fn archive_entry_linkresolver_free(_: *mut archive_entry_linkresolver);
    #[no_mangle]
    fn archive_entry_linkify(
        _: *mut archive_entry_linkresolver,
        _: *mut *mut archive_entry,
        _: *mut *mut archive_entry,
    );
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn do_chdir(_: *mut bsdtar);
    #[no_mangle]
    fn edit_pathname(_: *mut bsdtar, _: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn need_report() -> libc::c_int;
    #[no_mangle]
    fn safe_fprintf(_: *mut FILE, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn set_chdir(_: *mut bsdtar, newdir: *const libc::c_char);
    #[no_mangle]
    fn tar_i64toa(_: int64_t) -> *const libc::c_char;
    #[no_mangle]
    fn usage() -> !;
    #[no_mangle]
    fn yes(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn cset_get_format(_: *mut creation_set) -> *const libc::c_char;
    #[no_mangle]
    fn cset_set_format(_: *mut creation_set, _: *const libc::c_char);
    #[no_mangle]
    fn cset_write_add_filters(
        _: *mut creation_set,
        _: *mut archive,
        _: *mut *const libc::c_void,
    ) -> libc::c_int;
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
    #[no_mangle]
    fn lafe_line_reader(
        _: *const libc::c_char,
        nullSeparator: libc::c_int,
    ) -> *mut lafe_line_reader;
    #[no_mangle]
    fn lafe_line_reader_next(_: *mut lafe_line_reader) -> *const libc::c_char;
    #[no_mangle]
    fn lafe_line_reader_free(_: *mut lafe_line_reader);
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type time_t = __time_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type ssize_t = __ssize_t;
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
/* The la_ssize_t should match the type used in 'struct stat' */
/* Older code relied on the __LA_SSIZE_T macro; after 4.0 we'll switch to the typedef exclusively. */
/* ssize_t */
pub type la_ssize_t = ssize_t;
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
pub struct archive_dir {
    pub head: *mut archive_dir_entry,
    pub tail: *mut archive_dir_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_dir_entry {
    pub next: *mut archive_dir_entry,
    pub mtime_sec: time_t,
    pub mtime_nsec: libc::c_int,
    pub name: *mut libc::c_char,
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
/* Retry might succeed. */
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
/* Partial success. */
/* For example, if write_header "fails", then you can't push data. */
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
/* Current operation cannot complete. */
/* But if write_header is "fatal," then this archive is dead and useless. */
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
/*
 * Codes to identify various stream filters.
 */
pub const ARCHIVE_FILTER_NONE: libc::c_int = 0 as libc::c_int;
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
pub const ARCHIVE_FORMAT_BASE_MASK: libc::c_int = 0xff0000 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR: libc::c_int = 0x30000 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR_PAX_RESTRICTED: libc::c_int = ARCHIVE_FORMAT_TAR | 3 as libc::c_int;
pub const ARCHIVE_FORMAT_EMPTY: libc::c_int = 0x60000 as libc::c_int;
pub const ARCHIVE_MATCH_MTIME: libc::c_int = 0x100 as libc::c_int;
pub const ARCHIVE_MATCH_OLDER: libc::c_int = 0x2 as libc::c_int;
pub const ARCHIVE_MATCH_EQUAL: libc::c_int = 0x10 as libc::c_int;
pub const __S_IFMT: libc::c_int = 0o170000 as libc::c_int;
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
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const SEEK_SET: libc::c_int = 0 as libc::c_int;
pub const O_RDWR: libc::c_int = 0o2 as libc::c_int;
pub const O_CREAT: libc::c_int = 0o100 as libc::c_int;
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
pub const ENV_WRITER_OPTIONS: [libc::c_char; 19] =
    unsafe { *::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"TAR_WRITER_OPTIONS\x00") };
pub const IGNORE_WRONG_MODULE_NAME: [libc::c_char; 30] = unsafe {
    *::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(b"__ignore_wrong_module_name__,\x00")
};
/* for util.c */
/* Options for flags bitfield */
/* -a */
/* -P */
/* --chroot */
/* --fast-read */
/* --ignore-zeros */
pub const OPTFLAG_INTERACTIVE: libc::c_int = 0x20 as libc::c_int;
/* -w */
/* -o */
pub const OPTFLAG_NO_SUBDIRS: libc::c_int = 0x80 as libc::c_int;
/* -n */
pub const OPTFLAG_NULL: libc::c_int = 0x100 as libc::c_int;
/* --null */
/* --numeric-owner */
/* -o */
/* -O */
pub const OPTFLAG_TOTALS: libc::c_int = 0x1000 as libc::c_int;
pub const O_BINARY: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn set_writer_options(mut bsdtar: *mut bsdtar, mut a: *mut archive) {
    let mut writer_options: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    writer_options = getenv(ENV_WRITER_OPTIONS.as_ptr());
    if !writer_options.is_null() {
        let mut module_len: size_t = (::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut opt_len: size_t =
            strlen(writer_options).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        /* Set default write options. */
        p = malloc(module_len.wrapping_add(opt_len)) as *mut libc::c_char;
        if p.is_null() {
            lafe_errc(
                1 as libc::c_int,
                errno,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
        }
        /* Prepend magic code to ignore options for
         * a format or filters which are not added to
         * the archive write object. */
        memcpy(
            p as *mut libc::c_void,
            IGNORE_WRONG_MODULE_NAME.as_ptr(),
            module_len,
        );
        memcpy(
            p as *mut libc::c_void,
            writer_options as *const libc::c_void,
            opt_len,
        );
        r = archive_write_set_options(a, p);
        free(p as *mut libc::c_void);
        if r < ARCHIVE_WARN {
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
    if ARCHIVE_OK != archive_write_set_options(a, (*bsdtar).option_options) {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    };
}
unsafe extern "C" fn set_reader_options(mut bsdtar: *mut bsdtar, mut a: *mut archive) {
    let mut reader_options: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    /* UNUSED */
    reader_options = getenv(ENV_READER_OPTIONS.as_ptr());
    if !reader_options.is_null() {
        let mut module_len: size_t = (::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut opt_len: size_t =
            strlen(reader_options).wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        /* Set default write options. */
        p = malloc(module_len.wrapping_add(opt_len)) as *mut libc::c_char;
        if p.is_null() {
            if p.is_null() {
                lafe_errc(
                    1 as libc::c_int,
                    errno,
                    b"Out of memory\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        /* Prepend magic code to ignore options for
         * a format or filters which are not added to
         * the archive write object. */
        memcpy(
            p as *mut libc::c_void,
            IGNORE_WRONG_MODULE_NAME.as_ptr(),
            module_len,
        );
        memcpy(
            p as *mut libc::c_void,
            reader_options as *const libc::c_void,
            opt_len,
        );
        r = archive_read_set_options(a, p);
        free(p as *mut libc::c_void);
        if r < ARCHIVE_WARN {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
        } else {
            archive_clear_error(a);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn tar_mode_c(mut bsdtar: *mut bsdtar) {
    let mut a: *mut archive = 0 as *mut archive;
    let mut filter_name: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    if (*(*bsdtar).argv).is_null() && (*bsdtar).names_from_file.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"no files or directories specified\x00" as *const u8 as *const libc::c_char,
        );
    }
    a = archive_write_new();
    /* Support any format that the library supports. */
    if cset_get_format((*bsdtar).cset).is_null() {
        r = archive_write_set_format_pax_restricted(a);
        cset_set_format(
            (*bsdtar).cset,
            b"pax restricted\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        r = archive_write_set_format_by_name(a, cset_get_format((*bsdtar).cset))
    }
    if r != ARCHIVE_OK {
        fprintf(
            stderr,
            b"Can\'t use format %s: %s\n\x00" as *const u8 as *const libc::c_char,
            cset_get_format((*bsdtar).cset),
            archive_error_string(a),
        );
        usage();
    }
    archive_write_set_bytes_per_block(a, (*bsdtar).bytes_per_block);
    archive_write_set_bytes_in_last_block(a, (*bsdtar).bytes_in_last_block);
    r = cset_write_add_filters((*bsdtar).cset, a, &mut filter_name);
    if r < ARCHIVE_WARN {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Unsupported compression option --%s\x00" as *const u8 as *const libc::c_char,
            filter_name as *const libc::c_char,
        );
    }
    set_writer_options(bsdtar, a);
    if !(*bsdtar).passphrase.is_null() {
        r = archive_write_set_passphrase(a, (*bsdtar).passphrase)
    } else {
        r = archive_write_set_passphrase_callback(
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
    if ARCHIVE_OK != archive_write_open_filename(a, (*bsdtar).filename) {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    write_archive(a, bsdtar);
}
/*
 * Same as 'c', except we only support tar or empty formats in
 * uncompressed files on disk.
 */
#[no_mangle]
pub unsafe extern "C" fn tar_mode_r(mut bsdtar: *mut bsdtar) {
    let mut end_offset: int64_t = 0;
    let mut format: libc::c_int = 0;
    let mut a: *mut archive = 0 as *mut archive;
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut r: libc::c_int = 0;
    /* Sanity-test some arguments and the file. */
    test_for_append(bsdtar);
    format = ARCHIVE_FORMAT_TAR_PAX_RESTRICTED;
    (*bsdtar).fd = open(
        (*bsdtar).filename,
        O_RDWR | O_CREAT | O_BINARY,
        0o666 as libc::c_int,
    );
    if (*bsdtar).fd < 0 as libc::c_int {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Cannot open %s\x00" as *const u8 as *const libc::c_char,
            (*bsdtar).filename,
        );
    }
    a = archive_read_new();
    archive_read_support_filter_all(a);
    archive_read_support_format_empty(a);
    archive_read_support_format_tar(a);
    archive_read_support_format_gnutar(a);
    set_reader_options(bsdtar, a);
    r = archive_read_open_fd(a, (*bsdtar).fd, 10240 as libc::c_int as size_t);
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            archive_errno(a),
            b"Can\'t read archive %s: %s\x00" as *const u8 as *const libc::c_char,
            (*bsdtar).filename,
            archive_error_string(a),
        );
    }
    while 0 as libc::c_int == archive_read_next_header(a, &mut entry) {
        if archive_filter_code(a, 0 as libc::c_int) != ARCHIVE_FILTER_NONE {
            archive_read_free(a);
            close((*bsdtar).fd);
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"Cannot append to compressed archive.\x00" as *const u8 as *const libc::c_char,
            );
        }
        /* Keep going until we hit end-of-archive */
        format = archive_format(a)
    }
    end_offset = archive_read_header_position(a);
    archive_read_free(a);
    /* Re-open archive for writing */
    a = archive_write_new();
    /*
     * Set the format to be used for writing.  To allow people to
     * extend empty files, we need to allow them to specify the format,
     * which opens the possibility that they will specify a format that
     * doesn't match the existing format.  Hence, the following bit
     * of arcane ugliness.
     */
    if !cset_get_format((*bsdtar).cset).is_null() {
        /* If the user requested a format, use that, but ... */
        archive_write_set_format_by_name(a, cset_get_format((*bsdtar).cset));
        /* ... complain if it's not compatible. */
        format &= ARCHIVE_FORMAT_BASE_MASK;
        if format != archive_format(a) & ARCHIVE_FORMAT_BASE_MASK && format != ARCHIVE_FORMAT_EMPTY
        {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"Format %s is incompatible with the archive %s.\x00" as *const u8
                    as *const libc::c_char,
                cset_get_format((*bsdtar).cset),
                (*bsdtar).filename,
            );
        }
    } else {
        /*
         * Just preserve the current format, with a little care
         * for formats that libarchive can't write.
         */
        if format == ARCHIVE_FORMAT_EMPTY {
            format = ARCHIVE_FORMAT_TAR_PAX_RESTRICTED
        } /* XXX check return val XXX */
        archive_write_set_format(a, format);
    }
    if lseek((*bsdtar).fd, end_offset, SEEK_SET) < 0 as libc::c_int as libc::c_long {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Could not seek to archive end\x00" as *const u8 as *const libc::c_char,
        );
    }
    set_writer_options(bsdtar, a);
    if ARCHIVE_OK != archive_write_open_fd(a, (*bsdtar).fd) {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    write_archive(a, bsdtar);
    close((*bsdtar).fd);
    (*bsdtar).fd = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn tar_mode_u(mut bsdtar: *mut bsdtar) {
    let mut end_offset: int64_t = 0;
    let mut a: *mut archive = 0 as *mut archive;
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut format: libc::c_int = 0;
    let mut p: *mut archive_dir_entry = 0 as *mut archive_dir_entry;
    let mut archive_dir: archive_dir = archive_dir {
        head: 0 as *mut archive_dir_entry,
        tail: 0 as *mut archive_dir_entry,
    };
    (*bsdtar).archive_dir = &mut archive_dir;
    memset(
        &mut archive_dir as *mut archive_dir as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<archive_dir>() as libc::c_ulong,
    );
    format = ARCHIVE_FORMAT_TAR_PAX_RESTRICTED;
    /* Sanity-test some arguments and the file. */
    test_for_append(bsdtar);
    (*bsdtar).fd = open((*bsdtar).filename, O_RDWR | O_BINARY);
    if (*bsdtar).fd < 0 as libc::c_int {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Cannot open %s\x00" as *const u8 as *const libc::c_char,
            (*bsdtar).filename,
        );
    }
    a = archive_read_new();
    archive_read_support_filter_all(a);
    archive_read_support_format_tar(a);
    archive_read_support_format_gnutar(a);
    set_reader_options(bsdtar, a);
    if archive_read_open_fd(a, (*bsdtar).fd, (*bsdtar).bytes_per_block as size_t) != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Can\'t open %s: %s\x00" as *const u8 as *const libc::c_char,
            (*bsdtar).filename,
            archive_error_string(a),
        );
    }
    /* Build a list of all entries and their recorded mod times. */
    while 0 as libc::c_int == archive_read_next_header(a, &mut entry) {
        if archive_filter_code(a, 0 as libc::c_int) != ARCHIVE_FILTER_NONE {
            archive_read_free(a);
            close((*bsdtar).fd);
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"Cannot append to compressed archive.\x00" as *const u8 as *const libc::c_char,
            );
        }
        if archive_match_exclude_entry(
            (*bsdtar).matching,
            ARCHIVE_MATCH_MTIME | ARCHIVE_MATCH_OLDER | ARCHIVE_MATCH_EQUAL,
            entry,
        ) != ARCHIVE_OK
        {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"Error : %s\x00" as *const u8 as *const libc::c_char,
                archive_error_string((*bsdtar).matching),
            );
        }
        /* Keep going until we hit end-of-archive */
        format = archive_format(a)
    }
    end_offset = archive_read_header_position(a);
    archive_read_free(a);
    /* Record the last format determination we see */
    /* Re-open archive for writing. */
    a = archive_write_new();
    /*
     * Set format to same one auto-detected above.
     */
    archive_write_set_format(a, format);
    archive_write_set_bytes_per_block(a, (*bsdtar).bytes_per_block);
    archive_write_set_bytes_in_last_block(a, (*bsdtar).bytes_in_last_block);
    if lseek((*bsdtar).fd, end_offset, SEEK_SET) < 0 as libc::c_int as libc::c_long {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Could not seek to archive end\x00" as *const u8 as *const libc::c_char,
        );
    }
    set_writer_options(bsdtar, a);
    if ARCHIVE_OK != archive_write_open_fd(a, (*bsdtar).fd) {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    write_archive(a, bsdtar);
    close((*bsdtar).fd);
    (*bsdtar).fd = -(1 as libc::c_int);
    while !(*(*bsdtar).archive_dir).head.is_null() {
        p = (*(*(*bsdtar).archive_dir).head).next;
        free((*(*(*bsdtar).archive_dir).head).name as *mut libc::c_void);
        free((*(*bsdtar).archive_dir).head as *mut libc::c_void);
        (*(*bsdtar).archive_dir).head = p
    }
    (*(*bsdtar).archive_dir).tail = NULL as *mut archive_dir_entry;
}
/*
 * Write user-specified files/dirs to opened archive.
 */
unsafe extern "C" fn write_archive(mut a: *mut archive, mut bsdtar: *mut bsdtar) {
    let mut current_block: u64;
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut sparse_entry: *mut archive_entry = 0 as *mut archive_entry;
    /* Choose a suitable copy buffer size */
    (*bsdtar).buff_size = (64 as libc::c_int * 1024 as libc::c_int) as size_t;
    while (*bsdtar).buff_size < (*bsdtar).bytes_per_block as size_t {
        (*bsdtar).buff_size = ((*bsdtar).buff_size as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    /* Try to compensate for space we'll lose to alignment. */
    (*bsdtar).buff_size = ((*bsdtar).buff_size as libc::c_ulong)
        .wrapping_add((16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
        as size_t as size_t;
    /* Allocate a buffer for file data. */
    (*bsdtar).buff = malloc((*bsdtar).buff_size) as *mut libc::c_char;
    if (*bsdtar).buff.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"cannot allocate memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*bsdtar).resolver = archive_entry_linkresolver_new();
    if (*bsdtar).resolver.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"cannot create link resolver\x00" as *const u8 as *const libc::c_char,
        );
    }
    archive_entry_linkresolver_set_strategy((*bsdtar).resolver, archive_format(a));
    /* Create a read_disk object. */
    (*bsdtar).diskreader = archive_read_disk_new();
    if (*bsdtar).diskreader.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Cannot create read_disk object\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* Tell the read_disk how handle symlink. */
    match (*bsdtar).symlink_mode as libc::c_int {
        72 => {
            archive_read_disk_set_symlink_hybrid((*bsdtar).diskreader);
        }
        76 => {
            archive_read_disk_set_symlink_logical((*bsdtar).diskreader);
        }
        _ => {
            archive_read_disk_set_symlink_physical((*bsdtar).diskreader);
        }
    }
    /* Register entry filters. */
    archive_read_disk_set_matching(
        (*bsdtar).diskreader,
        (*bsdtar).matching,
        Some(
            excluded_callback
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: *mut archive_entry,
                ) -> (),
        ),
        bsdtar as *mut libc::c_void,
    );
    archive_read_disk_set_metadata_filter_callback(
        (*bsdtar).diskreader,
        Some(
            metadata_filter
                as unsafe extern "C" fn(
                    _: *mut archive,
                    _: *mut libc::c_void,
                    _: *mut archive_entry,
                ) -> libc::c_int,
        ),
        bsdtar as *mut libc::c_void,
    );
    /* Set the behavior of archive_read_disk. */
    archive_read_disk_set_behavior((*bsdtar).diskreader, (*bsdtar).readdisk_flags); /* Handle a deferred -C */
    archive_read_disk_set_standard_lookup((*bsdtar).diskreader);
    if !(*bsdtar).names_from_file.is_null() {
        archive_names_from_file(bsdtar, a);
    }
    loop {
        if (*(*bsdtar).argv).is_null() {
            current_block = 1356832168064818221;
            break;
        }
        arg = *(*bsdtar).argv;
        if *arg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
            && *arg.offset(1 as libc::c_int as isize) as libc::c_int == 'C' as i32
        {
            arg = arg.offset(2 as libc::c_int as isize);
            if *arg as libc::c_int == '\u{0}' as i32 {
                (*bsdtar).argv = (*bsdtar).argv.offset(1);
                arg = *(*bsdtar).argv;
                if arg.is_null() {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"Missing argument for -C\x00" as *const u8 as *const libc::c_char,
                    );
                    (*bsdtar).return_value = 1 as libc::c_int;
                    current_block = 13064676843759196241;
                    break;
                } else if *arg as libc::c_int == '\u{0}' as i32 {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"Meaningless argument for -C: \'\'\x00" as *const u8
                            as *const libc::c_char,
                    );
                    (*bsdtar).return_value = 1 as libc::c_int;
                    current_block = 13064676843759196241;
                    break;
                }
            }
            set_chdir(bsdtar, arg);
        } else {
            if *arg as libc::c_int != '/' as i32 {
                do_chdir(bsdtar);
            }
            if *arg as libc::c_int == '@' as i32 {
                if append_archive_filename(bsdtar, a, arg.offset(1 as libc::c_int as isize))
                    != 0 as libc::c_int
                {
                    current_block = 1356832168064818221;
                    break;
                }
            } else {
                write_hierarchy(bsdtar, a, arg);
            }
        }
        (*bsdtar).argv = (*bsdtar).argv.offset(1)
    }
    match current_block {
        1356832168064818221 => {
            archive_read_disk_set_matching(
                (*bsdtar).diskreader,
                NULL as *mut archive,
                ::std::mem::transmute::<
                    libc::intptr_t,
                    Option<
                        unsafe extern "C" fn(
                            _: *mut archive,
                            _: *mut libc::c_void,
                            _: *mut archive_entry,
                        ) -> (),
                    >,
                >(NULL as libc::intptr_t),
                NULL as *mut libc::c_void,
            );
            archive_read_disk_set_metadata_filter_callback(
                (*bsdtar).diskreader,
                ::std::mem::transmute::<
                    libc::intptr_t,
                    Option<
                        unsafe extern "C" fn(
                            _: *mut archive,
                            _: *mut libc::c_void,
                            _: *mut archive_entry,
                        ) -> libc::c_int,
                    >,
                >(NULL as libc::intptr_t),
                NULL as *mut libc::c_void,
            );
            entry = NULL as *mut archive_entry;
            archive_entry_linkify((*bsdtar).resolver, &mut entry, &mut sparse_entry);
            while !entry.is_null() {
                let mut r: libc::c_int = 0;
                let mut entry2: *mut archive_entry = 0 as *mut archive_entry;
                let mut disk: *mut archive = (*bsdtar).diskreader;
                /*
                 * This tricky code here is to correctly read the contents
                 * of the entry because the disk reader bsdtar->diskreader
                 * is pointing at does not have any information about the
                 * entry by this time and using archive_read_data_block()
                 * with the disk reader consequently must fail. And we
                 * have to re-open the entry to read the contents.
                 */
                /* TODO: Work with -C option as well. */
                r = archive_read_disk_open(disk, archive_entry_sourcepath(entry));
                if r != ARCHIVE_OK {
                    lafe_warnc(
                        archive_errno(disk),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string(disk),
                    );
                    (*bsdtar).return_value = 1 as libc::c_int
                } else {
                    /*
                     * Invoke archive_read_next_header2() to work
                     * archive_read_data_block(), which is called via write_file(),
                     * without failure.
                     */
                    entry2 = archive_entry_new();
                    r = archive_read_next_header2(disk, entry2);
                    archive_entry_free(entry2);
                    if r != ARCHIVE_OK {
                        lafe_warnc(
                            archive_errno(disk),
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            archive_error_string(disk),
                        );
                        if r == ARCHIVE_FATAL {
                            (*bsdtar).return_value = 1 as libc::c_int
                        }
                        archive_read_close(disk);
                    } else {
                        write_file(bsdtar, a, entry);
                        archive_read_close(disk);
                    }
                }
                archive_entry_free(entry);
                entry = NULL as *mut archive_entry;
                archive_entry_linkify((*bsdtar).resolver, &mut entry, &mut sparse_entry);
            }
            if archive_write_close(a) != 0 {
                lafe_warnc(
                    0 as libc::c_int,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    archive_error_string(a),
                );
                (*bsdtar).return_value = 1 as libc::c_int
            }
        }
        _ => {}
    }
    /* Free file data buffer. */
    free((*bsdtar).buff as *mut libc::c_void);
    archive_entry_linkresolver_free((*bsdtar).resolver);
    (*bsdtar).resolver = NULL as *mut archive_entry_linkresolver;
    archive_read_free((*bsdtar).diskreader);
    (*bsdtar).diskreader = NULL as *mut archive;
    if (*bsdtar).flags & OPTFLAG_TOTALS as libc::c_uint != 0 {
        fprintf(
            stderr,
            b"Total bytes written: %s\n\x00" as *const u8 as *const libc::c_char,
            tar_i64toa(archive_filter_bytes(a, -(1 as libc::c_int))),
        );
    }
    archive_write_free(a);
}
/*
 * Archive names specified in file.
 *
 * Unless --null was specified, a line containing exactly "-C" will
 * cause the next line to be a directory to pass to chdir().  If
 * --null is specified, then a line "-C" is just another filename.
 */
unsafe extern "C" fn archive_names_from_file(mut bsdtar: *mut bsdtar, mut a: *mut archive) {
    let mut lr: *mut lafe_line_reader = 0 as *mut lafe_line_reader; /* Handle a deferred -C */
    let mut line: *const libc::c_char = 0 as *const libc::c_char;
    (*bsdtar).next_line_is_dir = 0 as libc::c_int as libc::c_char;
    lr = lafe_line_reader(
        (*bsdtar).names_from_file,
        ((*bsdtar).flags & OPTFLAG_NULL as libc::c_uint) as libc::c_int,
    );
    loop {
        line = lafe_line_reader_next(lr);
        if line.is_null() {
            break;
        }
        if (*bsdtar).next_line_is_dir != 0 {
            if *line as libc::c_int != '\u{0}' as i32 {
                set_chdir(bsdtar, line);
            } else {
                lafe_warnc(
                    0 as libc::c_int,
                    b"Meaningless argument for -C: \'\'\x00" as *const u8 as *const libc::c_char,
                );
                (*bsdtar).return_value = 1 as libc::c_int
            }
            (*bsdtar).next_line_is_dir = 0 as libc::c_int as libc::c_char
        } else if (*bsdtar).flags & OPTFLAG_NULL as libc::c_uint == 0 as libc::c_int as libc::c_uint
            && strcmp(line, b"-C\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*bsdtar).next_line_is_dir = 1 as libc::c_int as libc::c_char
        } else {
            if *line as libc::c_int != '/' as i32 {
                do_chdir(bsdtar);
            }
            write_hierarchy(bsdtar, a, line);
        }
    }
    lafe_line_reader_free(lr);
    if (*bsdtar).next_line_is_dir != 0 {
        lafe_errc(
            1 as libc::c_int,
            errno,
            b"Unexpected end of filename list; directory expected after -C\x00" as *const u8
                as *const libc::c_char,
        );
    };
}
/*
 * Copy from specified archive to current archive.  Returns non-zero
 * for write errors (which force us to terminate the entire archiving
 * operation).  If there are errors reading the input archive, we set
 * bsdtar->return_value but return zero, so the overall archiving
 * operation will complete and return non-zero.
 */
unsafe extern "C" fn append_archive_filename(
    mut bsdtar: *mut bsdtar,
    mut a: *mut archive,
    mut raw_filename: *const libc::c_char,
) -> libc::c_int {
    let mut ina: *mut archive = 0 as *mut archive; /* Library uses NULL for stdio. */
    let mut filename: *const libc::c_char = raw_filename;
    let mut rc: libc::c_int = 0;
    if strcmp(filename, b"-\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        filename = NULL as *const libc::c_char
    }
    ina = archive_read_new();
    archive_read_support_format_all(ina);
    archive_read_support_filter_all(ina);
    set_reader_options(bsdtar, ina);
    archive_read_set_options(
        ina,
        b"mtree:checkfs\x00" as *const u8 as *const libc::c_char,
    );
    if !(*bsdtar).passphrase.is_null() {
        rc = archive_read_add_passphrase(a, (*bsdtar).passphrase)
    } else {
        rc = archive_read_set_passphrase_callback(
            ina,
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
    if rc != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    if archive_read_open_filename(ina, filename, (*bsdtar).bytes_per_block as size_t) != 0 {
        lafe_warnc(
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(ina),
        );
        (*bsdtar).return_value = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    rc = append_archive(bsdtar, a, ina);
    if rc != ARCHIVE_OK {
        lafe_warnc(
            0 as libc::c_int,
            b"Error reading archive %s: %s\x00" as *const u8 as *const libc::c_char,
            raw_filename,
            archive_error_string(ina),
        );
        (*bsdtar).return_value = 1 as libc::c_int
    }
    archive_read_free(ina);
    return rc;
}
unsafe extern "C" fn append_archive(
    mut bsdtar: *mut bsdtar,
    mut a: *mut archive,
    mut ina: *mut archive,
) -> libc::c_int {
    let mut in_entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut e: libc::c_int = 0;
    loop {
        e = archive_read_next_header(ina, &mut in_entry);
        if !(ARCHIVE_OK == e) {
            break;
        }
        if archive_match_excluded((*bsdtar).matching, in_entry) != 0 {
            continue;
        }
        if (*bsdtar).flags & OPTFLAG_INTERACTIVE as libc::c_uint != 0
            && yes(
                b"copy \'%s\'\x00" as *const u8 as *const libc::c_char,
                archive_entry_pathname(in_entry),
            ) == 0
        {
            continue;
        }
        if (*bsdtar).verbose > 1 as libc::c_int {
            safe_fprintf(stderr, b"a \x00" as *const u8 as *const libc::c_char);
            list_item_verbose(bsdtar, stderr, in_entry);
        } else if (*bsdtar).verbose > 0 as libc::c_int {
            safe_fprintf(
                stderr,
                b"a %s\x00" as *const u8 as *const libc::c_char,
                archive_entry_pathname(in_entry),
            );
        }
        if need_report() != 0 {
            report_write(bsdtar, a, in_entry, 0 as libc::c_int as int64_t);
        }
        e = archive_write_header(a, in_entry);
        if e != ARCHIVE_OK {
            if (*bsdtar).verbose == 0 {
                lafe_warnc(
                    0 as libc::c_int,
                    b"%s: %s\x00" as *const u8 as *const libc::c_char,
                    archive_entry_pathname(in_entry),
                    archive_error_string(a),
                );
            } else {
                fprintf(
                    stderr,
                    b": %s\x00" as *const u8 as *const libc::c_char,
                    archive_error_string(a),
                );
            }
        }
        if e == ARCHIVE_FATAL {
            exit(1 as libc::c_int);
        }
        if e >= ARCHIVE_WARN {
            if archive_entry_size(in_entry) == 0 as libc::c_int as libc::c_long {
                archive_read_data_skip(ina);
            } else if copy_file_data_block(bsdtar, a, ina, in_entry) != 0 {
                exit(1 as libc::c_int);
            }
        }
        if (*bsdtar).verbose != 0 {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    return if e == ARCHIVE_EOF { ARCHIVE_OK } else { e };
}
/* Helper function to copy file to archive. */
unsafe extern "C" fn copy_file_data_block(
    mut bsdtar: *mut bsdtar,
    mut a: *mut archive,
    mut in_a: *mut archive,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut bytes_read: size_t = 0;
    let mut bytes_written: ssize_t = 0;
    let mut offset: int64_t = 0;
    let mut progress: int64_t = 0 as libc::c_int as int64_t;
    let mut null_buff: *mut libc::c_char = NULL as *mut libc::c_char;
    let mut buff: *const libc::c_void = 0 as *const libc::c_void;
    let mut r: libc::c_int = 0;
    loop {
        r = archive_read_data_block(in_a, &mut buff, &mut bytes_read, &mut offset);
        if !(r == ARCHIVE_OK) {
            break;
        }
        if need_report() != 0 {
            report_write(bsdtar, a, entry, progress);
        }
        if offset > progress {
            let mut sparse: int64_t = offset - progress;
            let mut ns: size_t = 0;
            if null_buff.is_null() {
                null_buff = (*bsdtar).buff;
                memset(
                    null_buff as *mut libc::c_void,
                    0 as libc::c_int,
                    (*bsdtar).buff_size,
                );
            }
            while sparse > 0 as libc::c_int as libc::c_long {
                if sparse > (*bsdtar).buff_size as int64_t {
                    ns = (*bsdtar).buff_size
                } else {
                    ns = sparse as size_t
                }
                bytes_written = archive_write_data(a, null_buff as *const libc::c_void, ns);
                if bytes_written < 0 as libc::c_int as libc::c_long {
                    /* Write failed; this is bad */
                    lafe_warnc(
                        0 as libc::c_int,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string(a),
                    );
                    return -(1 as libc::c_int);
                }
                if (bytes_written as size_t) < ns {
                    /* Write was truncated; warn but
                     * continue. */
                    lafe_warnc(
                        0 as libc::c_int,
                        b"%s: Truncated write; file may have grown while being archived.\x00"
                            as *const u8 as *const libc::c_char,
                        archive_entry_pathname(entry),
                    );
                    return 0 as libc::c_int;
                }
                progress += bytes_written;
                sparse -= bytes_written
            }
        }
        bytes_written = archive_write_data(a, buff, bytes_read);
        if bytes_written < 0 as libc::c_int as libc::c_long {
            /* Write failed; this is bad */
            lafe_warnc(
                0 as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
            return -(1 as libc::c_int);
        }
        if (bytes_written as size_t) < bytes_read {
            /* Write was truncated; warn but continue. */
            lafe_warnc(
                0 as libc::c_int,
                b"%s: Truncated write; file may have grown while being archived.\x00" as *const u8
                    as *const libc::c_char,
                archive_entry_pathname(entry),
            );
            return 0 as libc::c_int;
        }
        progress += bytes_written
    }
    if r < ARCHIVE_WARN {
        lafe_warnc(
            archive_errno(a),
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn excluded_callback(
    mut a: *mut archive,
    mut _data: *mut libc::c_void,
    mut entry: *mut archive_entry,
) {
    let mut bsdtar: *mut bsdtar = _data as *mut bsdtar;
    if (*bsdtar).flags & OPTFLAG_NO_SUBDIRS as libc::c_uint != 0 {
        return;
    }
    if archive_read_disk_can_descend(a) == 0 {
        return;
    }
    if (*bsdtar).flags & OPTFLAG_INTERACTIVE as libc::c_uint != 0
        && yes(
            b"add \'%s\'\x00" as *const u8 as *const libc::c_char,
            archive_entry_pathname(entry),
        ) == 0
    {
        return;
    }
    archive_read_disk_descend(a);
}
unsafe extern "C" fn metadata_filter(
    mut a: *mut archive,
    mut _data: *mut libc::c_void,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut bsdtar: *mut bsdtar = _data as *mut bsdtar;
    /* XXX TODO: check whether this filesystem is
     * synthetic and/or local.  Add a new
     * --local-only option to skip non-local
     * filesystems.  Skip synthetic filesystems
     * regardless.
     *
     * The results should be cached, since
     * tree.c doesn't usually visit a directory
     * and the directory contents together.  A simple
     * move-to-front list should perform quite well.
     *
     * Use archive_read_disk_current_filesystem_is_remote().
     */
    /*
     * If the user vetoes this file/directory, skip it.
     * We want this to be fairly late; if some other
     * check would veto this file, we shouldn't bother
     * the user with it.
     */
    if (*bsdtar).flags & OPTFLAG_INTERACTIVE as libc::c_uint != 0
        && yes(
            b"add \'%s\'\x00" as *const u8 as *const libc::c_char,
            archive_entry_pathname(entry),
        ) == 0
    {
        return 0 as libc::c_int;
    }
    /* Note: if user vetoes, we won't descend. */
    if (*bsdtar).flags & OPTFLAG_NO_SUBDIRS as libc::c_uint == 0 as libc::c_int as libc::c_uint
        && archive_read_disk_can_descend(a) != 0
    {
        archive_read_disk_descend(a);
    }
    return 1 as libc::c_int;
}
/*
 * Add the file or dir hierarchy named by 'path' to the archive
 */
unsafe extern "C" fn write_hierarchy(
    mut bsdtar: *mut bsdtar,
    mut a: *mut archive,
    mut path: *const libc::c_char,
) {
    let mut disk: *mut archive = (*bsdtar).diskreader;
    let mut entry: *mut archive_entry = NULL as *mut archive_entry;
    let mut spare_entry: *mut archive_entry = NULL as *mut archive_entry;
    let mut r: libc::c_int = 0;
    r = archive_read_disk_open(disk, path);
    if r != ARCHIVE_OK {
        lafe_warnc(
            archive_errno(disk),
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(disk),
        );
        (*bsdtar).return_value = 1 as libc::c_int;
        return;
    }
    (*bsdtar).first_fs = -(1 as libc::c_int);
    loop {
        archive_entry_free(entry);
        entry = archive_entry_new();
        r = archive_read_next_header2(disk, entry);
        if r == ARCHIVE_EOF {
            break;
        }
        if r != ARCHIVE_OK {
            lafe_warnc(
                archive_errno(disk),
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(disk),
            );
            if r == ARCHIVE_FATAL || r == ARCHIVE_FAILED {
                (*bsdtar).return_value = 1 as libc::c_int;
                archive_entry_free(entry);
                archive_read_close(disk);
                return;
            } else if r < ARCHIVE_WARN {
                continue;
            }
        }
        if (*bsdtar).uid >= 0 as libc::c_int {
            archive_entry_set_uid(entry, (*bsdtar).uid as la_int64_t);
            if (*bsdtar).uname.is_null() {
                archive_entry_set_uname(
                    entry,
                    archive_read_disk_uname((*bsdtar).diskreader, (*bsdtar).uid as la_int64_t),
                );
            }
        }
        if (*bsdtar).gid >= 0 as libc::c_int {
            archive_entry_set_gid(entry, (*bsdtar).gid as la_int64_t);
            if (*bsdtar).gname.is_null() {
                archive_entry_set_gname(
                    entry,
                    archive_read_disk_gname((*bsdtar).diskreader, (*bsdtar).gid as la_int64_t),
                );
            }
        }
        if !(*bsdtar).uname.is_null() {
            archive_entry_set_uname(entry, (*bsdtar).uname);
        }
        if !(*bsdtar).gname.is_null() {
            archive_entry_set_gname(entry, (*bsdtar).gname);
        }
        /*
         * Rewrite the pathname to be archived.  If rewrite
         * fails, skip the entry.
         */
        if edit_pathname(bsdtar, entry) != 0 {
            continue;
        }
        /* Display entry as we process it. */
        if (*bsdtar).verbose > 1 as libc::c_int {
            safe_fprintf(stderr, b"a \x00" as *const u8 as *const libc::c_char);
            list_item_verbose(bsdtar, stderr, entry);
        } else if (*bsdtar).verbose > 0 as libc::c_int {
            /* This format is required by SUSv2. */
            safe_fprintf(
                stderr,
                b"a %s\x00" as *const u8 as *const libc::c_char,
                archive_entry_pathname(entry),
            );
        }
        /* Non-regular files get archived with zero size. */
        if archive_entry_filetype(entry) != AE_IFREG as mode_t {
            archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
        }
        archive_entry_linkify((*bsdtar).resolver, &mut entry, &mut spare_entry);
        while !entry.is_null() {
            write_file(bsdtar, a, entry);
            archive_entry_free(entry);
            entry = spare_entry;
            spare_entry = NULL as *mut archive_entry
        }
        if (*bsdtar).verbose != 0 {
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    archive_entry_free(entry);
    archive_read_close(disk);
}
/*
 * Write a single file (or directory or other filesystem object) to
 * the archive.
 */
unsafe extern "C" fn write_file(
    mut bsdtar: *mut bsdtar,
    mut a: *mut archive,
    mut entry: *mut archive_entry,
) {
    write_entry(bsdtar, a, entry);
}
/*
 * Write a single entry to the archive.
 */
unsafe extern "C" fn write_entry(
    mut bsdtar: *mut bsdtar,
    mut a: *mut archive,
    mut entry: *mut archive_entry,
) {
    let mut e: libc::c_int = 0;
    e = archive_write_header(a, entry);
    if e != ARCHIVE_OK {
        if (*bsdtar).verbose > 1 as libc::c_int {
            safe_fprintf(stderr, b"a \x00" as *const u8 as *const libc::c_char);
            list_item_verbose(bsdtar, stderr, entry);
            lafe_warnc(
                0 as libc::c_int,
                b": %s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
        } else if (*bsdtar).verbose > 0 as libc::c_int {
            lafe_warnc(
                0 as libc::c_int,
                b"%s: %s\x00" as *const u8 as *const libc::c_char,
                archive_entry_pathname(entry),
                archive_error_string(a),
            );
        } else {
            fprintf(
                stderr,
                b": %s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
        }
    }
    if e == ARCHIVE_FATAL {
        exit(1 as libc::c_int);
    }
    /*
     * If we opened a file earlier, write it out now.  Note that
     * the format handler might have reset the size field to zero
     * to inform us that the archive body won't get stored.  In
     * that case, just skip the write.
     */
    if e >= ARCHIVE_WARN && archive_entry_size(entry) > 0 as libc::c_int as libc::c_long {
        if copy_file_data_block(bsdtar, a, (*bsdtar).diskreader, entry) != 0 {
            exit(1 as libc::c_int);
        }
    };
}
unsafe extern "C" fn report_write(
    mut bsdtar: *mut bsdtar,
    mut a: *mut archive,
    mut entry: *mut archive_entry,
    mut progress: int64_t,
) {
    let mut comp: uint64_t = 0;
    let mut uncomp: uint64_t = 0;
    let mut compression: libc::c_int = 0;
    if (*bsdtar).verbose != 0 {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    comp = archive_filter_bytes(a, -(1 as libc::c_int)) as uint64_t;
    uncomp = archive_filter_bytes(a, 0 as libc::c_int) as uint64_t;
    fprintf(
        stderr,
        b"In: %d files, %s bytes;\x00" as *const u8 as *const libc::c_char,
        archive_file_count(a),
        tar_i64toa(uncomp as int64_t),
    );
    if comp >= uncomp {
        compression = 0 as libc::c_int
    } else {
        compression = uncomp
            .wrapping_sub(comp)
            .wrapping_mul(100 as libc::c_int as libc::c_ulong)
            .wrapping_div(uncomp) as libc::c_int
    }
    fprintf(
        stderr,
        b" Out: %s bytes, compression %d%%\n\x00" as *const u8 as *const libc::c_char,
        tar_i64toa(comp as int64_t),
        compression,
    );
    /* Can't have two calls to tar_i64toa() pending, so split the output. */
    safe_fprintf(
        stderr,
        b"Current: %s (%s\x00" as *const u8 as *const libc::c_char,
        archive_entry_pathname(entry),
        tar_i64toa(progress),
    );
    fprintf(
        stderr,
        b"/%s bytes)\n\x00" as *const u8 as *const libc::c_char,
        tar_i64toa(archive_entry_size(entry)),
    );
}
unsafe extern "C" fn test_for_append(mut bsdtar: *mut bsdtar) {
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if (*(*bsdtar).argv).is_null() && (*bsdtar).names_from_file.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"no files or directories specified\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*bsdtar).filename.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Cannot append to stdout.\x00" as *const u8 as *const libc::c_char,
        );
    }
    if stat((*bsdtar).filename, &mut s) != 0 as libc::c_int {
        return;
    }
    if !(s.st_mode & __S_IFMT as libc::c_uint == 0o100000 as libc::c_int as libc::c_uint)
        && !(s.st_mode & __S_IFMT as libc::c_uint == 0o60000 as libc::c_int as libc::c_uint)
    {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Cannot append to %s: not a regular file.\x00" as *const u8 as *const libc::c_char,
            (*bsdtar).filename,
        );
    };
    /* Is this an appropriate check here on Windows? */
    /*
        if (GetFileType(handle) != FILE_TYPE_DISK)
            lafe_errc(1, 0, "Cannot append");
    */
}
