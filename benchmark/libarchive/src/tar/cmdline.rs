use ::libc;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
}
pub type __uid_t = libc::c_uint;
pub type uid_t = __uid_t;
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
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
/* for util.c */
/* Options for flags bitfield */
/* -a */
/* -P */
/* --chroot */
/* --fast-read */
/* --ignore-zeros */
/* -w */
/* -o */
/* -n */
/* --null */
/* --numeric-owner */
/* -o */
/* -O */
/* --totals */
/* -U */
/* --check-links */
/* --no-xattrs */
/* --xattrs */
/* --no-acls */
/* --acls */
/* --no-fflags */
/* --fflags */
/* --no-mac-metadata */
/* --mac-metadata */
/* Fake short equivalents for long options that otherwise lack them. */
pub type C2RustUnnamed = libc::c_uint;
pub const OPTION_ZSTD: C2RustUnnamed = 57;
pub const OPTION_XATTRS: C2RustUnnamed = 56;
pub const OPTION_VERSION: C2RustUnnamed = 55;
pub const OPTION_UUENCODE: C2RustUnnamed = 54;
pub const OPTION_USE_COMPRESS_PROGRAM: C2RustUnnamed = 53;
pub const OPTION_UNAME: C2RustUnnamed = 52;
pub const OPTION_UID: C2RustUnnamed = 51;
pub const OPTION_TOTALS: C2RustUnnamed = 50;
pub const OPTION_STRIP_COMPONENTS: C2RustUnnamed = 49;
pub const OPTION_SAME_OWNER: C2RustUnnamed = 48;
pub const OPTION_SAFE_WRITES: C2RustUnnamed = 47;
pub const OPTION_POSIX: C2RustUnnamed = 46;
pub const OPTION_PASSPHRASE: C2RustUnnamed = 45;
pub const OPTION_OPTIONS: C2RustUnnamed = 44;
pub const OPTION_ONE_FILE_SYSTEM: C2RustUnnamed = 43;
pub const OPTION_OLDER_MTIME_THAN: C2RustUnnamed = 42;
pub const OPTION_OLDER_MTIME: C2RustUnnamed = 41;
pub const OPTION_OLDER_CTIME_THAN: C2RustUnnamed = 40;
pub const OPTION_OLDER_CTIME: C2RustUnnamed = 39;
pub const OPTION_NUMERIC_OWNER: C2RustUnnamed = 38;
pub const OPTION_NULL: C2RustUnnamed = 37;
pub const OPTION_NO_XATTRS: C2RustUnnamed = 36;
pub const OPTION_NO_SAME_PERMISSIONS: C2RustUnnamed = 35;
pub const OPTION_NO_SAME_OWNER: C2RustUnnamed = 34;
pub const OPTION_NO_SAFE_WRITES: C2RustUnnamed = 33;
pub const OPTION_NO_MAC_METADATA: C2RustUnnamed = 32;
pub const OPTION_NO_FFLAGS: C2RustUnnamed = 31;
pub const OPTION_NO_ACLS: C2RustUnnamed = 30;
pub const OPTION_NOPRESERVE_HFS_COMPRESSION: C2RustUnnamed = 29;
pub const OPTION_NODUMP: C2RustUnnamed = 28;
pub const OPTION_NEWER_MTIME_THAN: C2RustUnnamed = 27;
pub const OPTION_NEWER_MTIME: C2RustUnnamed = 26;
pub const OPTION_NEWER_CTIME_THAN: C2RustUnnamed = 25;
pub const OPTION_NEWER_CTIME: C2RustUnnamed = 24;
pub const OPTION_MAC_METADATA: C2RustUnnamed = 23;
pub const OPTION_LZOP: C2RustUnnamed = 22;
pub const OPTION_LZMA: C2RustUnnamed = 21;
pub const OPTION_LZIP: C2RustUnnamed = 20;
pub const OPTION_LZ4: C2RustUnnamed = 19;
pub const OPTION_LRZIP: C2RustUnnamed = 18;
pub const OPTION_KEEP_NEWER_FILES: C2RustUnnamed = 17;
pub const OPTION_INCLUDE: C2RustUnnamed = 16;
pub const OPTION_IGNORE_ZEROS: C2RustUnnamed = 15;
pub const OPTION_HFS_COMPRESSION: C2RustUnnamed = 14;
pub const OPTION_HELP: C2RustUnnamed = 13;
pub const OPTION_GRZIP: C2RustUnnamed = 12;
pub const OPTION_GNAME: C2RustUnnamed = 11;
pub const OPTION_GID: C2RustUnnamed = 10;
pub const OPTION_FORMAT: C2RustUnnamed = 9;
pub const OPTION_FFLAGS: C2RustUnnamed = 8;
pub const OPTION_EXCLUDE_VCS: C2RustUnnamed = 7;
pub const OPTION_EXCLUDE: C2RustUnnamed = 6;
pub const OPTION_CLEAR_NOCHANGE_FFLAGS: C2RustUnnamed = 5;
pub const OPTION_CHROOT: C2RustUnnamed = 4;
pub const OPTION_CHECK_LINKS: C2RustUnnamed = 3;
pub const OPTION_B64ENCODE: C2RustUnnamed = 2;
pub const OPTION_ACLS: C2RustUnnamed = 1;
/*
 * Long options for tar.  Please keep this list sorted.
 *
 * The symbolic names for options that lack a short equivalent are
 * defined in bsdtar.h.  Also note that so far I've found no need
 * to support optional arguments to long options.  That would be
 * a small change to the code below.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bsdtar_option {
    pub name: *const libc::c_char,
    pub required: libc::c_int,
    pub equivalent: libc::c_int,
}
pub const state_next_word: C2RustUnnamed_0 = 2;
pub const state_long: C2RustUnnamed_0 = 4;
pub const state_short: C2RustUnnamed_0 = 3;
pub const state_old_tar: C2RustUnnamed_0 = 1;
pub const state_start: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 * Short options for tar.  Please keep this sorted.
 */
