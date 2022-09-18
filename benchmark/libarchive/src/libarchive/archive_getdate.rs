use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
pub type __int32_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
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
/*
 * Parser state.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gdstate {
    pub tokenp: *mut token,
    pub HaveYear: libc::c_int,
    pub HaveMonth: libc::c_int,
    pub HaveDay: libc::c_int,
    pub HaveWeekDay: libc::c_int,
    pub HaveTime: libc::c_int,
    pub HaveZone: libc::c_int,
    pub HaveRel: libc::c_int,
    pub Timezone: time_t,
    pub Day: time_t,
    pub Hour: time_t,
    pub Minutes: time_t,
    pub Month: time_t,
    pub Seconds: time_t,
    pub Year: time_t,
    pub DSTmode: DSTMODE,
    pub DayOrdinal: time_t,
    pub DayNumber: time_t,
    pub RelMonth: time_t,
    pub RelSeconds: time_t,
}
/* Daylight-savings mode:  on, off, or not yet known. */
pub type DSTMODE = libc::c_uint;
pub const DSTmaybe: DSTMODE = 2;
pub const DSToff: DSTMODE = 1;
pub const DSTon: DSTMODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct token {
    pub token: libc::c_int,
    pub value: time_t,
}
pub const tUNUMBER: C2RustUnnamed_1 = 267;
pub const tAGO: C2RustUnnamed_1 = 260;
pub const tMONTH_UNIT: C2RustUnnamed_1 = 265;
pub const tSEC_UNIT: C2RustUnnamed_1 = 266;
pub const tDAY: C2RustUnnamed_1 = 261;
pub const tMONTH: C2RustUnnamed_1 = 264;
pub const tDAYZONE: C2RustUnnamed_1 = 262;
pub const tZONE: C2RustUnnamed_1 = 268;
pub const tDST: C2RustUnnamed_1 = 269;
pub const tPM: C2RustUnnamed_0 = 1;
pub const tAMPM: C2RustUnnamed_1 = 263;
/*
 * A dictionary of time words.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LEXICON {
    pub abbrev: size_t,
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub value: time_t,
}
pub const tAM: C2RustUnnamed_0 = 0;
/* Meridian:  am or pm. */
pub type C2RustUnnamed_0 = libc::c_uint;
/* Token types returned by nexttoken() */
pub type C2RustUnnamed_1 = libc::c_uint;
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 * This code is in the public domain and has no copyright.
 *
 * This is a plain C recursive-descent translation of an old
 * public-domain YACC grammar that has been used for parsing dates in
 * very many open-source projects.
 *
 * Since the original authors were generous enough to donate their
 * work to the public domain, I feel compelled to match their
 * generosity.
 *
 * Tim Kientzle, February 2009.
 */
/*
 * Header comment from original getdate.y:
 */
/*
**  Originally written by Steven M. Bellovin <smb@research.att.com> while
**  at the University of North Carolina at Chapel Hill.  Later tweaked by
**  a couple of people on Usenet.  Completely overhauled by Rich $alz
**  <rsalz@bbn.com> and Jim Berets <jberets@bbn.com> in August, 1990;
**
**  This grammar has 10 shift/reduce conflicts.
**
**  This code is in the public domain and has no copyright.
*/
/* Basic time units. */
pub const EPOCH: libc::c_int = 1970 as libc::c_int;
pub const MINUTE: libc::c_long = 60 as libc::c_long;
pub const HOUR: libc::c_long = 60 as libc::c_long * MINUTE;
pub const DAY: libc::c_long = 24 as libc::c_long * HOUR;
/*
 * A series of functions that recognize certain common time phrases.
 * Each function returns 1 if it managed to make sense of some of the
 * tokens, zero otherwise.
 */
/*
 *  hour:minute or hour:minute:second with optional AM, PM, or numeric
 *  timezone offset
 */
