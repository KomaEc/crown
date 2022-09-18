use ::libc;
extern "C" {
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn wcschr(_: *const libc::c_int, _: libc::c_int) -> *mut libc::c_int;
}
pub type wchar_t = libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
/*-
 * Copyright (c) 2003-2007 Tim Kientzle
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer
 *    in this position and unchanged.
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
 * $FreeBSD$
 */
/* Don't anchor at beginning unless the pattern starts with "^" */
pub const PATHMATCH_NO_ANCHOR_START: libc::c_int = 1 as libc::c_int;
/* Don't anchor at end unless the pattern ends with "$" */
pub const PATHMATCH_NO_ANCHOR_END: libc::c_int = 2 as libc::c_int;
/*
 * Check whether a character 'c' is matched by a list specification [...]:
 *    * Leading '!' or '^' negates the class.
 *    * <char>-<char> is a range of characters
 *    * \<char> removes any special meaning for <char>
 *
 * Some interesting boundary cases:
 *   a-d-e is one range (a-d) followed by two single characters - and e.
 *   \a-\d is same as a-d
 *   a\-d is three single characters: a, d, -
 *   Trailing - is not special (so [a-] is two characters a and -).
 *   Initial - is not special ([a-] is same as [-a] is same as [\\-a])
 *   This function never sees a trailing \.
 *   [] always fails
 *   [!] always succeeds
 */
unsafe extern "C" fn pm_list(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
    c: libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = start;
    let mut rangeStart: libc::c_char = '\u{0}' as i32 as libc::c_char;
    let mut nextRangeStart: libc::c_char = 0;
    let mut match_0: libc::c_int = 1 as libc::c_int;
    let mut nomatch: libc::c_int = 0 as libc::c_int;
    /* This will be used soon... */
    /* UNUSED */
    /* If this is a negated class, return success for nomatch. */
    if (*p as libc::c_int == '!' as i32 || *p as libc::c_int == '^' as i32) && p < end {
        match_0 = 0 as libc::c_int;
        nomatch = 1 as libc::c_int;
        p = p.offset(1)
    }
    while p < end {
        nextRangeStart = '\u{0}' as i32 as libc::c_char;
        let mut current_block_19: u64;
        match *p as libc::c_int {
            45 => {
                /* Trailing or initial '-' is not special. */
                if rangeStart as libc::c_int == '\u{0}' as i32
                    || p == end.offset(-(1 as libc::c_int as isize))
                {
                    if *p as libc::c_int == c as libc::c_int {
                        return match_0;
                    }
                } else {
                    p = p.offset(1);
                    let mut rangeEnd: libc::c_char = *p;
                    if rangeEnd as libc::c_int == '\\' as i32 {
                        p = p.offset(1);
                        rangeEnd = *p
                    }
                    if rangeStart as libc::c_int <= c as libc::c_int
                        && c as libc::c_int <= rangeEnd as libc::c_int
                    {
                        return match_0;
                    }
                }
                current_block_19 = 11307063007268554308;
                /* Possible start of range. */
            }
            92 => {
                p = p.offset(1);
                current_block_19 = 13371929842352464343;
            }
            _ => {
                current_block_19 = 13371929842352464343;
            }
        }
        match current_block_19 {
            13371929842352464343 =>
            /* Fall through */
            {
                if *p as libc::c_int == c as libc::c_int {
                    return match_0;
                }
                nextRangeStart = *p
            }
            _ => {}
        }
        rangeStart = nextRangeStart;
        p = p.offset(1)
    }
    return nomatch;
}
unsafe extern "C" fn pm_list_w(
    mut start: *const wchar_t,
    mut end: *const wchar_t,
    c: wchar_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut p: *const wchar_t = start;
    let mut rangeStart: wchar_t = '\u{0}' as i32;
    let mut nextRangeStart: wchar_t = 0;
    let mut match_0: libc::c_int = 1 as libc::c_int;
    let mut nomatch: libc::c_int = 0 as libc::c_int;
    /* This will be used soon... */
    /* UNUSED */
    /* If this is a negated class, return success for nomatch. */
    if (*p == '!' as i32 || *p == '^' as i32) && p < end {
        match_0 = 0 as libc::c_int;
        nomatch = 1 as libc::c_int;
        p = p.offset(1)
    }
    while p < end {
        nextRangeStart = '\u{0}' as i32;
        let mut current_block_19: u64;
        match *p {
            45 => {
                /* Trailing or initial '-' is not special. */
                if rangeStart == '\u{0}' as i32 || p == end.offset(-(1 as libc::c_int as isize)) {
                    if *p == c {
                        return match_0;
                    }
                } else {
                    p = p.offset(1);
                    let mut rangeEnd: wchar_t = *p;
                    if rangeEnd == '\\' as i32 {
                        p = p.offset(1);
                        rangeEnd = *p
                    }
                    if rangeStart <= c && c <= rangeEnd {
                        return match_0;
                    }
                }
                current_block_19 = 11307063007268554308;
                /* Possible start of range. */
            }
            92 => {
                p = p.offset(1);
                current_block_19 = 14501267525031084876;
            }
            _ => {
                current_block_19 = 14501267525031084876;
            }
        }
        match current_block_19 {
            14501267525031084876 =>
            /* Fall through */
            {
                if *p == c {
                    return match_0;
                }
                nextRangeStart = *p
            }
            _ => {}
        }
        rangeStart = nextRangeStart;
        p = p.offset(1)
    }
    return nomatch;
}
/*
 * If s is pointing to "./", ".//", "./././" or the like, skip it.
 */
