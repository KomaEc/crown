use ::libc;
extern "C" {
    pub type archive_entry;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* The 'clone' function does a deep copy; all of the strings are copied too. */
    #[no_mangle]
    fn archive_entry_clone(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    #[no_mangle]
    fn archive_entry_dev(_: *mut archive_entry) -> dev_t;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_ino64(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_nlink(_: *mut archive_entry) -> libc::c_uint;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_copy_hardlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_unset_size(_: *mut archive_entry);
}
pub type __int64_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type size_t = libc::c_ulong;
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
/* # links not yet seen */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_entry_linkresolver {
    pub buckets: *mut *mut links_entry,
    pub spare: *mut links_entry,
    pub number_entries: libc::c_ulong,
    pub number_buckets: size_t,
    pub strategy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct links_entry {
    pub next: *mut links_entry,
    pub previous: *mut links_entry,
    pub canonical: *mut archive_entry,
    pub entry: *mut archive_entry,
    pub hash: size_t,
    pub links: libc::c_uint,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
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
pub const ARCHIVE_FORMAT_BASE_MASK: libc::c_int = 0xff0000 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO: libc::c_int = 0x10000 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_SVR4_NOCRC: libc::c_int = ARCHIVE_FORMAT_CPIO | 4 as libc::c_int;
pub const ARCHIVE_FORMAT_CPIO_SVR4_CRC: libc::c_int = ARCHIVE_FORMAT_CPIO | 5 as libc::c_int;
pub const ARCHIVE_FORMAT_SHAR: libc::c_int = 0x20000 as libc::c_int;
pub const ARCHIVE_FORMAT_TAR: libc::c_int = 0x30000 as libc::c_int;
pub const ARCHIVE_FORMAT_ISO9660: libc::c_int = 0x40000 as libc::c_int;
pub const ARCHIVE_FORMAT_ZIP: libc::c_int = 0x50000 as libc::c_int;
pub const ARCHIVE_FORMAT_AR: libc::c_int = 0x70000 as libc::c_int;
pub const ARCHIVE_FORMAT_MTREE: libc::c_int = 0x80000 as libc::c_int;
pub const ARCHIVE_FORMAT_XAR: libc::c_int = 0xa0000 as libc::c_int;
pub const ARCHIVE_FORMAT_7ZIP: libc::c_int = 0xe0000 as libc::c_int;
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
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
/*
 * This is mostly a pretty straightforward hash table implementation.
 * The only interesting bit is the different strategies used to
 * match up links.  These strategies match those used by various
 * archiving formats:
 *   tar - content stored with first link, remainder refer back to it.
 *       This requires us to match each subsequent link up with the
 *       first appearance.
 *   cpio - Old cpio just stored body with each link, match-ups were
 *       implicit.  This is trivial.
 *   new cpio - New cpio only stores body with last link, match-ups
 *       are implicit.  This is actually quite tricky; see the notes
 *       below.
 */
/* Users pass us a format code, we translate that into a strategy here. */
pub const ARCHIVE_ENTRY_LINKIFY_LIKE_TAR: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_ENTRY_LINKIFY_LIKE_MTREE: libc::c_int = 1 as libc::c_int;
pub const ARCHIVE_ENTRY_LINKIFY_LIKE_OLD_CPIO: libc::c_int = 2 as libc::c_int;
pub const ARCHIVE_ENTRY_LINKIFY_LIKE_NEW_CPIO: libc::c_int = 3 as libc::c_int;
/* Initial size of link cache. */
pub const links_cache_initial_size: libc::c_int = 1024 as libc::c_int;
pub const NEXT_ENTRY_DEFERRED: libc::c_int = 1 as libc::c_int;
pub const NEXT_ENTRY_PARTIAL: libc::c_int = 2 as libc::c_int;
pub const NEXT_ENTRY_ALL: libc::c_int = NEXT_ENTRY_DEFERRED | NEXT_ENTRY_PARTIAL;
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
#[no_mangle]
pub unsafe extern "C" fn archive_entry_linkresolver_new() -> *mut archive_entry_linkresolver {
    let mut res: *mut archive_entry_linkresolver = 0 as *mut archive_entry_linkresolver;
    /* Check for positive power-of-two */
    if links_cache_initial_size == 0 as libc::c_int
        || links_cache_initial_size & links_cache_initial_size - 1 as libc::c_int
            != 0 as libc::c_int
    {
        return 0 as *mut archive_entry_linkresolver;
    } /* Default: Don't return a second entry. */
    res = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_entry_linkresolver>() as libc::c_ulong,
    ) as *mut archive_entry_linkresolver;
    if res.is_null() {
        return 0 as *mut archive_entry_linkresolver;
    }
    (*res).number_buckets = links_cache_initial_size as size_t;
    (*res).buckets = calloc(
        (*res).number_buckets,
        ::std::mem::size_of::<*mut links_entry>() as libc::c_ulong,
    ) as *mut *mut links_entry;
    if (*res).buckets.is_null() {
        free(res as *mut libc::c_void);
        return 0 as *mut archive_entry_linkresolver;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_linkresolver_set_strategy(
    mut res: *mut archive_entry_linkresolver,
    mut fmt: libc::c_int,
) {
    let mut fmtbase: libc::c_int = fmt & ARCHIVE_FORMAT_BASE_MASK;
    match fmtbase {
        ARCHIVE_FORMAT_7ZIP | ARCHIVE_FORMAT_AR | ARCHIVE_FORMAT_ZIP => {
            (*res).strategy = ARCHIVE_ENTRY_LINKIFY_LIKE_OLD_CPIO
        }
        ARCHIVE_FORMAT_CPIO => match fmt {
            65540 | 65541 => (*res).strategy = ARCHIVE_ENTRY_LINKIFY_LIKE_NEW_CPIO,
            _ => (*res).strategy = ARCHIVE_ENTRY_LINKIFY_LIKE_OLD_CPIO,
        },
        ARCHIVE_FORMAT_MTREE => (*res).strategy = ARCHIVE_ENTRY_LINKIFY_LIKE_MTREE,
        ARCHIVE_FORMAT_ISO9660 | ARCHIVE_FORMAT_SHAR | ARCHIVE_FORMAT_TAR | ARCHIVE_FORMAT_XAR => {
            (*res).strategy = ARCHIVE_ENTRY_LINKIFY_LIKE_TAR
        }
        _ => (*res).strategy = ARCHIVE_ENTRY_LINKIFY_LIKE_OLD_CPIO,
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_linkresolver_free(mut res: *mut archive_entry_linkresolver) {
    let mut le: *mut links_entry = 0 as *mut links_entry;
    if res.is_null() {
        return;
    }
    loop {
        le = next_entry(res, NEXT_ENTRY_ALL);
        if le.is_null() {
            break;
        }
        archive_entry_free((*le).entry);
    }
    free((*res).buckets as *mut libc::c_void);
    free(res as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_linkify(
    mut res: *mut archive_entry_linkresolver,
    mut e: *mut *mut archive_entry,
    mut f: *mut *mut archive_entry,
) {
    let mut le: *mut links_entry = 0 as *mut links_entry;
    let mut t: *mut archive_entry = 0 as *mut archive_entry;
    *f = NULL as *mut archive_entry;
    if (*e).is_null() {
        le = next_entry(res, NEXT_ENTRY_DEFERRED);
        if !le.is_null() {
            *e = (*le).entry;
            (*le).entry = NULL as *mut archive_entry
        }
        return;
    }
    /* If it has only one link, then we're done. */
    if archive_entry_nlink(*e) == 1 as libc::c_int as libc::c_uint {
        return;
    }
    /* Directories, devices never have hardlinks. */
    if archive_entry_filetype(*e) == AE_IFDIR as mode_t
        || archive_entry_filetype(*e) == AE_IFBLK as mode_t
        || archive_entry_filetype(*e) == AE_IFCHR as mode_t
    {
        return;
    }
    match (*res).strategy {
        ARCHIVE_ENTRY_LINKIFY_LIKE_TAR => {
            le = find_entry(res, *e);
            if !le.is_null() {
                archive_entry_unset_size(*e);
                archive_entry_copy_hardlink(*e, archive_entry_pathname((*le).canonical));
            } else {
                insert_entry(res, *e);
            }
            return;
        }
        ARCHIVE_ENTRY_LINKIFY_LIKE_MTREE => {
            le = find_entry(res, *e);
            if !le.is_null() {
                archive_entry_copy_hardlink(*e, archive_entry_pathname((*le).canonical));
            } else {
                insert_entry(res, *e);
            }
            return;
        }
        ARCHIVE_ENTRY_LINKIFY_LIKE_OLD_CPIO => {
            /* This one is trivial. */
            return;
        }
        ARCHIVE_ENTRY_LINKIFY_LIKE_NEW_CPIO => {
            le = find_entry(res, *e);
            if !le.is_null() {
                /*
                 * Put the new entry in le, return the
                 * old entry from le.
                 */
                t = *e;
                *e = (*le).entry;
                (*le).entry = t;
                /* Make the old entry into a hardlink. */
                archive_entry_unset_size(*e);
                archive_entry_copy_hardlink(*e, archive_entry_pathname((*le).canonical));
                /* If we ran out of links, return the
                 * final entry as well. */
                if (*le).links == 0 as libc::c_int as libc::c_uint {
                    *f = (*le).entry;
                    (*le).entry = NULL as *mut archive_entry
                }
            } else {
                /*
                 * If we haven't seen it, tuck it away
                 * for future use.
                 */
                le = insert_entry(res, *e);
                if le.is_null() {
                    /* XXX We should return an error code XXX */
                    return;
                }
                (*le).entry = *e;
                *e = NULL as *mut archive_entry
            }
            return;
        }
        _ => {}
    };
}
unsafe extern "C" fn find_entry(
    mut res: *mut archive_entry_linkresolver,
    mut entry: *mut archive_entry,
) -> *mut links_entry {
    let mut le: *mut links_entry = 0 as *mut links_entry;
    let mut hash: size_t = 0;
    let mut bucket: size_t = 0;
    let mut dev: dev_t = 0;
    let mut ino: int64_t = 0;
    /* Free a held entry. */
    if !(*res).spare.is_null() {
        archive_entry_free((*(*res).spare).canonical);
        archive_entry_free((*(*res).spare).entry);
        free((*res).spare as *mut libc::c_void);
        (*res).spare = NULL as *mut links_entry
    }
    dev = archive_entry_dev(entry);
    ino = archive_entry_ino64(entry);
    hash = dev ^ ino as libc::c_ulong;
    /* Try to locate this entry in the links cache. */
    bucket = hash
        & (*res)
            .number_buckets
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    le = *(*res).buckets.offset(bucket as isize);
    while !le.is_null() {
        if (*le).hash == hash
            && dev == archive_entry_dev((*le).canonical)
            && ino == archive_entry_ino64((*le).canonical)
        {
            /*
             * Decrement link count each time and release
             * the entry if it hits zero.  This saves
             * memory and is necessary for detecting
             * missed links.
             */
            (*le).links = (*le).links.wrapping_sub(1);
            if (*le).links > 0 as libc::c_int as libc::c_uint {
                return le;
            }
            /* Remove it from this hash bucket. */
            if !(*le).previous.is_null() {
                (*(*le).previous).next = (*le).next
            }
            if !(*le).next.is_null() {
                (*(*le).next).previous = (*le).previous
            }
            if *(*res).buckets.offset(bucket as isize) == le {
                let ref mut fresh0 = *(*res).buckets.offset(bucket as isize);
                *fresh0 = (*le).next
            }
            (*res).number_entries = (*res).number_entries.wrapping_sub(1);
            /* Defer freeing this entry. */
            (*res).spare = le;
            return le;
        }
        le = (*le).next
    }
    return 0 as *mut links_entry;
}
unsafe extern "C" fn next_entry(
    mut res: *mut archive_entry_linkresolver,
    mut mode: libc::c_int,
) -> *mut links_entry {
    let mut le: *mut links_entry = 0 as *mut links_entry;
    let mut bucket: size_t = 0;
    /* Free a held entry. */
    if !(*res).spare.is_null() {
        archive_entry_free((*(*res).spare).canonical);
        archive_entry_free((*(*res).spare).entry);
        free((*res).spare as *mut libc::c_void);
        (*res).spare = NULL as *mut links_entry
    }
    /* Look for next non-empty bucket in the links cache. */
    bucket = 0 as libc::c_int as size_t;
    while bucket < (*res).number_buckets {
        le = *(*res).buckets.offset(bucket as isize);
        while !le.is_null() {
            if !(!(*le).entry.is_null() && mode & NEXT_ENTRY_DEFERRED == 0 as libc::c_int) {
                if !((*le).entry.is_null() && mode & NEXT_ENTRY_PARTIAL == 0 as libc::c_int) {
                    /* Remove it from this hash bucket. */
                    if !(*le).next.is_null() {
                        (*(*le).next).previous = (*le).previous
                    }
                    if !(*le).previous.is_null() {
                        (*(*le).previous).next = (*le).next
                    } else {
                        let ref mut fresh1 = *(*res).buckets.offset(bucket as isize);
                        *fresh1 = (*le).next
                    }
                    (*res).number_entries = (*res).number_entries.wrapping_sub(1);
                    /* Defer freeing this entry. */
                    (*res).spare = le;
                    return le;
                }
            }
            le = (*le).next
        }
        bucket = bucket.wrapping_add(1)
    }
    return 0 as *mut links_entry;
}
unsafe extern "C" fn insert_entry(
    mut res: *mut archive_entry_linkresolver,
    mut entry: *mut archive_entry,
) -> *mut links_entry {
    let mut le: *mut links_entry = 0 as *mut links_entry;
    let mut hash: size_t = 0;
    let mut bucket: size_t = 0;
    /* Add this entry to the links cache. */
    le = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<links_entry>() as libc::c_ulong,
    ) as *mut links_entry;
    if le.is_null() {
        return 0 as *mut links_entry;
    }
    (*le).canonical = archive_entry_clone(entry);
    /* If the links cache is getting too full, enlarge the hash table. */
    if (*res).number_entries
        > (*res)
            .number_buckets
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
    {
        grow_hash(res);
    }
    hash = archive_entry_dev(entry) ^ archive_entry_ino64(entry) as libc::c_ulong;
    bucket = hash
        & (*res)
            .number_buckets
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    /* If we could allocate the entry, record it. */
    if !(*(*res).buckets.offset(bucket as isize)).is_null() {
        let ref mut fresh2 = (**(*res).buckets.offset(bucket as isize)).previous;
        *fresh2 = le
    }
    (*res).number_entries = (*res).number_entries.wrapping_add(1);
    (*le).next = *(*res).buckets.offset(bucket as isize);
    (*le).previous = NULL as *mut links_entry;
    let ref mut fresh3 = *(*res).buckets.offset(bucket as isize);
    *fresh3 = le;
    (*le).hash = hash;
    (*le).links = archive_entry_nlink(entry).wrapping_sub(1 as libc::c_int as libc::c_uint);
    return le;
}
unsafe extern "C" fn grow_hash(mut res: *mut archive_entry_linkresolver) {
    let mut le: *mut links_entry = 0 as *mut links_entry;
    let mut new_buckets: *mut *mut links_entry = 0 as *mut *mut links_entry;
    let mut new_size: size_t = 0;
    let mut i: size_t = 0;
    let mut bucket: size_t = 0;
    /* Try to enlarge the bucket list. */
    new_size = (*res)
        .number_buckets
        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    if new_size < (*res).number_buckets {
        return;
    }
    new_buckets = calloc(
        new_size,
        ::std::mem::size_of::<*mut links_entry>() as libc::c_ulong,
    ) as *mut *mut links_entry;
    if new_buckets.is_null() {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*res).number_buckets {
        while !(*(*res).buckets.offset(i as isize)).is_null() {
            /* Remove entry from old bucket. */
            le = *(*res).buckets.offset(i as isize);
            let ref mut fresh4 = *(*res).buckets.offset(i as isize);
            *fresh4 = (*le).next;
            /* Add entry to new bucket. */
            bucket = (*le).hash & new_size.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            if !(*new_buckets.offset(bucket as isize)).is_null() {
                let ref mut fresh5 = (**new_buckets.offset(bucket as isize)).previous;
                *fresh5 = le
            }
            (*le).next = *new_buckets.offset(bucket as isize);
            (*le).previous = NULL as *mut links_entry;
            let ref mut fresh6 = *new_buckets.offset(bucket as isize);
            *fresh6 = le
        }
        i = i.wrapping_add(1)
    }
    free((*res).buckets as *mut libc::c_void);
    (*res).buckets = new_buckets;
    (*res).number_buckets = new_size;
}
#[no_mangle]
pub unsafe extern "C" fn archive_entry_partial_links(
    mut res: *mut archive_entry_linkresolver,
    mut links: *mut libc::c_uint,
) -> *mut archive_entry {
    let mut e: *mut archive_entry = 0 as *mut archive_entry;
    let mut le: *mut links_entry = 0 as *mut links_entry;
    /* Free a held entry. */
    if !(*res).spare.is_null() {
        archive_entry_free((*(*res).spare).canonical);
        archive_entry_free((*(*res).spare).entry);
        free((*res).spare as *mut libc::c_void);
        (*res).spare = NULL as *mut links_entry
    }
    le = next_entry(res, NEXT_ENTRY_PARTIAL);
    if !le.is_null() {
        e = (*le).canonical;
        if !links.is_null() {
            *links = (*le).links
        }
        (*le).canonical = NULL as *mut archive_entry
    } else {
        e = NULL as *mut archive_entry;
        if !links.is_null() {
            *links = 0 as libc::c_int as libc::c_uint
        }
    }
    return e;
}