unsafe extern "C" fn timephrase(mut gds: *mut gdstate) -> libc::c_int {
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == ':' as i32
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(3 as libc::c_int as isize)).token == ':' as i32
        && (*(*gds).tokenp.offset(4 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* "12:14:18" or "22:08:07" */
        (*gds).HaveTime += 1;
        (*gds).Hour = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Minutes = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).Seconds = (*(*gds).tokenp.offset(4 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(5 as libc::c_int as isize)
    } else if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == ':' as i32
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* "12:14" or "22:08" */
        (*gds).HaveTime += 1;
        (*gds).Hour = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Minutes = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).Seconds = 0 as libc::c_int as time_t;
        (*gds).tokenp = (*gds).tokenp.offset(3 as libc::c_int as isize)
    } else if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tAMPM as libc::c_int
    {
        /* "7" is a time if it's followed by "am" or "pm" */
        (*gds).HaveTime += 1;
        (*gds).Hour = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Seconds = 0 as libc::c_int as time_t;
        (*gds).Minutes = (*gds).Seconds;
        /* We'll handle the AM/PM below. */
        (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize)
    } else {
        /* We can't handle this. */
        return 0 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tAMPM as libc::c_int {
        /* "7:12pm", "12:20:13am" */
        if (*gds).Hour == 12 as libc::c_int as libc::c_long {
            (*gds).Hour = 0 as libc::c_int as time_t
        }
        if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
            == tPM as libc::c_int as libc::c_long
        {
            (*gds).Hour += 12 as libc::c_int as libc::c_long
        }
        (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize)
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == '+' as i32
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* "7:14+0700" */
        (*gds).HaveZone += 1;
        (*gds).DSTmode = DSToff;
        (*gds).Timezone = -((*(*gds).tokenp.offset(1 as libc::c_int as isize)).value
            / 100 as libc::c_int as libc::c_long
            * HOUR
            + (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value
                % 100 as libc::c_int as libc::c_long
                * MINUTE);
        (*gds).tokenp = (*gds).tokenp.offset(2 as libc::c_int as isize)
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == '-' as i32
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* "19:14:12-0530" */
        (*gds).HaveZone += 1;
        (*gds).DSTmode = DSToff;
        (*gds).Timezone = (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value
            / 100 as libc::c_int as libc::c_long
            * HOUR
            + (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value
                % 100 as libc::c_int as libc::c_long
                * MINUTE;
        (*gds).tokenp = (*gds).tokenp.offset(2 as libc::c_int as isize)
    }
    return 1 as libc::c_int;
}
/*
 * Timezone name, possibly including DST.
 */
unsafe extern "C" fn zonephrase(mut gds: *mut gdstate) -> libc::c_int {
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tZONE as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tDST as libc::c_int
    {
        (*gds).HaveZone += 1;
        (*gds).Timezone = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).DSTmode = DSTon;
        (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tZONE as libc::c_int {
        (*gds).HaveZone += 1;
        (*gds).Timezone = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).DSTmode = DSToff;
        (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tDAYZONE as libc::c_int {
        (*gds).HaveZone += 1;
        (*gds).Timezone = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).DSTmode = DSTon;
        (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*
 * Year/month/day in various combinations.
 */
unsafe extern "C" fn datephrase(mut gds: *mut gdstate) -> libc::c_int {
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == '/' as i32
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(3 as libc::c_int as isize)).token == '/' as i32
        && (*(*gds).tokenp.offset(4 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        (*gds).HaveYear += 1;
        (*gds).HaveMonth += 1;
        (*gds).HaveDay += 1;
        if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
            >= 13 as libc::c_int as libc::c_long
        {
            /* First number is big:  2004/01/29, 99/02/17 */
            (*gds).Year = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
            (*gds).Month = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
            (*gds).Day = (*(*gds).tokenp.offset(4 as libc::c_int as isize)).value
        } else if (*(*gds).tokenp.offset(4 as libc::c_int as isize)).value
            >= 13 as libc::c_int as libc::c_long
            || (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value
                >= 13 as libc::c_int as libc::c_long
        {
            /* Last number is big:  01/07/98 */
            /* Middle number is big:  01/29/04 */
            (*gds).Month = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
            (*gds).Day = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
            (*gds).Year = (*(*gds).tokenp.offset(4 as libc::c_int as isize)).value
        } else {
            /* No significant clues: 02/03/04 */
            (*gds).Month = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
            (*gds).Day = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
            (*gds).Year = (*(*gds).tokenp.offset(4 as libc::c_int as isize)).value
        }
        (*gds).tokenp = (*gds).tokenp.offset(5 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == '/' as i32
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* "1/15" */
        (*gds).HaveMonth += 1;
        (*gds).HaveDay += 1;
        (*gds).Month = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Day = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(3 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == '-' as i32
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(3 as libc::c_int as isize)).token == '-' as i32
        && (*(*gds).tokenp.offset(4 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* ISO 8601 format.  yyyy-mm-dd.  */
        (*gds).HaveYear += 1;
        (*gds).HaveMonth += 1;
        (*gds).HaveDay += 1;
        (*gds).Year = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Month = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).Day = (*(*gds).tokenp.offset(4 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(5 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == '-' as i32
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tMONTH as libc::c_int
        && (*(*gds).tokenp.offset(3 as libc::c_int as isize)).token == '-' as i32
        && (*(*gds).tokenp.offset(4 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        (*gds).HaveYear += 1;
        (*gds).HaveMonth += 1;
        (*gds).HaveDay += 1;
        if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
            > 31 as libc::c_int as libc::c_long
        {
            /* e.g. 1992-Jun-17 */
            (*gds).Year = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
            (*gds).Month = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
            (*gds).Day = (*(*gds).tokenp.offset(4 as libc::c_int as isize)).value
        } else {
            /* e.g. 17-JUN-1992.  */
            (*gds).Day = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
            (*gds).Month = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
            (*gds).Year = (*(*gds).tokenp.offset(4 as libc::c_int as isize)).value
        }
        (*gds).tokenp = (*gds).tokenp.offset(5 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tMONTH as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == ',' as i32
        && (*(*gds).tokenp.offset(3 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* "June 17, 2001" */
        (*gds).HaveYear += 1;
        (*gds).HaveMonth += 1;
        (*gds).HaveDay += 1;
        (*gds).Month = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Day = (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value;
        (*gds).Year = (*(*gds).tokenp.offset(3 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(4 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tMONTH as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* "May 3" */
        (*gds).HaveMonth += 1;
        (*gds).HaveDay += 1;
        (*gds).Month = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Day = (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(2 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tMONTH as libc::c_int
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
    {
        /* "12 Sept 1997" */
        (*gds).HaveYear += 1;
        (*gds).HaveMonth += 1;
        (*gds).HaveDay += 1;
        (*gds).Day = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Month = (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value;
        (*gds).Year = (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(3 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tMONTH as libc::c_int
    {
        /* "12 Sept" */
        (*gds).HaveMonth += 1;
        (*gds).HaveDay += 1;
        (*gds).Day = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).Month = (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(2 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*
 * Relative time phrase: "tomorrow", "yesterday", "+1 hour", etc.
 */
unsafe extern "C" fn relunitphrase(mut gds: *mut gdstate) -> libc::c_int {
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == '-' as i32
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tSEC_UNIT as libc::c_int
    {
        /* "-3 hours" */
        (*gds).HaveRel += 1;
        (*gds).RelSeconds -= (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value
            * (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(3 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == '+' as i32
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tSEC_UNIT as libc::c_int
    {
        /* "+1 minute" */
        (*gds).HaveRel += 1;
        (*gds).RelSeconds += (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value
            * (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(3 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tSEC_UNIT as libc::c_int
    {
        /* "1 day" */
        (*gds).HaveRel += 1;
        (*gds).RelSeconds += (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
            * (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(2 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == '-' as i32
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tMONTH_UNIT as libc::c_int
    {
        /* "-3 months" */
        (*gds).HaveRel += 1;
        (*gds).RelMonth -= (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value
            * (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(3 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == '+' as i32
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(2 as libc::c_int as isize)).token == tMONTH_UNIT as libc::c_int
    {
        /* "+5 years" */
        (*gds).HaveRel += 1;
        (*gds).RelMonth += (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value
            * (*(*gds).tokenp.offset(2 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(3 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tMONTH_UNIT as libc::c_int
    {
        /* "2 years" */
        (*gds).HaveRel += 1;
        (*gds).RelMonth += (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
            * (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(2 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tSEC_UNIT as libc::c_int {
        /* "now", "tomorrow" */
        (*gds).HaveRel += 1;
        (*gds).RelSeconds += (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tMONTH_UNIT as libc::c_int {
        /* "month" */
        (*gds).HaveRel += 1;
        (*gds).RelMonth += (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*
 * Day of the week specification.
 */
unsafe extern "C" fn dayphrase(mut gds: *mut gdstate) -> libc::c_int {
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tDAY as libc::c_int {
        /* "tues", "wednesday," */
        (*gds).HaveWeekDay += 1;
        (*gds).DayOrdinal = 1 as libc::c_int as time_t;
        (*gds).DayNumber = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
        if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == ',' as i32 {
            (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize)
        }
        return 1 as libc::c_int;
    }
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int
        && (*(*gds).tokenp.offset(1 as libc::c_int as isize)).token == tDAY as libc::c_int
    {
        /* "second tues" "3 wed" */
        (*gds).HaveWeekDay += 1;
        (*gds).DayOrdinal = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
        (*gds).DayNumber = (*(*gds).tokenp.offset(1 as libc::c_int as isize)).value;
        (*gds).tokenp = (*gds).tokenp.offset(2 as libc::c_int as isize);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*
 * Try to match a phrase using one of the above functions.
 * This layer also deals with a couple of generic issues.
 */
unsafe extern "C" fn phrase(mut gds: *mut gdstate) -> libc::c_int {
    if timephrase(gds) != 0 {
        return 1 as libc::c_int;
    }
    if zonephrase(gds) != 0 {
        return 1 as libc::c_int;
    }
    if datephrase(gds) != 0 {
        return 1 as libc::c_int;
    }
    if dayphrase(gds) != 0 {
        return 1 as libc::c_int;
    }
    if relunitphrase(gds) != 0 {
        if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tAGO as libc::c_int {
            (*gds).RelSeconds = -(*gds).RelSeconds;
            (*gds).RelMonth = -(*gds).RelMonth;
            (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize)
        }
        return 1 as libc::c_int;
    }
    /* Bare numbers sometimes have meaning. */
    if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).token == tUNUMBER as libc::c_int {
        if (*gds).HaveTime != 0 && (*gds).HaveYear == 0 && (*gds).HaveRel == 0 {
            (*gds).HaveYear += 1;
            (*gds).Year = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
            (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
            return 1 as libc::c_int;
        }
        if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
            > 10000 as libc::c_int as libc::c_long
        {
            /* "20040301" */
            (*gds).HaveYear += 1;
            (*gds).HaveMonth += 1;
            (*gds).HaveDay += 1;
            (*gds).Day = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
                % 100 as libc::c_int as libc::c_long;
            (*gds).Month = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
                / 100 as libc::c_int as libc::c_long
                % 100 as libc::c_int as libc::c_long;
            (*gds).Year = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
                / 10000 as libc::c_int as libc::c_long;
            (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
            return 1 as libc::c_int;
        }
        if (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
            < 24 as libc::c_int as libc::c_long
        {
            (*gds).HaveTime += 1;
            (*gds).Hour = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value;
            (*gds).Minutes = 0 as libc::c_int as time_t;
            (*gds).Seconds = 0 as libc::c_int as time_t;
            (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
            return 1 as libc::c_int;
        }
        if ((*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
            / 100 as libc::c_int as libc::c_long)
            < 24 as libc::c_int as libc::c_long
            && ((*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
                % 100 as libc::c_int as libc::c_long)
                < 60 as libc::c_int as libc::c_long
        {
            /* "513" is same as "5:13" */
            (*gds).Hour = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
                / 100 as libc::c_int as libc::c_long;
            (*gds).Minutes = (*(*gds).tokenp.offset(0 as libc::c_int as isize)).value
                % 100 as libc::c_int as libc::c_long;
            (*gds).Seconds = 0 as libc::c_int as time_t;
            (*gds).tokenp = (*gds).tokenp.offset(1 as libc::c_int as isize);
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
static mut TimeWords: [LEXICON; 137] = [
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"am\x00" as *const u8 as *const libc::c_char,
            type_0: tAMPM as libc::c_int,
            value: tAM as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"pm\x00" as *const u8 as *const libc::c_char,
            type_0: tAMPM as libc::c_int,
            value: tPM as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"january\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 1 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"february\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 2 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"march\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 3 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"april\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 4 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"may\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 5 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"june\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 6 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"july\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 7 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"august\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 8 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"september\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 9 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"october\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 10 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"november\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 11 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"december\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 12 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 2 as libc::c_int as size_t,
            name: b"sunday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 0 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"monday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 1 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 2 as libc::c_int as size_t,
            name: b"tuesday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 2 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"wednesday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 3 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 2 as libc::c_int as size_t,
            name: b"thursday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 4 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 2 as libc::c_int as size_t,
            name: b"friday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 5 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 2 as libc::c_int as size_t,
            name: b"saturday\x00" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 6 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"gmt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"ut\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"utc\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"wet\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"bst\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 0 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"wat\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 1 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"at\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 2 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"nft\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 3 as libc::c_int as libc::c_long * HOUR
                + 30 as libc::c_int as libc::c_long * MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"nst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 3 as libc::c_int as libc::c_long * HOUR
                + 30 as libc::c_int as libc::c_long * MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"ndt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 3 as libc::c_int as libc::c_long * HOUR
                + 30 as libc::c_int as libc::c_long * MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"ast\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 4 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"adt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 4 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"est\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 5 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"edt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 5 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"cst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 6 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"cdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 6 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"mst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 7 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"mdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 7 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"pst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 8 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"pdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 8 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"yst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 9 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"ydt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 9 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"hst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 10 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"hdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 10 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"cat\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 10 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"ahst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 10 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"nt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 11 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"idlw\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 12 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"cet\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"met\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"mewt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"mest\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"swt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"sst\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"fwt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"fst\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"eet\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(2 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"bt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(3 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"it\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(3 as libc::c_int) as libc::c_long * HOUR
                - 30 as libc::c_int as libc::c_long * MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"zp4\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(4 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"zp5\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(5 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"ist\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(5 as libc::c_int) as libc::c_long * HOUR
                - 30 as libc::c_int as libc::c_long * MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"zp6\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(6 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"wast\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(7 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"wadt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(7 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"jt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(7 as libc::c_int) as libc::c_long * HOUR
                - 30 as libc::c_int as libc::c_long * MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"cct\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(8 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"jst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(9 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"cast\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(9 as libc::c_int) as libc::c_long * HOUR
                - 30 as libc::c_int as libc::c_long * MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"cadt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(9 as libc::c_int) as libc::c_long * HOUR
                - 30 as libc::c_int as libc::c_long * MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"east\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(10 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"eadt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(10 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"gst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(10 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"nzt\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(12 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"nzst\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(12 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"nzdt\x00" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(12 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"idle\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(12 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"dst\x00" as *const u8 as *const libc::c_char,
            type_0: tDST as libc::c_int,
            value: 0 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 4 as libc::c_int as size_t,
            name: b"years\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH_UNIT as libc::c_int,
            value: 12 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 5 as libc::c_int as size_t,
            name: b"months\x00" as *const u8 as *const libc::c_char,
            type_0: tMONTH_UNIT as libc::c_int,
            value: 1 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 9 as libc::c_int as size_t,
            name: b"fortnights\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 14 as libc::c_int as libc::c_long * DAY,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 4 as libc::c_int as size_t,
            name: b"weeks\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 7 as libc::c_int as libc::c_long * DAY,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"days\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: DAY,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 4 as libc::c_int as size_t,
            name: b"hours\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"minutes\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: MINUTE,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 3 as libc::c_int as size_t,
            name: b"seconds\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 1 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"tomorrow\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: DAY,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"yesterday\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: -DAY,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"today\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 0 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"now\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 0 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"last\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: -(1 as libc::c_int) as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"this\x00" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 0 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"next\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 2 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"first\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 1 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"1st\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 1 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"2nd\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 2 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"third\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 3 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"3rd\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 3 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"fourth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 4 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"4th\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 4 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"fifth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 5 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"5th\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 5 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"sixth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 6 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"seventh\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 7 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"eighth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 8 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"ninth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 9 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"tenth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 10 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"eleventh\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 11 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"twelfth\x00" as *const u8 as *const libc::c_char,
            type_0: tUNUMBER as libc::c_int,
            value: 12 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"ago\x00" as *const u8 as *const libc::c_char,
            type_0: tAGO as libc::c_int,
            value: 1 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"a\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 1 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"b\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 2 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"c\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 3 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"d\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 4 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"e\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 5 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"f\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 6 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"g\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 7 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"h\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 8 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"i\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 9 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"k\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 10 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"l\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 11 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"m\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 12 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"n\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(1 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"o\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(2 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"p\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(3 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"q\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(4 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"r\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(5 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"s\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(6 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"t\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(7 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"u\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(8 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"v\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(9 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"w\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(10 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"x\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(11 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"y\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(12 as libc::c_int) as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: b"z\x00" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 0 as libc::c_int as libc::c_long * HOUR,
        };
        init
    },
    {
        let mut init = LEXICON {
            abbrev: 0 as libc::c_int as size_t,
            name: NULL as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int as time_t,
        };
        init
    },
];
/*
 * Year is either:
 *  = A number from 0 to 99, which means a year from 1970 to 2069, or
 *  = The actual year (>=100).
 */
unsafe extern "C" fn Convert(
    mut Month: time_t,
    mut Day: time_t,
    mut Year: time_t,
    mut Hours: time_t,
    mut Minutes: time_t,
    mut Seconds: time_t,
    mut Timezone: time_t,
    mut DSTmode: DSTMODE,
) -> time_t {
    let mut DaysInMonth: [libc::c_schar; 12] = [
        31 as libc::c_int as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
    ];
    let mut Julian: time_t = 0;
    let mut i: libc::c_int = 0;
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
    if Year < 69 as libc::c_int as libc::c_long {
        Year += 2000 as libc::c_int as libc::c_long
    } else if Year < 100 as libc::c_int as libc::c_long {
        Year += 1900 as libc::c_int as libc::c_long
    }
    DaysInMonth[1 as libc::c_int as usize] = if Year % 4 as libc::c_int as libc::c_long
        == 0 as libc::c_int as libc::c_long
        && (Year % 100 as libc::c_int as libc::c_long != 0 as libc::c_int as libc::c_long
            || Year % 400 as libc::c_int as libc::c_long == 0 as libc::c_int as libc::c_long)
    {
        29 as libc::c_int
    } else {
        28 as libc::c_int
    } as libc::c_schar;
    /* Checking for 2038 bogusly assumes that time_t is 32 bits.  But
    I'm too lazy to try to check for time_t overflow in another way.  */
    if Year < EPOCH as libc::c_long
        || Year > 2038 as libc::c_int as libc::c_long
        || Month < 1 as libc::c_int as libc::c_long
        || Month > 12 as libc::c_int as libc::c_long
        || Day < 1 as libc::c_int as libc::c_long
        || {
            Month -= 1;
            (Day) > DaysInMonth[Month as libc::c_int as usize] as libc::c_long
        }
        || Hours < 0 as libc::c_int as libc::c_long
        || Hours > 23 as libc::c_int as libc::c_long
        || Minutes < 0 as libc::c_int as libc::c_long
        || Minutes > 59 as libc::c_int as libc::c_long
        || Seconds < 0 as libc::c_int as libc::c_long
        || Seconds > 59 as libc::c_int as libc::c_long
    {
        return -(1 as libc::c_int) as time_t;
    }
    Julian = Day - 1 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int;
    while (i as libc::c_long) < Month {
        Julian += DaysInMonth[i as usize] as libc::c_long;
        i += 1
    }
    i = EPOCH;
    while (i as libc::c_long) < Year {
        Julian += (365 as libc::c_int + (i % 4 as libc::c_int == 0 as libc::c_int) as libc::c_int)
            as libc::c_long;
        i += 1
    }
    Julian *= DAY;
    Julian += Timezone;
    Julian += Hours * HOUR + Minutes * MINUTE + Seconds;
    ltime = localtime_r(&mut Julian, &mut tmbuf);
    if DSTmode as libc::c_uint == DSTon as libc::c_int as libc::c_uint
        || DSTmode as libc::c_uint == DSTmaybe as libc::c_int as libc::c_uint
            && (*ltime).tm_isdst != 0
    {
        Julian -= HOUR
    }
    return Julian;
}
unsafe extern "C" fn DSTcorrect(mut Start: time_t, mut Future: time_t) -> time_t {
    let mut StartDay: time_t = 0;
    let mut FutureDay: time_t = 0;
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
    ltime = localtime_r(&mut Start, &mut tmbuf);
    StartDay = (((*ltime).tm_hour + 1 as libc::c_int) % 24 as libc::c_int) as time_t;
    ltime = localtime_r(&mut Future, &mut tmbuf);
    FutureDay = (((*ltime).tm_hour + 1 as libc::c_int) % 24 as libc::c_int) as time_t;
    return Future - Start + (StartDay - FutureDay) * HOUR;
}
unsafe extern "C" fn RelativeDate(
    mut Start: time_t,
    mut zone: time_t,
    mut dstmode: libc::c_int,
    mut DayOrdinal: time_t,
    mut DayNumber: time_t,
) -> time_t {
    let mut tm: *mut tm = 0 as *mut tm;
    let mut t: time_t = 0;
    let mut now: time_t = 0;
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
    t = Start - zone;
    tm = gmtime_r(&mut t, &mut tmbuf);
    now = Start;
    now += DAY
        * ((DayNumber - (*tm).tm_wday as libc::c_long + 7 as libc::c_int as libc::c_long)
            % 7 as libc::c_int as libc::c_long);
    now += 7 as libc::c_int as libc::c_long
        * DAY
        * (if DayOrdinal <= 0 as libc::c_int as libc::c_long {
            DayOrdinal
        } else {
            (DayOrdinal) - 1 as libc::c_int as libc::c_long
        });
    if dstmode == DSTmaybe as libc::c_int {
        return DSTcorrect(Start, now);
    }
    return now - Start;
}
unsafe extern "C" fn RelativeMonth(
    mut Start: time_t,
    mut Timezone: time_t,
    mut RelMonth: time_t,
) -> time_t {
    let mut tm: *mut tm = 0 as *mut tm;
    let mut Month: time_t = 0;
    let mut Year: time_t = 0;
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
    if RelMonth == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as time_t;
    }
    tm = localtime_r(&mut Start, &mut tmbuf);
    Month = (12 as libc::c_int * ((*tm).tm_year + 1900 as libc::c_int) + (*tm).tm_mon)
        as libc::c_long
        + RelMonth;
    Year = Month / 12 as libc::c_int as libc::c_long;
    Month = Month % 12 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long;
    return DSTcorrect(
        Start,
        Convert(
            Month,
            (*tm).tm_mday as time_t,
            Year,
            (*tm).tm_hour as time_t,
            (*tm).tm_min as time_t,
            (*tm).tm_sec as time_t,
            Timezone,
            DSTmaybe,
        ),
    );
}
/*
 * Tokenizer.
 */
unsafe extern "C" fn nexttoken(
    mut in_0: *mut *const libc::c_char,
    mut value: *mut time_t,
) -> libc::c_int {
    let mut c: libc::c_char = 0;
    let mut buff: [libc::c_char; 64] = [0; 64];
    loop {
        while *(*__ctype_b_loc()).offset(**in_0 as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            *in_0 = (*in_0).offset(1)
        }
        /* Skip parenthesized comments. */
        if **in_0 as libc::c_int == '(' as i32 {
            let mut Count: libc::c_int = 0 as libc::c_int;
            loop {
                let fresh0 = *in_0;
                *in_0 = (*in_0).offset(1);
                c = *fresh0;
                if c as libc::c_int == '\u{0}' as i32 {
                    return c as libc::c_int;
                }
                if c as libc::c_int == '(' as i32 {
                    Count += 1
                } else if c as libc::c_int == ')' as i32 {
                    Count -= 1
                }
                if !(Count > 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            /* Try the next token in the word table first. */
            /* This allows us to match "2nd", for example. */
            let mut src: *const libc::c_char = *in_0;
            let mut tp: *const LEXICON = 0 as *const LEXICON;
            let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            /* Force to lowercase and strip '.' characters. */
            while *src as libc::c_int != '\u{0}' as i32
                && (*(*__ctype_b_loc()).offset(*src as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                    || *src as libc::c_int == '.' as i32)
                && (i as libc::c_ulong)
                    < (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                if *src as libc::c_int != '.' as i32 {
                    if *(*__ctype_b_loc()).offset(*src as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                    {
                        let fresh1 = i;
                        i = i.wrapping_add(1);
                        buff[fresh1 as usize] = ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = *src as libc::c_uchar as libc::c_int;
                                    __res =
                                        if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        }
                                } else {
                                    __res = tolower(*src as libc::c_uchar as libc::c_int)
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(*src as libc::c_uchar as libc::c_int as isize)
                            }
                            __res
                        }) as libc::c_char
                    } else {
                        let fresh2 = i;
                        i = i.wrapping_add(1);
                        buff[fresh2 as usize] = *src
                    }
                }
                src = src.offset(1)
            }
            buff[i as usize] = '\u{0}' as i32 as libc::c_char;
            /*
             * Find the first match.  If the word can be
             * abbreviated, make sure we match at least
             * the minimum abbreviation.
             */
            tp = TimeWords.as_ptr();
            while !(*tp).name.is_null() {
                let mut abbrev: size_t = (*tp).abbrev;
                if abbrev == 0 as libc::c_int as libc::c_ulong {
                    abbrev = strlen((*tp).name)
                }
                if strlen(buff.as_mut_ptr()) >= abbrev
                    && strncmp((*tp).name, buff.as_mut_ptr(), strlen(buff.as_mut_ptr()))
                        == 0 as libc::c_int
                {
                    /* Skip over token. */
                    *in_0 = src;
                    /* Return the match. */
                    *value = (*tp).value;
                    return (*tp).type_0;
                }
                tp = tp.offset(1)
            }
            /*
             * Not in the word table, maybe it's a number.  Note:
             * Because '-' and '+' have other special meanings, I
             * don't deal with signed numbers here.
             */
            c = **in_0;
            if *(*__ctype_b_loc()).offset(c as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                *value = 0 as libc::c_int as time_t;
                loop {
                    let fresh3 = *in_0;
                    *in_0 = (*in_0).offset(1);
                    c = *fresh3;
                    if !(*(*__ctype_b_loc()).offset(c as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0)
                    {
                        break;
                    }
                    *value = 10 as libc::c_int as libc::c_long * *value + c as libc::c_long
                        - '0' as i32 as libc::c_long
                }
                *in_0 = (*in_0).offset(-1);
                return tUNUMBER as libc::c_int;
            }
            let fresh4 = *in_0;
            *in_0 = (*in_0).offset(1);
            return *fresh4 as libc::c_int;
        }
    }
}
pub const TM_YEAR_ORIGIN: libc::c_int = 1900 as libc::c_int;
/* Yield A - B, measured in seconds.  */
unsafe extern "C" fn difftm(mut a: *mut tm, mut b: *mut tm) -> libc::c_long {
    let mut ay: libc::c_int = (*a).tm_year + (TM_YEAR_ORIGIN - 1 as libc::c_int);
    let mut by: libc::c_int = (*b).tm_year + (TM_YEAR_ORIGIN - 1 as libc::c_int);
    let mut days: libc::c_int =
        (((*a).tm_yday - (*b).tm_yday + ((ay >> 2 as libc::c_int) - (by >> 2 as libc::c_int))
            - (ay / 100 as libc::c_int - by / 100 as libc::c_int)
            + ((ay / 100 as libc::c_int >> 2 as libc::c_int)
                - (by / 100 as libc::c_int >> 2 as libc::c_int))) as libc::c_long
            + (ay - by) as libc::c_long * 365 as libc::c_int as libc::c_long)
            as libc::c_int;
    return days as libc::c_long * DAY
        + ((*a).tm_hour - (*b).tm_hour) as libc::c_long * HOUR
        + ((*a).tm_min - (*b).tm_min) as libc::c_long * MINUTE
        + ((*a).tm_sec - (*b).tm_sec) as libc::c_long;
}
/*-
 * Copyright (c) 2003-2015 Tim Kientzle
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
 * $FreeBSD$
 */
/*
 *
 * The public function.
 *
 * TODO: tokens[] array should be dynamically sized.
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_get_date(mut now: time_t, mut p: *const libc::c_char) -> time_t {
    let mut tokens: [token; 256] = [token { token: 0, value: 0 }; 256];
    let mut _gds: gdstate = gdstate {
        tokenp: 0 as *mut token,
        HaveYear: 0,
        HaveMonth: 0,
        HaveDay: 0,
        HaveWeekDay: 0,
        HaveTime: 0,
        HaveZone: 0,
        HaveRel: 0,
        Timezone: 0,
        Day: 0,
        Hour: 0,
        Minutes: 0,
        Month: 0,
        Seconds: 0,
        Year: 0,
        DSTmode: DSTon,
        DayOrdinal: 0,
        DayNumber: 0,
        RelMonth: 0,
        RelSeconds: 0,
    };
    let mut lasttoken: *mut token = 0 as *mut token;
    let mut gds: *mut gdstate = 0 as *mut gdstate;
    let mut local: tm = tm {
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
    let mut tm: *mut tm = 0 as *mut tm;
    let mut gmt: tm = tm {
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
    let mut gmt_ptr: *mut tm = 0 as *mut tm;
    let mut Start: time_t = 0;
    let mut tod: time_t = 0;
    let mut tzone: libc::c_long = 0;
    /* Clear out the parsed token array. */
    memset(
        tokens.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[token; 256]>() as libc::c_ulong,
    );
    /* Initialize the parser state. */
    memset(
        &mut _gds as *mut gdstate as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<gdstate>() as libc::c_ulong,
    );
    gds = &mut _gds;
    /* Look up the current time. */
    tm = localtime_r(&mut now, &mut local);
    if tm.is_null() {
        return -(1 as libc::c_int) as time_t;
    }
    /* Look up UTC if we can and use that to determine the current
     * timezone offset. */
    gmt_ptr = gmtime_r(&mut now, &mut gmt);
    if !gmt_ptr.is_null() {
        tzone = difftm(&mut gmt, &mut local)
    } else {
        /* This system doesn't understand timezones; fake it. */
        tzone = 0 as libc::c_int as libc::c_long
    }
    if local.tm_isdst != 0 {
        tzone += HOUR
    }
    /* Tokenize the input string. */
    lasttoken = tokens.as_mut_ptr();
    loop {
        (*lasttoken).token = nexttoken(&mut p, &mut (*lasttoken).value);
        if !((*lasttoken).token != 0 as libc::c_int) {
            break;
        }
        lasttoken = lasttoken.offset(1);
        if lasttoken > tokens.as_mut_ptr().offset(255 as libc::c_int as isize) {
            return -(1 as libc::c_int) as time_t;
        }
    }
    (*gds).tokenp = tokens.as_mut_ptr();
    /* Match phrases until we run out of input tokens. */
    while (*gds).tokenp < lasttoken {
        if phrase(gds) == 0 {
            return -(1 as libc::c_int) as time_t;
        }
    }
    /* Use current local timezone if none was specified. */
    if (*gds).HaveZone == 0 {
        (*gds).Timezone = tzone;
        (*gds).DSTmode = DSTmaybe
    }
    /* If a timezone was specified, use that for generating the default
     * time components instead of the local timezone. */
    if (*gds).HaveZone != 0 && !gmt_ptr.is_null() {
        now -= (*gds).Timezone;
        gmt_ptr = gmtime_r(&mut now, &mut gmt);
        if !gmt_ptr.is_null() {
            local = *gmt_ptr
        }
        now += (*gds).Timezone
    }
    if (*gds).HaveYear == 0 {
        (*gds).Year = (local.tm_year + 1900 as libc::c_int) as time_t
    }
    if (*gds).HaveMonth == 0 {
        (*gds).Month = (local.tm_mon + 1 as libc::c_int) as time_t
    }
    if (*gds).HaveDay == 0 {
        (*gds).Day = local.tm_mday as time_t
    }
    /* Note: No default for hour/min/sec; a specifier that just
     * gives date always refers to 00:00 on that date. */
    /* If we saw more than one time, timezone, weekday, year, month,
     * or day, then give up. */
    if (*gds).HaveTime > 1 as libc::c_int
        || (*gds).HaveZone > 1 as libc::c_int
        || (*gds).HaveWeekDay > 1 as libc::c_int
        || (*gds).HaveYear > 1 as libc::c_int
        || (*gds).HaveMonth > 1 as libc::c_int
        || (*gds).HaveDay > 1 as libc::c_int
    {
        return -(1 as libc::c_int) as time_t;
    }
    /* Compute an absolute time based on whatever absolute information
     * we collected. */
    if (*gds).HaveYear != 0
        || (*gds).HaveMonth != 0
        || (*gds).HaveDay != 0
        || (*gds).HaveTime != 0
        || (*gds).HaveWeekDay != 0
    {
        Start = Convert(
            (*gds).Month,
            (*gds).Day,
            (*gds).Year,
            (*gds).Hour,
            (*gds).Minutes,
            (*gds).Seconds,
            (*gds).Timezone,
            (*gds).DSTmode,
        );
        if Start < 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int) as time_t;
        }
    } else {
        Start = now;
        if (*gds).HaveRel == 0 {
            Start -= local.tm_hour as libc::c_long * HOUR
                + local.tm_min as libc::c_long * MINUTE
                + local.tm_sec as libc::c_long
        }
    }
    /* Add the relative offset. */
    Start += (*gds).RelSeconds;
    Start += RelativeMonth(Start, (*gds).Timezone, (*gds).RelMonth);
    /* Adjust for day-of-week offsets. */
    if (*gds).HaveWeekDay != 0
        && !((*gds).HaveYear != 0 || (*gds).HaveMonth != 0 || (*gds).HaveDay != 0)
    {
        tod = RelativeDate(
            Start,
            (*gds).Timezone,
            (*gds).DSTmode as libc::c_int,
            (*gds).DayOrdinal,
            (*gds).DayNumber,
        );
        Start += tod
    }
    /* -1 is an error indicator, so return 0 instead of -1 if
     * that's the actual time. */
    return if Start == -(1 as libc::c_int) as libc::c_long {
        0 as libc::c_int as libc::c_long
    } else {
        Start
    };
}
/* defined(TEST) */