unsafe extern "C" fn pm_slashskip(mut s: *const libc::c_char) -> *const libc::c_char {
    while *s as libc::c_int == '/' as i32
        || *s.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
        || *s.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
    {
        s = s.offset(1)
    }
    return s;
}
unsafe extern "C" fn pm_slashskip_w(mut s: *const wchar_t) -> *const wchar_t {
    while *s == '/' as i32
        || *s.offset(0 as libc::c_int as isize) == '.' as i32
            && *s.offset(1 as libc::c_int as isize) == '/' as i32
        || *s.offset(0 as libc::c_int as isize) == '.' as i32
            && *s.offset(1 as libc::c_int as isize) == '\u{0}' as i32
    {
        s = s.offset(1)
    }
    return s;
}
unsafe extern "C" fn pm(
    mut p: *const libc::c_char,
    mut s: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    /*
     * Ignore leading './', './/', '././', etc.
     */
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *s.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        s = pm_slashskip(s.offset(1 as libc::c_int as isize))
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        p = pm_slashskip(p.offset(1 as libc::c_int as isize))
    }
    loop {
        let mut current_block_53: u64;
        match *p as libc::c_int {
            0 => {
                if *s.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                    if flags & PATHMATCH_NO_ANCHOR_END != 0 {
                        return 1 as libc::c_int;
                    }
                    /* "dir" == "dir/" == "dir/." */
                    s = pm_slashskip(s)
                }
                return (*s as libc::c_int == '\u{0}' as i32) as libc::c_int;
            }
            63 => {
                /* ? always succeeds, unless we hit end of 's' */
                if *s as libc::c_int == '\u{0}' as i32 {
                    return 0 as libc::c_int;
                }
                current_block_53 = 7343950298149844727;
            }
            42 => {
                /* "*" == "**" == "***" ... */
                while *p as libc::c_int == '*' as i32 {
                    p = p.offset(1)
                }
                /* Trailing '*' always succeeds. */
                if *p as libc::c_int == '\u{0}' as i32 {
                    return 1 as libc::c_int;
                }
                while *s != 0 {
                    if __archive_pathmatch(p, s, flags) != 0 {
                        return 1 as libc::c_int;
                    }
                    s = s.offset(1)
                }
                return 0 as libc::c_int;
            }
            91 => {
                /*
                 * Find the end of the [...] character class,
                 * ignoring \] that might occur within the class.
                 */
                end = p.offset(1 as libc::c_int as isize);
                while *end as libc::c_int != '\u{0}' as i32 && *end as libc::c_int != ']' as i32 {
                    if *end as libc::c_int == '\\' as i32
                        && *end.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
                    {
                        end = end.offset(1)
                    }
                    end = end.offset(1)
                }
                if *end as libc::c_int == ']' as i32 {
                    /* We found [...], try to match it. */
                    if pm_list(p.offset(1 as libc::c_int as isize), end, *s, flags) == 0 {
                        return 0 as libc::c_int;
                    } /* Jump to trailing ']' char. */
                    p = end
                } else if *p as libc::c_int != *s as libc::c_int {
                    return 0 as libc::c_int;
                }
                current_block_53 = 7343950298149844727;
            }
            92 => {
                /* No final ']', so just match '['. */
                /* Trailing '\\' matches itself. */
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32 {
                    if *s as libc::c_int != '\\' as i32 {
                        return 0 as libc::c_int;
                    }
                } else {
                    p = p.offset(1);
                    if *p as libc::c_int != *s as libc::c_int {
                        return 0 as libc::c_int;
                    }
                }
                current_block_53 = 7343950298149844727;
            }
            47 => {
                if *s as libc::c_int != '/' as i32 && *s as libc::c_int != '\u{0}' as i32 {
                    return 0 as libc::c_int;
                }
                /* Note: pattern "/\./" won't match "/";
                 * pm_slashskip() correctly stops at backslash. */
                p = pm_slashskip(p); /* Counteract the increment below. */
                s = pm_slashskip(s);
                if *p as libc::c_int == '\u{0}' as i32 && flags & PATHMATCH_NO_ANCHOR_END != 0 {
                    return 1 as libc::c_int;
                }
                p = p.offset(-1);
                s = s.offset(-1);
                current_block_53 = 7343950298149844727;
            }
            36 => {
                /* '$' is special only at end of pattern and only
                 * if PATHMATCH_NO_ANCHOR_END is specified. */
                if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
                    && flags & PATHMATCH_NO_ANCHOR_END != 0
                {
                    /* "dir" == "dir/" == "dir/." */
                    return (*pm_slashskip(s) as libc::c_int == '\u{0}' as i32) as libc::c_int;
                }
                current_block_53 = 15070770426069950050;
            }
            _ => {
                current_block_53 = 15070770426069950050;
            }
        }
        match current_block_53 {
            15070770426069950050 =>
            /* Otherwise, '$' is not special. */
            /* FALL THROUGH */
            {
                if *p as libc::c_int != *s as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        s = s.offset(1)
    }
}
unsafe extern "C" fn pm_w(
    mut p: *const wchar_t,
    mut s: *const wchar_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut end: *const wchar_t = 0 as *const wchar_t;
    /*
     * Ignore leading './', './/', '././', etc.
     */
    if *s.offset(0 as libc::c_int as isize) == '.' as i32
        && *s.offset(1 as libc::c_int as isize) == '/' as i32
    {
        s = pm_slashskip_w(s.offset(1 as libc::c_int as isize))
    }
    if *p.offset(0 as libc::c_int as isize) == '.' as i32
        && *p.offset(1 as libc::c_int as isize) == '/' as i32
    {
        p = pm_slashskip_w(p.offset(1 as libc::c_int as isize))
    }
    loop {
        let mut current_block_53: u64;
        match *p {
            0 => {
                if *s.offset(0 as libc::c_int as isize) == '/' as i32 {
                    if flags & PATHMATCH_NO_ANCHOR_END != 0 {
                        return 1 as libc::c_int;
                    }
                    /* "dir" == "dir/" == "dir/." */
                    s = pm_slashskip_w(s)
                }
                return (*s == '\u{0}' as i32) as libc::c_int;
            }
            63 => {
                /* ? always succeeds, unless we hit end of 's' */
                if *s == '\u{0}' as i32 {
                    return 0 as libc::c_int;
                }
                current_block_53 = 7343950298149844727;
            }
            42 => {
                /* "*" == "**" == "***" ... */
                while *p == '*' as i32 {
                    p = p.offset(1)
                }
                /* Trailing '*' always succeeds. */
                if *p == '\u{0}' as i32 {
                    return 1 as libc::c_int;
                }
                while *s != 0 {
                    if __archive_pathmatch_w(p, s, flags) != 0 {
                        return 1 as libc::c_int;
                    }
                    s = s.offset(1)
                }
                return 0 as libc::c_int;
            }
            91 => {
                /*
                 * Find the end of the [...] character class,
                 * ignoring \] that might occur within the class.
                 */
                end = p.offset(1 as libc::c_int as isize);
                while *end != '\u{0}' as i32 && *end != ']' as i32 {
                    if *end == '\\' as i32
                        && *end.offset(1 as libc::c_int as isize) != '\u{0}' as i32
                    {
                        end = end.offset(1)
                    }
                    end = end.offset(1)
                }
                if *end == ']' as i32 {
                    /* We found [...], try to match it. */
                    if pm_list_w(p.offset(1 as libc::c_int as isize), end, *s, flags) == 0 {
                        return 0 as libc::c_int;
                    } /* Jump to trailing ']' char. */
                    p = end
                } else if *p != *s {
                    return 0 as libc::c_int;
                }
                current_block_53 = 7343950298149844727;
            }
            92 => {
                /* No final ']', so just match '['. */
                /* Trailing '\\' matches itself. */
                if *p.offset(1 as libc::c_int as isize) == '\u{0}' as i32 {
                    if *s != '\\' as i32 {
                        return 0 as libc::c_int;
                    }
                } else {
                    p = p.offset(1);
                    if *p != *s {
                        return 0 as libc::c_int;
                    }
                }
                current_block_53 = 7343950298149844727;
            }
            47 => {
                if *s != '/' as i32 && *s != '\u{0}' as i32 {
                    return 0 as libc::c_int;
                }
                /* Note: pattern "/\./" won't match "/";
                 * pm_slashskip() correctly stops at backslash. */
                p = pm_slashskip_w(p); /* Counteract the increment below. */
                s = pm_slashskip_w(s);
                if *p == '\u{0}' as i32 && flags & PATHMATCH_NO_ANCHOR_END != 0 {
                    return 1 as libc::c_int;
                }
                p = p.offset(-1);
                s = s.offset(-1);
                current_block_53 = 7343950298149844727;
            }
            36 => {
                /* '$' is special only at end of pattern and only
                 * if PATHMATCH_NO_ANCHOR_END is specified. */
                if *p.offset(1 as libc::c_int as isize) == '\u{0}' as i32
                    && flags & PATHMATCH_NO_ANCHOR_END != 0
                {
                    /* "dir" == "dir/" == "dir/." */
                    return (*pm_slashskip_w(s) == '\u{0}' as i32) as libc::c_int;
                }
                current_block_53 = 12997017103097597755;
            }
            _ => {
                current_block_53 = 12997017103097597755;
            }
        }
        match current_block_53 {
            12997017103097597755 =>
            /* Otherwise, '$' is not special. */
            /* FALL THROUGH */
            {
                if *p != *s {
                    return 0 as libc::c_int;
                }
            }
            _ => {}
        }
        p = p.offset(1);
        s = s.offset(1)
    }
}
/* Note that "^" and "$" are not special unless you set the corresponding
 * flag above. */