static mut short_options: *const libc::c_char =
    b"aBb:C:cf:HhI:JjkLlmnOoPpqrSs:T:tUuvW:wX:xyZz\x00" as *const u8 as *const libc::c_char;
/* Equivalent short option. */
static mut tar_longopts: [bsdtar_option; 102] = [
    {
        let mut init = bsdtar_option {
            name: b"absolute-paths\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'P' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"append\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'r' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"acls\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_ACLS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"auto-compress\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'a' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"b64encode\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_B64ENCODE as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"block-size\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'b' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"blocking-factor\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'b' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"bunzip2\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'j' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"bzip\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'j' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"bzip2\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'j' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"cd\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'C' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"check-links\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_CHECK_LINKS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"chroot\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_CHROOT as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"clear-nochange-fflags\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_CLEAR_NOCHANGE_FFLAGS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"compress\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'Z' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"confirmation\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'w' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"create\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'c' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"dereference\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'L' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"directory\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'C' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"disable-copyfile\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_MAC_METADATA as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"exclude\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_EXCLUDE as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"exclude-from\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'X' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"exclude-vcs\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_EXCLUDE_VCS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"extract\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'x' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"fast-read\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'q' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"fflags\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_FFLAGS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"file\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'f' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"files-from\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'T' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"format\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_FORMAT as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"gid\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_GID as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"gname\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_GNAME as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"grzip\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_GRZIP as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"gunzip\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'z' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"gzip\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'z' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"help\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_HELP as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"hfsCompression\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_HFS_COMPRESSION as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"ignore-zeros\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_IGNORE_ZEROS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"include\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_INCLUDE as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"insecure\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'P' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"interactive\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'w' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"keep-newer-files\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_KEEP_NEWER_FILES as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"keep-old-files\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'k' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"list\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 't' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"lrzip\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LRZIP as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"lz4\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LZ4 as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"lzip\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LZIP as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"lzma\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LZMA as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"lzop\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LZOP as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"mac-metadata\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_MAC_METADATA as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"modification-time\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'm' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"newer\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_NEWER_CTIME as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"newer-ctime\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_NEWER_CTIME as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"newer-ctime-than\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_NEWER_CTIME_THAN as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"newer-mtime\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_NEWER_MTIME as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"newer-mtime-than\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_NEWER_MTIME_THAN as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"newer-than\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_NEWER_CTIME_THAN as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-acls\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_ACLS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-fflags\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_FFLAGS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-mac-metadata\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_MAC_METADATA as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-recursion\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'n' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-safe-writes\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_SAFE_WRITES as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-same-owner\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_SAME_OWNER as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-same-permissions\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_SAME_PERMISSIONS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-xattr\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_XATTRS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"no-xattrs\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_XATTRS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"nodump\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NODUMP as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"nopreserveHFSCompression\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NOPRESERVE_HFS_COMPRESSION as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"norecurse\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'n' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"null\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NULL as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"numeric-owner\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NUMERIC_OWNER as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"older\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_OLDER_CTIME as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"older-ctime\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_OLDER_CTIME as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"older-ctime-than\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_OLDER_CTIME_THAN as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"older-mtime\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_OLDER_MTIME as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"older-mtime-than\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_OLDER_MTIME_THAN as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"older-than\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_OLDER_CTIME_THAN as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"one-file-system\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_ONE_FILE_SYSTEM as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"options\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_OPTIONS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"passphrase\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_PASSPHRASE as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"posix\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_POSIX as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"preserve-permissions\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'p' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"read-full-blocks\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'B' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"safe-writes\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_SAFE_WRITES as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"same-owner\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_SAME_OWNER as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"same-permissions\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'p' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"strip-components\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_STRIP_COMPONENTS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"to-stdout\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'O' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"totals\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_TOTALS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"uid\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_UID as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"uname\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_UNAME as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"uncompress\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'Z' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"unlink\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'U' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"unlink-first\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'U' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"update\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'u' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"use-compress-program\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_USE_COMPRESS_PROGRAM as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"uuencode\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_UUENCODE as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"verbose\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'v' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"version\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_VERSION as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"xattrs\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_XATTRS as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"xz\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'J' as i32,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: b"zstd\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_ZSTD as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdtar_option {
            name: NULL as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 0 as libc::c_int,
        };
        init
    },
];
/*
 * This getopt implementation has two key features that common
 * getopt_long() implementations lack.  Apart from those, it's a
 * straightforward option parser, considerably simplified by not
 * needing to support the wealth of exotic getopt_long() features.  It
 * has, of course, been shamelessly tailored for bsdtar.  (If you're
 * looking for a generic getopt_long() implementation for your
 * project, I recommend Gregory Pietsch's public domain getopt_long()
 * implementation.)  The two additional features are:
 *
 * Old-style tar arguments: The original tar implementation treated
 * the first argument word as a list of single-character option
 * letters.  All arguments follow as separate words.  For example,
 *    tar xbf 32 /dev/tape
 * Here, the "xbf" is three option letters, "32" is the argument for
 * "b" and "/dev/tape" is the argument for "f".  We support this usage
 * if the first command-line argument does not begin with '-'.  We
 * also allow regular short and long options to follow, e.g.,
 *    tar xbf 32 /dev/tape -P --format=pax
 *
 * -W long options: There's an obscure GNU convention (only rarely
 * supported even there) that allows "-W option=argument" as an
 * alternative way to support long options.  This was supported in
 * early bsdtar as a way to access long options on platforms that did
 * not support getopt_long() and is preserved here for backwards
 * compatibility.  (Of course, if I'd started with a custom
 * command-line parser from the beginning, I would have had normal
 * long option support on every platform so that hack wouldn't have
 * been necessary.  Oh, well.  Some mistakes you just have to live
 * with.)
 *
 * TODO: We should be able to use this to pull files and intermingled
 * options (such as -C) from the command line in write mode.  That
 * will require a little rethinking of the argument handling in
 * bsdtar.c.
 *
 * TODO: If we want to support arbitrary command-line options from -T
 * input (as GNU tar does), we may need to extend this to handle option
 * words from sources other than argv/argc.  I'm not really sure if I
 * like that feature of GNU tar, so it's certainly not a priority.
 */
