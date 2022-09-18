use ::libc;
extern "C" {
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
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
/*-
 * Copyright (c) 2014, Mike Kazantsev
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
/* Not having a config.h of some sort is a serious problem. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bsdcat {
    pub getopt_state: libc::c_int,
    pub getopt_word: *mut libc::c_char,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub argument: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const OPTION_VERSION: C2RustUnnamed = 0;
/*
 * Long options for tar.  Please keep this list sorted.
 *
 * The symbolic names for options that lack a short equivalent are
 * defined in bsdcat.h.  Also note that so far I've found no need
 * to support optional arguments to long options.  That would be
 * a small change to the code below.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bsdcat_option {
    pub name: *const libc::c_char,
    pub required: libc::c_int,
    pub equivalent: libc::c_int,
}
pub const state_next_word: C2RustUnnamed_0 = 2;
pub const state_long: C2RustUnnamed_0 = 4;
pub const state_short: C2RustUnnamed_0 = 3;
pub const state_start: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const state_old_tar: C2RustUnnamed_0 = 1;
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 * Short options for tar.  Please keep this sorted.
 */
static mut short_options: *const libc::c_char = b"h\x00" as *const u8 as *const libc::c_char;
/* Equivalent short option. */
static mut tar_longopts: [bsdcat_option; 3] = [
    {
        let mut init = bsdcat_option {
            name: b"help\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: 'h' as i32,
        };
        init
    },
    {
        let mut init = bsdcat_option {
            name: b"version\x00" as *const u8 as *const libc::c_char,
            required: 0 as libc::c_int,
            equivalent: OPTION_VERSION as libc::c_int,
        };
        init
    },
    {
        let mut init = bsdcat_option {
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
 * has, of course, been shamelessly tailored for bsdcat.  (If you're
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
 * early bsdcat as a way to access long options on platforms that did
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
 * bsdcat.c.
 *
 * TODO: If we want to support arbitrary command-line options from -T
 * input (as GNU tar does), we may need to extend this to handle option
 * words from sources other than argv/argc.  I'm not really sure if I
 * like that feature of GNU tar, so it's certainly not a priority.
 */
#[no_mangle]
pub unsafe extern "C" fn bsdcat_getopt(mut bsdcat: *mut bsdcat) -> libc::c_int {
    let mut popt: *const bsdcat_option = 0 as *const bsdcat_option;
    let mut match_0: *const bsdcat_option = NULL as *const bsdcat_option;
    let mut match2: *const bsdcat_option = NULL as *const bsdcat_option;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut long_prefix: *const libc::c_char = b"--\x00" as *const u8 as *const libc::c_char;
    let mut optlength: size_t = 0;
    let mut opt: libc::c_int = '?' as i32;
    let mut required: libc::c_int = 0 as libc::c_int;
    (*bsdcat).argument = NULL as *const libc::c_char;
    /* First time through, initialize everything. */
    if (*bsdcat).getopt_state == state_start as libc::c_int {
        /* Skip program name. */
        (*bsdcat).argv = (*bsdcat).argv.offset(1);
        (*bsdcat).argc -= 1;
        if (*(*bsdcat).argv).is_null() {
            return -(1 as libc::c_int);
        }
        /* Decide between "new style" and "old style" arguments. */
        (*bsdcat).getopt_state = state_next_word as libc::c_int
    }
    /*
     * We're ready to look at the next word in argv.
     */
    if (*bsdcat).getopt_state == state_next_word as libc::c_int {
        /* No more arguments, so no more options. */
        if (*(*bsdcat).argv.offset(0 as libc::c_int as isize)).is_null() {
            return -(1 as libc::c_int);
        }
        /* Doesn't start with '-', so no more options. */
        if *(*(*bsdcat).argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int
            != '-' as i32
        {
            return -(1 as libc::c_int);
        }
        /* "--" marks end of options; consume it and return. */
        if strcmp(
            *(*bsdcat).argv.offset(0 as libc::c_int as isize),
            b"--\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*bsdcat).argv = (*bsdcat).argv.offset(1);
            (*bsdcat).argc -= 1;
            return -(1 as libc::c_int);
        }
        /* Get next word for parsing. */
        let fresh0 = (*bsdcat).argv;
        (*bsdcat).argv = (*bsdcat).argv.offset(1);
        (*bsdcat).getopt_word = *fresh0;
        (*bsdcat).argc -= 1;
        if *(*bsdcat).getopt_word.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            /* Set up long option parser. */
            (*bsdcat).getopt_state = state_long as libc::c_int;
            (*bsdcat).getopt_word = (*bsdcat).getopt_word.offset(2 as libc::c_int as isize)
        /* Skip leading '--' */
        } else {
            /* Set up short option parser. */
            (*bsdcat).getopt_state = state_short as libc::c_int;
            (*bsdcat).getopt_word = (*bsdcat).getopt_word.offset(1)
            /* Skip leading '-' */
        }
    }
    /*
     * We're parsing a group of POSIX-style single-character options.
     */
    if (*bsdcat).getopt_state == state_short as libc::c_int {
        /* Peel next option off of a group of short options. */
        let fresh1 = (*bsdcat).getopt_word;
        (*bsdcat).getopt_word = (*bsdcat).getopt_word.offset(1);
        opt = *fresh1 as libc::c_int;
        if opt == '\u{0}' as i32 {
            /* End of this group; recurse to get next option. */
            (*bsdcat).getopt_state = state_next_word as libc::c_int;
            return bsdcat_getopt(bsdcat);
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
            /* If arg is run-in, bsdcat->getopt_word already points to it. */
            if *(*bsdcat).getopt_word.offset(0 as libc::c_int as isize) as libc::c_int
                == '\u{0}' as i32
            {
                /* Otherwise, pick up the next word. */
                (*bsdcat).getopt_word = *(*bsdcat).argv;
                if (*bsdcat).getopt_word.is_null() {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"Option -%c requires an argument\x00" as *const u8 as *const libc::c_char,
                        opt,
                    );
                    return '?' as i32;
                }
                (*bsdcat).argv = (*bsdcat).argv.offset(1);
                (*bsdcat).argc -= 1
            }
            if opt == 'W' as i32 {
                (*bsdcat).getopt_state = state_long as libc::c_int;
                long_prefix = b"-W \x00" as *const u8 as *const libc::c_char
            /* For clearer errors. */
            } else {
                (*bsdcat).getopt_state = state_next_word as libc::c_int;
                (*bsdcat).argument = (*bsdcat).getopt_word
            }
        }
    }
    /* We're reading a long option, including -W long=arg convention. */
    if (*bsdcat).getopt_state == state_long as libc::c_int {
        /* After this long option, we'll be starting a new word. */
        (*bsdcat).getopt_state = state_next_word as libc::c_int;
        /* Option name ends at '=' if there is one. */
        p = strchr((*bsdcat).getopt_word, '=' as i32);
        if !p.is_null() {
            optlength = p.offset_from((*bsdcat).getopt_word) as libc::c_long as size_t;
            (*bsdcat).argument =
                p.offset(1 as libc::c_int as isize) as uintptr_t as *mut libc::c_char
        } else {
            optlength = strlen((*bsdcat).getopt_word)
        }
        /* Search the table for an unambiguous match. */
        popt = tar_longopts.as_ptr();
        while !(*popt).name.is_null() {
            /* Short-circuit if first chars don't match. */
            if !(*(*popt).name.offset(0 as libc::c_int as isize) as libc::c_int
                != *(*bsdcat).getopt_word.offset(0 as libc::c_int as isize) as libc::c_int)
            {
                /* If option is a prefix of name in table, record it.*/
                if strncmp((*bsdcat).getopt_word, (*popt).name, optlength) == 0 as libc::c_int {
                    match2 = match_0; /* Record up to two matches. */
                    match_0 = popt;
                    /* If it's an exact match, we're done. */
                    if strlen((*popt).name) == optlength {
                        match2 = NULL as *const bsdcat_option; /* Forget the others. */
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
                (*bsdcat).getopt_word,
            );
            return '?' as i32;
        }
        if !match2.is_null() {
            lafe_warnc(
                0 as libc::c_int,
                b"Ambiguous option %s%s (matches --%s and --%s)\x00" as *const u8
                    as *const libc::c_char,
                long_prefix,
                (*bsdcat).getopt_word,
                (*match_0).name,
                (*match2).name,
            );
            return '?' as i32;
        }
        /* We've found a unique match; does it need an argument? */
        if (*match_0).required != 0 {
            /* Argument required: get next word if necessary. */
            if (*bsdcat).argument.is_null() {
                (*bsdcat).argument = *(*bsdcat).argv;
                if (*bsdcat).argument.is_null() {
                    lafe_warnc(
                        0 as libc::c_int,
                        b"Option %s%s requires an argument\x00" as *const u8 as *const libc::c_char,
                        long_prefix,
                        (*match_0).name,
                    );
                    return '?' as i32;
                }
                (*bsdcat).argv = (*bsdcat).argv.offset(1);
                (*bsdcat).argc -= 1
            }
        } else if !(*bsdcat).argument.is_null() {
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
