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
    pub type name_cache;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn getgrnam(__name: *const libc::c_char) -> *mut group;
    #[no_mangle]
    fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
    #[no_mangle]
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
pub type __gid_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
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
/* Fake short equivalents for long options that otherwise lack them. */
pub type C2RustUnnamed = libc::c_uint;
pub const OPTION_ZSTD: C2RustUnnamed = 14;
pub const OPTION_VERSION: C2RustUnnamed = 13;
pub const OPTION_UUENCODE: C2RustUnnamed = 12;
pub const OPTION_QUIET: C2RustUnnamed = 11;
pub const OPTION_PRESERVE_OWNER: C2RustUnnamed = 10;
pub const OPTION_NO_PRESERVE_OWNER: C2RustUnnamed = 9;
pub const OPTION_PASSPHRASE: C2RustUnnamed = 8;
pub const OPTION_LZOP: C2RustUnnamed = 7;
pub const OPTION_LZMA: C2RustUnnamed = 6;
pub const OPTION_LZ4: C2RustUnnamed = 5;
pub const OPTION_LRZIP: C2RustUnnamed = 4;
pub const OPTION_INSECURE: C2RustUnnamed = 3;
pub const OPTION_GRZIP: C2RustUnnamed = 2;
pub const OPTION_B64ENCODE: C2RustUnnamed = 1;
/*
 * Long options for cpio.  Please keep this sorted.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub required: libc::c_int,
    pub equivalent: libc::c_int,
}
pub const state_next_word: C2RustUnnamed_0 = 1;
pub const state_start: C2RustUnnamed_0 = 0;
pub const state_long: C2RustUnnamed_0 = 3;
pub const state_short: C2RustUnnamed_0 = 2;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
/*
 * Short options for cpio.  Please keep this sorted.
 */
static mut short_options: *const libc::c_char =
    b"0AaBC:cdE:F:f:H:hI:iJjLlmnO:opR:rtuVvW:yZz\x00" as *const u8 as *const libc::c_char;
/* Equivalent short option. */
static mut cpio_longopts: [option; 33] = [
    {
        let mut init = option {
            name: b"b64encode\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_B64ENCODE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"create\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dereference\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'L' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dot\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"extract\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"file\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'F' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"format\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"grzip\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_GRZIP as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"insecure\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_INSECURE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"link\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"list\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lrzip\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LRZIP as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"lz4\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LZ4 as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"lzma\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LZMA as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"lzop\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_LZOP as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"make-directories\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-preserve-owner\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_NO_PRESERVE_OWNER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"null\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"numeric-uid-gid\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"owner\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"passphrase\x00" as *const u8 as *const libc::c_char,
            required: 1 as libc::c_int,
            equivalent: OPTION_PASSPHRASE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"pass-through\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-modification-time\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"preserve-owner\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_PRESERVE_OWNER as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_QUIET as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"unconditional\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"uuencode\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_UUENCODE as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_VERSION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"xz\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'J' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"zstd\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_ZSTD as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: NULL as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 0 as libc::c_int,
        };
        init
    },
];
/*
 * I used to try to select platform-provided getopt() or
 * getopt_long(), but that caused a lot of headaches.  In particular,
 * I couldn't consistently use long options in the test harness
 * because not all platforms have getopt_long().  That in turn led to
 * overuse of the -W hack in the test harness, which made it rough to
 * run the test harness against GNU cpio.  (I periodically run the
 * test harness here against GNU cpio as a sanity-check.  Yes,
 * I've found a couple of bugs in GNU cpio that way.)
 */