/* Main entry point. */
#[no_mangle]
pub unsafe extern "C" fn __archive_pathmatch(
    mut p: *const libc::c_char,
    mut s: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    /* Empty pattern only matches the empty string. */
    if p.is_null() || *p as libc::c_int == '\u{0}' as i32 {
        return (s.is_null() || *s as libc::c_int == '\u{0}' as i32) as libc::c_int;
    }
    /* Leading '^' anchors the start of the pattern. */
    if *p as libc::c_int == '^' as i32 {
        p = p.offset(1);
        flags &= !PATHMATCH_NO_ANCHOR_START
    }
    if *p as libc::c_int == '/' as i32 && *s as libc::c_int != '/' as i32 {
        return 0 as libc::c_int;
    }
    /* Certain patterns anchor implicitly. */
    if *p as libc::c_int == '*' as i32 || *p as libc::c_int == '/' as i32 {
        while *p as libc::c_int == '/' as i32 {
            p = p.offset(1)
        }
        while *s as libc::c_int == '/' as i32 {
            s = s.offset(1)
        }
        return pm(p, s, flags);
    }
    /* If start is unanchored, try to match start of each path element. */
    if flags & PATHMATCH_NO_ANCHOR_START != 0 {
        while !s.is_null() {
            if *s as libc::c_int == '/' as i32 {
                s = s.offset(1)
            }
            if pm(p, s, flags) != 0 {
                return 1 as libc::c_int;
            }
            s = strchr(s, '/' as i32)
        }
        return 0 as libc::c_int;
    }
    /* Default: Match from beginning. */
    return pm(p, s, flags);
}
#[no_mangle]
pub unsafe extern "C" fn __archive_pathmatch_w(
    mut p: *const wchar_t,
    mut s: *const wchar_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    /* Empty pattern only matches the empty string. */
    if p.is_null() || *p == '\u{0}' as i32 {
        return (s.is_null() || *s == '\u{0}' as i32) as libc::c_int;
    }
    /* Leading '^' anchors the start of the pattern. */
    if *p == '^' as i32 {
        p = p.offset(1);
        flags &= !PATHMATCH_NO_ANCHOR_START
    }
    if *p == '/' as i32 && *s != '/' as i32 {
        return 0 as libc::c_int;
    }
    /* Certain patterns anchor implicitly. */
    if *p == '*' as i32 || *p == '/' as i32 {
        while *p == '/' as i32 {
            p = p.offset(1)
        }
        while *s == '/' as i32 {
            s = s.offset(1)
        }
        return pm_w(p, s, flags);
    }
    /* If start is unanchored, try to match start of each path element. */
    if flags & PATHMATCH_NO_ANCHOR_START != 0 {
        while !s.is_null() {
            if *s == '/' as i32 {
                s = s.offset(1)
            }
            if pm_w(p, s, flags) != 0 {
                return 1 as libc::c_int;
            }
            s = wcschr(s, '/' as i32)
        }
        return 0 as libc::c_int;
    }
    /* Default: Match from beginning. */
    return pm_w(p, s, flags);
}
