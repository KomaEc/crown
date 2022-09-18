use ::libc;
extern "C" {
    pub type internal_state;
    pub type archive_string_conv;
    /*-
     * Copyright (c) 2011 Michihiro NAKAJIMA
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
    pub type archive_entry;
    pub type hmac_ctx_st;
    pub type evp_cipher_st;
    pub type evp_cipher_ctx_st;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn deflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn deflateInit2_(
        strm: z_streamp,
        level: libc::c_int,
        method: libc::c_int,
        windowBits: libc::c_int,
        memLevel: libc::c_int,
        strategy: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
    /* Minimal interface to cryptographic functionality for internal use in
     * libarchive */
    /* PKCS5 PBKDF2 HMAC-SHA1 */
    /* AES CTR mode(little endian version) */
    #[no_mangle]
    static __archive_cryptor: archive_cryptor;
    /* The 'clone' function does a deep copy; all of the strings are copied too. */
    #[no_mangle]
    fn archive_entry_clone(_: *mut archive_entry) -> *mut archive_entry;
    #[no_mangle]
    fn archive_entry_free(_: *mut archive_entry);
    /*
     * Retrieve fields from an archive_entry.
     *
     * There are a number of implicit conversions among these fields.  For
     * example, if a regular string field is set and you read the _w wide
     * character field, the entry will implicitly convert narrow-to-wide
     * using the current locale.  Similarly, dev values are automatically
     * updated when you write devmajor or devminor and vice versa.
     *
     * In addition, fields can be "set" or "unset."  Unset string fields
     * return NULL, non-string fields have _is_set() functions to test
     * whether they've been set.  You can "unset" a string field by
     * assigning NULL; non-string fields have _unset() functions to
     * unset them.
     *
     * Note: There is one ambiguity in the above; string fields will
     * also return NULL when implicit character set conversions fail.
     * This is usually what you want.
     */
    #[no_mangle]
    fn archive_entry_atime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_atime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_ctime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_ctime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_filetype(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_gid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_mode(_: *mut archive_entry) -> mode_t;
    #[no_mangle]
    fn archive_entry_mtime(_: *mut archive_entry) -> time_t;
    #[no_mangle]
    fn archive_entry_mtime_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_pathname(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_size(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_size_is_set(_: *mut archive_entry) -> libc::c_int;
    #[no_mangle]
    fn archive_entry_symlink(_: *mut archive_entry) -> *const libc::c_char;
    #[no_mangle]
    fn archive_entry_uid(_: *mut archive_entry) -> la_int64_t;
    #[no_mangle]
    fn archive_entry_set_pathname(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn archive_entry_set_size(_: *mut archive_entry, _: la_int64_t);
    #[no_mangle]
    fn archive_entry_set_symlink(_: *mut archive_entry, _: *const libc::c_char);
    #[no_mangle]
    fn _archive_entry_pathname_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    fn _archive_entry_symlink_l(
        _: *mut archive_entry,
        _: *mut *const libc::c_char,
        _: *mut size_t,
        _: *mut archive_string_conv,
    ) -> libc::c_int;
    #[no_mangle]
    static __archive_hmac: archive_hmac;
    #[no_mangle]
    fn archive_string_conversion_charset_name(_: *mut archive_string_conv) -> *const libc::c_char;
    #[no_mangle]
    fn __archive_check_magic(
        _: *mut archive,
        magic: libc::c_uint,
        state: libc::c_uint,
        func: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn archive_string_conversion_to_charset(
        _: *mut archive,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> *mut archive_string_conv;
    #[no_mangle]
    fn archive_string_default_conversion_for_write(_: *mut archive) -> *mut archive_string_conv;
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
    /* Random number generator. */
    #[no_mangle]
    fn archive_random(buf: *mut libc::c_void, nbytes: size_t) -> libc::c_int;
    #[no_mangle]
    fn __archive_write_output(
        _: *mut archive_write,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    /*
     * Get a encryption passphrase.
     */
    #[no_mangle]
    fn __archive_write_get_passphrase(a: *mut archive_write) -> *const libc::c_char;
    /*-
     * Copyright (c) 2020 Martin Matuska
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
    #[no_mangle]
    fn __archive_write_entry_filetype_unsupported(
        a: *mut archive,
        entry: *mut archive_entry,
        format: *const libc::c_char,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type nl_item = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed = 327684;
pub const __NOSTR: C2RustUnnamed = 327683;
pub const __YESSTR: C2RustUnnamed = 327682;
pub const __NOEXPR: C2RustUnnamed = 327681;
pub const __YESEXPR: C2RustUnnamed = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed = 65539;
pub const __GROUPING: C2RustUnnamed = 65538;
pub const THOUSEP: C2RustUnnamed = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed = 65537;
pub const RADIXCHAR: C2RustUnnamed = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed = 262149;
pub const __MON_GROUPING: C2RustUnnamed = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed = 15;
pub const CODESET: C2RustUnnamed = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed = 131195;
pub const __ALTMON_12: C2RustUnnamed = 131194;
pub const __ALTMON_11: C2RustUnnamed = 131193;
pub const __ALTMON_10: C2RustUnnamed = 131192;
pub const __ALTMON_9: C2RustUnnamed = 131191;
pub const __ALTMON_8: C2RustUnnamed = 131190;
pub const __ALTMON_7: C2RustUnnamed = 131189;
pub const __ALTMON_6: C2RustUnnamed = 131188;
pub const __ALTMON_5: C2RustUnnamed = 131187;
pub const __ALTMON_4: C2RustUnnamed = 131186;
pub const __ALTMON_3: C2RustUnnamed = 131185;
pub const __ALTMON_2: C2RustUnnamed = 131184;
pub const __ALTMON_1: C2RustUnnamed = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed = 131181;
pub const _DATE_FMT: C2RustUnnamed = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed = 131167;
pub const _NL_WT_FMT: C2RustUnnamed = 131166;
pub const _NL_WD_FMT: C2RustUnnamed = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed = 131164;
pub const _NL_WPM_STR: C2RustUnnamed = 131163;
pub const _NL_WAM_STR: C2RustUnnamed = 131162;
pub const _NL_WMON_12: C2RustUnnamed = 131161;
pub const _NL_WMON_11: C2RustUnnamed = 131160;
pub const _NL_WMON_10: C2RustUnnamed = 131159;
pub const _NL_WMON_9: C2RustUnnamed = 131158;
pub const _NL_WMON_8: C2RustUnnamed = 131157;
pub const _NL_WMON_7: C2RustUnnamed = 131156;
pub const _NL_WMON_6: C2RustUnnamed = 131155;
pub const _NL_WMON_5: C2RustUnnamed = 131154;
pub const _NL_WMON_4: C2RustUnnamed = 131153;
pub const _NL_WMON_3: C2RustUnnamed = 131152;
pub const _NL_WMON_2: C2RustUnnamed = 131151;
pub const _NL_WMON_1: C2RustUnnamed = 131150;
pub const _NL_WABMON_12: C2RustUnnamed = 131149;
pub const _NL_WABMON_11: C2RustUnnamed = 131148;
pub const _NL_WABMON_10: C2RustUnnamed = 131147;
pub const _NL_WABMON_9: C2RustUnnamed = 131146;
pub const _NL_WABMON_8: C2RustUnnamed = 131145;
pub const _NL_WABMON_7: C2RustUnnamed = 131144;
pub const _NL_WABMON_6: C2RustUnnamed = 131143;
pub const _NL_WABMON_5: C2RustUnnamed = 131142;
pub const _NL_WABMON_4: C2RustUnnamed = 131141;
pub const _NL_WABMON_3: C2RustUnnamed = 131140;
pub const _NL_WABMON_2: C2RustUnnamed = 131139;
pub const _NL_WABMON_1: C2RustUnnamed = 131138;
pub const _NL_WDAY_7: C2RustUnnamed = 131137;
pub const _NL_WDAY_6: C2RustUnnamed = 131136;
pub const _NL_WDAY_5: C2RustUnnamed = 131135;
pub const _NL_WDAY_4: C2RustUnnamed = 131134;
pub const _NL_WDAY_3: C2RustUnnamed = 131133;
pub const _NL_WDAY_2: C2RustUnnamed = 131132;
pub const _NL_WDAY_1: C2RustUnnamed = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed = 131122;
pub const ERA_T_FMT: C2RustUnnamed = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed = 131120;
pub const ALT_DIGITS: C2RustUnnamed = 131119;
pub const ERA_D_FMT: C2RustUnnamed = 131118;
pub const __ERA_YEAR: C2RustUnnamed = 131117;
pub const ERA: C2RustUnnamed = 131116;
pub const T_FMT_AMPM: C2RustUnnamed = 131115;
pub const T_FMT: C2RustUnnamed = 131114;
pub const D_FMT: C2RustUnnamed = 131113;
pub const D_T_FMT: C2RustUnnamed = 131112;
pub const PM_STR: C2RustUnnamed = 131111;
pub const AM_STR: C2RustUnnamed = 131110;
pub const MON_12: C2RustUnnamed = 131109;
pub const MON_11: C2RustUnnamed = 131108;
pub const MON_10: C2RustUnnamed = 131107;
pub const MON_9: C2RustUnnamed = 131106;
pub const MON_8: C2RustUnnamed = 131105;
pub const MON_7: C2RustUnnamed = 131104;
pub const MON_6: C2RustUnnamed = 131103;
pub const MON_5: C2RustUnnamed = 131102;
pub const MON_4: C2RustUnnamed = 131101;
pub const MON_3: C2RustUnnamed = 131100;
pub const MON_2: C2RustUnnamed = 131099;
pub const MON_1: C2RustUnnamed = 131098;
pub const ABMON_12: C2RustUnnamed = 131097;
pub const ABMON_11: C2RustUnnamed = 131096;
pub const ABMON_10: C2RustUnnamed = 131095;
pub const ABMON_9: C2RustUnnamed = 131094;
pub const ABMON_8: C2RustUnnamed = 131093;
pub const ABMON_7: C2RustUnnamed = 131092;
pub const ABMON_6: C2RustUnnamed = 131091;
pub const ABMON_5: C2RustUnnamed = 131090;
pub const ABMON_4: C2RustUnnamed = 131089;
pub const ABMON_3: C2RustUnnamed = 131088;
pub const ABMON_2: C2RustUnnamed = 131087;
pub const ABMON_1: C2RustUnnamed = 131086;
pub const DAY_7: C2RustUnnamed = 131085;
pub const DAY_6: C2RustUnnamed = 131084;
pub const DAY_5: C2RustUnnamed = 131083;
pub const DAY_4: C2RustUnnamed = 131082;
pub const DAY_3: C2RustUnnamed = 131081;
pub const DAY_2: C2RustUnnamed = 131080;
pub const DAY_1: C2RustUnnamed = 131079;
pub const ABDAY_7: C2RustUnnamed = 131078;
pub const ABDAY_6: C2RustUnnamed = 131077;
pub const ABDAY_5: C2RustUnnamed = 131076;
pub const ABDAY_4: C2RustUnnamed = 131075;
pub const ABDAY_3: C2RustUnnamed = 131074;
pub const ABDAY_2: C2RustUnnamed = 131073;
pub const ABDAY_1: C2RustUnnamed = 131072;
pub type size_t = libc::c_ulong;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
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
pub type la_int64_t = int64_t;
pub type la_ssize_t = ssize_t;
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
 * $FreeBSD: head/lib/libarchive/archive_private.h 201098 2009-12-28 02:58:14Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive {
    pub magic: libc::c_uint,
    pub state: libc::c_uint,
    pub vtable: *mut archive_vtable,
    pub archive_format: libc::c_int,
    pub archive_format_name: *const libc::c_char,
    pub compression_code: libc::c_int,
    pub compression_name: *const libc::c_char,
    pub file_count: libc::c_int,
    pub archive_error_number: libc::c_int,
    pub error: *const libc::c_char,
    pub error_string: archive_string,
    pub current_code: *mut libc::c_char,
    pub current_codepage: libc::c_uint,
    pub current_oemcp: libc::c_uint,
    pub sconv: *mut archive_string_conv,
    pub read_data_block: *const libc::c_char,
    pub read_data_offset: int64_t,
    pub read_data_output_offset: int64_t,
    pub read_data_remaining: size_t,
    pub read_data_is_posix_read: libc::c_char,
    pub read_data_requested: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_string {
    pub s: *mut libc::c_char,
    pub length: size_t,
    pub buffer_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_vtable {
    pub archive_close: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_free: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_write_finish_entry: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_write_data:
        Option<unsafe extern "C" fn(_: *mut archive, _: *const libc::c_void, _: size_t) -> ssize_t>,
    pub archive_write_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *const libc::c_void,
            _: size_t,
            _: int64_t,
        ) -> ssize_t,
    >,
    pub archive_read_next_header:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut *mut archive_entry) -> libc::c_int>,
    pub archive_read_next_header2:
        Option<unsafe extern "C" fn(_: *mut archive, _: *mut archive_entry) -> libc::c_int>,
    pub archive_read_data_block: Option<
        unsafe extern "C" fn(
            _: *mut archive,
            _: *mut *const libc::c_void,
            _: *mut size_t,
            _: *mut int64_t,
        ) -> libc::c_int,
    >,
    pub archive_filter_count: Option<unsafe extern "C" fn(_: *mut archive) -> libc::c_int>,
    pub archive_filter_bytes:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> int64_t>,
    pub archive_filter_code:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> libc::c_int>,
    pub archive_filter_name:
        Option<unsafe extern "C" fn(_: *mut archive, _: libc::c_int) -> *const libc::c_char>,
}
pub type archive_write_callback = unsafe extern "C" fn(
    _: *mut archive,
    _: *mut libc::c_void,
    _: *const libc::c_void,
    _: size_t,
) -> la_ssize_t;
pub type archive_open_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_close_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> libc::c_int;
pub type archive_passphrase_callback =
    unsafe extern "C" fn(_: *mut archive, _: *mut libc::c_void) -> *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write {
    pub archive: archive,
    pub skip_file_set: libc::c_int,
    pub skip_file_dev: int64_t,
    pub skip_file_ino: int64_t,
    pub nulls: *const libc::c_uchar,
    pub null_length: size_t,
    pub client_opener: Option<archive_open_callback>,
    pub client_writer: Option<archive_write_callback>,
    pub client_closer: Option<archive_close_callback>,
    pub client_data: *mut libc::c_void,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub filter_first: *mut archive_write_filter,
    pub filter_last: *mut archive_write_filter,
    pub format_data: *mut libc::c_void,
    pub format_name: *const libc::c_char,
    pub format_init: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub format_finish_entry: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_write_header:
        Option<unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int>,
    pub format_write_data: Option<
        unsafe extern "C" fn(_: *mut archive_write, _: *const libc::c_void, _: size_t) -> ssize_t,
    >,
    pub format_close: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub format_free: Option<unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int>,
    pub passphrase: *mut libc::c_char,
    pub passphrase_callback: Option<archive_passphrase_callback>,
    pub passphrase_client_data: *mut libc::c_void,
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
 * $FreeBSD: head/lib/libarchive/archive_write_private.h 201155 2009-12-29 05:20:12Z kientzle $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_write_filter {
    pub bytes_written: int64_t,
    pub archive: *mut archive,
    pub next_filter: *mut archive_write_filter,
    pub options: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_char,
            _: *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub open: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub write: Option<
        unsafe extern "C" fn(
            _: *mut archive_write_filter,
            _: *const libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub free: Option<unsafe extern "C" fn(_: *mut archive_write_filter) -> libc::c_int>,
    pub data: *mut libc::c_void,
    pub name: *const libc::c_char,
    pub code: libc::c_int,
    pub bytes_per_block: libc::c_int,
    pub bytes_in_last_block: libc::c_int,
    pub state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip {
    pub entry_offset: int64_t,
    pub entry_compressed_size: int64_t,
    pub entry_uncompressed_size: int64_t,
    pub entry_compressed_written: int64_t,
    pub entry_uncompressed_written: int64_t,
    pub entry_uncompressed_limit: int64_t,
    pub entry: *mut archive_entry,
    pub entry_crc32: uint32_t,
    pub entry_compression: compression,
    pub entry_encryption: encryption,
    pub entry_flags: libc::c_int,
    pub entry_uses_zip64: libc::c_int,
    pub experiments: libc::c_int,
    pub tctx: trad_enc_ctx,
    pub tctx_valid: libc::c_char,
    pub trad_chkdat: libc::c_uchar,
    pub aes_vendor: libc::c_uint,
    pub cctx: archive_crypto_ctx,
    pub cctx_valid: libc::c_char,
    pub hctx: archive_hmac_sha1_ctx,
    pub hctx_valid: libc::c_char,
    pub file_header: *mut libc::c_uchar,
    pub file_header_extra_offset: size_t,
    pub crc32func: Option<
        unsafe extern "C" fn(_: libc::c_ulong, _: *const libc::c_void, _: size_t) -> libc::c_ulong,
    >,
    pub central_directory: *mut cd_segment,
    pub central_directory_last: *mut cd_segment,
    pub central_directory_bytes: size_t,
    pub central_directory_entries: size_t,
    pub written_bytes: int64_t,
    pub opt_sconv: *mut archive_string_conv,
    pub sconv_default: *mut archive_string_conv,
    pub requested_compression: compression,
    pub deflate_compression_level: libc::c_int,
    pub init_default_conversion: libc::c_int,
    pub encryption_type: encryption,
    pub flags: libc::c_int,
    pub stream: z_stream,
    pub len_buf: size_t,
    pub buf: *mut libc::c_uchar,
}
pub type encryption = libc::c_uint;
/* WinZIP AES-256 encryption. */
/* WinZIP AES-128 encryption. */
pub const ENCRYPTION_WINZIP_AES256: encryption = 3;
/* Traditional PKWARE encryption. */
pub const ENCRYPTION_WINZIP_AES128: encryption = 2;
pub const ENCRYPTION_TRADITIONAL: encryption = 1;
pub const ENCRYPTION_NONE: encryption = 0;
pub type compression = libc::c_int;
pub const COMPRESSION_DEFLATE: compression = 8;
pub const COMPRESSION_STORE: compression = 0;
pub const COMPRESSION_UNSPECIFIED: compression = -1;
/* */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cd_segment {
    pub next: *mut cd_segment,
    pub buff_size: size_t,
    pub buff: *mut libc::c_uchar,
    pub p: *mut libc::c_uchar,
}
pub type archive_hmac_sha1_ctx = *mut HMAC_CTX;
pub type HMAC_CTX = hmac_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_crypto_ctx {
    pub ctx: *mut EVP_CIPHER_CTX,
    pub type_0: *const EVP_CIPHER,
    pub key: [uint8_t; 32],
    pub key_len: libc::c_uint,
    pub nonce: [uint8_t; 16],
    pub encr_buf: [uint8_t; 16],
    pub encr_pos: libc::c_uint,
}
pub type EVP_CIPHER = evp_cipher_st;
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trad_enc_ctx {
    pub keys: [uint32_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_hmac {
    pub __hmac_sha1_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_hmac_sha1_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub __hmac_sha1_update: Option<
        unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx, _: *const uint8_t, _: size_t) -> (),
    >,
    pub __hmac_sha1_final: Option<
        unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx, _: *mut uint8_t, _: *mut size_t) -> (),
    >,
    pub __hmac_sha1_cleanup: Option<unsafe extern "C" fn(_: *mut archive_hmac_sha1_ctx) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_cryptor {
    pub pbkdf2sha1: Option<
        unsafe extern "C" fn(
            _: *const libc::c_char,
            _: size_t,
            _: *const uint8_t,
            _: size_t,
            _: libc::c_uint,
            _: *mut uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_update: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
            _: *mut uint8_t,
            _: *mut size_t,
        ) -> libc::c_int,
    >,
    pub decrypto_aes_ctr_release:
        Option<unsafe extern "C" fn(_: *mut archive_crypto_ctx) -> libc::c_int>,
    pub encrypto_aes_ctr_init: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub encrypto_aes_ctr_update: Option<
        unsafe extern "C" fn(
            _: *mut archive_crypto_ctx,
            _: *const uint8_t,
            _: size_t,
            _: *mut uint8_t,
            _: *mut size_t,
        ) -> libc::c_int,
    >,
    pub encrypto_aes_ctr_release:
        Option<unsafe extern "C" fn(_: *mut archive_crypto_ctx) -> libc::c_int>,
}
pub const INT64_MAX: libc::c_long = 9223372036854775807 as libc::c_long;
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
 * $FreeBSD: head/lib/libarchive/archive_platform.h 201090 2009-12-28 02:22:04Z kientzle $
 */
/* !!ONLY FOR USE INTERNALLY TO LIBARCHIVE!! */
/*
 * This header is the first thing included in any of the libarchive
 * source files.  As far as possible, platform-specific issues should
 * be dealt with here and not within individual source files.  I'm
 * actively trying to minimize #if blocks within the main source,
 * since they obfuscate the code.
 */
/* archive.h and archive_entry.h require this. */
/* Most POSIX platforms use the 'configure' script to build config.h */
/* On macOS check for some symbols based on the deployment target version.  */
/* It should be possible to get rid of this by extending the feature-test
 * macros to cover Windows API functions, probably along with non-trivial
 * refactoring of code to find structures that sit more cleanly on top of
 * either Windows or Posix APIs. */
/*
 * The config files define a lot of feature macros.  The following
 * uses those macros to select/define replacements and include key
 * headers as required.
 */
/* Get a real definition for __FBSDID or __RCSID if we can */
/* If not, define them so as to avoid dangling semicolons. */
/* Try to get standard C99-style integer type definitions. */
/* Borland warns about its own constants!  */
/* Some platforms lack the standard *_MAX definitions. */
/*
 * If we can't restore metadata using a file descriptor, then
 * for compatibility's sake, close files before trying to restore metadata.
 */
/*
 * glibc 2.24 deprecates readdir_r
 */
/* Set up defaults for internal error codes. */
pub const ARCHIVE_ERRNO_FILE_FORMAT: libc::c_int = EILSEQ;
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const errno: libc::c_int = *__errno_location();
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const CODESET_0: libc::c_int = CODESET as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const ZLIB_VERSION: [libc::c_char; 7] =
    unsafe { *::std::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b"1.2.11\x00") };
pub const Z_NO_FLUSH: libc::c_int = 0 as libc::c_int;
pub const Z_FINISH: libc::c_int = 4 as libc::c_int;
pub const Z_OK: libc::c_int = 0 as libc::c_int;
pub const Z_STREAM_ERROR: libc::c_int = -(2 as libc::c_int);
pub const Z_DEFAULT_COMPRESSION: libc::c_int = -(1 as libc::c_int);
pub const Z_NULL: libc::c_int = 0 as libc::c_int;
pub const ARCHIVE_WARN: libc::c_int = -(20 as libc::c_int);
pub const ARCHIVE_FAILED: libc::c_int = -(25 as libc::c_int);
pub const ARCHIVE_FATAL: libc::c_int = -(30 as libc::c_int);
pub const ARCHIVE_FORMAT_ZIP: libc::c_int = 0x50000 as libc::c_int;
pub const ARCHIVE_OK: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn archive_le16dec(mut pp: *const libc::c_void) -> uint16_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return (p1 << 8 as libc::c_int | p0) as uint16_t;
}
#[inline]
unsafe extern "C" fn archive_le16enc(mut pp: *mut libc::c_void, mut u: uint16_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_le32enc(mut pp: *mut libc::c_void, mut u: uint32_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(2 as libc::c_int as isize) =
        (u >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    *p.offset(3 as libc::c_int as isize) =
        (u >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_le64enc(mut pp: *mut libc::c_void, mut u: uint64_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    archive_le32enc(
        p as *mut libc::c_void,
        (u & 0xffffffff as libc::c_uint as libc::c_ulong) as uint32_t,
    );
    archive_le32enc(
        p.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        (u >> 32 as libc::c_int) as uint32_t,
    );
}
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
pub const AE_IFLNK: libc::c_int = 0o120000 as libc::c_int;
pub const AE_IFDIR: libc::c_int = 0o40000 as libc::c_int;
pub const archive_entry_pathname_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_pathname_l;
pub const archive_entry_symlink_l: unsafe extern "C" fn(
    _: *mut archive_entry,
    _: *mut *const libc::c_char,
    _: *mut size_t,
    _: *mut archive_string_conv,
) -> libc::c_int = _archive_entry_symlink_l;
pub const ZIP_ENTRY_FLAG_ENCRYPTED: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
pub const ZIP_ENTRY_FLAG_LENGTH_AT_END: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
pub const ZIP_ENTRY_FLAG_UTF8_NAME: libc::c_int = (1 as libc::c_int) << 11 as libc::c_int;
pub const COMPRESSION_DEFAULT: libc::c_int = COMPRESSION_DEFLATE as libc::c_int;
pub const TRAD_HEADER_SIZE: libc::c_int = 12 as libc::c_int;
/*
 * See "WinZip - AES Encryption Information"
 *     http://www.winzip.com/aes_info.htm
 */
/* Value used in compression method. */
pub const WINZIP_AES_ENCRYPTION: libc::c_int = 99 as libc::c_int;
/* A WinZip AES header size which is stored at the beginning of
 * file contents. */
pub const WINZIP_AES128_HEADER_SIZE: libc::c_int = 8 as libc::c_int + 2 as libc::c_int;
pub const WINZIP_AES256_HEADER_SIZE: libc::c_int = 16 as libc::c_int + 2 as libc::c_int;
/* AES vendor version. */
pub const AES_VENDOR_AE_1: libc::c_int = 0x1 as libc::c_int;
pub const AES_VENDOR_AE_2: libc::c_int = 0x2 as libc::c_int;
/* Authentication code size. */
pub const AUTH_CODE_SIZE: libc::c_int = 10 as libc::c_int;
pub const ZIP_FLAG_AVOID_ZIP64: libc::c_int = 1 as libc::c_int;
pub const ZIP_FLAG_FORCE_ZIP64: libc::c_int = 2 as libc::c_int;
pub const ZIP_FLAG_EXPERIMENT_xl: libc::c_int = 4 as libc::c_int;
unsafe extern "C" fn cd_alloc(mut zip: *mut zip, mut length: size_t) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*zip).central_directory.is_null()
        || (*(*zip).central_directory_last).p.offset(length as isize)
            > (*(*zip).central_directory_last)
                .buff
                .offset((*(*zip).central_directory_last).buff_size as isize)
    {
        let mut segment: *mut cd_segment = calloc(
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<cd_segment>() as libc::c_ulong,
        ) as *mut cd_segment;
        if segment.is_null() {
            return NULL as *mut libc::c_uchar;
        }
        (*segment).buff_size = (64 as libc::c_int * 1024 as libc::c_int) as size_t;
        (*segment).buff = malloc((*segment).buff_size) as *mut libc::c_uchar;
        if (*segment).buff.is_null() {
            free(segment as *mut libc::c_void);
            return NULL as *mut libc::c_uchar;
        }
        (*segment).p = (*segment).buff;
        if (*zip).central_directory.is_null() {
            (*zip).central_directory_last = segment;
            (*zip).central_directory = (*zip).central_directory_last
        } else {
            (*(*zip).central_directory_last).next = segment;
            (*zip).central_directory_last = segment
        }
    }
    p = (*(*zip).central_directory_last).p;
    (*(*zip).central_directory_last).p = (*(*zip).central_directory_last).p.offset(length as isize);
    (*zip).central_directory_bytes =
        ((*zip).central_directory_bytes as libc::c_ulong).wrapping_add(length) as size_t as size_t;
    return p;
}
unsafe extern "C" fn real_crc32(
    mut crc: libc::c_ulong,
    mut buff: *const libc::c_void,
    mut len: size_t,
) -> libc::c_ulong {
    return crc32(crc, buff as *const Bytef, len as libc::c_uint);
}
unsafe extern "C" fn fake_crc32(
    mut crc: libc::c_ulong,
    mut buff: *const libc::c_void,
    mut len: size_t,
) -> libc::c_ulong {
    /* UNUSED */
    return 0 as libc::c_int as libc::c_ulong;
}
unsafe extern "C" fn archive_write_zip_options(
    mut a: *mut archive_write,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
) -> libc::c_int {
    let mut zip: *mut zip = (*a).format_data as *mut zip;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    if strcmp(key, b"compression\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        /*
         * Set compression to use on all future entries.
         * This only affects regular files.
         */
        if val.is_null()
            || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"%s: compression option needs a compression name\x00" as *const u8
                    as *const libc::c_char,
                (*a).format_name,
            );
        } else if strcmp(val, b"deflate\x00" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            (*zip).requested_compression = COMPRESSION_DEFLATE;
            ret = ARCHIVE_OK
        } else if strcmp(val, b"store\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int
        {
            (*zip).requested_compression = COMPRESSION_STORE;
            ret = ARCHIVE_OK
        }
        return ret;
    } else {
        if strcmp(
            key,
            b"compression-level\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if val.is_null()
                || !(*val.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
                    && *val.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32)
                || *val.offset(1 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
            {
                return ARCHIVE_WARN;
            }
            if *val.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
                (*zip).requested_compression = COMPRESSION_STORE;
                return ARCHIVE_OK;
            } else {
                (*zip).requested_compression = COMPRESSION_DEFLATE;
                (*zip).deflate_compression_level =
                    *val.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
                return ARCHIVE_OK;
            }
        } else {
            if strcmp(key, b"encryption\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                if val.is_null() {
                    (*zip).encryption_type = ENCRYPTION_NONE;
                    ret = ARCHIVE_OK
                } else if *val.offset(0 as libc::c_int as isize) as libc::c_int == '1' as i32
                    || strcmp(val, b"traditional\x00" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(val, b"zipcrypt\x00" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    || strcmp(val, b"ZipCrypt\x00" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    if is_traditional_pkware_encryption_supported() != 0 {
                        (*zip).encryption_type = ENCRYPTION_TRADITIONAL;
                        ret = ARCHIVE_OK
                    } else {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"encryption not supported\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                } else if strcmp(val, b"aes128\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if is_winzip_aes_encryption_supported(ENCRYPTION_WINZIP_AES128 as libc::c_int)
                        != 0
                    {
                        (*zip).encryption_type = ENCRYPTION_WINZIP_AES128;
                        ret = ARCHIVE_OK
                    } else {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"encryption not supported\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                } else if strcmp(val, b"aes256\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if is_winzip_aes_encryption_supported(ENCRYPTION_WINZIP_AES256 as libc::c_int)
                        != 0
                    {
                        (*zip).encryption_type = ENCRYPTION_WINZIP_AES256;
                        ret = ARCHIVE_OK
                    } else {
                        archive_set_error(
                            &mut (*a).archive as *mut archive,
                            ARCHIVE_ERRNO_MISC,
                            b"encryption not supported\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"%s: unknown encryption \'%s\'\x00" as *const u8 as *const libc::c_char,
                        (*a).format_name,
                        val,
                    );
                }
                return ret;
            } else {
                if strcmp(key, b"experimental\x00" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    if val.is_null()
                        || *val.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                    {
                        (*zip).flags &= !ZIP_FLAG_EXPERIMENT_xl
                    } else {
                        (*zip).flags |= ZIP_FLAG_EXPERIMENT_xl
                    }
                    return 0 as libc::c_int;
                } else {
                    if strcmp(key, b"fakecrc32\x00" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                    {
                        /*
                         * FOR TESTING ONLY:  disable CRC calculation to speed up
                         * certain complex tests.
                         */
                        if val.is_null()
                            || *val.offset(0 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                        {
                            (*zip).crc32func = Some(
                                real_crc32
                                    as unsafe extern "C" fn(
                                        _: libc::c_ulong,
                                        _: *const libc::c_void,
                                        _: size_t,
                                    )
                                        -> libc::c_ulong,
                            )
                        } else {
                            (*zip).crc32func = Some(
                                fake_crc32
                                    as unsafe extern "C" fn(
                                        _: libc::c_ulong,
                                        _: *const libc::c_void,
                                        _: size_t,
                                    )
                                        -> libc::c_ulong,
                            )
                        }
                        return 0 as libc::c_int;
                    } else {
                        if strcmp(key, b"hdrcharset\x00" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            /*
                             * Set the character set used in translating filenames.
                             */
                            if val.is_null()
                                || *val.offset(0 as libc::c_int as isize) as libc::c_int
                                    == 0 as libc::c_int
                            {
                                archive_set_error(
                                    &mut (*a).archive as *mut archive,
                                    ARCHIVE_ERRNO_MISC,
                                    b"%s: hdrcharset option needs a character-set name\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                    (*a).format_name,
                                );
                            } else {
                                (*zip).opt_sconv = archive_string_conversion_to_charset(
                                    &mut (*a).archive,
                                    val,
                                    0 as libc::c_int,
                                );
                                if !(*zip).opt_sconv.is_null() {
                                    ret = ARCHIVE_OK
                                } else {
                                    ret = ARCHIVE_FATAL
                                }
                            }
                            return ret;
                        } else {
                            if strcmp(key, b"zip64\x00" as *const u8 as *const libc::c_char)
                                == 0 as libc::c_int
                            {
                                /*
                                 * Bias decisions about Zip64: force them to be
                                 * generated in certain cases where they are not
                                 * forbidden or avoid them in certain cases where they
                                 * are not strictly required.
                                 */
                                if !val.is_null() && *val as libc::c_int != '\u{0}' as i32 {
                                    (*zip).flags |= ZIP_FLAG_FORCE_ZIP64;
                                    (*zip).flags &= !ZIP_FLAG_AVOID_ZIP64
                                } else {
                                    (*zip).flags &= !ZIP_FLAG_FORCE_ZIP64;
                                    (*zip).flags |= ZIP_FLAG_AVOID_ZIP64
                                }
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    /* Note: The "warn" return is just to inform the options
     * supervisor that we didn't handle it.  It will generate
     * a suitable error if no one used this option. */
    return -(20 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_zip_set_compression_deflate(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint | 2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_write_zip_set_compression_deflate\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.archive_format != ARCHIVE_FORMAT_ZIP {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Can only use archive_write_zip_set_compression_deflate with zip format\x00"
                as *const u8 as *const libc::c_char,
        );
        ret = ARCHIVE_FATAL
    } else {
        let mut zip: *mut zip = (*a).format_data as *mut zip;
        (*zip).requested_compression = COMPRESSION_DEFLATE;
        ret = ARCHIVE_OK
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_zip_set_compression_store(
    mut _a: *mut archive,
) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut zip: *mut zip = (*a).format_data as *mut zip;
    let mut ret: libc::c_int = ARCHIVE_FAILED;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint | 2 as libc::c_uint | 4 as libc::c_uint,
        b"archive_write_zip_set_compression_deflate\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    if (*a).archive.archive_format != ARCHIVE_FORMAT_ZIP {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Can only use archive_write_zip_set_compression_store with zip format\x00" as *const u8
                as *const libc::c_char,
        );
        ret = ARCHIVE_FATAL
    } else {
        (*zip).requested_compression = COMPRESSION_STORE;
        ret = ARCHIVE_OK
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_set_format_zip(mut _a: *mut archive) -> libc::c_int {
    let mut a: *mut archive_write = _a as *mut archive_write;
    let mut zip: *mut zip = 0 as *mut zip;
    let mut magic_test: libc::c_int = __archive_check_magic(
        _a,
        0xb0c5c0de as libc::c_uint,
        1 as libc::c_uint,
        b"archive_write_set_format_zip\x00" as *const u8 as *const libc::c_char,
    );
    if magic_test == ARCHIVE_FATAL {
        return ARCHIVE_FATAL;
    }
    /* If another format was already registered, unregister it. */
    if (*a).format_free.is_some() {
        (*a).format_free.expect("non-null function pointer")(a);
    }
    zip = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<zip>() as libc::c_ulong,
    ) as *mut zip;
    if zip.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate zip data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    /* "Unspecified" lets us choose the appropriate compression. */
    (*zip).requested_compression = COMPRESSION_UNSPECIFIED;
    (*zip).deflate_compression_level = Z_DEFAULT_COMPRESSION;
    (*zip).crc32func = Some(
        real_crc32
            as unsafe extern "C" fn(
                _: libc::c_ulong,
                _: *const libc::c_void,
                _: size_t,
            ) -> libc::c_ulong,
    );
    /* A buffer used for both compression and encryption. */
    (*zip).len_buf = 65536 as libc::c_int as size_t;
    (*zip).buf = malloc((*zip).len_buf) as *mut libc::c_uchar;
    if (*zip).buf.is_null() {
        free(zip as *mut libc::c_void);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate compression buffer\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    (*a).format_data = zip as *mut libc::c_void;
    (*a).format_name = b"zip\x00" as *const u8 as *const libc::c_char;
    (*a).format_options = Some(
        archive_write_zip_options
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_char,
                _: *const libc::c_char,
            ) -> libc::c_int,
    );
    (*a).format_write_header = Some(
        archive_write_zip_header
            as unsafe extern "C" fn(_: *mut archive_write, _: *mut archive_entry) -> libc::c_int,
    );
    (*a).format_write_data = Some(
        archive_write_zip_data
            as unsafe extern "C" fn(
                _: *mut archive_write,
                _: *const libc::c_void,
                _: size_t,
            ) -> ssize_t,
    );
    (*a).format_finish_entry = Some(
        archive_write_zip_finish_entry
            as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int,
    );
    (*a).format_close =
        Some(archive_write_zip_close as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).format_free =
        Some(archive_write_zip_free as unsafe extern "C" fn(_: *mut archive_write) -> libc::c_int);
    (*a).archive.archive_format = ARCHIVE_FORMAT_ZIP;
    (*a).archive.archive_format_name = b"ZIP\x00" as *const u8 as *const libc::c_char;
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_all_ascii(mut p: *const libc::c_char) -> libc::c_int {
    let mut pp: *const libc::c_uchar = p as *const libc::c_uchar;
    while *pp != 0 {
        let fresh0 = pp;
        pp = pp.offset(1);
        if *fresh0 as libc::c_int > 127 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn archive_write_zip_header(
    mut a: *mut archive_write,
    mut entry: *mut archive_entry,
) -> libc::c_int {
    let mut local_header: [libc::c_uchar; 32] = [0; 32];
    let mut local_extra: [libc::c_uchar; 144] = [0; 144];
    let mut zip: *mut zip = (*a).format_data as *mut zip;
    let mut e: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cd_extra: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut filename_length: size_t = 0;
    let mut slink: *const libc::c_char = NULL as *const libc::c_char;
    let mut slink_size: size_t = 0 as libc::c_int as size_t;
    let mut sconv: *mut archive_string_conv = get_sconv(a, zip);
    let mut ret: libc::c_int = 0;
    let mut ret2: libc::c_int = ARCHIVE_OK;
    let mut type_0: mode_t = 0;
    let mut version_needed: libc::c_int = 10 as libc::c_int;
    /* Ignore types of entries that we don't support. */
    type_0 = archive_entry_filetype(entry);
    if type_0 != AE_IFREG as mode_t && type_0 != AE_IFDIR as mode_t && type_0 != AE_IFLNK as mode_t
    {
        __archive_write_entry_filetype_unsupported(
            &mut (*a).archive,
            entry,
            b"zip\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FAILED;
    }
    /* If we're not using Zip64, reject large files. */
    if (*zip).flags & ZIP_FLAG_AVOID_ZIP64 != 0 {
        /* Reject entries over 4GB. */
        if archive_entry_size_is_set(entry) != 0
            && archive_entry_size(entry) as libc::c_longlong > 0xffffffff as libc::c_longlong
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Files > 4GB require Zip64 extensions\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FAILED;
        }
        /* Reject entries if archive is > 4GB. */
        if (*zip).written_bytes as libc::c_longlong > 0xffffffff as libc::c_longlong {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Archives > 4GB require Zip64 extensions\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FAILED;
        }
    }
    /* Only regular files can have size > 0. */
    if type_0 != AE_IFREG as mode_t {
        archive_entry_set_size(entry, 0 as libc::c_int as la_int64_t);
    }
    /* Reset information from last entry. */
    (*zip).entry_offset = (*zip).written_bytes;
    (*zip).entry_uncompressed_limit = INT64_MAX;
    (*zip).entry_compressed_size = 0 as libc::c_int as int64_t;
    (*zip).entry_uncompressed_size = 0 as libc::c_int as int64_t;
    (*zip).entry_compressed_written = 0 as libc::c_int as int64_t;
    (*zip).entry_uncompressed_written = 0 as libc::c_int as int64_t;
    (*zip).entry_flags = 0 as libc::c_int;
    (*zip).entry_uses_zip64 = 0 as libc::c_int;
    (*zip).entry_crc32 = (*zip).crc32func.expect("non-null function pointer")(
        0 as libc::c_int as libc::c_ulong,
        NULL as *const libc::c_void,
        0 as libc::c_int as size_t,
    ) as uint32_t;
    (*zip).entry_encryption = ENCRYPTION_NONE;
    archive_entry_free((*zip).entry);
    (*zip).entry = NULL as *mut archive_entry;
    if (*zip).cctx_valid != 0 {
        __archive_cryptor
            .encrypto_aes_ctr_release
            .expect("non-null function pointer")(&mut (*zip).cctx);
    }
    if (*zip).hctx_valid != 0 {
        __archive_hmac
            .__hmac_sha1_cleanup
            .expect("non-null function pointer")(&mut (*zip).hctx);
    }
    (*zip).hctx_valid = 0 as libc::c_int as libc::c_char;
    (*zip).cctx_valid = (*zip).hctx_valid;
    (*zip).tctx_valid = (*zip).cctx_valid;
    if type_0 == AE_IFREG as mode_t
        && (archive_entry_size_is_set(entry) == 0
            || archive_entry_size(entry) > 0 as libc::c_int as libc::c_long)
    {
        match (*zip).encryption_type as libc::c_uint {
            1 | 2 | 3 => {
                (*zip).entry_flags |= ZIP_ENTRY_FLAG_ENCRYPTED;
                (*zip).entry_encryption = (*zip).encryption_type
            }
            _ => {}
        }
    }
    (*zip).entry = archive_entry_clone(entry);
    if (*zip).entry.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ENOMEM,
            b"Can\'t allocate zip header data\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    if !sconv.is_null() {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut len: size_t = 0;
        if _archive_entry_pathname_l(entry, &mut p, &mut len, sconv) != 0 as libc::c_int {
            if errno == ENOMEM {
                archive_set_error(
                    &mut (*a).archive as *mut archive,
                    ENOMEM,
                    b"Can\'t allocate memory for Pathname\x00" as *const u8 as *const libc::c_char,
                );
                return -(30 as libc::c_int);
            }
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_FILE_FORMAT,
                b"Can\'t translate Pathname \'%s\' to %s\x00" as *const u8 as *const libc::c_char,
                archive_entry_pathname(entry),
                archive_string_conversion_charset_name(sconv),
            );
            ret2 = ARCHIVE_WARN
        }
        if len > 0 as libc::c_int as libc::c_ulong {
            archive_entry_set_pathname((*zip).entry, p);
        }
        /*
         * There is no standard for symlink handling; we convert
         * it using the same character-set translation that we use
         * for filename.
         */
        if type_0 == AE_IFLNK as mode_t {
            if _archive_entry_symlink_l(entry, &mut p, &mut len, sconv) != 0 {
                if errno == ENOMEM {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ENOMEM,
                        b"Can\'t allocate memory  for Symlink\x00" as *const u8
                            as *const libc::c_char,
                    );
                    return -(30 as libc::c_int);
                }
            /* No error if we can't convert. */
            } else if len > 0 as libc::c_int as libc::c_ulong {
                archive_entry_set_symlink((*zip).entry, p);
            }
        }
    }
    /* If filename isn't ASCII and we can use UTF-8, set the UTF-8 flag. */
    if is_all_ascii(archive_entry_pathname((*zip).entry)) == 0 {
        if !(*zip).opt_sconv.is_null() {
            if strcmp(
                archive_string_conversion_charset_name((*zip).opt_sconv),
                b"UTF-8\x00" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                (*zip).entry_flags |= ZIP_ENTRY_FLAG_UTF8_NAME
            }
        } else if strcmp(
            nl_langinfo(CODESET_0),
            b"UTF-8\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            (*zip).entry_flags |= ZIP_ENTRY_FLAG_UTF8_NAME
        }
    }
    filename_length = path_length((*zip).entry);
    /* Determine appropriate compression and size for this entry. */
    if type_0 == AE_IFLNK as mode_t {
        slink = archive_entry_symlink((*zip).entry);
        if !slink.is_null() {
            slink_size = strlen(slink)
        } else {
            slink_size = 0 as libc::c_int as size_t
        }
        (*zip).entry_uncompressed_limit = slink_size as int64_t;
        (*zip).entry_compressed_size = slink_size as int64_t;
        (*zip).entry_uncompressed_size = slink_size as int64_t;
        (*zip).entry_crc32 = (*zip).crc32func.expect("non-null function pointer")(
            (*zip).entry_crc32 as libc::c_ulong,
            slink as *const libc::c_uchar as *const libc::c_void,
            slink_size,
        ) as uint32_t;
        (*zip).entry_compression = COMPRESSION_STORE;
        version_needed = 20 as libc::c_int
    } else if type_0 != AE_IFREG as mode_t {
        (*zip).entry_compression = COMPRESSION_STORE;
        (*zip).entry_uncompressed_limit = 0 as libc::c_int as int64_t;
        version_needed = 20 as libc::c_int
    } else if archive_entry_size_is_set((*zip).entry) != 0 {
        let mut size: int64_t = archive_entry_size((*zip).entry);
        let mut additional_size: int64_t = 0 as libc::c_int as int64_t;
        (*zip).entry_uncompressed_limit = size;
        (*zip).entry_compression = (*zip).requested_compression;
        if (*zip).entry_compression as libc::c_int == COMPRESSION_UNSPECIFIED as libc::c_int {
            (*zip).entry_compression = COMPRESSION_DEFAULT as compression
        }
        if (*zip).entry_compression as libc::c_int == COMPRESSION_STORE as libc::c_int {
            (*zip).entry_compressed_size = size;
            (*zip).entry_uncompressed_size = size;
            version_needed = 10 as libc::c_int
        } else {
            (*zip).entry_uncompressed_size = size;
            version_needed = 20 as libc::c_int
        }
        if (*zip).entry_flags & ZIP_ENTRY_FLAG_ENCRYPTED != 0 {
            match (*zip).entry_encryption as libc::c_uint {
                1 => {
                    additional_size = TRAD_HEADER_SIZE as int64_t;
                    version_needed = 20 as libc::c_int
                }
                2 => {
                    additional_size = (WINZIP_AES128_HEADER_SIZE + AUTH_CODE_SIZE) as int64_t;
                    version_needed = 20 as libc::c_int
                }
                3 => {
                    additional_size = (WINZIP_AES256_HEADER_SIZE + AUTH_CODE_SIZE) as int64_t;
                    version_needed = 20 as libc::c_int
                }
                _ => {}
            }
            if (*zip).entry_compression as libc::c_int == COMPRESSION_STORE as libc::c_int {
                (*zip).entry_compressed_size += additional_size
            }
        }
        /*
         * Set Zip64 extension in any of the following cases
         * (this was suggested by discussion on info-zip-dev
         * mailing list):
         *  = Zip64 is being forced by user
         *  = File is over 4GiB uncompressed
         *    (including encryption header, if any)
         *  = File is close to 4GiB and is being compressed
         *    (compression might make file larger)
         */
        if (*zip).flags & ZIP_FLAG_FORCE_ZIP64 != 0
            || ((*zip).entry_uncompressed_size + additional_size) as libc::c_longlong
                > 0xffffffff as libc::c_longlong
            || (*zip).entry_uncompressed_size as libc::c_longlong > 0xff000000 as libc::c_longlong
                && (*zip).entry_compression as libc::c_int != COMPRESSION_STORE as libc::c_int
        {
            (*zip).entry_uses_zip64 = 1 as libc::c_int;
            version_needed = 45 as libc::c_int
        }
        /* We may know the size, but never the CRC. */
        (*zip).entry_flags |= ZIP_ENTRY_FLAG_LENGTH_AT_END
    } else {
        /* We don't know the size.  In this case, we prefer
         * deflate (it has a clear end-of-data marker which
         * makes length-at-end more reliable) and will
         * enable Zip64 extensions unless we're told not to.
         */
        (*zip).entry_compression = COMPRESSION_DEFAULT as compression;
        (*zip).entry_flags |= ZIP_ENTRY_FLAG_LENGTH_AT_END;
        if (*zip).flags & ZIP_FLAG_AVOID_ZIP64 == 0 as libc::c_int {
            (*zip).entry_uses_zip64 = 1 as libc::c_int;
            version_needed = 45 as libc::c_int
        } else if (*zip).entry_compression as libc::c_int == COMPRESSION_STORE as libc::c_int {
            version_needed = 10 as libc::c_int
        } else {
            version_needed = 20 as libc::c_int
        }
        if (*zip).entry_flags & ZIP_ENTRY_FLAG_ENCRYPTED != 0 {
            match (*zip).entry_encryption as libc::c_uint {
                1 | 2 | 3 => {
                    if version_needed < 20 as libc::c_int {
                        version_needed = 20 as libc::c_int
                    }
                }
                _ => {}
            }
        }
    }
    /* Format the local header. */
    memset(
        local_header.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
    );
    memcpy(
        local_header.as_mut_ptr() as *mut libc::c_void,
        b"PK\x03\x04\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    archive_le16enc(
        local_header.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
        version_needed as uint16_t,
    );
    archive_le16enc(
        local_header.as_mut_ptr().offset(6 as libc::c_int as isize) as *mut libc::c_void,
        (*zip).entry_flags as uint16_t,
    );
    if (*zip).entry_encryption as libc::c_uint
        == ENCRYPTION_WINZIP_AES128 as libc::c_int as libc::c_uint
        || (*zip).entry_encryption as libc::c_uint
            == ENCRYPTION_WINZIP_AES256 as libc::c_int as libc::c_uint
    {
        archive_le16enc(
            local_header.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut libc::c_void,
            WINZIP_AES_ENCRYPTION as uint16_t,
        );
    } else {
        archive_le16enc(
            local_header.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut libc::c_void,
            (*zip).entry_compression as uint16_t,
        );
    }
    archive_le32enc(
        local_header.as_mut_ptr().offset(10 as libc::c_int as isize) as *mut libc::c_void,
        dos_time(archive_entry_mtime((*zip).entry)),
    );
    archive_le32enc(
        local_header.as_mut_ptr().offset(14 as libc::c_int as isize) as *mut libc::c_void,
        (*zip).entry_crc32,
    );
    if (*zip).entry_uses_zip64 != 0 {
        /* Zip64 data in the local header "must" include both
         * compressed and uncompressed sizes AND those fields
         * are included only if these are 0xffffffff;
         * THEREFORE these must be set this way, even if we
         * know one of them is smaller. */
        archive_le32enc(
            local_header.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut libc::c_void,
            0xffffffff as libc::c_longlong as uint32_t,
        );
        archive_le32enc(
            local_header.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut libc::c_void,
            0xffffffff as libc::c_longlong as uint32_t,
        );
    } else {
        archive_le32enc(
            local_header.as_mut_ptr().offset(18 as libc::c_int as isize) as *mut libc::c_void,
            (*zip).entry_compressed_size as uint32_t,
        );
        archive_le32enc(
            local_header.as_mut_ptr().offset(22 as libc::c_int as isize) as *mut libc::c_void,
            (*zip).entry_uncompressed_size as uint32_t,
        );
    }
    archive_le16enc(
        local_header.as_mut_ptr().offset(26 as libc::c_int as isize) as *mut libc::c_void,
        filename_length as uint16_t,
    );
    if (*zip).entry_encryption as libc::c_uint
        == ENCRYPTION_TRADITIONAL as libc::c_int as libc::c_uint
    {
        if (*zip).entry_flags & ZIP_ENTRY_FLAG_LENGTH_AT_END != 0 {
            (*zip).trad_chkdat = local_header[11 as libc::c_int as usize]
        } else {
            (*zip).trad_chkdat = local_header[17 as libc::c_int as usize]
        }
    }
    /* Format as much of central directory file header as we can: */
    (*zip).file_header = cd_alloc(zip, 46 as libc::c_int as size_t);
    /* If (zip->file_header == NULL) XXXX */
    (*zip).central_directory_entries = (*zip).central_directory_entries.wrapping_add(1);
    memset(
        (*zip).file_header as *mut libc::c_void,
        0 as libc::c_int,
        46 as libc::c_int as libc::c_ulong,
    );
    memcpy(
        (*zip).file_header as *mut libc::c_void,
        b"PK\x01\x02\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    /* "Made by PKZip 2.0 on Unix." */
    archive_le16enc(
        (*zip).file_header.offset(4 as libc::c_int as isize) as *mut libc::c_void,
        (3 as libc::c_int * 256 as libc::c_int + version_needed) as uint16_t,
    );
    archive_le16enc(
        (*zip).file_header.offset(6 as libc::c_int as isize) as *mut libc::c_void,
        version_needed as uint16_t,
    );
    archive_le16enc(
        (*zip).file_header.offset(8 as libc::c_int as isize) as *mut libc::c_void,
        (*zip).entry_flags as uint16_t,
    );
    if (*zip).entry_encryption as libc::c_uint
        == ENCRYPTION_WINZIP_AES128 as libc::c_int as libc::c_uint
        || (*zip).entry_encryption as libc::c_uint
            == ENCRYPTION_WINZIP_AES256 as libc::c_int as libc::c_uint
    {
        archive_le16enc(
            (*zip).file_header.offset(10 as libc::c_int as isize) as *mut libc::c_void,
            WINZIP_AES_ENCRYPTION as uint16_t,
        );
    } else {
        archive_le16enc(
            (*zip).file_header.offset(10 as libc::c_int as isize) as *mut libc::c_void,
            (*zip).entry_compression as uint16_t,
        );
    }
    archive_le32enc(
        (*zip).file_header.offset(12 as libc::c_int as isize) as *mut libc::c_void,
        dos_time(archive_entry_mtime((*zip).entry)),
    );
    archive_le16enc(
        (*zip).file_header.offset(28 as libc::c_int as isize) as *mut libc::c_void,
        filename_length as uint16_t,
    );
    /* Following Info-Zip, store mode in the "external attributes" field. */
    archive_le32enc(
        (*zip).file_header.offset(38 as libc::c_int as isize) as *mut libc::c_void,
        archive_entry_mode((*zip).entry) << 16 as libc::c_int,
    );
    e = cd_alloc(zip, filename_length);
    /* If (e == NULL) XXXX */
    copy_path((*zip).entry, e);
    /* Format extra data. */
    memset(
        local_extra.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_uchar; 144]>() as libc::c_ulong,
    );
    e = local_extra.as_mut_ptr();
    /* First, extra blocks that are the same between
     * the local file header and the central directory.
     * We format them once and then duplicate them. */
    /* UT timestamp, length depends on what timestamps are set. */
    memcpy(
        e as *mut libc::c_void,
        b"UT\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    );
    archive_le16enc(
        e.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        (1 as libc::c_int
            + (if archive_entry_mtime_is_set(entry) != 0 {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            })
            + (if archive_entry_atime_is_set(entry) != 0 {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            })
            + (if archive_entry_ctime_is_set(entry) != 0 {
                4 as libc::c_int
            } else {
                0 as libc::c_int
            })) as uint16_t,
    );
    e = e.offset(4 as libc::c_int as isize);
    let fresh1 = e;
    e = e.offset(1);
    *fresh1 = ((if archive_entry_mtime_is_set(entry) != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) | (if archive_entry_atime_is_set(entry) != 0 {
        2 as libc::c_int
    } else {
        0 as libc::c_int
    }) | (if archive_entry_ctime_is_set(entry) != 0 {
        4 as libc::c_int
    } else {
        0 as libc::c_int
    })) as libc::c_uchar;
    if archive_entry_mtime_is_set(entry) != 0 {
        archive_le32enc(
            e as *mut libc::c_void,
            archive_entry_mtime(entry) as uint32_t,
        );
        e = e.offset(4 as libc::c_int as isize)
    }
    if archive_entry_atime_is_set(entry) != 0 {
        archive_le32enc(
            e as *mut libc::c_void,
            archive_entry_atime(entry) as uint32_t,
        );
        e = e.offset(4 as libc::c_int as isize)
    }
    if archive_entry_ctime_is_set(entry) != 0 {
        archive_le32enc(
            e as *mut libc::c_void,
            archive_entry_ctime(entry) as uint32_t,
        );
        e = e.offset(4 as libc::c_int as isize)
    }
    /* ux Unix extra data, length 11, version 1 */
    /* TODO: If uid < 64k, use 2 bytes, ditto for gid. */
    memcpy(
        e as *mut libc::c_void,
        b"ux\x0b\x00\x01\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_int as libc::c_ulong,
    ); /* Length of following UID */
    e = e.offset(5 as libc::c_int as isize); /* Length of following GID */
    let fresh2 = e;
    e = e.offset(1);
    *fresh2 = 4 as libc::c_int as libc::c_uchar;
    archive_le32enc(e as *mut libc::c_void, archive_entry_uid(entry) as uint32_t);
    e = e.offset(4 as libc::c_int as isize);
    let fresh3 = e;
    e = e.offset(1);
    *fresh3 = 4 as libc::c_int as libc::c_uchar;
    archive_le32enc(e as *mut libc::c_void, archive_entry_gid(entry) as uint32_t);
    e = e.offset(4 as libc::c_int as isize);
    /* AES extra data field: WinZIP AES information, ID=0x9901 */
    if (*zip).entry_flags & ZIP_ENTRY_FLAG_ENCRYPTED != 0
        && ((*zip).entry_encryption as libc::c_uint
            == ENCRYPTION_WINZIP_AES128 as libc::c_int as libc::c_uint
            || (*zip).entry_encryption as libc::c_uint
                == ENCRYPTION_WINZIP_AES256 as libc::c_int as libc::c_uint)
    {
        memcpy(
            e as *mut libc::c_void,
            b"\x01\x99\x07\x00\x01\x00AE\x00" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        );
        /* AES vendor version AE-2 does not store a CRC.
         * WinZip 11 uses AE-1, which does store the CRC,
         * but it does not store the CRC when the file size
         * is less than 20 bytes. So we simulate what
         * WinZip 11 does.
         * NOTE: WinZip 9.0 and 10.0 uses AE-2 by default. */
        if archive_entry_size_is_set((*zip).entry) != 0
            && archive_entry_size((*zip).entry) < 20 as libc::c_int as libc::c_long
        {
            archive_le16enc(
                e.offset(4 as libc::c_int as isize) as *mut libc::c_void,
                AES_VENDOR_AE_2 as uint16_t,
            );
            (*zip).aes_vendor = AES_VENDOR_AE_2 as libc::c_uint
        /* no CRC. */
        } else {
            (*zip).aes_vendor = AES_VENDOR_AE_1 as libc::c_uint
        }
        e = e.offset(8 as libc::c_int as isize);
        /* AES encryption strength. */
        let fresh4 = e;
        e = e.offset(1);
        *fresh4 = if (*zip).entry_encryption as libc::c_uint
            == ENCRYPTION_WINZIP_AES128 as libc::c_int as libc::c_uint
        {
            1 as libc::c_int
        } else {
            3 as libc::c_int
        } as libc::c_uchar;
        /* Actual compression method. */
        archive_le16enc(e as *mut libc::c_void, (*zip).entry_compression as uint16_t);
        e = e.offset(2 as libc::c_int as isize)
    }
    /* Copy UT ,ux, and AES-extra into central directory as well. */
    (*zip).file_header_extra_offset = (*zip).central_directory_bytes;
    cd_extra = cd_alloc(
        zip,
        e.offset_from(local_extra.as_mut_ptr()) as libc::c_long as size_t,
    );
    memcpy(
        cd_extra as *mut libc::c_void,
        local_extra.as_mut_ptr() as *const libc::c_void,
        e.offset_from(local_extra.as_mut_ptr()) as libc::c_long as libc::c_ulong,
    );
    /*
     * Following extra blocks vary between local header and
     * central directory. These are the local header versions.
     * Central directory versions get formatted in
     * archive_write_zip_finish_entry() below.
     */
    /* "[Zip64 entry] in the local header MUST include BOTH
     * original [uncompressed] and compressed size fields." */
    if (*zip).entry_uses_zip64 != 0 {
        let mut zip64_start: *mut libc::c_uchar = e;
        memcpy(
            e as *mut libc::c_void,
            b"\x01\x00\x10\x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        e = e.offset(4 as libc::c_int as isize);
        archive_le64enc(
            e as *mut libc::c_void,
            (*zip).entry_uncompressed_size as uint64_t,
        );
        e = e.offset(8 as libc::c_int as isize);
        archive_le64enc(
            e as *mut libc::c_void,
            (*zip).entry_compressed_size as uint64_t,
        );
        e = e.offset(8 as libc::c_int as isize);
        archive_le16enc(
            zip64_start.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            e.offset_from(zip64_start.offset(4 as libc::c_int as isize)) as libc::c_long
                as uint16_t,
        );
    }
    if (*zip).flags & ZIP_FLAG_EXPERIMENT_xl != 0 {
        /* Experimental 'xl' extension to improve streaming. */
        let mut external_info: *mut libc::c_uchar = e; // 0x6c65 + 2-byte length
        let mut included: libc::c_int = 7 as libc::c_int; /* bitmap of included fields */
        memcpy(
            e as *mut libc::c_void,
            b"xl\x00\x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ); /* internal file attributes */
        e = e.offset(4 as libc::c_int as isize);
        *e.offset(0 as libc::c_int as isize) = included as libc::c_uchar;
        e = e.offset(1 as libc::c_int as isize);
        if included & 1 as libc::c_int != 0 {
            archive_le16enc(
                e as *mut libc::c_void,
                (3 as libc::c_int * 256 as libc::c_int + version_needed) as uint16_t,
            );
            e = e.offset(2 as libc::c_int as isize)
        }
        if included & 2 as libc::c_int != 0 {
            archive_le16enc(e as *mut libc::c_void, 0 as libc::c_int as uint16_t);
            e = e.offset(2 as libc::c_int as isize)
        }
        if included & 4 as libc::c_int != 0 {
            archive_le32enc(
                e as *mut libc::c_void,
                archive_entry_mode((*zip).entry) << 16 as libc::c_int,
            );
            e = e.offset(4 as libc::c_int as isize)
        }
        (included & 8 as libc::c_int) != 0;
        archive_le16enc(
            external_info.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            e.offset_from(external_info.offset(4 as libc::c_int as isize)) as libc::c_long
                as uint16_t,
        );
    }
    /* Update local header with size of extra data and write it all out: */
    archive_le16enc(
        local_header.as_mut_ptr().offset(28 as libc::c_int as isize) as *mut libc::c_void,
        e.offset_from(local_extra.as_mut_ptr()) as libc::c_long as uint16_t,
    );
    ret = __archive_write_output(
        a,
        local_header.as_mut_ptr() as *const libc::c_void,
        30 as libc::c_int as size_t,
    );
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*zip).written_bytes += 30 as libc::c_int as libc::c_long;
    ret = write_path((*zip).entry, a);
    if ret <= ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*zip).written_bytes += ret as libc::c_long;
    ret = __archive_write_output(
        a,
        local_extra.as_mut_ptr() as *const libc::c_void,
        e.offset_from(local_extra.as_mut_ptr()) as libc::c_long as size_t,
    );
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*zip).written_bytes += e.offset_from(local_extra.as_mut_ptr()) as libc::c_long;
    /* For symlinks, write the body now. */
    if !slink.is_null() {
        ret = __archive_write_output(a, slink as *const libc::c_void, slink_size);
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        (*zip).entry_compressed_written = ((*zip).entry_compressed_written as libc::c_ulong)
            .wrapping_add(slink_size) as int64_t
            as int64_t;
        (*zip).entry_uncompressed_written = ((*zip).entry_uncompressed_written as libc::c_ulong)
            .wrapping_add(slink_size) as int64_t
            as int64_t;
        (*zip).written_bytes =
            ((*zip).written_bytes as libc::c_ulong).wrapping_add(slink_size) as int64_t as int64_t
    }
    if (*zip).entry_compression as libc::c_int == COMPRESSION_DEFLATE as libc::c_int {
        (*zip).stream.zalloc =
            ::std::mem::transmute::<libc::intptr_t, alloc_func>(Z_NULL as libc::intptr_t);
        (*zip).stream.zfree =
            ::std::mem::transmute::<libc::intptr_t, free_func>(Z_NULL as libc::intptr_t);
        (*zip).stream.opaque = Z_NULL as voidpf;
        (*zip).stream.next_out = (*zip).buf;
        (*zip).stream.avail_out = (*zip).len_buf as uInt;
        if deflateInit2_(
            &mut (*zip).stream,
            (*zip).deflate_compression_level,
            8 as libc::c_int,
            -(15 as libc::c_int),
            8 as libc::c_int,
            0 as libc::c_int,
            ZLIB_VERSION.as_ptr(),
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        ) != Z_OK
        {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t init deflate compressor\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
    }
    return ret2;
}
unsafe extern "C" fn archive_write_zip_data(
    mut a: *mut archive_write,
    mut buff: *const libc::c_void,
    mut s: size_t,
) -> ssize_t {
    let mut ret: libc::c_int = 0;
    let mut zip: *mut zip = (*a).format_data as *mut zip;
    if s as int64_t > (*zip).entry_uncompressed_limit {
        s = (*zip).entry_uncompressed_limit as size_t
    }
    (*zip).entry_uncompressed_written =
        ((*zip).entry_uncompressed_written as libc::c_ulong).wrapping_add(s) as int64_t as int64_t;
    if s == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    if (*zip).entry_flags & ZIP_ENTRY_FLAG_ENCRYPTED != 0 {
        match (*zip).entry_encryption as libc::c_uint {
            1 => {
                /* Initialize traditional PKWARE encryption context. */
                if (*zip).tctx_valid == 0 {
                    ret = init_traditional_pkware_encryption(a);
                    if ret != ARCHIVE_OK {
                        return ret as ssize_t;
                    }
                    (*zip).tctx_valid = 1 as libc::c_int as libc::c_char
                }
            }
            2 | 3 => {
                if (*zip).cctx_valid == 0 {
                    ret = init_winzip_aes_encryption(a);
                    if ret != ARCHIVE_OK {
                        return ret as ssize_t;
                    }
                    (*zip).hctx_valid = 1 as libc::c_int as libc::c_char;
                    (*zip).cctx_valid = (*zip).hctx_valid
                }
            }
            _ => {}
        }
    }
    match (*zip).entry_compression as libc::c_int {
        0 => {
            if (*zip).tctx_valid as libc::c_int != 0 || (*zip).cctx_valid as libc::c_int != 0 {
                let mut rb: *const uint8_t = buff as *const uint8_t;
                let re: *const uint8_t = rb.offset(s as isize);
                while rb < re {
                    let mut l: size_t = 0;
                    if (*zip).tctx_valid != 0 {
                        l = trad_enc_encrypt_update(
                            &mut (*zip).tctx,
                            rb,
                            re.offset_from(rb) as libc::c_long as size_t,
                            (*zip).buf,
                            (*zip).len_buf,
                        ) as size_t
                    } else {
                        l = (*zip).len_buf;
                        ret = __archive_cryptor
                            .encrypto_aes_ctr_update
                            .expect("non-null function pointer")(
                            &mut (*zip).cctx,
                            rb,
                            re.offset_from(rb) as libc::c_long as size_t,
                            (*zip).buf,
                            &mut l,
                        );
                        if ret < 0 as libc::c_int {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_MISC,
                                b"Failed to encrypt file\x00" as *const u8 as *const libc::c_char,
                            );
                            return -(25 as libc::c_int) as ssize_t;
                        }
                        __archive_hmac
                            .__hmac_sha1_update
                            .expect("non-null function pointer")(
                            &mut (*zip).hctx, (*zip).buf, l
                        );
                    }
                    ret = __archive_write_output(a, (*zip).buf as *const libc::c_void, l);
                    if ret != ARCHIVE_OK {
                        return ret as ssize_t;
                    }
                    (*zip).entry_compressed_written =
                        ((*zip).entry_compressed_written as libc::c_ulong).wrapping_add(l)
                            as int64_t as int64_t;
                    (*zip).written_bytes = ((*zip).written_bytes as libc::c_ulong).wrapping_add(l)
                        as int64_t as int64_t;
                    rb = rb.offset(l as isize)
                }
            } else {
                ret = __archive_write_output(a, buff, s);
                if ret != ARCHIVE_OK {
                    return ret as ssize_t;
                }
                (*zip).written_bytes =
                    ((*zip).written_bytes as libc::c_ulong).wrapping_add(s) as int64_t as int64_t;
                (*zip).entry_compressed_written = ((*zip).entry_compressed_written as libc::c_ulong)
                    .wrapping_add(s) as int64_t
                    as int64_t
            }
        }
        8 => {
            (*zip).stream.next_in = buff as uintptr_t as *mut libc::c_uchar;
            (*zip).stream.avail_in = s as uInt;
            loop {
                ret = deflate(&mut (*zip).stream, Z_NO_FLUSH);
                if ret == Z_STREAM_ERROR {
                    return -(30 as libc::c_int) as ssize_t;
                }
                if (*zip).stream.avail_out == 0 as libc::c_int as libc::c_uint {
                    if (*zip).tctx_valid != 0 {
                        trad_enc_encrypt_update(
                            &mut (*zip).tctx,
                            (*zip).buf,
                            (*zip).len_buf,
                            (*zip).buf,
                            (*zip).len_buf,
                        );
                    } else if (*zip).cctx_valid != 0 {
                        let mut outl: size_t = (*zip).len_buf;
                        ret = __archive_cryptor
                            .encrypto_aes_ctr_update
                            .expect("non-null function pointer")(
                            &mut (*zip).cctx,
                            (*zip).buf,
                            (*zip).len_buf,
                            (*zip).buf,
                            &mut outl,
                        );
                        if ret < 0 as libc::c_int {
                            archive_set_error(
                                &mut (*a).archive as *mut archive,
                                ARCHIVE_ERRNO_MISC,
                                b"Failed to encrypt file\x00" as *const u8 as *const libc::c_char,
                            );
                            return -(25 as libc::c_int) as ssize_t;
                        }
                        __archive_hmac
                            .__hmac_sha1_update
                            .expect("non-null function pointer")(
                            &mut (*zip).hctx,
                            (*zip).buf,
                            (*zip).len_buf,
                        );
                    }
                    ret = __archive_write_output(
                        a,
                        (*zip).buf as *const libc::c_void,
                        (*zip).len_buf,
                    );
                    if ret != ARCHIVE_OK {
                        return ret as ssize_t;
                    }
                    (*zip).entry_compressed_written = ((*zip).entry_compressed_written
                        as libc::c_ulong)
                        .wrapping_add((*zip).len_buf)
                        as int64_t as int64_t;
                    (*zip).written_bytes = ((*zip).written_bytes as libc::c_ulong)
                        .wrapping_add((*zip).len_buf)
                        as int64_t as int64_t;
                    (*zip).stream.next_out = (*zip).buf;
                    (*zip).stream.avail_out = (*zip).len_buf as uInt
                }
                if !((*zip).stream.avail_in != 0 as libc::c_int as libc::c_uint) {
                    break;
                }
            }
        }
        _ => {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ARCHIVE_ERRNO_MISC,
                b"Invalid ZIP compression type\x00" as *const u8 as *const libc::c_char,
            );
            return ARCHIVE_FATAL as ssize_t;
        }
    }
    (*zip).entry_uncompressed_limit =
        ((*zip).entry_uncompressed_limit as libc::c_ulong).wrapping_sub(s) as int64_t as int64_t;
    if (*zip).cctx_valid == 0 || (*zip).aes_vendor != AES_VENDOR_AE_2 as libc::c_uint {
        (*zip).entry_crc32 = (*zip).crc32func.expect("non-null function pointer")(
            (*zip).entry_crc32 as libc::c_ulong,
            buff,
            s as libc::c_uint as size_t,
        ) as uint32_t
    }
    return s as ssize_t;
}
unsafe extern "C" fn archive_write_zip_finish_entry(mut a: *mut archive_write) -> libc::c_int {
    let mut zip: *mut zip = (*a).format_data as *mut zip;
    let mut ret: libc::c_int = 0;
    if (*zip).entry_compression as libc::c_int == COMPRESSION_DEFLATE as libc::c_int {
        loop {
            let mut remainder: size_t = 0;
            ret = deflate(&mut (*zip).stream, Z_FINISH);
            if ret == Z_STREAM_ERROR {
                return -(30 as libc::c_int);
            }
            remainder = (*zip)
                .len_buf
                .wrapping_sub((*zip).stream.avail_out as libc::c_ulong);
            if (*zip).tctx_valid != 0 {
                trad_enc_encrypt_update(
                    &mut (*zip).tctx,
                    (*zip).buf,
                    remainder,
                    (*zip).buf,
                    remainder,
                );
            } else if (*zip).cctx_valid != 0 {
                let mut outl: size_t = remainder;
                ret = __archive_cryptor
                    .encrypto_aes_ctr_update
                    .expect("non-null function pointer")(
                    &mut (*zip).cctx,
                    (*zip).buf,
                    remainder,
                    (*zip).buf,
                    &mut outl,
                );
                if ret < 0 as libc::c_int {
                    archive_set_error(
                        &mut (*a).archive as *mut archive,
                        ARCHIVE_ERRNO_MISC,
                        b"Failed to encrypt file\x00" as *const u8 as *const libc::c_char,
                    );
                    return -(25 as libc::c_int);
                }
                __archive_hmac
                    .__hmac_sha1_update
                    .expect("non-null function pointer")(
                    &mut (*zip).hctx, (*zip).buf, remainder
                );
            }
            ret = __archive_write_output(a, (*zip).buf as *const libc::c_void, remainder);
            if ret != ARCHIVE_OK {
                return ret;
            }
            (*zip).entry_compressed_written = ((*zip).entry_compressed_written as libc::c_ulong)
                .wrapping_add(remainder) as int64_t
                as int64_t;
            (*zip).written_bytes = ((*zip).written_bytes as libc::c_ulong).wrapping_add(remainder)
                as int64_t as int64_t;
            (*zip).stream.next_out = (*zip).buf;
            if (*zip).stream.avail_out != 0 as libc::c_int as libc::c_uint {
                break;
            }
            (*zip).stream.avail_out = (*zip).len_buf as uInt
        }
        deflateEnd(&mut (*zip).stream);
    }
    if (*zip).hctx_valid != 0 {
        let mut hmac: [uint8_t; 20] = [0; 20];
        let mut hmac_len: size_t = 20 as libc::c_int as size_t;
        __archive_hmac
            .__hmac_sha1_final
            .expect("non-null function pointer")(
            &mut (*zip).hctx,
            hmac.as_mut_ptr(),
            &mut hmac_len,
        );
        ret = __archive_write_output(
            a,
            hmac.as_mut_ptr() as *const libc::c_void,
            AUTH_CODE_SIZE as size_t,
        );
        if ret != ARCHIVE_OK {
            return ret;
        }
        (*zip).entry_compressed_written += AUTH_CODE_SIZE as libc::c_long;
        (*zip).written_bytes += AUTH_CODE_SIZE as libc::c_long
    }
    /* Write trailing data descriptor. */
    if (*zip).entry_flags & ZIP_ENTRY_FLAG_LENGTH_AT_END != 0 as libc::c_int {
        let mut d: [libc::c_char; 24] = [0; 24]; /* no CRC.*/
        memcpy(
            d.as_mut_ptr() as *mut libc::c_void,
            b"PK\x07\x08\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        if (*zip).cctx_valid as libc::c_int != 0
            && (*zip).aes_vendor == AES_VENDOR_AE_2 as libc::c_uint
        {
            archive_le32enc(
                d.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
                0 as libc::c_int as uint32_t,
            );
        } else {
            archive_le32enc(
                d.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
                (*zip).entry_crc32,
            );
        }
        if (*zip).entry_uses_zip64 != 0 {
            archive_le64enc(
                d.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut libc::c_void,
                (*zip).entry_compressed_written as uint64_t,
            );
            archive_le64enc(
                d.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
                (*zip).entry_uncompressed_written as uint64_t,
            );
            ret = __archive_write_output(
                a,
                d.as_mut_ptr() as *const libc::c_void,
                24 as libc::c_int as size_t,
            );
            (*zip).written_bytes += 24 as libc::c_int as libc::c_long
        } else {
            archive_le32enc(
                d.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut libc::c_void,
                (*zip).entry_compressed_written as uint32_t,
            );
            archive_le32enc(
                d.as_mut_ptr().offset(12 as libc::c_int as isize) as *mut libc::c_void,
                (*zip).entry_uncompressed_written as uint32_t,
            );
            ret = __archive_write_output(
                a,
                d.as_mut_ptr() as *const libc::c_void,
                16 as libc::c_int as size_t,
            );
            (*zip).written_bytes += 16 as libc::c_int as libc::c_long
        }
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
    }
    /* Append Zip64 extra data to central directory information. */
    if (*zip).entry_compressed_written as libc::c_longlong > 0xffffffff as libc::c_longlong
        || (*zip).entry_uncompressed_written as libc::c_longlong > 0xffffffff as libc::c_longlong
        || (*zip).entry_offset as libc::c_longlong > 0xffffffff as libc::c_longlong
    {
        let mut zip64: [libc::c_uchar; 32] = [0; 32];
        let mut z: *mut libc::c_uchar = zip64.as_mut_ptr();
        let mut zd: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        memcpy(
            z as *mut libc::c_void,
            b"\x01\x00\x00\x00\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        z = z.offset(4 as libc::c_int as isize);
        if (*zip).entry_uncompressed_written as libc::c_longlong >= 0xffffffff as libc::c_longlong {
            archive_le64enc(
                z as *mut libc::c_void,
                (*zip).entry_uncompressed_written as uint64_t,
            );
            z = z.offset(8 as libc::c_int as isize)
        }
        if (*zip).entry_compressed_written as libc::c_longlong >= 0xffffffff as libc::c_longlong {
            archive_le64enc(
                z as *mut libc::c_void,
                (*zip).entry_compressed_written as uint64_t,
            );
            z = z.offset(8 as libc::c_int as isize)
        }
        if (*zip).entry_offset as libc::c_longlong >= 0xffffffff as libc::c_longlong {
            archive_le64enc(z as *mut libc::c_void, (*zip).entry_offset as uint64_t);
            z = z.offset(8 as libc::c_int as isize)
        }
        archive_le16enc(
            zip64.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut libc::c_void,
            z.offset_from(zip64.as_mut_ptr().offset(4 as libc::c_int as isize))
                as libc::c_long as uint16_t,
        );
        zd = cd_alloc(
            zip,
            z.offset_from(zip64.as_mut_ptr()) as libc::c_long as size_t,
        );
        if zd.is_null() {
            archive_set_error(
                &mut (*a).archive as *mut archive,
                ENOMEM,
                b"Can\'t allocate zip data\x00" as *const u8 as *const libc::c_char,
            );
            return -(30 as libc::c_int);
        }
        memcpy(
            zd as *mut libc::c_void,
            zip64.as_mut_ptr() as *const libc::c_void,
            z.offset_from(zip64.as_mut_ptr()) as libc::c_long as libc::c_ulong,
        );
        /* Zip64 means version needs to be set to at least 4.5 */
        if (archive_le16dec(
            (*zip).file_header.offset(6 as libc::c_int as isize) as *const libc::c_void
        ) as libc::c_int)
            < 45 as libc::c_int
        {
            archive_le16enc(
                (*zip).file_header.offset(6 as libc::c_int as isize) as *mut libc::c_void,
                45 as libc::c_int as uint16_t,
            );
        }
    }
    /* Fix up central directory file header. */
    if (*zip).cctx_valid as libc::c_int != 0 && (*zip).aes_vendor == AES_VENDOR_AE_2 as libc::c_uint
    {
        archive_le32enc(
            (*zip).file_header.offset(16 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int as uint32_t,
        ); /* no CRC.*/
    } else {
        archive_le32enc(
            (*zip).file_header.offset(16 as libc::c_int as isize) as *mut libc::c_void,
            (*zip).entry_crc32,
        );
    }
    archive_le32enc(
        (*zip).file_header.offset(20 as libc::c_int as isize) as *mut libc::c_void,
        if (*zip).entry_compressed_written as libc::c_longlong > 0xffffffff as libc::c_longlong {
            0xffffffff as libc::c_longlong
        } else {
            (*zip).entry_compressed_written as libc::c_longlong
        } as uint32_t,
    );
    archive_le32enc(
        (*zip).file_header.offset(24 as libc::c_int as isize) as *mut libc::c_void,
        if (*zip).entry_uncompressed_written as libc::c_longlong > 0xffffffff as libc::c_longlong {
            0xffffffff as libc::c_longlong
        } else {
            (*zip).entry_uncompressed_written as libc::c_longlong
        } as uint32_t,
    );
    archive_le16enc(
        (*zip).file_header.offset(30 as libc::c_int as isize) as *mut libc::c_void,
        (*zip)
            .central_directory_bytes
            .wrapping_sub((*zip).file_header_extra_offset) as uint16_t,
    );
    archive_le32enc(
        (*zip).file_header.offset(42 as libc::c_int as isize) as *mut libc::c_void,
        if (*zip).entry_offset as libc::c_longlong > 0xffffffff as libc::c_longlong {
            0xffffffff as libc::c_longlong
        } else {
            (*zip).entry_offset as libc::c_longlong
        } as uint32_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_zip_close(mut a: *mut archive_write) -> libc::c_int {
    let mut buff: [uint8_t; 64] = [0; 64];
    let mut offset_start: int64_t = 0;
    let mut offset_end: int64_t = 0;
    let mut zip: *mut zip = (*a).format_data as *mut zip;
    let mut segment: *mut cd_segment = 0 as *mut cd_segment;
    let mut ret: libc::c_int = 0;
    offset_start = (*zip).written_bytes;
    segment = (*zip).central_directory;
    while !segment.is_null() {
        ret = __archive_write_output(
            a,
            (*segment).buff as *const libc::c_void,
            (*segment).p.offset_from((*segment).buff) as libc::c_long as size_t,
        );
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        (*zip).written_bytes += (*segment).p.offset_from((*segment).buff) as libc::c_long;
        segment = (*segment).next
    }
    offset_end = (*zip).written_bytes;
    /* If central dir info is too large, write Zip64 end-of-cd */
    if (offset_end - offset_start) as libc::c_longlong > 0xffffffff as libc::c_longlong
        || offset_start as libc::c_longlong > 0xffffffff as libc::c_longlong
        || (*zip).central_directory_entries > 0xffff as libc::c_ulong
        || (*zip).flags & ZIP_FLAG_FORCE_ZIP64 != 0
    {
        /* Zip64 end-of-cd record */
        memset(
            buff.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            56 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            buff.as_mut_ptr() as *mut libc::c_void,
            b"PK\x06\x06\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        archive_le64enc(
            buff.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
            44 as libc::c_int as uint64_t,
        );
        archive_le16enc(
            buff.as_mut_ptr().offset(12 as libc::c_int as isize) as *mut libc::c_void,
            45 as libc::c_int as uint16_t,
        );
        archive_le16enc(
            buff.as_mut_ptr().offset(14 as libc::c_int as isize) as *mut libc::c_void,
            45 as libc::c_int as uint16_t,
        );
        /* This is disk 0 of 0. */
        archive_le64enc(
            buff.as_mut_ptr().offset(24 as libc::c_int as isize) as *mut libc::c_void,
            (*zip).central_directory_entries,
        );
        archive_le64enc(
            buff.as_mut_ptr().offset(32 as libc::c_int as isize) as *mut libc::c_void,
            (*zip).central_directory_entries,
        );
        archive_le64enc(
            buff.as_mut_ptr().offset(40 as libc::c_int as isize) as *mut libc::c_void,
            (offset_end - offset_start) as uint64_t,
        );
        archive_le64enc(
            buff.as_mut_ptr().offset(48 as libc::c_int as isize) as *mut libc::c_void,
            offset_start as uint64_t,
        );
        ret = __archive_write_output(
            a,
            buff.as_mut_ptr() as *const libc::c_void,
            56 as libc::c_int as size_t,
        );
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        (*zip).written_bytes += 56 as libc::c_int as libc::c_long;
        /* Zip64 end-of-cd locator record. */
        memset(
            buff.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            20 as libc::c_int as libc::c_ulong,
        );
        memcpy(
            buff.as_mut_ptr() as *mut libc::c_void,
            b"PK\x06\x07\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        archive_le32enc(
            buff.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int as uint32_t,
        );
        archive_le64enc(
            buff.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut libc::c_void,
            offset_end as uint64_t,
        );
        archive_le32enc(
            buff.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
            1 as libc::c_int as uint32_t,
        );
        ret = __archive_write_output(
            a,
            buff.as_mut_ptr() as *const libc::c_void,
            20 as libc::c_int as size_t,
        );
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        (*zip).written_bytes += 20 as libc::c_int as libc::c_long
    }
    /* Format and write end of central directory. */
    memset(
        buff.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong,
    );
    memcpy(
        buff.as_mut_ptr() as *mut libc::c_void,
        b"PK\x05\x06\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    archive_le16enc(
        buff.as_mut_ptr().offset(8 as libc::c_int as isize) as *mut libc::c_void,
        if 0xffff as libc::c_uint as libc::c_ulong > (*zip).central_directory_entries {
            (*zip).central_directory_entries
        } else {
            0xffff as libc::c_uint as libc::c_ulong
        } as uint16_t,
    );
    archive_le16enc(
        buff.as_mut_ptr().offset(10 as libc::c_int as isize) as *mut libc::c_void,
        if 0xffff as libc::c_uint as libc::c_ulong > (*zip).central_directory_entries {
            (*zip).central_directory_entries
        } else {
            0xffff as libc::c_uint as libc::c_ulong
        } as uint16_t,
    );
    archive_le32enc(
        buff.as_mut_ptr().offset(12 as libc::c_int as isize) as *mut libc::c_void,
        if 0xffffffff as libc::c_longlong > (offset_end - offset_start) as libc::c_longlong {
            (offset_end - offset_start) as libc::c_longlong
        } else {
            0xffffffff as libc::c_longlong
        } as uint32_t,
    );
    archive_le32enc(
        buff.as_mut_ptr().offset(16 as libc::c_int as isize) as *mut libc::c_void,
        if 0xffffffff as libc::c_longlong > offset_start as libc::c_longlong {
            offset_start as libc::c_longlong
        } else {
            0xffffffff as libc::c_longlong
        } as uint32_t,
    );
    ret = __archive_write_output(
        a,
        buff.as_mut_ptr() as *const libc::c_void,
        22 as libc::c_int as size_t,
    );
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    (*zip).written_bytes += 22 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
unsafe extern "C" fn archive_write_zip_free(mut a: *mut archive_write) -> libc::c_int {
    let mut zip: *mut zip = 0 as *mut zip;
    let mut segment: *mut cd_segment = 0 as *mut cd_segment;
    zip = (*a).format_data as *mut zip;
    while !(*zip).central_directory.is_null() {
        segment = (*zip).central_directory;
        (*zip).central_directory = (*segment).next;
        free((*segment).buff as *mut libc::c_void);
        free(segment as *mut libc::c_void);
    }
    free((*zip).buf as *mut libc::c_void);
    archive_entry_free((*zip).entry);
    if (*zip).cctx_valid != 0 {
        __archive_cryptor
            .encrypto_aes_ctr_release
            .expect("non-null function pointer")(&mut (*zip).cctx);
    }
    if (*zip).hctx_valid != 0 {
        __archive_hmac
            .__hmac_sha1_cleanup
            .expect("non-null function pointer")(&mut (*zip).hctx);
    }
    /* TODO: Free opt_sconv, sconv_default */
    free(zip as *mut libc::c_void);
    (*a).format_data = NULL as *mut libc::c_void;
    return 0 as libc::c_int;
}
/* Convert into MSDOS-style date/time. */
unsafe extern "C" fn dos_time(unix_time: time_t) -> libc::c_uint {
    let mut t: *mut tm = 0 as *mut tm;
    let mut dt: libc::c_uint = 0;
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
    /* This will not preserve time when creating/extracting the archive
     * on two systems with different time zones. */
    t = localtime_r(&unix_time, &mut tmbuf);
    /* MSDOS-style date/time is only between 1980-01-01 and 2107-12-31 */
    if (*t).tm_year < 1980 as libc::c_int - 1900 as libc::c_int {
        /* Set minimum date/time '1980-01-01 00:00:00'. */
        dt = 0x210000 as libc::c_uint
    } else if (*t).tm_year > 2107 as libc::c_int - 1900 as libc::c_int {
        /* Set maximum date/time '2107-12-31 23:59:58'. */
        dt = 0xff9fbf7d as libc::c_uint
    } else {
        dt = 0 as libc::c_int as libc::c_uint;
        dt = dt.wrapping_add(
            (((*t).tm_year - 80 as libc::c_int & 0x7f as libc::c_int) << 9 as libc::c_int)
                as libc::c_uint,
        );
        dt = dt.wrapping_add(
            (((*t).tm_mon + 1 as libc::c_int & 0xf as libc::c_int) << 5 as libc::c_int)
                as libc::c_uint,
        );
        dt = dt.wrapping_add(((*t).tm_mday & 0x1f as libc::c_int) as libc::c_uint);
        dt <<= 16 as libc::c_int;
        dt = dt.wrapping_add(
            (((*t).tm_hour & 0x1f as libc::c_int) << 11 as libc::c_int) as libc::c_uint,
        );
        dt = dt.wrapping_add(
            (((*t).tm_min & 0x3f as libc::c_int) << 5 as libc::c_int) as libc::c_uint,
        );
        dt = dt
            .wrapping_add((((*t).tm_sec & 0x3e as libc::c_int) >> 1 as libc::c_int) as libc::c_uint)
        /* Only counting every 2 seconds. */
    } /* Space for the trailing / */
    return dt;
}
unsafe extern "C" fn path_length(mut entry: *mut archive_entry) -> size_t {
    let mut type_0: mode_t = 0;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    type_0 = archive_entry_filetype(entry);
    path = archive_entry_pathname(entry);
    if path.is_null() {
        return 0 as libc::c_int as size_t;
    }
    len = strlen(path);
    if type_0 == AE_IFDIR as mode_t
        && (*path.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
            || *path.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int
                != '/' as i32)
    {
        len = len.wrapping_add(1)
    }
    return len;
}
unsafe extern "C" fn write_path(
    mut entry: *mut archive_entry,
    mut archive: *mut archive_write,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut type_0: mode_t = 0;
    let mut written_bytes: size_t = 0;
    path = archive_entry_pathname(entry);
    type_0 = archive_entry_filetype(entry);
    written_bytes = 0 as libc::c_int as size_t;
    if path.is_null() {
        return -(30 as libc::c_int);
    }
    ret = __archive_write_output(archive, path as *const libc::c_void, strlen(path));
    if ret != ARCHIVE_OK {
        return -(30 as libc::c_int);
    }
    written_bytes = (written_bytes as libc::c_ulong).wrapping_add(strlen(path)) as size_t as size_t;
    /* Folders are recognized by a trailing slash. */
    if (type_0 == AE_IFDIR as mode_t) as libc::c_int
        & (*path.offset(strlen(path).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            != '/' as i32) as libc::c_int
        != 0
    {
        ret = __archive_write_output(
            archive,
            b"/\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        if ret != ARCHIVE_OK {
            return -(30 as libc::c_int);
        }
        written_bytes = (written_bytes as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
            as size_t
    }
    return written_bytes as libc::c_int;
}
unsafe extern "C" fn copy_path(mut entry: *mut archive_entry, mut p: *mut libc::c_uchar) {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut pathlen: size_t = 0;
    let mut type_0: mode_t = 0;
    path = archive_entry_pathname(entry);
    pathlen = strlen(path);
    type_0 = archive_entry_filetype(entry);
    memcpy(p as *mut libc::c_void, path as *const libc::c_void, pathlen);
    /* Folders are recognized by a trailing slash. */
    if type_0 == AE_IFDIR as mode_t
        && *path.offset(pathlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int
            != '/' as i32
    {
        *p.offset(pathlen as isize) = '/' as i32 as libc::c_uchar
    };
}
unsafe extern "C" fn get_sconv(
    mut a: *mut archive_write,
    mut zip: *mut zip,
) -> *mut archive_string_conv {
    if !(*zip).opt_sconv.is_null() {
        return (*zip).opt_sconv;
    }
    if (*zip).init_default_conversion == 0 {
        (*zip).sconv_default = archive_string_default_conversion_for_write(&mut (*a).archive);
        (*zip).init_default_conversion = 1 as libc::c_int
    }
    return (*zip).sconv_default;
}
/*
 Traditional PKWARE Decryption functions.
*/
unsafe extern "C" fn trad_enc_update_keys(mut ctx: *mut trad_enc_ctx, mut c: uint8_t) {
    let mut t: uint8_t = 0;
    (*ctx).keys[0 as libc::c_int as usize] = (crc32(
        (*ctx).keys[0 as libc::c_int as usize] as libc::c_ulong ^ 0xffffffff as libc::c_ulong,
        &mut c,
        1 as libc::c_int as uInt,
    ) ^ 0xffffffff as libc::c_ulong) as uint32_t;
    (*ctx).keys[1 as libc::c_int as usize] = ((*ctx).keys[1 as libc::c_int as usize]
        .wrapping_add((*ctx).keys[0 as libc::c_int as usize] & 0xff as libc::c_int as libc::c_uint)
        as libc::c_long
        * 134775813 as libc::c_long
        + 1 as libc::c_int as libc::c_long)
        as uint32_t;
    t = ((*ctx).keys[1 as libc::c_int as usize] >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as uint8_t;
    (*ctx).keys[2 as libc::c_int as usize] = (crc32(
        (*ctx).keys[2 as libc::c_int as usize] as libc::c_ulong ^ 0xffffffff as libc::c_ulong,
        &mut t,
        1 as libc::c_int as uInt,
    ) ^ 0xffffffff as libc::c_ulong) as uint32_t;
}
unsafe extern "C" fn trad_enc_decrypt_byte(mut ctx: *mut trad_enc_ctx) -> uint8_t {
    let mut temp: libc::c_uint =
        (*ctx).keys[2 as libc::c_int as usize] | 2 as libc::c_int as libc::c_uint;
    return ((temp.wrapping_mul(temp ^ 1 as libc::c_int as libc::c_uint) >> 8 as libc::c_int)
        as uint8_t as libc::c_int
        & 0xff as libc::c_int) as uint8_t;
}
unsafe extern "C" fn trad_enc_encrypt_update(
    mut ctx: *mut trad_enc_ctx,
    mut in_0: *const uint8_t,
    mut in_len: size_t,
    mut out: *mut uint8_t,
    mut out_len: size_t,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    let mut max: libc::c_uint = 0;
    max = if in_len < out_len { in_len } else { out_len } as libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while i < max {
        let mut t: uint8_t = *in_0.offset(i as isize);
        *out.offset(i as isize) =
            (t as libc::c_int ^ trad_enc_decrypt_byte(ctx) as libc::c_int) as uint8_t;
        trad_enc_update_keys(ctx, t);
        i = i.wrapping_add(1)
    }
    return i;
}
unsafe extern "C" fn trad_enc_init(
    mut ctx: *mut trad_enc_ctx,
    mut pw: *const libc::c_char,
    mut pw_len: size_t,
) -> libc::c_int {
    (*ctx).keys[0 as libc::c_int as usize] = 305419896 as libc::c_long as uint32_t;
    (*ctx).keys[1 as libc::c_int as usize] = 591751049 as libc::c_long as uint32_t;
    (*ctx).keys[2 as libc::c_int as usize] = 878082192 as libc::c_long as uint32_t;
    while pw_len != 0 {
        let fresh5 = pw;
        pw = pw.offset(1);
        trad_enc_update_keys(ctx, *fresh5 as uint8_t);
        pw_len = pw_len.wrapping_sub(1)
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_traditional_pkware_encryption_supported() -> libc::c_int {
    let mut key: [uint8_t; 12] = [0; 12];
    if archive_random(
        key.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[uint8_t; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) != ARCHIVE_OK
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn init_traditional_pkware_encryption(mut a: *mut archive_write) -> libc::c_int {
    let mut zip: *mut zip = (*a).format_data as *mut zip;
    let mut passphrase: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: [uint8_t; 12] = [0; 12];
    let mut key_encrypted: [uint8_t; 12] = [0; 12];
    let mut ret: libc::c_int = 0;
    passphrase = __archive_write_get_passphrase(a);
    if passphrase.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Encryption needs passphrase\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FAILED;
    }
    if archive_random(
        key.as_mut_ptr() as *mut libc::c_void,
        (::std::mem::size_of::<[uint8_t; 12]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) != ARCHIVE_OK
    {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Can\'t generate random number for encryption\x00" as *const u8 as *const libc::c_char,
        );
        return ARCHIVE_FATAL;
    }
    trad_enc_init(&mut (*zip).tctx, passphrase, strlen(passphrase));
    /* Set the last key code which will be used as a check code
     * for verifying passphrase in decryption. */
    key[(TRAD_HEADER_SIZE - 1 as libc::c_int) as usize] = (*zip).trad_chkdat;
    trad_enc_encrypt_update(
        &mut (*zip).tctx,
        key.as_mut_ptr(),
        TRAD_HEADER_SIZE as size_t,
        key_encrypted.as_mut_ptr(),
        TRAD_HEADER_SIZE as size_t,
    );
    /* Write encrypted keys in the top of the file content. */
    ret = __archive_write_output(
        a,
        key_encrypted.as_mut_ptr() as *const libc::c_void,
        TRAD_HEADER_SIZE as size_t,
    );
    if ret != ARCHIVE_OK {
        return ret;
    }
    (*zip).written_bytes += TRAD_HEADER_SIZE as libc::c_long;
    (*zip).entry_compressed_written += TRAD_HEADER_SIZE as libc::c_long;
    return ret;
}
unsafe extern "C" fn init_winzip_aes_encryption(mut a: *mut archive_write) -> libc::c_int {
    let mut zip: *mut zip = (*a).format_data as *mut zip;
    let mut passphrase: *const libc::c_char = 0 as *const libc::c_char;
    let mut key_len: size_t = 0;
    let mut salt_len: size_t = 0;
    let mut salt: [uint8_t; 18] = [0; 18];
    let mut derived_key: [uint8_t; 66] = [0; 66];
    let mut ret: libc::c_int = 0;
    passphrase = __archive_write_get_passphrase(a);
    if passphrase.is_null() {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Encryption needs passphrase\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    if (*zip).entry_encryption as libc::c_uint
        == ENCRYPTION_WINZIP_AES128 as libc::c_int as libc::c_uint
    {
        salt_len = 8 as libc::c_int as size_t;
        key_len = 16 as libc::c_int as size_t
    } else {
        /* AES 256 */
        salt_len = 16 as libc::c_int as size_t;
        key_len = 32 as libc::c_int as size_t
    }
    if archive_random(salt.as_mut_ptr() as *mut libc::c_void, salt_len) != ARCHIVE_OK {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Can\'t generate random number for encryption\x00" as *const u8 as *const libc::c_char,
        );
        return -(30 as libc::c_int);
    }
    __archive_cryptor
        .pbkdf2sha1
        .expect("non-null function pointer")(
        passphrase,
        strlen(passphrase),
        salt.as_mut_ptr(),
        salt_len,
        1000 as libc::c_int as libc::c_uint,
        derived_key.as_mut_ptr(),
        key_len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    );
    ret = __archive_cryptor
        .encrypto_aes_ctr_init
        .expect("non-null function pointer")(
        &mut (*zip).cctx, derived_key.as_mut_ptr(), key_len
    );
    if ret != 0 as libc::c_int {
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Decryption is unsupported due to lack of crypto library\x00" as *const u8
                as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    ret = __archive_hmac
        .__hmac_sha1_init
        .expect("non-null function pointer")(
        &mut (*zip).hctx,
        derived_key.as_mut_ptr().offset(key_len as isize),
        key_len,
    );
    if ret != 0 as libc::c_int {
        __archive_cryptor
            .encrypto_aes_ctr_release
            .expect("non-null function pointer")(&mut (*zip).cctx);
        archive_set_error(
            &mut (*a).archive as *mut archive,
            ARCHIVE_ERRNO_MISC,
            b"Failed to initialize HMAC-SHA1\x00" as *const u8 as *const libc::c_char,
        );
        return -(25 as libc::c_int);
    }
    /* Set a password verification value after the 'salt'. */
    salt[salt_len as usize] =
        derived_key[key_len.wrapping_mul(2 as libc::c_int as libc::c_ulong) as usize];
    salt[salt_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize] = derived_key[key_len
        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        as usize];
    /* Write encrypted keys in the top of the file content. */
    ret = __archive_write_output(
        a,
        salt.as_mut_ptr() as *const libc::c_void,
        salt_len.wrapping_add(2 as libc::c_int as libc::c_ulong),
    );
    if ret != ARCHIVE_OK {
        return ret;
    }
    (*zip).written_bytes = ((*zip).written_bytes as libc::c_ulong)
        .wrapping_add(salt_len.wrapping_add(2 as libc::c_int as libc::c_ulong))
        as int64_t as int64_t;
    (*zip).entry_compressed_written = ((*zip).entry_compressed_written as libc::c_ulong)
        .wrapping_add(salt_len.wrapping_add(2 as libc::c_int as libc::c_ulong))
        as int64_t as int64_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn is_winzip_aes_encryption_supported(
    mut encryption: libc::c_int,
) -> libc::c_int {
    let mut key_len: size_t = 0;
    let mut salt_len: size_t = 0;
    let mut salt: [uint8_t; 18] = [0; 18];
    let mut derived_key: [uint8_t; 66] = [0; 66];
    let mut cctx: archive_crypto_ctx = archive_crypto_ctx {
        ctx: 0 as *mut EVP_CIPHER_CTX,
        type_0: 0 as *const EVP_CIPHER,
        key: [0; 32],
        key_len: 0,
        nonce: [0; 16],
        encr_buf: [0; 16],
        encr_pos: 0,
    };
    let mut hctx: archive_hmac_sha1_ctx = 0 as *mut HMAC_CTX;
    let mut ret: libc::c_int = 0;
    if encryption == ENCRYPTION_WINZIP_AES128 as libc::c_int {
        salt_len = 8 as libc::c_int as size_t;
        key_len = 16 as libc::c_int as size_t
    } else {
        /* AES 256 */
        salt_len = 16 as libc::c_int as size_t;
        key_len = 32 as libc::c_int as size_t
    }
    if archive_random(salt.as_mut_ptr() as *mut libc::c_void, salt_len) != ARCHIVE_OK {
        return 0 as libc::c_int;
    }
    ret = __archive_cryptor
        .pbkdf2sha1
        .expect("non-null function pointer")(
        b"p\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        salt.as_mut_ptr(),
        salt_len,
        1000 as libc::c_int as libc::c_uint,
        derived_key.as_mut_ptr(),
        key_len
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    );
    if ret != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ret = __archive_cryptor
        .encrypto_aes_ctr_init
        .expect("non-null function pointer")(&mut cctx, derived_key.as_mut_ptr(), key_len);
    if ret != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ret = __archive_hmac
        .__hmac_sha1_init
        .expect("non-null function pointer")(
        &mut hctx,
        derived_key.as_mut_ptr().offset(key_len as isize),
        key_len,
    );
    __archive_cryptor
        .encrypto_aes_ctr_release
        .expect("non-null function pointer")(&mut cctx);
    if ret != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    __archive_hmac
        .__hmac_sha1_cleanup
        .expect("non-null function pointer")(&mut hctx);
    return 1 as libc::c_int;
}
