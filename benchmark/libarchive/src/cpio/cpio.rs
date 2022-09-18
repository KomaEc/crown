use ::libc;
extern "C" {
    /* Declare our basic types. */
    pub type archive;
    pub type archive_entry;
    pub type archive_entry_linkresolver;
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
     * Detailed textual name/version of the library and its dependencies.
     * This has the form:
     *    "libarchive x.y.z zlib/a.b.c liblzma/d.e.f ... etc ..."
     * the list of libraries described here will vary depending on how
     * libarchive was compiled.
     */
    #[no_mangle]
    fn archive_version_details() -> *const libc::c_char;
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
    #[no_mangle]
    fn archive_write_add_filter_b64encode(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_bzip2(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_compress(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_grzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_gzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_lrzip(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_lz4(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_lzma(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_lzop(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_none(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_uuencode(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_xz(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_add_filter_zstd(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_write_set_format_by_name(_: *mut archive, name: *const libc::c_char) -> libc::c_int;
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
    /* This interface is currently only available for archive_write_disk handles.  */
    #[no_mangle]
    fn archive_write_data_block(
        _: *mut archive,
        _: *const libc::c_void,
        _: size_t,
        _: la_int64_t,
    ) -> la_ssize_t;
    #[no_mangle]
    fn archive_write_close(_: *mut archive) -> libc::c_int;
    /* This can fail if the archive wasn't already closed, in which case
     * archive_write_free() will implicitly call archive_write_close(). */
    #[no_mangle]
    fn archive_write_free(_: *mut archive) -> libc::c_int;
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
    /* TODO: Handle Linux stat32/stat64 ugliness. <sigh> */
    #[no_mangle]
    fn archive_read_disk_entry_from_file(
        _: *mut archive,
        _: *mut archive_entry,
        _: libc::c_int,
        _: *const stat,
    ) -> libc::c_int;
    /* "Standard" implementation uses getpwuid_r, getgrgid_r and caches the
     * results for performance. */
    #[no_mangle]
    fn archive_read_disk_set_standard_lookup(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_filter_bytes(_: *mut archive, _: libc::c_int) -> la_int64_t;
    #[no_mangle]
    fn archive_errno(_: *mut archive) -> libc::c_int;
    #[no_mangle]
    fn archive_error_string(_: *mut archive) -> *const libc::c_char;
    #[no_mangle]
    fn archive_format(_: *mut archive) -> libc::c_int;
    /*
     * ARCHIVE_MATCH API
     */
    #[no_mangle]
    fn archive_match_new() -> *mut archive;
    #[no_mangle]
    fn archive_match_free(_: *mut archive) -> libc::c_int;
    /*
     * Test if pathname is excluded. The conditions are set by following functions.
     */
    #[no_mangle]
    fn archive_match_path_excluded(_: *mut archive, _: *mut archive_entry) -> libc::c_int;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn geteuid() -> __uid_t;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    #[no_mangle]
    fn putchar(__c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    /* Add exclusion pathname pattern. */
    #[no_mangle]
    fn archive_match_exclude_pattern(_: *mut archive, _: *const libc::c_char) -> libc::c_int;
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
    #[no_mangle]
    fn archive_entry_clone(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_new() -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_atime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_atime_nsec(_: *mut archive_entry) -> libc::c_long;
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
    fn archive_entry_mtime_nsec(_: *mut archive_entry) -> libc::c_long;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
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
    /*
     * Routines to bulk copy fields to/from a platform-native "struct
     * stat."  Libarchive used to just store a struct stat inside of each
     * archive_entry object, but this created issues when trying to
     * manipulate archives on systems different than the ones they were
     * created on.
     *
     * TODO: On Linux and other LFS systems, provide both stat32 and
     * stat64 versions of these functions and all of the macro glue so
     * that archive_entry_stat is magically defined to
     * archive_entry_stat32 or archive_entry_stat64 as appropriate.
     */
    /*
     * Storage for Mac OS-specific AppleDouble metadata information.
     * Apple-format tar files store a separate binary blob containing
     * encoded metadata with ACL, extended attributes, etc.
     * This provides a place to store that blob.
     */
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
    /* POSIX.1e only */
    /* POSIX.1e only */
    /* NFS4 only */
    /* NFS4 only */
    /* NFS4 only */
    /* NFS4 only */
    /* Tag values mimic POSIX.1e */
    /* Specified user. */
    /* User who owns the file. */
    /* Specified group. */
    /* Group who owns the file. */
    /* Modify group access (POSIX.1e only) */
    /* Public (POSIX.1e only) */
    /* Everyone (NFS4 only) */
    /*
     * Set the ACL by clearing it and adding entries one at a time.
     * Unlike the POSIX.1e ACL routines, you must specify the type
     * (access/default) for each entry.  Internally, the ACL data is just
     * a soup of entries.  API calls here allow you to retrieve just the
     * entries of interest.  This design (which goes against the spirit of
     * POSIX.1e) is useful for handling archive formats that combine
     * default and access information in a single ACL list.
     */
    /* type */
    /* permset */
    /* tag */
    /* qual */
    /* name */
    /* type */
    /* permset */
    /* tag */
    /* qual */
    /* name */
    /*
     * To retrieve the ACL, first "reset", then repeatedly ask for the
     * "next" entry.  The want_type parameter allows you to request only
     * certain types of entries.
     */
    /* want_type */
    /* want_type */
    /* type */
    /* permset */
    /* tag */
    /* qual */
    /* name */
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
    /* len */
    /* flags */
    /* len */
    /* flags */
    /* wtext */
    /* type */
    /* text */
    /* type */
    /* Deprecated constants */
    /* Deprecated functions */
    /* flags */
    /* flags */
    /* Return bitmask of ACL types in an archive entry */
    /* Return a count of entries matching 'want_type' */
    /* want_type */
    /* Return an opaque ACL object. */
    /* There's not yet anything clients can actually do with this... */
    /*
     * extended attributes
     */
    /* name */
    /* value */
    /* size */
    /*
     * To retrieve the xattr list, first "reset", then repeatedly ask for the
     * "next" entry.
     */
    /* name */
    /* value */
    /*
     * sparse
     */
    /* offset */
    /* length */
    /*
     * To retrieve the xattr list, first "reset", then repeatedly ask for the
     * "next" entry.
     */
    /* offset */
    /* length */
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
    /* format_code */
    #[no_mangle]
    fn archive_entry_linkify(
        _: *mut archive_entry_linkresolver,
        _: *mut *mut archive_entry,
        _: *mut *mut archive_entry,
    );
    #[no_mangle]
    fn archive_entry_set_gid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_gname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_hardlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_copy_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_copy_sourcepath(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_uid(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_uname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_linkresolver_new() -> *mut archive_entry_linkresolver;
    #[no_mangle]
    fn archive_entry_linkresolver_set_strategy(_: *mut archive_entry_linkresolver, _: libc::c_int);
    #[no_mangle]
    fn archive_entry_linkresolver_free(_: *mut archive_entry_linkresolver);
    #[no_mangle]
    fn archive_entry_rdevmajor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_rdevminor(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_sourcepath(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_size_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_strmode(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_uname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn lutimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn futimes(__fd: libc::c_int, __tvp: *const timeval) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn getgrgid(__gid: __gid_t) -> *mut group;
    #[no_mangle]
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn owner_parse(
        _: *const libc::c_char,
        _: *mut libc::c_int,
        _: *mut libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn cpio_getopt(cpio: *mut cpio) -> libc::c_int;
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
    #[no_mangle]
    fn lafe_line_reader(
        _: *const libc::c_char,
        nullSeparator: libc::c_int,
    ) -> *mut lafe_line_reader;
    #[no_mangle]
    fn lafe_line_reader_next(_: *mut lafe_line_reader) -> *const libc::c_char;
    #[no_mangle]
    fn lafe_line_reader_free(_: *mut lafe_line_reader);
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
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __id_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type int64_t = __int64_t;
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
pub type id_t = __id_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
 * $FreeBSD: src/usr.bin/cpio/cpio.h,v 1.7 2008/12/06 07:30:40 kientzle Exp $
 */
/*
 * The internal state for the "cpio" program.
 *
 * Keeping all of the state in a structure like this simplifies memory
 * leak testing (at exit, anything left on the heap is suspect).  A
 * pointer to this structure is passed to most cpio internal
 * functions.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpio {
    pub argument: *const libc::c_char,
    pub add_filter: libc::c_int,
    pub filename: *const libc::c_char,
    pub mode: libc::c_int,
    pub compress: libc::c_int,
    pub format: *const libc::c_char,
    pub bytes_per_block: libc::c_int,
    pub verbose: libc::c_int,
    pub dot: libc::c_int,
    pub quiet: libc::c_int,
    pub extract_flags: libc::c_int,
    pub compress_program: *const libc::c_char,
    pub option_append: libc::c_int,
    pub option_atime_restore: libc::c_int,
    pub option_follow_links: libc::c_int,
    pub option_link: libc::c_int,
    pub option_list: libc::c_int,
    pub option_null: libc::c_char,
    pub option_numeric_uid_gid: libc::c_int,
    pub option_rename: libc::c_int,
    pub destdir: *mut libc::c_char,
    pub destdir_len: size_t,
    pub pass_destpath_alloc: size_t,
    pub pass_destpath: *mut libc::c_char,
    pub uid_override: libc::c_int,
    pub uname_override: *mut libc::c_char,
    pub gid_override: libc::c_int,
    pub gname_override: *mut libc::c_char,
    pub day_first: libc::c_int,
    pub passphrase: *const libc::c_char,
    pub fd: libc::c_int,
    pub archive: *mut archive,
    pub archive_read_disk: *mut archive,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub return_value: libc::c_int,
    pub linkresolver: *mut archive_entry_linkresolver,
    pub uname_cache: *mut name_cache,
    pub gname_cache: *mut name_cache,
    pub matching: *mut archive,
    pub buff: *mut libc::c_char,
    pub buff_size: size_t,
    pub ppbuff: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_cache {
    pub probes: libc::c_int,
    pub hits: libc::c_int,
    pub size: size_t,
    pub cache: [C2RustUnnamed_10; 101],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub id: id_t,
    pub name: *mut libc::c_char,
}
/* Fake short equivalents for long options that otherwise lack them. */
pub type C2RustUnnamed_11 = libc::c_uint;
pub const OPTION_ZSTD: C2RustUnnamed_11 = 14;
pub const OPTION_VERSION: C2RustUnnamed_11 = 13;
pub const OPTION_UUENCODE: C2RustUnnamed_11 = 12;
pub const OPTION_QUIET: C2RustUnnamed_11 = 11;
pub const OPTION_PRESERVE_OWNER: C2RustUnnamed_11 = 10;
pub const OPTION_NO_PRESERVE_OWNER: C2RustUnnamed_11 = 9;
pub const OPTION_PASSPHRASE: C2RustUnnamed_11 = 8;
pub const OPTION_LZOP: C2RustUnnamed_11 = 7;
pub const OPTION_LZMA: C2RustUnnamed_11 = 6;
pub const OPTION_LZ4: C2RustUnnamed_11 = 5;
pub const OPTION_LRZIP: C2RustUnnamed_11 = 4;
pub const OPTION_INSECURE: C2RustUnnamed_11 = 3;
pub const OPTION_GRZIP: C2RustUnnamed_11 = 2;
pub const OPTION_B64ENCODE: C2RustUnnamed_11 = 1;
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
/* Default: Try create first, unlink only if create fails with EEXIST. */
/* Default: Do not restore ACLs. */
pub const ARCHIVE_EXTRACT_ACL: libc::c_int = 0x20 as libc::c_int;
/* Default: Do not restore fflags. */
pub const ARCHIVE_EXTRACT_FFLAGS: libc::c_int = 0x40 as libc::c_int;
/* Default: Do not restore xattrs. */
/* Default: Do not try to guard against extracts redirected by symlinks. */
/* Note: With ARCHIVE_EXTRACT_UNLINK, will remove any intermediate symlink. */
pub const ARCHIVE_EXTRACT_SECURE_SYMLINKS: libc::c_int = 0x100 as libc::c_int;
/* Default: Do not reject entries with '..' as path elements. */
pub const ARCHIVE_EXTRACT_SECURE_NODOTDOT: libc::c_int = 0x200 as libc::c_int;
/* Default: Create parent directories as needed. */
pub const ARCHIVE_EXTRACT_NO_AUTODIR: libc::c_int = 0x400 as libc::c_int;
/* Default: Overwrite files, even if one on disk is newer. */
pub const ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER: libc::c_int = 0x800 as libc::c_int;
/* Detect blocks of 0 and write holes instead. */
/* Default: Do not restore Mac extended metadata. */
/* This has no effect except on Mac OS. */
/* Default: Use HFS+ compression if it was compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not use HFS+ compression if it was not compressed. */
/* This has no effect except on Mac OS v10.6 or later. */
/* Default: Do not reject entries with absolute paths */
pub const ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS: libc::c_int = 0x10000 as libc::c_int;
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
pub const BSDCPIO_VERSION_STRING: [libc::c_char; 6] =
    unsafe { *::std::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"3.4.3\x00") };
pub const AE_IFCHR: libc::c_int = 0o20000 as libc::c_int;
pub const AE_IFBLK: libc::c_int = 0o60000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const AE_IFREG: libc::c_int = 0o100000 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ENOENT: libc::c_int = 2 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const EXDEV: libc::c_int = 18 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const O_RDONLY: libc::c_int = 0 as libc::c_int;
pub const LC_ALL: libc::c_int = __LC_ALL;
pub const __LC_ALL: libc::c_int = 6 as libc::c_int;
pub const SIG_IGN: libc::c_int = 1 as libc::c_int;
pub const SIGPIPE: libc::c_int = 13 as libc::c_int;
/* Fixed size of uname/gname caches. */
pub const name_cache_size: libc::c_int = 101 as libc::c_int;
pub const O_BINARY: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    static mut buff: [libc::c_char; 16384] = [0; 16384]; /* Allocated on stack. */
    let mut _cpio: cpio = cpio {
        argument: 0 as *const libc::c_char,
        add_filter: 0,
        filename: 0 as *const libc::c_char,
        mode: 0,
        compress: 0,
        format: 0 as *const libc::c_char,
        bytes_per_block: 0,
        verbose: 0,
        dot: 0,
        quiet: 0,
        extract_flags: 0,
        compress_program: 0 as *const libc::c_char,
        option_append: 0,
        option_atime_restore: 0,
        option_follow_links: 0,
        option_link: 0,
        option_list: 0,
        option_null: 0,
        option_numeric_uid_gid: 0,
        option_rename: 0,
        destdir: 0 as *mut libc::c_char,
        destdir_len: 0,
        pass_destpath_alloc: 0,
        pass_destpath: 0 as *mut libc::c_char,
        uid_override: 0,
        uname_override: 0 as *mut libc::c_char,
        gid_override: 0,
        gname_override: 0 as *mut libc::c_char,
        day_first: 0,
        passphrase: 0 as *const libc::c_char,
        fd: 0,
        archive: 0 as *mut archive,
        archive_read_disk: 0 as *mut archive,
        argc: 0,
        argv: 0 as *mut *mut libc::c_char,
        return_value: 0,
        linkresolver: 0 as *mut archive_entry_linkresolver,
        uname_cache: 0 as *mut name_cache,
        gname_cache: 0 as *mut name_cache,
        matching: 0 as *mut archive,
        buff: 0 as *mut libc::c_char,
        buff_size: 0,
        ppbuff: 0 as *mut libc::c_char,
    };
    let mut cpio: *mut cpio = 0 as *mut cpio;
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut tptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uid: libc::c_int = 0;
    let mut gid: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    cpio = &mut _cpio;
    memset(
        cpio as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cpio>() as libc::c_ulong,
    );
    (*cpio).buff = buff.as_mut_ptr();
    (*cpio).buff_size = ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong;
    /* Ignore SIGPIPE signals. */
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = 0 as libc::c_int;
    sa.__sigaction_handler.sa_handler =
        ::std::mem::transmute::<libc::intptr_t, __sighandler_t>(SIG_IGN as libc::intptr_t);
    sigaction(SIGPIPE, &mut sa, NULL as *mut sigaction);
    /* Set lafe_progname before calling lafe_warnc. */
    lafe_setprogname(*argv, b"bsdcpio\x00" as *const u8 as *const libc::c_char);
    if setlocale(LC_ALL, b"\x00" as *const u8 as *const libc::c_char).is_null() {
        lafe_warnc(
            0 as libc::c_int,
            b"Failed to set default locale\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*cpio).uid_override = -(1 as libc::c_int);
    (*cpio).gid_override = -(1 as libc::c_int);
    (*cpio).argv = argv;
    (*cpio).argc = argc;
    (*cpio).mode = '\u{0}' as i32;
    (*cpio).verbose = 0 as libc::c_int;
    (*cpio).compress = '\u{0}' as i32;
    (*cpio).extract_flags = ARCHIVE_EXTRACT_NO_AUTODIR;
    (*cpio).extract_flags |= ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER;
    (*cpio).extract_flags |= ARCHIVE_EXTRACT_SECURE_SYMLINKS;
    (*cpio).extract_flags |= ARCHIVE_EXTRACT_SECURE_NODOTDOT;
    (*cpio).extract_flags |= ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS;
    (*cpio).extract_flags |= ARCHIVE_EXTRACT_PERM;
    (*cpio).extract_flags |= ARCHIVE_EXTRACT_FFLAGS;
    (*cpio).extract_flags |= ARCHIVE_EXTRACT_ACL;
    if geteuid() == 0 as libc::c_int as libc::c_uint {
        (*cpio).extract_flags |= ARCHIVE_EXTRACT_OWNER
    }
    (*cpio).bytes_per_block = 512 as libc::c_int;
    (*cpio).filename = NULL as *const libc::c_char;
    (*cpio).matching = archive_match_new();
    if (*cpio).matching.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        opt = cpio_getopt(cpio);
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_106: u64;
        match opt {
            48 => {
                /* GNU convention: --null, -0 */
                (*cpio).option_null = 1 as libc::c_int as libc::c_char;
                current_block_106 = 17336970397495664729;
            }
            65 => {
                /* NetBSD/OpenBSD */
                (*cpio).option_append = 1 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            97 => {
                /* POSIX 1997 */
                (*cpio).option_atime_restore = 1 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            66 => {
                /* POSIX 1997 */
                (*cpio).bytes_per_block = 5120 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            1 => {
                (*cpio).add_filter = opt;
                current_block_106 = 17336970397495664729;
            }
            67 => {
                /* NetBSD/OpenBSD */
                errno = 0 as libc::c_int;
                tptr = NULL as *mut libc::c_char;
                t = strtol((*cpio).argument, &mut tptr, 10 as libc::c_int) as libc::c_int;
                if errno != 0
                    || t <= 0 as libc::c_int
                    || *(*cpio).argument as libc::c_int == '\u{0}' as i32
                    || tptr.is_null()
                    || *tptr as libc::c_int != '\u{0}' as i32
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Invalid blocksize: %s\x00" as *const u8 as *const libc::c_char,
                        (*cpio).argument,
                    );
                }
                (*cpio).bytes_per_block = t;
                current_block_106 = 17336970397495664729;
            }
            99 => {
                /* POSIX 1997 */
                (*cpio).format = b"odc\x00" as *const u8 as *const libc::c_char;
                current_block_106 = 17336970397495664729;
            }
            100 => {
                /* POSIX 1997 */
                (*cpio).extract_flags &= !ARCHIVE_EXTRACT_NO_AUTODIR;
                current_block_106 = 17336970397495664729;
            }
            69 => {
                /* NetBSD/OpenBSD */
                if archive_match_include_pattern_from_file(
                    (*cpio).matching,
                    (*cpio).argument,
                    (*cpio).option_null as libc::c_int,
                ) != ARCHIVE_OK
                {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*cpio).matching),
                    );
                }
                current_block_106 = 17336970397495664729;
            }
            70 => {
                /* NetBSD/OpenBSD/GNU cpio */
                (*cpio).filename = (*cpio).argument;
                current_block_106 = 17336970397495664729;
            }
            102 => {
                /* POSIX 1997 */
                if archive_match_exclude_pattern((*cpio).matching, (*cpio).argument) != ARCHIVE_OK {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*cpio).matching),
                    );
                }
                current_block_106 = 17336970397495664729;
            }
            2 => {
                (*cpio).compress = opt;
                current_block_106 = 17336970397495664729;
            }
            72 => {
                /* GNU cpio (also --format) */
                (*cpio).format = (*cpio).argument;
                current_block_106 = 17336970397495664729;
            }
            104 => {
                long_help();
            }
            73 => {
                /* NetBSD/OpenBSD */
                (*cpio).filename = (*cpio).argument;
                current_block_106 = 17336970397495664729;
            }
            105 => {
                /* POSIX 1997 */
                if (*cpio).mode != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Cannot use both -i and -%c\x00" as *const u8 as *const libc::c_char,
                        (*cpio).mode,
                    );
                }
                (*cpio).mode = opt;
                current_block_106 = 17336970397495664729;
            }
            74 => {
                /* GNU tar, others */
                (*cpio).compress = opt;
                current_block_106 = 17336970397495664729;
            }
            106 => {
                /* GNU tar, others */
                (*cpio).compress = opt;
                current_block_106 = 17336970397495664729;
            }
            3 => {
                (*cpio).extract_flags &= !ARCHIVE_EXTRACT_SECURE_SYMLINKS;
                (*cpio).extract_flags &= !ARCHIVE_EXTRACT_SECURE_NODOTDOT;
                (*cpio).extract_flags &= !ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS;
                current_block_106 = 17336970397495664729;
            }
            76 => {
                /* GNU cpio */
                (*cpio).option_follow_links = 1 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            108 => {
                /* POSIX 1997 */
                (*cpio).option_link = 1 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            4 | 5 | 6 => {
                /* GNU tar, others */
                current_block_106 = 15819562344915545825;
            }
            7 | 14 => {
                current_block_106 = 15819562344915545825;
            }
            109 => {
                /* POSIX 1997 */
                (*cpio).extract_flags |= ARCHIVE_EXTRACT_TIME;
                current_block_106 = 17336970397495664729;
            }
            110 => {
                /* GNU cpio */
                (*cpio).option_numeric_uid_gid = 1 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            9 => {
                /* GNU cpio */
                (*cpio).extract_flags &= !ARCHIVE_EXTRACT_OWNER;
                current_block_106 = 17336970397495664729;
            }
            79 => {
                /* GNU cpio */
                (*cpio).filename = (*cpio).argument;
                current_block_106 = 17336970397495664729;
            }
            111 => {
                /* POSIX 1997 */
                if (*cpio).mode != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Cannot use both -o and -%c\x00" as *const u8 as *const libc::c_char,
                        (*cpio).mode,
                    );
                }
                (*cpio).mode = opt;
                current_block_106 = 17336970397495664729;
            }
            112 => {
                /* POSIX 1997 */
                if (*cpio).mode != '\u{0}' as i32 {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Cannot use both -p and -%c\x00" as *const u8 as *const libc::c_char,
                        (*cpio).mode,
                    );
                }
                (*cpio).mode = opt;
                (*cpio).extract_flags &= !ARCHIVE_EXTRACT_SECURE_NODOTDOT;
                (*cpio).extract_flags &= !ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS;
                current_block_106 = 17336970397495664729;
            }
            8 => {
                (*cpio).passphrase = (*cpio).argument;
                current_block_106 = 17336970397495664729;
            }
            10 => {
                (*cpio).extract_flags |= ARCHIVE_EXTRACT_OWNER;
                current_block_106 = 17336970397495664729;
            }
            11 => {
                /* GNU cpio */
                (*cpio).quiet = 1 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            82 => {
                /* GNU cpio, also --owner */
                /* TODO: owner_parse should return uname/gname
                 * also; use that to set [ug]name_override. */
                errmsg = owner_parse((*cpio).argument, &mut uid, &mut gid);
                if !errmsg.is_null() {
                    lafe_warnc(
                        -(1 as libc::c_int),
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        errmsg,
                    );
                    usage();
                }
                if uid != -(1 as libc::c_int) {
                    (*cpio).uid_override = uid;
                    (*cpio).uname_override = NULL as *mut libc::c_char
                }
                if gid != -(1 as libc::c_int) {
                    (*cpio).gid_override = gid;
                    (*cpio).gname_override = NULL as *mut libc::c_char
                }
                current_block_106 = 17336970397495664729;
            }
            114 => {
                /* POSIX 1997 */
                (*cpio).option_rename = 1 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            116 => {
                /* POSIX 1997 */
                (*cpio).option_list = 1 as libc::c_int;
                current_block_106 = 17336970397495664729;
            }
            117 => {
                /* POSIX 1997 */
                (*cpio).extract_flags &= !ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER;
                current_block_106 = 17336970397495664729;
            }
            12 => {
                (*cpio).add_filter = opt;
                current_block_106 = 17336970397495664729;
            }
            118 => {
                /* POSIX 1997 */
                (*cpio).verbose += 1;
                current_block_106 = 17336970397495664729;
            }
            86 => {
                /* GNU cpio */
                (*cpio).dot += 1;
                current_block_106 = 17336970397495664729;
            }
            13 => {
                /* GNU convention */
                version();
            }
            121 => {
                /* tar convention */
                (*cpio).compress = opt;
                current_block_106 = 17336970397495664729;
            }
            90 => {
                /* tar convention */
                (*cpio).compress = opt;
                current_block_106 = 17336970397495664729;
            }
            122 => {
                /* tar convention */
                (*cpio).compress = opt;
                current_block_106 = 17336970397495664729;
            }
            _ => {
                usage();
            }
        }
        match current_block_106 {
            15819562344915545825 =>
            /* GNU tar, others */
            {
                (*cpio).compress = opt
            }
            _ => {}
        }
    }
    /*
     * Sanity-check args, error out on nonsensical combinations.
     */
    /* -t implies -i if no mode was specified. */
    if (*cpio).option_list != 0 && (*cpio).mode == '\u{0}' as i32 {
        (*cpio).mode = 'i' as i32
    }
    /* -t requires -i */
    if (*cpio).option_list != 0 && (*cpio).mode != 'i' as i32 {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Option -t requires -i\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* -n requires -it */
    if (*cpio).option_numeric_uid_gid != 0 && (*cpio).option_list == 0 {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Option -n requires -it\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* Can only specify format when writing */
    if !(*cpio).format.is_null() && (*cpio).mode != 'o' as i32 {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Option --format requires -o\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* -l requires -p */
    if (*cpio).option_link != 0 && (*cpio).mode != 'p' as i32 {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Option -l requires -p\x00" as *const u8 as *const libc::c_char,
        );
    }
    /* -v overrides -V */
    if (*cpio).dot != 0 && (*cpio).verbose != 0 {
        (*cpio).dot = 0 as libc::c_int
    }
    /* TODO: Flag other nonsensical combinations. */
    match (*cpio).mode {
        111 => {
            if (*cpio).format.is_null() {
                (*cpio).format = b"odc\x00" as *const u8 as *const libc::c_char
            } /* Default format */
            mode_out(cpio);
        }
        105 => {
            while !(*(*cpio).argv).is_null() {
                if archive_match_include_pattern((*cpio).matching, *(*cpio).argv) != ARCHIVE_OK {
                    lafe_errc(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        b"Error : %s\x00" as *const u8 as *const libc::c_char,
                        archive_error_string((*cpio).matching),
                    );
                }
                (*cpio).argc -= 1;
                (*cpio).argv = (*cpio).argv.offset(1)
            }
            if (*cpio).option_list != 0 {
                mode_list(cpio);
            } else {
                mode_in(cpio);
            }
        }
        112 => {
            if (*(*cpio).argv).is_null() || **(*cpio).argv as libc::c_int == '\u{0}' as i32 {
                lafe_errc(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    b"-p mode requires a target directory\x00" as *const u8 as *const libc::c_char,
                );
            }
            mode_pass(cpio, *(*cpio).argv);
        }
        _ => {
            lafe_errc(
                1 as libc::c_int,
                0 as libc::c_int,
                b"Must specify at least one of -i, -o, or -p\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    archive_match_free((*cpio).matching);
    free_cache((*cpio).gname_cache);
    free_cache((*cpio).uname_cache);
    free((*cpio).destdir as *mut libc::c_void);
    passphrase_free((*cpio).ppbuff);
    return (*cpio).return_value;
}
unsafe extern "C" fn usage() -> ! {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = lafe_getprogname();
    fprintf(
        stderr,
        b"Brief Usage:\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"  List:    %s -it < archive\n\x00" as *const u8 as *const libc::c_char,
        p,
    );
    fprintf(
        stderr,
        b"  Extract: %s -i < archive\n\x00" as *const u8 as *const libc::c_char,
        p,
    );
    fprintf(
        stderr,
        b"  Create:  %s -o < filenames > archive\n\x00" as *const u8 as *const libc::c_char,
        p,
    );
    fprintf(
        stderr,
        b"  Help:    %s --help\n\x00" as *const u8 as *const libc::c_char,
        p,
    );
    exit(1 as libc::c_int);
}
static mut long_help_msg: *const libc::c_char =
    b"First option must be a mode specifier:\n  -i Input  -o Output  -p Pass\nCommon Options:\n  -v Verbose filenames     -V  one dot per file\nCreate: %p -o [options]  < [list of files] > [archive]\n  -J,-y,-z,--lzma  Compress archive with xz/bzip2/gzip/lzma\n  --format {odc|newc|ustar}  Select archive format\nList: %p -it < [archive]\nExtract: %p -i [options] < [archive]\n\x00"
        as *const u8 as *const libc::c_char;
/* TODO: Implement old binary format in libarchive,
use that here. */
/*
 * Note that the word 'bsdcpio' will always appear in the first line
 * of output.
 *
 * In particular, /bin/sh scripts that need to test for the presence
 * of bsdcpio can use the following template:
 *
 * if (cpio --help 2>&1 | grep bsdcpio >/dev/null 2>&1 ) then \
 *          echo bsdcpio; else echo not bsdcpio; fi
 */
unsafe extern "C" fn long_help() -> ! {
    let mut prog: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    prog = lafe_getprogname();
    fflush(stderr);
    p = if strcmp(prog, b"bsdcpio\x00" as *const u8 as *const libc::c_char) != 0 as libc::c_int {
        b"(bsdcpio)\x00" as *const u8 as *const libc::c_char
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
unsafe extern "C" fn version() -> ! {
    fprintf(
        stdout,
        b"bsdcpio %s - %s \n\x00" as *const u8 as *const libc::c_char,
        BSDCPIO_VERSION_STRING.as_ptr(),
        archive_version_details(),
    );
    exit(0 as libc::c_int);
}
unsafe extern "C" fn mode_out(mut cpio: *mut cpio) {
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut spare: *mut archive_entry = 0 as *mut archive_entry;
    let mut lr: *mut lafe_line_reader = 0 as *mut lafe_line_reader;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    if (*cpio).option_append != 0 {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Append mode not yet supported.\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*cpio).archive_read_disk = archive_read_disk_new();
    if (*cpio).archive_read_disk.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Failed to allocate archive object\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*cpio).option_follow_links != 0 {
        archive_read_disk_set_symlink_logical((*cpio).archive_read_disk);
    } else {
        archive_read_disk_set_symlink_physical((*cpio).archive_read_disk);
    }
    archive_read_disk_set_standard_lookup((*cpio).archive_read_disk);
    (*cpio).archive = archive_write_new();
    if (*cpio).archive.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Failed to allocate archive object\x00" as *const u8 as *const libc::c_char,
        );
    }
    match (*cpio).compress {
        2 => r = archive_write_add_filter_grzip((*cpio).archive),
        74 => r = archive_write_add_filter_xz((*cpio).archive),
        4 => r = archive_write_add_filter_lrzip((*cpio).archive),
        5 => r = archive_write_add_filter_lz4((*cpio).archive),
        6 => r = archive_write_add_filter_lzma((*cpio).archive),
        7 => r = archive_write_add_filter_lzop((*cpio).archive),
        14 => r = archive_write_add_filter_zstd((*cpio).archive),
        106 | 121 => r = archive_write_add_filter_bzip2((*cpio).archive),
        122 => r = archive_write_add_filter_gzip((*cpio).archive),
        90 => r = archive_write_add_filter_compress((*cpio).archive),
        _ => r = archive_write_add_filter_none((*cpio).archive),
    }
    if r < ARCHIVE_WARN {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Requested compression not available\x00" as *const u8 as *const libc::c_char,
        );
    }
    match (*cpio).add_filter {
        0 => r = ARCHIVE_OK,
        1 => r = archive_write_add_filter_b64encode((*cpio).archive),
        12 => r = archive_write_add_filter_uuencode((*cpio).archive),
        _ => {}
    }
    if r < ARCHIVE_WARN {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Requested filter not available\x00" as *const u8 as *const libc::c_char,
        );
    }
    r = archive_write_set_format_by_name((*cpio).archive, (*cpio).format);
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string((*cpio).archive),
        );
    }
    archive_write_set_bytes_per_block((*cpio).archive, (*cpio).bytes_per_block);
    (*cpio).linkresolver = archive_entry_linkresolver_new();
    archive_entry_linkresolver_set_strategy((*cpio).linkresolver, archive_format((*cpio).archive));
    if !(*cpio).passphrase.is_null() {
        r = archive_write_set_passphrase((*cpio).archive, (*cpio).passphrase)
    } else {
        r = archive_write_set_passphrase_callback(
            (*cpio).archive,
            cpio as *mut libc::c_void,
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
            archive_error_string((*cpio).archive),
        );
    }
    /*
     * The main loop:  Copy each file into the output archive.
     */
    r = archive_write_open_filename((*cpio).archive, (*cpio).filename);
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string((*cpio).archive),
        );
    }
    lr = lafe_line_reader(
        b"-\x00" as *const u8 as *const libc::c_char,
        (*cpio).option_null as libc::c_int,
    );
    loop {
        p = lafe_line_reader_next(lr);
        if p.is_null() {
            break;
        }
        file_to_archive(cpio, p);
    }
    lafe_line_reader_free(lr);
    /*
     * The hardlink detection may have queued up a couple of entries
     * that can now be flushed.
     */
    entry = NULL as *mut archive_entry;
    archive_entry_linkify((*cpio).linkresolver, &mut entry, &mut spare);
    while !entry.is_null() {
        entry_to_archive(cpio, entry);
        archive_entry_free(entry);
        entry = NULL as *mut archive_entry;
        archive_entry_linkify((*cpio).linkresolver, &mut entry, &mut spare);
    }
    r = archive_write_close((*cpio).archive);
    if (*cpio).dot != 0 {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string((*cpio).archive),
        );
    }
    if (*cpio).quiet == 0 {
        let mut blocks: int64_t = (archive_filter_bytes((*cpio).archive, 0 as libc::c_int)
            + 511 as libc::c_int as libc::c_long)
            / 512 as libc::c_int as libc::c_long;
        fprintf(
            stderr,
            b"%lu %s\n\x00" as *const u8 as *const libc::c_char,
            blocks as libc::c_ulong,
            if blocks == 1 as libc::c_int as libc::c_long {
                b"block\x00" as *const u8 as *const libc::c_char
            } else {
                b"blocks\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    archive_write_free((*cpio).archive);
    archive_entry_linkresolver_free((*cpio).linkresolver);
}
unsafe extern "C" fn remove_leading_slash(mut p: *const libc::c_char) -> *const libc::c_char {
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
    }
    loop {
        rp = p;
        /* Remove leading drive letter from archives created
         * on Windows. */
        if (*p.offset(0 as libc::c_int as isize) as libc::c_int >= 'a' as i32
            && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
            || *p.offset(0 as libc::c_int as isize) as libc::c_int >= 'A' as i32
                && *p.offset(0 as libc::c_int as isize) as libc::c_int <= 'Z' as i32)
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
        {
            p = p.offset(2 as libc::c_int as isize)
        }
        /* Remove leading "/../", "//", etc. */
        while *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *p.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
        {
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *p.offset(2 as libc::c_int as isize) as libc::c_int == '.' as i32
                && (*p.offset(3 as libc::c_int as isize) as libc::c_int == '/' as i32
                    || *p.offset(3 as libc::c_int as isize) as libc::c_int == '\\' as i32)
            {
                p = p.offset(3 as libc::c_int as isize)
            /* Remove "/..", leave "/"
             * for next pass. */
            } else {
                p = p.offset(1 as libc::c_int as isize)
            }
            /* Remove "/". */
        }
        if !(rp != p) {
            break;
        }
    }
    return p;
}
/*
 * This is used by both out mode (to copy objects from disk into
 * an archive) and pass mode (to copy objects from disk to
 * an archive_write_disk "archive").
 */
unsafe extern "C" fn file_to_archive(
    mut cpio: *mut cpio,
    mut srcpath: *const libc::c_char,
) -> libc::c_int {
    let mut destpath: *const libc::c_char = 0 as *const libc::c_char;
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut spare: *mut archive_entry = 0 as *mut archive_entry;
    let mut len: size_t = 0;
    let mut r: libc::c_int = 0;
    /*
     * Create an archive_entry describing the source file.
     *
     */
    entry = archive_entry_new();
    if entry.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Couldn\'t allocate entry\x00" as *const u8 as *const libc::c_char,
        );
    }
    archive_entry_copy_sourcepath(entry, srcpath);
    r = archive_read_disk_entry_from_file(
        (*cpio).archive_read_disk,
        entry,
        -(1 as libc::c_int),
        NULL as *const stat,
    );
    if r < ARCHIVE_FAILED {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string((*cpio).archive_read_disk),
        );
    }
    if r < ARCHIVE_OK {
        lafe_warnc(
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string((*cpio).archive_read_disk),
        );
    }
    if r <= ARCHIVE_FAILED {
        archive_entry_free(entry);
        (*cpio).return_value = 1 as libc::c_int;
        return r;
    }
    if (*cpio).uid_override >= 0 as libc::c_int {
        archive_entry_set_uid(entry, (*cpio).uid_override as la_int64_t);
        archive_entry_set_uname(entry, (*cpio).uname_override);
    }
    if (*cpio).gid_override >= 0 as libc::c_int {
        archive_entry_set_gid(entry, (*cpio).gid_override as la_int64_t);
        archive_entry_set_gname(entry, (*cpio).gname_override);
    }
    /*
     * Generate a destination path for this entry.
     * "destination path" is the name to which it will be copied in
     * pass mode or the name that will go into the archive in
     * output mode.
     */
    destpath = srcpath;
    if !(*cpio).destdir.is_null() {
        len = (*cpio)
            .destdir_len
            .wrapping_add(strlen(srcpath))
            .wrapping_add(8 as libc::c_int as libc::c_ulong);
        if len >= (*cpio).pass_destpath_alloc {
            while len >= (*cpio).pass_destpath_alloc {
                (*cpio).pass_destpath_alloc = ((*cpio).pass_destpath_alloc as libc::c_ulong)
                    .wrapping_add(512 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
                (*cpio).pass_destpath_alloc = ((*cpio).pass_destpath_alloc as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t
            }
            free((*cpio).pass_destpath as *mut libc::c_void);
            (*cpio).pass_destpath = malloc((*cpio).pass_destpath_alloc) as *mut libc::c_char;
            if (*cpio).pass_destpath.is_null() {
                lafe_errc(
                    1 as libc::c_int,
                    ENOMEM,
                    b"Can\'t allocate path buffer\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        strcpy((*cpio).pass_destpath, (*cpio).destdir);
        strcat((*cpio).pass_destpath, remove_leading_slash(srcpath));
        destpath = (*cpio).pass_destpath
    }
    if (*cpio).option_rename != 0 {
        destpath = cpio_rename(destpath)
    }
    if destpath.is_null() {
        archive_entry_free(entry);
        return 0 as libc::c_int;
    }
    archive_entry_copy_pathname(entry, destpath);
    /*
     * If we're trying to preserve hardlinks, match them here.
     */
    spare = NULL as *mut archive_entry;
    if !(*cpio).linkresolver.is_null() && archive_entry_filetype(entry) != AE_IFDIR as mode_t {
        archive_entry_linkify((*cpio).linkresolver, &mut entry, &mut spare);
    }
    if !entry.is_null() {
        r = entry_to_archive(cpio, entry);
        archive_entry_free(entry);
        if !spare.is_null() {
            if r == 0 as libc::c_int {
                r = entry_to_archive(cpio, spare)
            }
            archive_entry_free(spare);
        }
    }
    return r;
}
unsafe extern "C" fn entry_to_archive(
    mut cpio: *mut cpio,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut current_block: u64;
    let mut destpath: *const libc::c_char = archive_entry_pathname(entry);
    let mut srcpath: *const libc::c_char = archive_entry_sourcepath(entry);
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut bytes_read: ssize_t = 0;
    let mut r: libc::c_int = 0;
    /* Print out the destination name to the user. */
    if (*cpio).verbose != 0 {
        fprintf(
            stderr,
            b"%s\x00" as *const u8 as *const libc::c_char,
            destpath,
        );
    }
    if (*cpio).dot != 0 {
        fprintf(stderr, b".\x00" as *const u8 as *const libc::c_char);
    }
    /*
     * Option_link only makes sense in pass mode and for
     * regular files.  Also note: if a link operation fails
     * because of cross-device restrictions, we'll fall back
     * to copy mode for that entry.
     *
     * TODO: Test other cpio implementations to see if they
     * hard-link anything other than regular files here.
     */
    if (*cpio).option_link != 0 && archive_entry_filetype(entry) == AE_IFREG as mode_t {
        let mut t: *mut archive_entry = 0 as *mut archive_entry;
        /* Save the original entry in case we need it later. */
        t = archive_entry_clone(entry);
        if t.is_null() {
            lafe_errc(
                1 as libc::c_int,
                ENOMEM,
                b"Can\'t create link\x00" as *const u8 as *const libc::c_char,
            );
        }
        /* Note: link(2) doesn't create parent directories,
         * so we use archive_write_header() instead as a
         * convenience. */
        archive_entry_set_hardlink(t, srcpath);
        /* This is a straight link that carries no data. */
        archive_entry_set_size(t, 0 as libc::c_int as la_int64_t);
        r = archive_write_header((*cpio).archive, t);
        archive_entry_free(t);
        if r != ARCHIVE_OK {
            lafe_warnc(
                archive_errno((*cpio).archive),
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string((*cpio).archive),
            );
        }
        if r == ARCHIVE_FATAL {
            exit(1 as libc::c_int);
        }
        if r != ARCHIVE_OK && archive_errno((*cpio).archive) == EXDEV {
            /* Cross-device link:  Just fall through and use
             * the original entry to copy the file over. */
            lafe_warnc(
                0 as libc::c_int,
                b"Copying file instead\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            return 0 as libc::c_int;
        }
    }
    /*
     * Make sure we can open the file (if necessary) before
     * trying to write the header.
     */
    if archive_entry_filetype(entry) == AE_IFREG as mode_t {
        if archive_entry_size(entry) > 0 as libc::c_int as libc::c_long {
            fd = open(srcpath, O_RDONLY | O_BINARY);
            if fd < 0 as libc::c_int {
                lafe_warnc(
                    errno,
                    b"%s: could not open file\x00" as *const u8 as *const libc::c_char,
                    srcpath,
                );
                current_block = 16904887316549377858;
            } else {
                current_block = 5689316957504528238;
            }
        } else {
            current_block = 5689316957504528238;
        }
    } else {
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
        current_block = 5689316957504528238;
    }
    match current_block {
        5689316957504528238 => {
            r = archive_write_header((*cpio).archive, entry);
            if r != ARCHIVE_OK {
                lafe_warnc(
                    archive_errno((*cpio).archive),
                    b"%s: %s\x00" as *const u8 as *const libc::c_char,
                    srcpath,
                    archive_error_string((*cpio).archive),
                );
            }
            if r == ARCHIVE_FATAL {
                exit(1 as libc::c_int);
            }
            if r >= ARCHIVE_WARN
                && archive_entry_size(entry) > 0 as libc::c_int as libc::c_long
                && fd >= 0 as libc::c_int
            {
                bytes_read = read(
                    fd,
                    (*cpio).buff as *mut libc::c_void,
                    (*cpio).buff_size as libc::c_uint as size_t,
                );
                while bytes_read > 0 as libc::c_int as libc::c_long {
                    let mut bytes_write: ssize_t = 0;
                    bytes_write = archive_write_data(
                        (*cpio).archive,
                        (*cpio).buff as *const libc::c_void,
                        bytes_read as size_t,
                    );
                    if bytes_write < 0 as libc::c_int as libc::c_long {
                        lafe_errc(
                            1 as libc::c_int,
                            archive_errno((*cpio).archive),
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            archive_error_string((*cpio).archive),
                        );
                    }
                    if bytes_write < bytes_read {
                        lafe_warnc(
                            0 as libc::c_int,
                            b"Truncated write; file may have grown while being archived.\x00"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    bytes_read = read(
                        fd,
                        (*cpio).buff as *mut libc::c_void,
                        (*cpio).buff_size as libc::c_uint as size_t,
                    )
                }
            }
            fd = restore_time(cpio, entry, srcpath, fd)
        }
        _ => {}
    }
    if (*cpio).verbose != 0 {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if fd >= 0 as libc::c_int {
        close(fd);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn restore_time(
    mut cpio: *mut cpio,
    mut entry: *mut archive_entry,
    mut name: *const libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut times: [timeval; 2] = [timeval {
        tv_sec: 0,
        tv_usec: 0,
    }; 2];
    if (*cpio).option_atime_restore == 0 {
        return fd;
    }
    times[1 as libc::c_int as usize].tv_sec = archive_entry_mtime(entry);
    times[1 as libc::c_int as usize].tv_usec =
        archive_entry_mtime_nsec(entry) / 1000 as libc::c_int as libc::c_long;
    times[0 as libc::c_int as usize].tv_sec = archive_entry_atime(entry);
    times[0 as libc::c_int as usize].tv_usec =
        archive_entry_atime_nsec(entry) / 1000 as libc::c_int as libc::c_long;
    if fd >= 0 as libc::c_int
        && futimes(fd, times.as_mut_ptr() as *const timeval) == 0 as libc::c_int
    {
        return fd;
    }
    /*
     * Some platform cannot restore access times if the file descriptor
     * is still opened.
     */
    if fd >= 0 as libc::c_int {
        close(fd);
        fd = -(1 as libc::c_int)
    }
    if lutimes(name, times.as_mut_ptr() as *const timeval) != 0 as libc::c_int {
        lafe_warnc(
            errno,
            b"Can\'t update time for %s\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return fd;
}
unsafe extern "C" fn mode_in(mut cpio: *mut cpio) -> ! {
    let mut a: *mut archive = 0 as *mut archive;
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut ext: *mut archive = 0 as *mut archive;
    let mut destpath: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    ext = archive_write_disk_new();
    if ext.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Couldn\'t allocate restore object\x00" as *const u8 as *const libc::c_char,
        );
    }
    r = archive_write_disk_set_options(ext, (*cpio).extract_flags);
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(ext),
        );
    }
    a = archive_read_new();
    if a.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Couldn\'t allocate archive object\x00" as *const u8 as *const libc::c_char,
        );
    }
    archive_read_support_filter_all(a);
    archive_read_support_format_all(a);
    if !(*cpio).passphrase.is_null() {
        r = archive_read_add_passphrase(a, (*cpio).passphrase)
    } else {
        r = archive_read_set_passphrase_callback(
            a,
            cpio as *mut libc::c_void,
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
    if archive_read_open_filename(a, (*cpio).filename, (*cpio).bytes_per_block as size_t) != 0 {
        lafe_errc(
            1 as libc::c_int,
            archive_errno(a),
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    loop {
        r = archive_read_next_header(a, &mut entry);
        if r == ARCHIVE_EOF {
            break;
        }
        if r != ARCHIVE_OK {
            lafe_errc(
                1 as libc::c_int,
                archive_errno(a),
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
        }
        if archive_match_path_excluded((*cpio).matching, entry) != 0 {
            continue;
        }
        if (*cpio).option_rename != 0 {
            destpath = cpio_rename(archive_entry_pathname(entry));
            archive_entry_set_pathname(entry, destpath);
        } else {
            destpath = archive_entry_pathname(entry)
        }
        if destpath.is_null() {
            continue;
        }
        if (*cpio).verbose != 0 {
            fprintf(
                stderr,
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                destpath,
            );
        }
        if (*cpio).dot != 0 {
            fprintf(stderr, b".\x00" as *const u8 as *const libc::c_char);
        }
        if (*cpio).uid_override >= 0 as libc::c_int {
            archive_entry_set_uid(entry, (*cpio).uid_override as la_int64_t);
        }
        if (*cpio).gid_override >= 0 as libc::c_int {
            archive_entry_set_gid(entry, (*cpio).gid_override as la_int64_t);
        }
        r = archive_write_header(ext, entry);
        if r != ARCHIVE_OK {
            fprintf(
                stderr,
                b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
                archive_entry_pathname(entry),
                archive_error_string(ext),
            );
        } else if archive_entry_size_is_set(entry) == 0
            || archive_entry_size(entry) > 0 as libc::c_int as libc::c_long
        {
            r = extract_data(a, ext);
            if r != ARCHIVE_OK {
                (*cpio).return_value = 1 as libc::c_int
            }
        }
    }
    r = archive_read_close(a);
    if (*cpio).dot != 0 {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    r = archive_write_close(ext);
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(ext),
        );
    }
    if (*cpio).quiet == 0 {
        let mut blocks: int64_t = (archive_filter_bytes(a, 0 as libc::c_int)
            + 511 as libc::c_int as libc::c_long)
            / 512 as libc::c_int as libc::c_long;
        fprintf(
            stderr,
            b"%lu %s\n\x00" as *const u8 as *const libc::c_char,
            blocks as libc::c_ulong,
            if blocks == 1 as libc::c_int as libc::c_long {
                b"block\x00" as *const u8 as *const libc::c_char
            } else {
                b"blocks\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    archive_read_free(a);
    archive_write_free(ext);
    exit((*cpio).return_value);
}
/*
 * Exits if there's a fatal error.  Returns ARCHIVE_OK
 * if everything is kosher.
 */
unsafe extern "C" fn extract_data(mut ar: *mut archive, mut aw: *mut archive) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut size: size_t = 0;
    let mut block: *const libc::c_void = 0 as *const libc::c_void;
    let mut offset: int64_t = 0;
    loop {
        r = archive_read_data_block(ar, &mut block, &mut size, &mut offset);
        if r == ARCHIVE_EOF {
            return 0 as libc::c_int;
        }
        if r != ARCHIVE_OK {
            lafe_warnc(
                archive_errno(ar),
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(ar),
            );
            exit(1 as libc::c_int);
        }
        r = archive_write_data_block(aw, block, size, offset) as libc::c_int;
        if r != ARCHIVE_OK {
            lafe_warnc(
                archive_errno(aw),
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(aw),
            );
            return r;
        }
    }
}
unsafe extern "C" fn mode_list(mut cpio: *mut cpio) -> ! {
    let mut a: *mut archive = 0 as *mut archive;
    let mut entry: *mut archive_entry = 0 as *mut archive_entry;
    let mut r: libc::c_int = 0;
    a = archive_read_new();
    if a.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Couldn\'t allocate archive object\x00" as *const u8 as *const libc::c_char,
        );
    }
    archive_read_support_filter_all(a);
    archive_read_support_format_all(a);
    if !(*cpio).passphrase.is_null() {
        r = archive_read_add_passphrase(a, (*cpio).passphrase)
    } else {
        r = archive_read_set_passphrase_callback(
            a,
            cpio as *mut libc::c_void,
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
    if archive_read_open_filename(a, (*cpio).filename, (*cpio).bytes_per_block as size_t) != 0 {
        lafe_errc(
            1 as libc::c_int,
            archive_errno(a),
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    loop {
        r = archive_read_next_header(a, &mut entry);
        if r == ARCHIVE_EOF {
            break;
        }
        if r != ARCHIVE_OK {
            lafe_errc(
                1 as libc::c_int,
                archive_errno(a),
                b"%s\x00" as *const u8 as *const libc::c_char,
                archive_error_string(a),
            );
        }
        if archive_match_path_excluded((*cpio).matching, entry) != 0 {
            continue;
        }
        if (*cpio).verbose != 0 {
            list_item_verbose(cpio, entry);
        } else {
            fprintf(
                stdout,
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                archive_entry_pathname(entry),
            );
        }
    }
    r = archive_read_close(a);
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string(a),
        );
    }
    if (*cpio).quiet == 0 {
        let mut blocks: int64_t = (archive_filter_bytes(a, 0 as libc::c_int)
            + 511 as libc::c_int as libc::c_long)
            / 512 as libc::c_int as libc::c_long;
        fprintf(
            stderr,
            b"%lu %s\n\x00" as *const u8 as *const libc::c_char,
            blocks as libc::c_ulong,
            if blocks == 1 as libc::c_int as libc::c_long {
                b"block\x00" as *const u8 as *const libc::c_char
            } else {
                b"blocks\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    archive_read_free(a);
    exit(0 as libc::c_int);
}
/*
 * Display information about the current file.
 *
 * The format here roughly duplicates the output of 'ls -l'.
 * This is based on SUSv2, where 'tar tv' is documented as
 * listing additional information in an "unspecified format,"
 * and 'pax -l' is documented as using the same format as 'ls -l'.
 */
unsafe extern "C" fn list_item_verbose(mut cpio: *mut cpio, mut entry: *mut archive_entry) {
    let mut size: [libc::c_char; 32] = [0; 32];
    let mut date: [libc::c_char; 32] = [0; 32];
    let mut uids: [libc::c_char; 16] = [0; 16];
    let mut gids: [libc::c_char; 16] = [0; 16];
    let mut uname: *const libc::c_char = 0 as *const libc::c_char;
    let mut gname: *const libc::c_char = 0 as *const libc::c_char;
    let mut out: *mut FILE = stdout;
    let mut fmt: *const libc::c_char = 0 as *const libc::c_char;
    let mut mtime: time_t = 0;
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
    if now == 0 {
        time(&mut now);
    }
    if (*cpio).option_numeric_uid_gid != 0 {
        /* Format numeric uid/gid for display. */
        strcpy(uids.as_mut_ptr(), cpio_i64toa(archive_entry_uid(entry)));
        uname = uids.as_mut_ptr();
        strcpy(gids.as_mut_ptr(), cpio_i64toa(archive_entry_gid(entry)));
        gname = gids.as_mut_ptr()
    } else {
        /* Use uname if it's present, else lookup name from uid. */
        uname = archive_entry_uname(entry);
        if uname.is_null() {
            uname = lookup_uname(cpio, archive_entry_uid(entry) as uid_t)
        }
        /* Use gname if it's present, else lookup name from gid. */
        gname = archive_entry_gname(entry);
        if gname.is_null() {
            gname = lookup_gname(cpio, archive_entry_gid(entry) as uid_t)
        }
    }
    /* Print device number or file size. */
    if archive_entry_filetype(entry) == AE_IFCHR as mode_t
        || archive_entry_filetype(entry) == AE_IFBLK as mode_t
    {
        snprintf(
            size.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%lu,%lu\x00" as *const u8 as *const libc::c_char,
            archive_entry_rdevmajor(entry),
            archive_entry_rdevminor(entry),
        );
    } else {
        strcpy(size.as_mut_ptr(), cpio_i64toa(archive_entry_size(entry)));
    }
    /* Format the time using 'ls -l' conventions. */
    mtime = archive_entry_mtime(entry);
    if mtime - now > (365 as libc::c_int * 86400 as libc::c_int / 2 as libc::c_int) as libc::c_long
        || mtime - now
            < (-(365 as libc::c_int) * 86400 as libc::c_int / 2 as libc::c_int) as libc::c_long
    {
        fmt = if (*cpio).day_first != 0 {
            b"%e %b  %Y\x00" as *const u8 as *const libc::c_char
        } else {
            b"%b %e  %Y\x00" as *const u8 as *const libc::c_char
        }
    } else {
        fmt = if (*cpio).day_first != 0 {
            b"%e %b %H:%M\x00" as *const u8 as *const libc::c_char
        } else {
            b"%b %e %H:%M\x00" as *const u8 as *const libc::c_char
        }
    }
    ltime = localtime_r(&mut mtime, &mut tmbuf);
    strftime(
        date.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        fmt,
        ltime,
    );
    fprintf(
        out,
        b"%s%3d %-8s %-8s %8s %12s %s\x00" as *const u8 as *const libc::c_char,
        archive_entry_strmode(entry),
        archive_entry_nlink(entry),
        uname,
        gname,
        size.as_mut_ptr(),
        date.as_mut_ptr(),
        archive_entry_pathname(entry),
    );
    /* Extra information for links. */
    if !archive_entry_hardlink(entry).is_null() {
        /* Hard link */
        fprintf(
            out,
            b" link to %s\x00" as *const u8 as *const libc::c_char,
            archive_entry_hardlink(entry),
        );
    } else if !archive_entry_symlink(entry).is_null() {
        /* Symbolic link */
        fprintf(
            out,
            b" -> %s\x00" as *const u8 as *const libc::c_char,
            archive_entry_symlink(entry),
        );
    }
    fprintf(out, b"\n\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn mode_pass(mut cpio: *mut cpio, mut destdir: *const libc::c_char) {
    let mut lr: *mut lafe_line_reader = 0 as *mut lafe_line_reader;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    /* Ensure target dir has a trailing '/' to simplify path surgery. */
    (*cpio).destdir_len = strlen(destdir);
    (*cpio).destdir = malloc(
        (*cpio)
            .destdir_len
            .wrapping_add(8 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*cpio).destdir as *mut libc::c_void,
        destdir as *const libc::c_void,
        (*cpio).destdir_len,
    );
    if (*cpio).destdir_len == 0 as libc::c_int as libc::c_ulong
        || *destdir.offset(
            (*cpio)
                .destdir_len
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) as libc::c_int
            != '/' as i32
    {
        let fresh0 = (*cpio).destdir_len;
        (*cpio).destdir_len = (*cpio).destdir_len.wrapping_add(1);
        *(*cpio).destdir.offset(fresh0 as isize) = '/' as i32 as libc::c_char
    }
    *(*cpio).destdir.offset((*cpio).destdir_len as isize) = '\u{0}' as i32 as libc::c_char;
    (*cpio).archive = archive_write_disk_new();
    if (*cpio).archive.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Failed to allocate archive object\x00" as *const u8 as *const libc::c_char,
        );
    }
    r = archive_write_disk_set_options((*cpio).archive, (*cpio).extract_flags);
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string((*cpio).archive),
        );
    }
    (*cpio).linkresolver = archive_entry_linkresolver_new();
    archive_write_disk_set_standard_lookup((*cpio).archive);
    (*cpio).archive_read_disk = archive_read_disk_new();
    if (*cpio).archive_read_disk.is_null() {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"Failed to allocate archive object\x00" as *const u8 as *const libc::c_char,
        );
    }
    if (*cpio).option_follow_links != 0 {
        archive_read_disk_set_symlink_logical((*cpio).archive_read_disk);
    } else {
        archive_read_disk_set_symlink_physical((*cpio).archive_read_disk);
    }
    archive_read_disk_set_standard_lookup((*cpio).archive_read_disk);
    lr = lafe_line_reader(
        b"-\x00" as *const u8 as *const libc::c_char,
        (*cpio).option_null as libc::c_int,
    );
    loop {
        p = lafe_line_reader_next(lr);
        if p.is_null() {
            break;
        }
        file_to_archive(cpio, p);
    }
    lafe_line_reader_free(lr);
    archive_entry_linkresolver_free((*cpio).linkresolver);
    r = archive_write_close((*cpio).archive);
    if (*cpio).dot != 0 {
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    if r != ARCHIVE_OK {
        lafe_errc(
            1 as libc::c_int,
            0 as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            archive_error_string((*cpio).archive),
        );
    }
    if (*cpio).quiet == 0 {
        let mut blocks: int64_t = (archive_filter_bytes((*cpio).archive, 0 as libc::c_int)
            + 511 as libc::c_int as libc::c_long)
            / 512 as libc::c_int as libc::c_long;
        fprintf(
            stderr,
            b"%lu %s\n\x00" as *const u8 as *const libc::c_char,
            blocks as libc::c_ulong,
            if blocks == 1 as libc::c_int as libc::c_long {
                b"block\x00" as *const u8 as *const libc::c_char
            } else {
                b"blocks\x00" as *const u8 as *const libc::c_char
            },
        );
    }
    archive_write_free((*cpio).archive);
    free((*cpio).pass_destpath as *mut libc::c_void);
}
/*
 * Prompt for a new name for this entry.  Returns a pointer to the
 * new name or NULL if the entry should not be copied.  This
 * implements the semantics defined in POSIX.1-1996, which specifies
 * that an input of '.' means the name should be unchanged.  GNU cpio
 * treats '.' as a literal new name.
 */
unsafe extern "C" fn cpio_rename(mut name: *const libc::c_char) -> *const libc::c_char {
    static mut buff: [libc::c_char; 1024] = [0; 1024];
    let mut t: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    t = fopen(
        b"/dev/tty\x00" as *const u8 as *const libc::c_char,
        b"r+\x00" as *const u8 as *const libc::c_char,
    );
    if t.is_null() {
        return name;
    }
    fprintf(
        t,
        b"%s (Enter/./(new name))? \x00" as *const u8 as *const libc::c_char,
        name,
    );
    fflush(t);
    p = fgets(
        buff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        t,
    );
    fclose(t);
    if p.is_null() {
        /* End-of-file is a blank line. */
        return 0 as *const libc::c_char;
    }
    while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
        p = p.offset(1)
    }
    if *p as libc::c_int == '\n' as i32 || *p as libc::c_int == '\u{0}' as i32 {
        /* Empty line. */
        return 0 as *const libc::c_char;
    }
    if *p as libc::c_int == '.' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32
    {
        /* Single period preserves original name. */
        return name;
    }
    ret = p;
    /* Trim the final newline. */
    while *p as libc::c_int != '\u{0}' as i32 && *p as libc::c_int != '\n' as i32 {
        p = p.offset(1)
    }
    /* Overwrite the final \n with a null character. */
    *p = '\u{0}' as i32 as libc::c_char;
    return ret;
}
unsafe extern "C" fn free_cache(mut cache: *mut name_cache) {
    let mut i: size_t = 0;
    if !cache.is_null() {
        i = 0 as libc::c_int as size_t;
        while i < (*cache).size {
            free((*cache).cache[i as usize].name as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        free(cache as *mut libc::c_void);
    };
}
/*
 * Lookup uname/gname from uid/gid, return NULL if no match.
 */
unsafe extern "C" fn lookup_name(
    mut cpio: *mut cpio,
    mut name_cache_variable: *mut *mut name_cache,
    mut lookup_fn: Option<
        unsafe extern "C" fn(_: *mut cpio, _: *mut *const libc::c_char, _: id_t) -> libc::c_int,
    >,
    mut id: id_t,
) -> *const libc::c_char {
    let mut asnum: [libc::c_char; 16] = [0; 16];
    let mut cache: *mut name_cache = 0 as *mut name_cache;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut slot: libc::c_int = 0;
    if (*name_cache_variable).is_null() {
        *name_cache_variable = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<name_cache>() as libc::c_ulong,
        ) as *mut name_cache;
        if (*name_cache_variable).is_null() {
            lafe_errc(
                1 as libc::c_int,
                ENOMEM,
                b"No more memory\x00" as *const u8 as *const libc::c_char,
            );
        }
        (**name_cache_variable).size = name_cache_size as size_t
    }
    cache = *name_cache_variable;
    (*cache).probes += 1;
    slot = (id as libc::c_ulong).wrapping_rem((*cache).size) as libc::c_int;
    if !(*cache).cache[slot as usize].name.is_null() {
        if (*cache).cache[slot as usize].id == id {
            (*cache).hits += 1;
            return (*cache).cache[slot as usize].name;
        }
        free((*cache).cache[slot as usize].name as *mut libc::c_void);
        (*cache).cache[slot as usize].name = NULL as *mut libc::c_char
    }
    if lookup_fn.expect("non-null function pointer")(cpio, &mut name, id) != 0 {
        /* If lookup failed, format it as a number. */
        snprintf(
            asnum.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            b"%u\x00" as *const u8 as *const libc::c_char,
            id,
        );
        name = asnum.as_mut_ptr()
    }
    (*cache).cache[slot as usize].name = strdup(name);
    if !(*cache).cache[slot as usize].name.is_null() {
        (*cache).cache[slot as usize].id = id;
        return (*cache).cache[slot as usize].name;
    }
    /*
     * Conveniently, NULL marks an empty slot, so
     * if the strdup() fails, we've just failed to
     * cache it.  No recovery necessary.
     */
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn lookup_uname(mut cpio: *mut cpio, mut uid: uid_t) -> *const libc::c_char {
    return lookup_name(
        cpio,
        &mut (*cpio).uname_cache,
        Some(
            lookup_uname_helper
                as unsafe extern "C" fn(
                    _: *mut cpio,
                    _: *mut *const libc::c_char,
                    _: id_t,
                ) -> libc::c_int,
        ),
        uid,
    );
}
unsafe extern "C" fn lookup_uname_helper(
    mut cpio: *mut cpio,
    mut name: *mut *const libc::c_char,
    mut id: id_t,
) -> libc::c_int {
    let mut pwent: *mut passwd = 0 as *mut passwd;
    /* UNUSED */
    errno = 0 as libc::c_int;
    pwent = getpwuid(id);
    if pwent.is_null() {
        if errno != 0 && errno != ENOENT {
            lafe_warnc(
                errno,
                b"getpwuid(%s) failed\x00" as *const u8 as *const libc::c_char,
                cpio_i64toa(id as int64_t),
            );
        }
        return 1 as libc::c_int;
    }
    *name = (*pwent).pw_name;
    return 0 as libc::c_int;
}
unsafe extern "C" fn lookup_gname(mut cpio: *mut cpio, mut gid: gid_t) -> *const libc::c_char {
    return lookup_name(
        cpio,
        &mut (*cpio).gname_cache,
        Some(
            lookup_gname_helper
                as unsafe extern "C" fn(
                    _: *mut cpio,
                    _: *mut *const libc::c_char,
                    _: id_t,
                ) -> libc::c_int,
        ),
        gid,
    );
}
unsafe extern "C" fn lookup_gname_helper(
    mut cpio: *mut cpio,
    mut name: *mut *const libc::c_char,
    mut id: id_t,
) -> libc::c_int {
    let mut grent: *mut group = 0 as *mut group;
    /* UNUSED */
    errno = 0 as libc::c_int;
    grent = getgrgid(id);
    if grent.is_null() {
        if errno != 0 && errno != ENOENT {
            lafe_warnc(
                errno,
                b"getgrgid(%s) failed\x00" as *const u8 as *const libc::c_char,
                cpio_i64toa(id as int64_t),
            );
        }
        return 1 as libc::c_int;
    }
    *name = (*grent).gr_name;
    return 0 as libc::c_int;
}
/*
 * It would be nice to just use printf() for formatting large numbers,
 * but the compatibility problems are a big headache.  Hence the
 * following simple utility function.
 */
#[no_mangle]
pub unsafe extern "C" fn cpio_i64toa(mut n0: int64_t) -> *const libc::c_char {
    /* 2^64 =~ 1.8 * 10^19, so 20 decimal digits suffice.
     * We also need 1 byte for '-' and 1 for '\0'.
     */
    static mut buff: [libc::c_char; 22] = [0; 22];
    let mut n: int64_t = if n0 < 0 as libc::c_int as libc::c_long {
        -n0
    } else {
        n0
    };
    let mut p: *mut libc::c_char = buff
        .as_mut_ptr()
        .offset(::std::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong as isize);
    p = p.offset(-1);
    *p = '\u{0}' as i32 as libc::c_char;
    loop {
        p = p.offset(-1);
        *p = ('0' as i32 + (n % 10 as libc::c_int as libc::c_long) as libc::c_int) as libc::c_char;
        n /= 10 as libc::c_int as libc::c_long;
        if !(n > 0 as libc::c_int as libc::c_long) {
            break;
        }
    }
    if n0 < 0 as libc::c_int as libc::c_long {
        p = p.offset(-1);
        *p = '-' as i32 as libc::c_char
    }
    return p;
}
pub const PPBUFF_SIZE: libc::c_int = 1024 as libc::c_int;
unsafe extern "C" fn passphrase_callback(
    mut a: *mut archive,
    mut _client_data: *mut libc::c_void,
) -> *const libc::c_char {
    let mut cpio: *mut cpio = _client_data as *mut cpio;
    /* UNUSED */
    if (*cpio).ppbuff.is_null() {
        (*cpio).ppbuff = malloc(PPBUFF_SIZE as libc::c_ulong) as *mut libc::c_char;
        if (*cpio).ppbuff.is_null() {
            lafe_errc(
                1 as libc::c_int,
                errno,
                b"Out of memory\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    return lafe_readpassphrase(
        b"Enter passphrase:\x00" as *const u8 as *const libc::c_char,
        (*cpio).ppbuff,
        PPBUFF_SIZE as size_t,
    );
}
unsafe extern "C" fn passphrase_free(mut ppbuff: *mut libc::c_char) {
    if !ppbuff.is_null() {
        memset(
            ppbuff as *mut libc::c_void,
            0 as libc::c_int,
            PPBUFF_SIZE as libc::c_ulong,
        );
        free(ppbuff as *mut libc::c_void);
    };
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