#[no_mangle]
pub unsafe extern "C" fn bsdtar_getopt(mut bsdtar: *mut bsdtar) -> libc::c_int {
    let mut popt: *const bsdtar_option = 0 as *const bsdtar_option;
    let mut match_0: *const bsdtar_option = NULL as *const bsdtar_option;
    let mut match2: *const bsdtar_option = NULL as *const bsdtar_option;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut long_prefix: *const libc::c_char = b"--\x00" as *const u8 as *const libc::c_char;
    let mut optlength: size_t = 0;
    let mut opt: libc::c_int = '?' as i32;
    let mut required: libc::c_int = 0 as libc::c_int;
    (*bsdtar).argument = NULL as *const libc::c_char;
    /* First time through, initialize everything. */
    if (*bsdtar).getopt_state == state_start as libc::c_int {
        /* Skip program name. */
        (*bsdtar).argv = (*bsdtar).argv.offset(1);
        (*bsdtar).argc -= 1;
        if (*(*bsdtar).argv).is_null() {
            return -(1 as libc::c_int);
        }
        /* Decide between "new style" and "old style" arguments. */
        if *(*(*bsdtar).argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int
            == '-' as i32
        {
            (*bsdtar).getopt_state = state_next_word as libc::c_int
        } else {
            (*bsdtar).getopt_state = state_old_tar as libc::c_int;
            let fresh0 = (*bsdtar).argv;
            (*bsdtar).argv = (*bsdtar).argv.offset(1);
            (*bsdtar).getopt_word = *fresh0;
            (*bsdtar).argc -= 1
        }
    }
    /*
     * We're parsing old-style tar arguments
     */
    if (*bsdtar).getopt_state == state_old_tar as libc::c_int {
        /* Get the next option character. */
        let fresh1 = (*bsdtar).getopt_word;
        (*bsdtar).getopt_word = (*bsdtar).getopt_word.offset(1);
        opt = *fresh1 as libc::c_int;
        if opt == '\u{0}' as i32 {
            /* New-style args can follow old-style. */
            (*bsdtar).getopt_state = state_next_word as libc::c_int
        } else {
            /* See if it takes an argument. */
            p = strchr(short_options, opt);
            if p.is_null() {
                return '?' as i32;
            }
            if *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
                (*bsdtar).argument = *(*bsdtar).argv;
                if (*bsdtar).argument.is_null() {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"Option %c requires an argument\x00" as *const u8 as *const libc::c_char,
                        opt,
                    );
                    return '?' as i32;
                }
                (*bsdtar).argv = (*bsdtar).argv.offset(1);
                (*bsdtar).argc -= 1
            }
        }
    }
    /*
     * We're ready to look at the next word in argv.
     */
    if (*bsdtar).getopt_state == state_next_word as libc::c_int {
        /* No more arguments, so no more options. */
        if (*(*bsdtar).argv.offset(0 as libc::c_int as isize)).is_null() {
            return -(1 as libc::c_int);
        }
        /* Doesn't start with '-', so no more options. */
        if *(*(*bsdtar).argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int
            != '-' as i32
        {
            return -(1 as libc::c_int);
        }
        /* "--" marks end of options; consume it and return. */
        if strcmp(
            *(*bsdtar).argv.offset(0 as libc::c_int as isize),
            b"--\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*bsdtar).argv = (*bsdtar).argv.offset(1);
            (*bsdtar).argc -= 1;
            return -(1 as libc::c_int);
        }
        /* Get next word for parsing. */
        let fresh2 = (*bsdtar).argv;
        (*bsdtar).argv = (*bsdtar).argv.offset(1);
        (*bsdtar).getopt_word = *fresh2;
        (*bsdtar).argc -= 1;
        if *(*bsdtar).getopt_word.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            /* Set up long option parser. */
            (*bsdtar).getopt_state = state_long as libc::c_int;
            (*bsdtar).getopt_word = (*bsdtar).getopt_word.offset(2 as libc::c_int as isize)
        /* Skip leading '--' */
        } else {
            /* Set up short option parser. */
            (*bsdtar).getopt_state = state_short as libc::c_int;
            (*bsdtar).getopt_word = (*bsdtar).getopt_word.offset(1)
            /* Skip leading '-' */
        }
    }
    /*
     * We're parsing a group of POSIX-style single-character options.
     */
    if (*bsdtar).getopt_state == state_short as libc::c_int {
        /* Peel next option off of a group of short options. */
        let fresh3 = (*bsdtar).getopt_word;
        (*bsdtar).getopt_word = (*bsdtar).getopt_word.offset(1);
        opt = *fresh3 as libc::c_int;
        if opt == '\u{0}' as i32 {
            /* End of this group; recurse to get next option. */
            (*bsdtar).getopt_state = state_next_word as libc::c_int;
            return bsdtar_getopt(bsdtar);
        }
        /* Does this option take an argument? */
        p = strchr(short_options, opt);
        if p.is_null() {
            return '?' as i32;
        }
        if *p.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
            required = 1 as libc::c_int
        }
        /* If it takes an argument, parse that. */
        if required != 0 {
            /* If arg is run-in, bsdtar->getopt_word already points to it. */
            if *(*bsdtar).getopt_word.offset(0 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32
            {
                /* Otherwise, pick up the next word. */
                (*bsdtar).getopt_word = *(*bsdtar).argv;
                if (*bsdtar).getopt_word.is_null() {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"Option -%c requires an argument\x00" as *const u8 as *const libc::c_char,
                        opt,
                    );
                    return '?' as i32;
                }
                (*bsdtar).argv = (*bsdtar).argv.offset(1);
                (*bsdtar).argc -= 1
            }
            if opt == 'W' as i32 {
                (*bsdtar).getopt_state = state_long as libc::c_int;
                long_prefix = b"-W \x00" as *const u8 as *const libc::c_char
            /* For clearer errors. */
            } else {
                (*bsdtar).getopt_state = state_next_word as libc::c_int;
                (*bsdtar).argument = (*bsdtar).getopt_word
            }
        }
    }
    /* We're reading a long option, including -W long=arg convention. */
    if (*bsdtar).getopt_state == state_long as libc::c_int {
        /* After this long option, we'll be starting a new word. */
        (*bsdtar).getopt_state = state_next_word as libc::c_int;
        /* Option name ends at '=' if there is one. */
        p = strchr((*bsdtar).getopt_word, '=' as i32);
        if !p.is_null() {
            optlength = p.offset_from((*bsdtar).getopt_word) as libc::c_long as size_t;
            (*bsdtar).argument =
                p.offset(1 as libc::c_int as isize) as uintptr_t as *mut libc::c_char
        } else {
            optlength = strlen((*bsdtar).getopt_word)
        }
        /* Search the table for an unambiguous match. */
        popt = tar_longopts.as_ptr();
        while !(*popt).name.is_null() {
            /* Short-circuit if first chars don't match. */
            if !(*(*popt).name.offset(0 as libc::c_int as isize) as libc::c_int
                != *(*bsdtar).getopt_word.offset(0 as libc::c_int as isize) as libc::c_int)
            {
                /* If option is a prefix of name in table, record it.*/
                if strncmp((*bsdtar).getopt_word, (*popt).name, optlength) == 0 as libc::c_int {
                    match2 = match_0; /* Record up to two matches. */
                    match_0 = popt;
                    /* If it's an exact match, we're done. */
                    if strlen((*popt).name) == optlength {
                        match2 = NULL as *const bsdtar_option; /* Forget the others. */
                        break;
                    }
                }
            }
            popt = popt.offset(1)
        }
        /* Fail if there wasn't a unique match. */
        if match_0.is_null() {
            lafe_warnc(
                0 as libc::c_int,
                b"Option %s%s is not supported\x00" as *const u8 as *const libc::c_char,
                long_prefix,
                (*bsdtar).getopt_word,
            );
            return '?' as i32;
        }
        if !match2.is_null() {
            lafe_warnc(
                0 as libc::c_int,
                b"Ambiguous option %s%s (matches --%s and --%s)\x00" as *const u8
                    as *const libc::c_char,
                long_prefix,
                (*bsdtar).getopt_word,
                (*match_0).name,
                (*match2).name,
            );
            return '?' as i32;
        }
        /* We've found a unique match; does it need an argument? */
        if (*match_0).required != 0 {
            /* Argument required: get next word if necessary. */
            if (*bsdtar).argument.is_null() {
                (*bsdtar).argument = *(*bsdtar).argv;
                if (*bsdtar).argument.is_null() {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"Option %s%s requires an argument\x00" as *const u8 as *const libc::c_char,
                        long_prefix,
                        (*match_0).name,
                    );
                    return '?' as i32;
                }
                (*bsdtar).argv = (*bsdtar).argv.offset(1);
                (*bsdtar).argc -= 1
            }
        } else if !(*bsdtar).argument.is_null() {
            lafe_warnc(
                0 as libc::c_int,
                b"Option %s%s does not allow an argument\x00" as *const u8 as *const libc::c_char,
                long_prefix,
                (*match_0).name,
            );
            return '?' as i32;
        }
        return (*match_0).equivalent;
    }
    return opt;
}
/* Argument forbidden: fail if there is one. */