#[no_mangle]
pub unsafe extern "C" fn cpio_getopt(mut cpio: *mut cpio) -> libc::c_int {
    static mut state: libc::c_int = state_start as libc::c_int;
    static mut opt_word: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut popt: *const option = 0 as *const option;
    let mut match_0: *const option = NULL as *const option;
    let mut match2: *const option = NULL as *const option;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut long_prefix: *const libc::c_char = b"--\x00" as *const u8 as *const libc::c_char;
    let mut optlength: size_t = 0;
    let mut opt: libc::c_int = '?' as i32;
    let mut required: libc::c_int = 0 as libc::c_int;
    (*cpio).argument = NULL as *const libc::c_char;
    /* First time through, initialize everything. */
    if state == state_start as libc::c_int {
        /* Skip program name. */
        (*cpio).argv = (*cpio).argv.offset(1);
        (*cpio).argc -= 1;
        state = state_next_word as libc::c_int
    }
    /*
     * We're ready to look at the next word in argv.
     */
    if state == state_next_word as libc::c_int {
        /* No more arguments, so no more options. */
        if (*(*cpio).argv.offset(0 as libc::c_int as isize)).is_null() {
            return -(1 as libc::c_int);
        }
        /* Doesn't start with '-', so no more options. */
        if *(*(*cpio).argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int
            != '-' as i32
        {
            return -(1 as libc::c_int);
        }
        /* "--" marks end of options; consume it and return. */
        if strcmp(
            *(*cpio).argv.offset(0 as libc::c_int as isize),
            b"--\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*cpio).argv = (*cpio).argv.offset(1);
            (*cpio).argc -= 1;
            return -(1 as libc::c_int);
        }
        /* Get next word for parsing. */
        let fresh0 = (*cpio).argv;
        (*cpio).argv = (*cpio).argv.offset(1);
        opt_word = *fresh0;
        (*cpio).argc -= 1;
        if *opt_word.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            /* Set up long option parser. */
            state = state_long as libc::c_int;
            opt_word = opt_word.offset(2 as libc::c_int as isize)
        /* Skip leading '--' */
        } else {
            /* Set up short option parser. */
            state = state_short as libc::c_int;
            opt_word = opt_word.offset(1)
            /* Skip leading '-' */
        }
    }
    /*
     * We're parsing a group of POSIX-style single-character options.
     */
    if state == state_short as libc::c_int {
        /* Peel next option off of a group of short options. */
        let fresh1 = opt_word;
        opt_word = opt_word.offset(1);
        opt = *fresh1 as libc::c_int;
        if opt == '\u{0}' as i32 {
            /* End of this group; recurse to get next option. */
            state = state_next_word as libc::c_int;
            return cpio_getopt(cpio);
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
            /* If arg is run-in, opt_word already points to it. */
            if *opt_word.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
                /* Otherwise, pick up the next word. */
                opt_word = *(*cpio).argv;
                if opt_word.is_null() {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"Option -%c requires an argument\x00" as *const u8 as *const libc::c_char,
                        opt,
                    );
                    return '?' as i32;
                }
                (*cpio).argv = (*cpio).argv.offset(1);
                (*cpio).argc -= 1
            }
            if opt == 'W' as i32 {
                state = state_long as libc::c_int;
                long_prefix = b"-W \x00" as *const u8 as *const libc::c_char
            /* For clearer errors. */
            } else {
                state = state_next_word as libc::c_int;
                (*cpio).argument = opt_word
            }
        }
    }
    /* We're reading a long option, including -W long=arg convention. */
    if state == state_long as libc::c_int {
        /* After this long option, we'll be starting a new word. */
        state = state_next_word as libc::c_int;
        /* Option name ends at '=' if there is one. */
        p = strchr(opt_word, '=' as i32);
        if !p.is_null() {
            optlength = p.offset_from(opt_word) as libc::c_long as size_t;
            (*cpio).argument = p.offset(1 as libc::c_int as isize) as uintptr_t as *mut libc::c_char
        } else {
            optlength = strlen(opt_word)
        }
        /* Search the table for an unambiguous match. */
        popt = cpio_longopts.as_ptr();
        while !(*popt).name.is_null() {
            /* Short-circuit if first chars don't match. */
            if !(*(*popt).name.offset(0 as libc::c_int as isize) as libc::c_int
                != *opt_word.offset(0 as libc::c_int as isize) as libc::c_int)
            {
                /* If option is a prefix of name in table, record it.*/
                if strncmp(opt_word, (*popt).name, optlength) == 0 as libc::c_int {
                    match2 = match_0; /* Record up to two matches. */
                    match_0 = popt;
                    /* If it's an exact match, we're done. */
                    if strlen((*popt).name) == optlength {
                        match2 = NULL as *const option; /* Forget the others. */
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
                opt_word,
            );
            return '?' as i32;
        }
        if !match2.is_null() {
            lafe_warnc(
                0 as libc::c_int,
                b"Ambiguous option %s%s (matches --%s and --%s)\x00" as *const u8
                    as *const libc::c_char,
                long_prefix,
                opt_word,
                (*match_0).name,
                (*match2).name,
            );
            return '?' as i32;
        }
        /* We've found a unique match; does it need an argument? */
        if (*match_0).required != 0 {
            /* Argument required: get next word if necessary. */
            if (*cpio).argument.is_null() {
                (*cpio).argument = *(*cpio).argv;
                if (*cpio).argument.is_null() {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"Option %s%s requires an argument\x00" as *const u8 as *const libc::c_char,
                        long_prefix,
                        (*match_0).name,
                    );
                    return '?' as i32;
                }
                (*cpio).argv = (*cpio).argv.offset(1);
                (*cpio).argc -= 1
            }
        } else if !(*cpio).argument.is_null() {
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
/*
 * Parse the argument to the -R or --owner flag.
 *
 * The format is one of the following:
 *   <username|uid>    - Override user but not group
 *   <username>:   - Override both, group is user's default group
 *   <uid>:    - Override user but not group
 *   <username|uid>:<groupname|gid> - Override both
 *   :<groupname|gid>  - Override group but not user
 *
 * Where uid/gid are decimal representations and groupname/username
 * are names to be looked up in system database.  Note that we try
 * to look up an argument as a name first, then try numeric parsing.
 *
 * A period can be used instead of the colon.
 *
 * Sets uid/gid return as appropriate, -1 indicates uid/gid not specified.
 * TODO: If the spec uses uname/gname, then return those to the caller
 * as well.  If the spec provides uid/gid, just return names as NULL.
 *
 * Returns NULL if no error, otherwise returns error string for display.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn owner_parse(
    mut spec: *const libc::c_char,
    mut uid: *mut libc::c_int,
    mut gid: *mut libc::c_int,
) -> *const libc::c_char {
    static mut errbuff: [libc::c_char; 128] = [0; 128];
    let mut u: *const libc::c_char = 0 as *const libc::c_char;
    let mut ue: *const libc::c_char = 0 as *const libc::c_char;
    let mut g: *const libc::c_char = 0 as *const libc::c_char;
    *uid = -(1 as libc::c_int);
    *gid = -(1 as libc::c_int);
    if *spec.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
        return b"Invalid empty user/group spec\x00" as *const u8 as *const libc::c_char;
    }
    /*
     * Split spec into [user][:.][group]
     *  u -> first char of username, NULL if no username
     *  ue -> first char after username (colon, period, or \0)
     *  g -> first char of group name
     */
    if *spec as libc::c_int == ':' as i32 || *spec as libc::c_int == '.' as i32 {
        /* If spec starts with ':' or '.', then just group. */
        u = NULL as *const libc::c_char;
        ue = u;
        g = spec.offset(1 as libc::c_int as isize)
    } else {
        /* Otherwise, [user] or [user][:] or [user][:][group] */
        u = spec;
        ue = u;
        while *ue as libc::c_int != ':' as i32
            && *ue as libc::c_int != '.' as i32
            && *ue as libc::c_int != '\u{0}' as i32
        {
            ue = ue.offset(1)
        }
        g = ue;
        if *g as libc::c_int != '\u{0}' as i32 {
            /* Skip : or . to find first char of group. */
            g = g.offset(1)
        }
    }
    if !u.is_null() {
        /* Look up user: ue is first char after end of user. */
        let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut pwent: *mut passwd = 0 as *mut passwd;
        user = malloc(
            (ue.offset_from(u) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        ) as *mut libc::c_char;
        if user.is_null() {
            return b"Couldn\'t allocate memory\x00" as *const u8 as *const libc::c_char;
        }
        memcpy(
            user as *mut libc::c_void,
            u as *const libc::c_void,
            ue.offset_from(u) as libc::c_long as libc::c_ulong,
        );
        *user.offset(ue.offset_from(u) as libc::c_long as isize) =
            '\u{0}' as i32 as libc::c_char;
        pwent = getpwnam(user);
        if !pwent.is_null() {
            *uid = (*pwent).pw_uid as libc::c_int;
            if *ue as libc::c_int != '\u{0}' as i32 {
                *gid = (*pwent).pw_gid as libc::c_int
            }
        } else {
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            errno = 0 as libc::c_int;
            *uid = strtoul(user, &mut end, 10 as libc::c_int) as libc::c_int;
            if errno != 0 || *end as libc::c_int != '\u{0}' as i32 {
                snprintf(
                    errbuff.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    b"Couldn\'t lookup user ``%s\'\'\x00" as *const u8 as *const libc::c_char,
                    user,
                );
                errbuff[(::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as usize] = '\u{0}' as i32 as libc::c_char;
                free(user as *mut libc::c_void);
                return errbuff.as_mut_ptr();
            }
        }
        free(user as *mut libc::c_void);
    }
    if *g as libc::c_int != '\u{0}' as i32 {
        let mut grp: *mut group = 0 as *mut group;
        grp = getgrnam(g);
        if !grp.is_null() {
            *gid = (*grp).gr_gid as libc::c_int
        } else {
            let mut end_0: *mut libc::c_char = 0 as *mut libc::c_char;
            errno = 0 as libc::c_int;
            *gid = strtoul(g, &mut end_0, 10 as libc::c_int) as libc::c_int;
            if errno != 0 || *end_0 as libc::c_int != '\u{0}' as i32 {
                snprintf(
                    errbuff.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    b"Couldn\'t lookup group ``%s\'\'\x00" as *const u8 as *const libc::c_char,
                    g,
                );
                errbuff[(::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as usize] = '\u{0}' as i32 as libc::c_char;
                return errbuff.as_mut_ptr();
            }
        }
    }
    return 0 as *const libc::c_char;
}
