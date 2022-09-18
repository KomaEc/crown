use ::libc;
extern "C" {
    pub type archive_entry;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn iconv_open(__tocode: *const libc::c_char, __fromcode: *const libc::c_char) -> iconv_t;
    #[no_mangle]
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut libc::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut libc::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    #[no_mangle]
    fn iconv_close(__cd: iconv_t) -> libc::c_int;
    #[no_mangle]
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    #[no_mangle]
    fn __ctype_get_mb_cur_max() -> size_t;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    #[no_mangle]
    fn wmemmove(__s1: *mut wchar_t, __s2: *const wchar_t, __n: size_t) -> *mut wchar_t;
    #[no_mangle]
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    #[no_mangle]
    fn wcrtomb(__s: *mut libc::c_char, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    #[no_mangle]
    fn __archive_errx(retvalue: libc::c_int, msg: *const libc::c_char) -> !;
    #[no_mangle]
    fn archive_set_error(_: *mut archive, _err: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type iconv_t = *mut libc::c_void;
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
pub type wchar_t = libc::c_int;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type mbstate_t = __mbstate_t;
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
pub struct archive_string_conv {
    pub next: *mut archive_string_conv,
    pub from_charset: *mut libc::c_char,
    pub to_charset: *mut libc::c_char,
    pub from_cp: libc::c_uint,
    pub to_cp: libc::c_uint,
    pub same: libc::c_int,
    pub flag: libc::c_int,
    pub cd: iconv_t,
    pub cd_w: iconv_t,
    pub utftmp: archive_string,
    pub converter: [Option<
        unsafe extern "C" fn(
            _: *mut archive_string,
            _: *const libc::c_void,
            _: size_t,
            _: *mut archive_string_conv,
        ) -> libc::c_int,
    >; 2],
    pub nconverter: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_wstring {
    pub s: *mut wchar_t,
    pub length: size_t,
    pub buffer_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub uc: uint32_t,
    pub ccc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unicode_decomposition_table {
    pub nfc: uint32_t,
    pub cp1: uint32_t,
    pub cp2: uint32_t,
}
/*-
 * Copyright (c) 2011-2012 libarchive Project
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
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) AS IS'' AND ANY EXPRESS OR
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
 *
 */
/*
 * ATTENTION!
 *  This file is generated by build/utils/gen_archive_string_composition_h.sh
 *  from http://unicode.org/Public/6.0.0/ucd/UnicodeData.txt
 *
 *  See also http://unicode.org/report/tr15/
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unicode_composition_table {
    pub cp1: uint32_t,
    pub cp2: uint32_t,
    pub nfc: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_mstring {
    pub aes_mbs: archive_string,
    pub aes_utf8: archive_string,
    pub aes_wcs: archive_wstring,
    pub aes_mbs_in_locale: archive_string,
    pub aes_set: libc::c_int,
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
pub const ARCHIVE_ERRNO_MISC: libc::c_int = -(1 as libc::c_int);
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const EILSEQ: libc::c_int = 84 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const ENOMEM: libc::c_int = 12 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const CODESET_0: libc::c_int = CODESET as libc::c_int;
pub const MB_CUR_MAX: size_t = __ctype_get_mb_cur_max();
/*-
 * Copyright (c) 2002 Thomas Moestl <tmm@FreeBSD.org>
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
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * $FreeBSD: head/lib/libarchive/archive_endian.h 201085 2009-12-28 02:17:15Z kientzle $
 *
 * Borrowed from FreeBSD's <sys/endian.h>
 */
/* Note:  This is a purely internal header! */
/* Do not use this outside of libarchive internal code! */
/*
 * Disabling inline keyword for compilers known to choke on it:
 * - Watcom C++ in C code.  (For any version?)
 * - SGI MIPSpro
 * - Microsoft Visual C++ 6.0 (supposedly newer versions too)
 * - IBM VisualAge 6 (XL v6)
 * - Sun WorkShop C (SunPro) before 5.9
 */
/* Alignment-agnostic encode/decode bytestream to/from little/big endian. */
#[inline]
unsafe extern "C" fn archive_be16dec(mut pp: *const libc::c_void) -> uint16_t {
    let mut p: *const libc::c_uchar = pp as *const libc::c_uchar;
    /* Store into unsigned temporaries before left shifting, to avoid
    promotion to signed int and then left shifting into the sign bit,
    which is undefined behaviour. */
    let mut p1: libc::c_uint = *p.offset(1 as libc::c_int as isize) as libc::c_uint;
    let mut p0: libc::c_uint = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    return (p0 << 8 as libc::c_int | p1) as uint16_t;
}
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
unsafe extern "C" fn archive_be16enc(mut pp: *mut libc::c_void, mut u: uint16_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn archive_le16enc(mut pp: *mut libc::c_void, mut u: uint16_t) {
    let mut p: *mut libc::c_uchar = pp as *mut libc::c_uchar;
    *p.offset(0 as libc::c_int as isize) =
        (u as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *p.offset(1 as libc::c_int as isize) =
        (u as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
}
pub const AES_SET_UTF8: libc::c_int = 2 as libc::c_int;
pub const AES_SET_WCS: libc::c_int = 4 as libc::c_int;
pub const AES_SET_MBS: libc::c_int = 1 as libc::c_int;
pub const SCONV_SET_OPT_UTF8_LIBARCHIVE2X: libc::c_int = 1 as libc::c_int;
pub const SCONV_SET_OPT_NORMALIZATION_C: libc::c_int = 2 as libc::c_int;
pub const SCONV_SET_OPT_NORMALIZATION_D: libc::c_int = 4 as libc::c_int;
static mut u_composition_table: [unicode_composition_table; 931] = [
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x226e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3d as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2260 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3e as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x226f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xc0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xc1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xc2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0xc3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x100 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x102 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x226 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xc4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ea2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
            nfc: 0xc5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1cd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x200 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x202 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ea0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x325 as libc::c_int as uint32_t,
            nfc: 0x1e00 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x104 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x42 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e02 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x42 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e04 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x42 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e06 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x106 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x108 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x10a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x10c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0xc7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e0a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x10e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e0c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x1e10 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e12 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e0e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xc8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xc9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xca as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ebc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x112 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x114 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x116 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xcb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1eba as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x11a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x204 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x206 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1eb8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x228 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x118 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e18 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
            nfc: 0x1e1a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x46 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e1e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x11c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e20 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x11e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x120 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1e6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x122 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x124 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e22 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e26 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x21e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e24 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x1e28 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x32e as libc::c_int as uint32_t,
            nfc: 0x1e2a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xcc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xcd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xce as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x128 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x12a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x12c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x130 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xcf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ec8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1cf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x208 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x20a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1eca as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x12e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
            nfc: 0x1e2c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4a as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x134 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e30 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1e8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e32 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x136 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e34 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x139 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x13d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e36 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x13b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e3c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e3a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4d as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e3e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4d as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e40 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4d as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e42 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x143 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0xd1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e44 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x147 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e46 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x145 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e4a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e48 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xd2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xd3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xd4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0xd5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x14c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x14e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x22e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xd6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ece as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
            nfc: 0x150 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1d1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x20c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x20e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x31b as libc::c_int as uint32_t,
            nfc: 0x1a0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ecc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x1ea as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x50 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e54 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x50 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e56 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x154 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e58 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x158 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x210 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x212 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e5a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x156 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e5e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x15a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x15c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e60 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x160 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e62 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x326 as libc::c_int as uint32_t,
            nfc: 0x218 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x15e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e6a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x164 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e6c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x326 as libc::c_int as uint32_t,
            nfc: 0x21a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x162 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e70 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e6e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xd9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xda as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xdb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x168 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x16a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x16c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xdc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ee6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
            nfc: 0x16e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
            nfc: 0x170 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1d3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x214 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x216 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x31b as libc::c_int as uint32_t,
            nfc: 0x1af as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ee4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x324 as libc::c_int as uint32_t,
            nfc: 0x1e72 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x172 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e76 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
            nfc: 0x1e74 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x56 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1e7c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x56 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e7e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1e80 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e82 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x174 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e86 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e84 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e88 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x58 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e8a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x58 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e8c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ef2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xdd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x176 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ef8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x232 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e8e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x178 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ef6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ef4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x179 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x1e90 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x17b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x17d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e92 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e94 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xe0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xe1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xe2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0xe3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x101 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x103 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x227 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xe4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ea3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
            nfc: 0xe5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1ce as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x201 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x203 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ea1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x325 as libc::c_int as uint32_t,
            nfc: 0x1e01 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x105 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x62 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e03 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x62 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e05 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x62 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e07 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x107 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x109 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x10b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x10d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0xe7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e0b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x10f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e0d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x1e11 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e13 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e0f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xe8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xe9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xea as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ebd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x113 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x115 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x117 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xeb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ebb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x11b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x205 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x207 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1eb9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x229 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x119 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e19 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
            nfc: 0x1e1b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x66 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e1f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x11d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e21 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x11f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x121 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1e7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x123 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x125 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e23 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e27 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x21f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e25 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x1e29 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x32e as libc::c_int as uint32_t,
            nfc: 0x1e2b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e96 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xec as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xed as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xee as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x129 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x12b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x12d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xef as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ec9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1d0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x209 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x20b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ecb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x12f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
            nfc: 0x1e2d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6a as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x135 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6a as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1f0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e31 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1e9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e33 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x137 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x13a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x13e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e37 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x13c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e3d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e3b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6d as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e3f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6d as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e41 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6d as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e43 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x144 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0xf1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e45 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x148 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e47 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x146 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e4b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e49 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xf2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xf3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xf4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0xf5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x14d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x14f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x22f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xf6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ecf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
            nfc: 0x151 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1d2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x20d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x20f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x31b as libc::c_int as uint32_t,
            nfc: 0x1a1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ecd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x1eb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x70 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e55 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x70 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e57 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x155 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e59 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x159 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x211 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x213 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e5b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x157 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e5f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x15b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x15d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e61 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x161 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e63 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x326 as libc::c_int as uint32_t,
            nfc: 0x219 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x15f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e6b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e97 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x165 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e6d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x326 as libc::c_int as uint32_t,
            nfc: 0x21b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
            nfc: 0x163 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e71 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e6f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0xf9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xfa as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0xfb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x169 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x16b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x16d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xfc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ee7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
            nfc: 0x16f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
            nfc: 0x171 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1d4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x215 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
            nfc: 0x217 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x31b as libc::c_int as uint32_t,
            nfc: 0x1b0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ee5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x324 as libc::c_int as uint32_t,
            nfc: 0x1e73 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
            nfc: 0x173 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
            nfc: 0x1e77 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
            nfc: 0x1e75 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x76 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1e7d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x76 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e7f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1e81 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e83 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x175 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e87 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e85 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
            nfc: 0x1e98 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e89 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x78 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e8b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x78 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e8d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ef3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0xfd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x177 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ef9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x233 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e8f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0xff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ef7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
            nfc: 0x1e99 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ef5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x17a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x1e91 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x17c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x17e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1e93 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
            nfc: 0x1e95 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xa8 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fed as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xa8 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x385 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xa8 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fc1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc2 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ea6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc2 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ea4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc2 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1eaa as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc2 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ea8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc4 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1de as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1fa as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc6 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1fc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc6 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc7 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e08 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xca as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ec0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xca as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ebe as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xca as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ec4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xca as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ec2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xcf as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e2e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd4 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ed2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd4 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ed0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd4 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ed6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd4 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ed4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e4c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd5 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x22c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd5 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e4e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd6 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x22a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd8 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1fe as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xdc as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1db as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xdc as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1d7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xdc as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1d5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xdc as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1d9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe2 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ea7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe2 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ea5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe2 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1eab as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe2 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ea9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe4 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1df as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1fb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe6 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1fd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe6 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xe7 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e09 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xea as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ec1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xea as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ebf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xea as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ec5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xea as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ec3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xef as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e2f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf4 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ed3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf4 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ed1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf4 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ed7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf4 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ed5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e4d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf5 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x22d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf5 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e4f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf6 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x22b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xf8 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ff as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xfc as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1dc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xfc as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1d8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xfc as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1d6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xfc as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1da as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x102 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1eb0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x102 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1eae as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x102 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1eb4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x102 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1eb2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x103 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1eb1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x103 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1eaf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x103 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1eb5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x103 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1eb3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x112 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1e14 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x112 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e16 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x113 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1e15 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x113 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e17 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x14c as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1e50 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x14c as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e52 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x14d as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1e51 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x14d as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e53 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x15a as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e64 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x15b as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e65 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x160 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e66 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x161 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e67 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x168 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e78 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x169 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1e79 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x16a as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e7a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x16b as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x1e7b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x17f as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e9b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1edc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1eda as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ee0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1ede as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ee2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1edd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1edb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1ee1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1edf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ee3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1eea as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ee8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1eee as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1eec as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ef0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1eeb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1ee9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
            nfc: 0x1eef as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
            nfc: 0x1eed as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
            nfc: 0x1ef1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b7 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1ee as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ea as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1ec as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1eb as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1ed as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x226 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x227 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x228 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1e1c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x229 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1e1d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22e as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x230 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22f as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x231 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x292 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
            nfc: 0x1ef as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fba as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x386 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1fb9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1fb8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f08 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f09 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fbc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x395 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fc8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x395 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x388 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x395 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f18 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x395 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f19 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fca as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x389 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f28 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f29 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fcc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fda as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x38a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1fd9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1fd8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x3aa as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f38 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f39 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x39f as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ff8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x39f as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x38c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x39f as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f48 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x39f as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f49 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a1 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1fec as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fea as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x38e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1fe9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1fe8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x3ab as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f59 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1ffa as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x38f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f68 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f69 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1ffc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3ac as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fb4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3ae as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fc4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f70 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3ac as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1fb1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1fb0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f00 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f01 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fb6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fb3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b5 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f72 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3ad as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b5 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f10 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b5 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f11 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f74 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3ae as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f20 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f21 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fc6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fc3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f76 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3af as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1fd1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1fd0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x3ca as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f30 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f31 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fd6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3bf as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f78 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3bf as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3cc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3bf as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f40 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3bf as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f41 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c1 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1fe4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c1 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1fe5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f7a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3cd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1fe1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1fe0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x3cb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f50 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f51 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fe6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f7c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3ce as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
            nfc: 0x1f60 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
            nfc: 0x1f61 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1ff6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1ff3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3ca as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fd2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3ca as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x390 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3ca as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fd7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3cb as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fe2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3cb as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3b0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3cb as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fe7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3ce as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1ff4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3d2 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x3d3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3d2 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x3d4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x406 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x407 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x410 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x4d0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x410 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4d2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x413 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x403 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x415 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x400 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x415 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x4d6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x415 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x401 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x416 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x4c1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x416 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4dc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x417 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4de as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x418 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x40d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x418 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x4e2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x418 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x419 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x418 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4e4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41a as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x40c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x41e as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4e6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x423 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x4ee as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x423 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x40e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x423 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4f0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x423 as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
            nfc: 0x4f2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x427 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4f4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x42b as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4f8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x42d as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4ec as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x430 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x4d1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x430 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4d3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x433 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x453 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x435 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x450 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x435 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x4d7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x435 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x451 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x436 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x4c2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x436 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4dd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x437 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4df as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x438 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x45d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x438 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x4e3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x438 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x439 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x438 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4e5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x43a as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x45c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x43e as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4e7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x443 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x4ef as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x443 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x45e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x443 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4f1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x443 as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
            nfc: 0x4f3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x447 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4f5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x44b as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4f9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x44d as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4ed as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x456 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x457 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x474 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x476 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x475 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
            nfc: 0x477 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4d8 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4da as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4d9 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4db as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e8 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4ea as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x4e9 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
            nfc: 0x4eb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x627 as libc::c_int as uint32_t,
            cp2: 0x653 as libc::c_int as uint32_t,
            nfc: 0x622 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x627 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
            nfc: 0x623 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x627 as libc::c_int as uint32_t,
            cp2: 0x655 as libc::c_int as uint32_t,
            nfc: 0x625 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x648 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
            nfc: 0x624 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x64a as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
            nfc: 0x626 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6c1 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
            nfc: 0x6c2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6d2 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
            nfc: 0x6d3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x6d5 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
            nfc: 0x6c0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x928 as libc::c_int as uint32_t,
            cp2: 0x93c as libc::c_int as uint32_t,
            nfc: 0x929 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x930 as libc::c_int as uint32_t,
            cp2: 0x93c as libc::c_int as uint32_t,
            nfc: 0x931 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x933 as libc::c_int as uint32_t,
            cp2: 0x93c as libc::c_int as uint32_t,
            nfc: 0x934 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x9c7 as libc::c_int as uint32_t,
            cp2: 0x9be as libc::c_int as uint32_t,
            nfc: 0x9cb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x9c7 as libc::c_int as uint32_t,
            cp2: 0x9d7 as libc::c_int as uint32_t,
            nfc: 0x9cc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xb47 as libc::c_int as uint32_t,
            cp2: 0xb3e as libc::c_int as uint32_t,
            nfc: 0xb4b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xb47 as libc::c_int as uint32_t,
            cp2: 0xb56 as libc::c_int as uint32_t,
            nfc: 0xb48 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xb47 as libc::c_int as uint32_t,
            cp2: 0xb57 as libc::c_int as uint32_t,
            nfc: 0xb4c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xb92 as libc::c_int as uint32_t,
            cp2: 0xbd7 as libc::c_int as uint32_t,
            nfc: 0xb94 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xbc6 as libc::c_int as uint32_t,
            cp2: 0xbbe as libc::c_int as uint32_t,
            nfc: 0xbca as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xbc6 as libc::c_int as uint32_t,
            cp2: 0xbd7 as libc::c_int as uint32_t,
            nfc: 0xbcc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xbc7 as libc::c_int as uint32_t,
            cp2: 0xbbe as libc::c_int as uint32_t,
            nfc: 0xbcb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xc46 as libc::c_int as uint32_t,
            cp2: 0xc56 as libc::c_int as uint32_t,
            nfc: 0xc48 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xcbf as libc::c_int as uint32_t,
            cp2: 0xcd5 as libc::c_int as uint32_t,
            nfc: 0xcc0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xcc6 as libc::c_int as uint32_t,
            cp2: 0xcc2 as libc::c_int as uint32_t,
            nfc: 0xcca as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xcc6 as libc::c_int as uint32_t,
            cp2: 0xcd5 as libc::c_int as uint32_t,
            nfc: 0xcc7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xcc6 as libc::c_int as uint32_t,
            cp2: 0xcd6 as libc::c_int as uint32_t,
            nfc: 0xcc8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xcca as libc::c_int as uint32_t,
            cp2: 0xcd5 as libc::c_int as uint32_t,
            nfc: 0xccb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd46 as libc::c_int as uint32_t,
            cp2: 0xd3e as libc::c_int as uint32_t,
            nfc: 0xd4a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd46 as libc::c_int as uint32_t,
            cp2: 0xd57 as libc::c_int as uint32_t,
            nfc: 0xd4c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xd47 as libc::c_int as uint32_t,
            cp2: 0xd3e as libc::c_int as uint32_t,
            nfc: 0xd4b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xdd9 as libc::c_int as uint32_t,
            cp2: 0xdca as libc::c_int as uint32_t,
            nfc: 0xdda as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xdd9 as libc::c_int as uint32_t,
            cp2: 0xdcf as libc::c_int as uint32_t,
            nfc: 0xddc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xdd9 as libc::c_int as uint32_t,
            cp2: 0xddf as libc::c_int as uint32_t,
            nfc: 0xdde as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0xddc as libc::c_int as uint32_t,
            cp2: 0xdca as libc::c_int as uint32_t,
            nfc: 0xddd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1025 as libc::c_int as uint32_t,
            cp2: 0x102e as libc::c_int as uint32_t,
            nfc: 0x1026 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b05 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b06 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b07 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b08 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b09 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b0a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b0b as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b0c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b0d as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b0e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b11 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b12 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b3a as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b3b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b3c as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b3d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b3e as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b40 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b3f as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b41 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1b42 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
            nfc: 0x1b43 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1e36 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e38 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1e37 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e39 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1e5a as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e5c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1e5b as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
            nfc: 0x1e5d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1e62 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e68 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1e63 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
            nfc: 0x1e69 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ea0 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x1eac as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ea0 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1eb6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ea1 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x1ead as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ea1 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
            nfc: 0x1eb7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1eb8 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x1ec6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1eb9 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x1ec7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ecc as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x1ed8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ecd as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
            nfc: 0x1ed9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f00 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f02 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f00 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f04 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f00 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f06 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f00 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f80 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f01 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f03 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f01 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f05 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f01 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f07 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f01 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f81 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f02 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f82 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f03 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f83 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f04 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f84 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f05 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f85 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f06 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f86 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f07 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f87 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f08 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f0a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f08 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f0c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f08 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f0e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f08 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f88 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f09 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f0b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f09 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f0d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f09 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f0f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f09 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f89 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f0a as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f8a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f0b as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f8b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f0c as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f8c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f0d as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f8d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f0e as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f8e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f0f as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f8f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f10 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f12 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f10 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f14 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f11 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f13 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f11 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f15 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f18 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f1a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f18 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f1c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f19 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f1b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f19 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f1d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f20 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f22 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f20 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f24 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f20 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f26 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f20 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f90 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f21 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f23 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f21 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f25 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f21 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f27 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f21 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f91 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f22 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f92 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f23 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f93 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f24 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f94 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f25 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f95 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f26 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f96 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f27 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f97 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f28 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f2a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f28 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f2c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f28 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f2e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f28 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f98 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f29 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f2b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f29 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f2d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f29 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f2f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f29 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f99 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f2a as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f9a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f2b as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f9b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f2c as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f9c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f2d as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f9d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f2e as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f9e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f2f as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1f9f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f30 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f32 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f30 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f34 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f30 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f36 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f31 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f33 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f31 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f31 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f37 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f38 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f3a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f38 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f3c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f38 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f3e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f39 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f3b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f39 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f3d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f39 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f3f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f40 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f42 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f40 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f44 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f41 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f43 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f41 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f45 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f48 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f4a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f48 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f4c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f49 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f4b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f49 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f4d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f50 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f52 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f50 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f54 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f50 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f56 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f51 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f53 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f51 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f55 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f51 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f57 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f59 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f5b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f59 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f5d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f59 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f5f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f60 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f62 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f60 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f64 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f60 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f66 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f60 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f61 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f63 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f61 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f65 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f61 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f67 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f61 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f62 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f63 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f64 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f65 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f66 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f67 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f68 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f6a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f68 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f6c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f68 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f6e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f68 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f69 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1f6b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f69 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1f6d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f69 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1f6f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f69 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fa9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f6a as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1faa as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f6b as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fab as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f6c as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fac as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f6d as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fad as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f6e as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fae as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f6f as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1faf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f70 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fb2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f74 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fc2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1f7c as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1ff2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1fb6 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fb7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1fbf as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fcd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1fbf as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1fce as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1fbf as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fcf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1fc6 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1fc7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ff6 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
            nfc: 0x1ff7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ffe as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
            nfc: 0x1fdd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ffe as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
            nfc: 0x1fde as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1ffe as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
            nfc: 0x1fdf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2190 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x219a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2192 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x219b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2194 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x21ae as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x21d0 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x21cd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x21d2 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x21cf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x21d4 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x21ce as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2203 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2204 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2208 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2209 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x220b as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x220c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2223 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2224 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2225 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2226 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x223c as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2241 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2243 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2244 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2245 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2247 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2248 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2249 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x224d as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x226d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2261 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2262 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2264 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2270 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2265 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2271 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2272 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2274 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2273 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2275 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2276 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2278 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2277 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2279 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x227a as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2280 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x227b as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2281 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x227c as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22e0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x227d as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22e1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2282 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2284 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2283 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2285 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2286 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2288 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2287 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x2289 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2291 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22e2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x2292 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22e3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22a2 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22ac as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22a8 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22ad as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22a9 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22ae as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22ab as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22af as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22b2 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22ea as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22b3 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22eb as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22b4 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22ec as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x22b5 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
            nfc: 0x22ed as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3046 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3094 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x304b as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x304c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x304d as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x304e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x304f as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3050 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3051 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3052 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3053 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3054 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3055 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3056 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3057 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3058 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3059 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x305a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x305b as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x305c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x305d as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x305e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x305f as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3060 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3061 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3062 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3064 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3065 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3066 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3067 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3068 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3069 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x306f as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3070 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x306f as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x3071 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3072 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3073 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3072 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x3074 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3075 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3076 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3075 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x3077 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3078 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x3079 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x3078 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x307a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x307b as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x307c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x307b as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x307d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x309d as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x309e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30a6 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30f4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30ab as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30ac as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30ad as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30ae as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30af as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30b0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30b1 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30b2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30b3 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30b4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30b5 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30b6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30b7 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30b8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30b9 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30ba as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30bb as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30bc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30bd as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30be as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30bf as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30c0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30c1 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30c2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30c4 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30c5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30c6 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30c7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30c8 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30c9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30cf as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30d0 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30cf as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x30d1 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30d2 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30d3 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30d2 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x30d4 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30d5 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30d6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30d5 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x30d7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30d8 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30d9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30d8 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x30da as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30db as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30dc as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30db as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
            nfc: 0x30dd as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30ef as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30f7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30f0 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30f8 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30f1 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30f9 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30f2 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30fa as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x30fd as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
            nfc: 0x30fe as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x11099 as libc::c_int as uint32_t,
            cp2: 0x110ba as libc::c_int as uint32_t,
            nfc: 0x1109a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x1109b as libc::c_int as uint32_t,
            cp2: 0x110ba as libc::c_int as uint32_t,
            nfc: 0x1109c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_composition_table {
            cp1: 0x110a5 as libc::c_int as uint32_t,
            cp2: 0x110ba as libc::c_int as uint32_t,
            nfc: 0x110ab as libc::c_int as uint32_t,
        };
        init
    },
];
static mut u_decomposable_blocks: [libc::c_char; 467] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
];
/* The table of the value of Canonical Combining Class */
static mut ccc_val: [[libc::c_uchar; 16]; 115] = [
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        232 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        232 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        220 as libc::c_int as libc::c_uchar,
        202 as libc::c_int as libc::c_uchar,
        202 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        202 as libc::c_int as libc::c_uchar,
        202 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        232 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        233 as libc::c_int as libc::c_uchar,
        234 as libc::c_int as libc::c_uchar,
        234 as libc::c_int as libc::c_uchar,
        233 as libc::c_int as libc::c_uchar,
    ],
    [
        234 as libc::c_int as libc::c_uchar,
        234 as libc::c_int as libc::c_uchar,
        233 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        222 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        222 as libc::c_int as libc::c_uchar,
        228 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        10 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        22 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        30 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        27 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        29 as libc::c_int as libc::c_uchar,
        30 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
    ],
    [
        32 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        35 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        36 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        84 as libc::c_int as libc::c_uchar,
        91 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        103 as libc::c_int as libc::c_uchar,
        103 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        118 as libc::c_int as libc::c_uchar,
        118 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        129 as libc::c_int as libc::c_uchar,
        130 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        132 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        130 as libc::c_int as libc::c_uchar,
        130 as libc::c_int as libc::c_uchar,
        130 as libc::c_int as libc::c_uchar,
        130 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        130 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        228 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        222 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        234 as libc::c_int as libc::c_uchar,
        214 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        202 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        233 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        218 as libc::c_int as libc::c_uchar,
        228 as libc::c_int as libc::c_uchar,
        232 as libc::c_int as libc::c_uchar,
        222 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        226 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
    ],
    [
        216 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
    ],
    [
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
];
/* The index table to ccc_val[*][16] */
static mut ccc_val_index: [[libc::c_uchar; 16]; 39] = [
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        22 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        27 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        29 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        30 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        36 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        38 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        39 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        41 as libc::c_int as libc::c_uchar,
        42 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        43 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        44 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        45 as libc::c_int as libc::c_uchar,
        46 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        47 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        49 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
        52 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        53 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        54 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        55 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        56 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        57 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        61 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        62 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        67 as libc::c_int as libc::c_uchar,
        68 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        69 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        70 as libc::c_int as libc::c_uchar,
        71 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        72 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        73 as libc::c_int as libc::c_uchar,
        74 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        75 as libc::c_int as libc::c_uchar,
        76 as libc::c_int as libc::c_uchar,
        77 as libc::c_int as libc::c_uchar,
        78 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        79 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        81 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        82 as libc::c_int as libc::c_uchar,
        83 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        84 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        85 as libc::c_int as libc::c_uchar,
        86 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        87 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        88 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        89 as libc::c_int as libc::c_uchar,
        90 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        91 as libc::c_int as libc::c_uchar,
    ],
    [
        92 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        93 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        94 as libc::c_int as libc::c_uchar,
        95 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        96 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        97 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        98 as libc::c_int as libc::c_uchar,
        99 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        100 as libc::c_int as libc::c_uchar,
        101 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        102 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        103 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        104 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        105 as libc::c_int as libc::c_uchar,
    ],
    [
        106 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        108 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        109 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        110 as libc::c_int as libc::c_uchar,
        111 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        113 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
    [
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        114 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
    ],
];
/* The index table to ccc_val_index[*][16] */
static mut ccc_index: [libc::c_uchar; 467] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
];
static mut u_decomposition_table: [unicode_decomposition_table; 931] = [
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc0 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc1 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc2 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc3 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc4 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc5 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc7 as libc::c_int as uint32_t,
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc8 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc9 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xca as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xcb as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xcc as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xcd as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xce as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xcf as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd1 as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd2 as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd3 as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd4 as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd5 as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd6 as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd9 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xda as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xdb as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xdc as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xdd as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe0 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe1 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe2 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe3 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe4 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe5 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe7 as libc::c_int as uint32_t,
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe8 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xe9 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xea as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xeb as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xec as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xed as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xee as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xef as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xf1 as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xf2 as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xf3 as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xf4 as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xf5 as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xf6 as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xf9 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xfa as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xfb as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xfc as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xfd as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xff as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x100 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x101 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x102 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x103 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x104 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x105 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x106 as libc::c_int as uint32_t,
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x107 as libc::c_int as uint32_t,
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x108 as libc::c_int as uint32_t,
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x109 as libc::c_int as uint32_t,
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x10a as libc::c_int as uint32_t,
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x10b as libc::c_int as uint32_t,
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x10c as libc::c_int as uint32_t,
            cp1: 0x43 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x10d as libc::c_int as uint32_t,
            cp1: 0x63 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x10e as libc::c_int as uint32_t,
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x10f as libc::c_int as uint32_t,
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x112 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x113 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x114 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x115 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x116 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x117 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x118 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x119 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x11a as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x11b as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x11c as libc::c_int as uint32_t,
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x11d as libc::c_int as uint32_t,
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x11e as libc::c_int as uint32_t,
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x11f as libc::c_int as uint32_t,
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x120 as libc::c_int as uint32_t,
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x121 as libc::c_int as uint32_t,
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x122 as libc::c_int as uint32_t,
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x123 as libc::c_int as uint32_t,
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x124 as libc::c_int as uint32_t,
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x125 as libc::c_int as uint32_t,
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x128 as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x129 as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x12a as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x12b as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x12c as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x12d as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x12e as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x12f as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x130 as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x134 as libc::c_int as uint32_t,
            cp1: 0x4a as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x135 as libc::c_int as uint32_t,
            cp1: 0x6a as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x136 as libc::c_int as uint32_t,
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x137 as libc::c_int as uint32_t,
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x139 as libc::c_int as uint32_t,
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x13a as libc::c_int as uint32_t,
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x13b as libc::c_int as uint32_t,
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x13c as libc::c_int as uint32_t,
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x13d as libc::c_int as uint32_t,
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x13e as libc::c_int as uint32_t,
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x143 as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x144 as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x145 as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x146 as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x147 as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x148 as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x14c as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x14d as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x14e as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x14f as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x150 as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x151 as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x154 as libc::c_int as uint32_t,
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x155 as libc::c_int as uint32_t,
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x156 as libc::c_int as uint32_t,
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x157 as libc::c_int as uint32_t,
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x158 as libc::c_int as uint32_t,
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x159 as libc::c_int as uint32_t,
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x15a as libc::c_int as uint32_t,
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x15b as libc::c_int as uint32_t,
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x15c as libc::c_int as uint32_t,
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x15d as libc::c_int as uint32_t,
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x15e as libc::c_int as uint32_t,
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x15f as libc::c_int as uint32_t,
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x160 as libc::c_int as uint32_t,
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x161 as libc::c_int as uint32_t,
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x162 as libc::c_int as uint32_t,
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x163 as libc::c_int as uint32_t,
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x164 as libc::c_int as uint32_t,
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x165 as libc::c_int as uint32_t,
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x168 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x169 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x16a as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x16b as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x16c as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x16d as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x16e as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x16f as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x170 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x171 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x172 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x173 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x174 as libc::c_int as uint32_t,
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x175 as libc::c_int as uint32_t,
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x176 as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x177 as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x178 as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x179 as libc::c_int as uint32_t,
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x17a as libc::c_int as uint32_t,
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x17b as libc::c_int as uint32_t,
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x17c as libc::c_int as uint32_t,
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x17d as libc::c_int as uint32_t,
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x17e as libc::c_int as uint32_t,
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1a0 as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x31b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1a1 as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x31b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1af as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x31b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b0 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x31b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1cd as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ce as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1cf as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d0 as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d1 as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d2 as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d3 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d4 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d5 as libc::c_int as uint32_t,
            cp1: 0xdc as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d6 as libc::c_int as uint32_t,
            cp1: 0xfc as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d7 as libc::c_int as uint32_t,
            cp1: 0xdc as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d8 as libc::c_int as uint32_t,
            cp1: 0xfc as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1d9 as libc::c_int as uint32_t,
            cp1: 0xdc as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1da as libc::c_int as uint32_t,
            cp1: 0xfc as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1db as libc::c_int as uint32_t,
            cp1: 0xdc as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1dc as libc::c_int as uint32_t,
            cp1: 0xfc as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1de as libc::c_int as uint32_t,
            cp1: 0xc4 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1df as libc::c_int as uint32_t,
            cp1: 0xe4 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e0 as libc::c_int as uint32_t,
            cp1: 0x226 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e1 as libc::c_int as uint32_t,
            cp1: 0x227 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e2 as libc::c_int as uint32_t,
            cp1: 0xc6 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e3 as libc::c_int as uint32_t,
            cp1: 0xe6 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e6 as libc::c_int as uint32_t,
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e7 as libc::c_int as uint32_t,
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e8 as libc::c_int as uint32_t,
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e9 as libc::c_int as uint32_t,
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x328 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec as libc::c_int as uint32_t,
            cp1: 0x1ea as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed as libc::c_int as uint32_t,
            cp1: 0x1eb as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee as libc::c_int as uint32_t,
            cp1: 0x1b7 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef as libc::c_int as uint32_t,
            cp1: 0x292 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f0 as libc::c_int as uint32_t,
            cp1: 0x6a as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f4 as libc::c_int as uint32_t,
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f5 as libc::c_int as uint32_t,
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f8 as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f9 as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa as libc::c_int as uint32_t,
            cp1: 0xc5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb as libc::c_int as uint32_t,
            cp1: 0xe5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fc as libc::c_int as uint32_t,
            cp1: 0xc6 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fd as libc::c_int as uint32_t,
            cp1: 0xe6 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe as libc::c_int as uint32_t,
            cp1: 0xd8 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ff as libc::c_int as uint32_t,
            cp1: 0xf8 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x200 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x201 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x202 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x203 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x204 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x205 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x206 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x207 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x208 as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x209 as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x20a as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x20b as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x20c as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x20d as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x20e as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x20f as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x210 as libc::c_int as uint32_t,
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x211 as libc::c_int as uint32_t,
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x212 as libc::c_int as uint32_t,
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x213 as libc::c_int as uint32_t,
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x214 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x215 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x216 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x217 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x311 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x218 as libc::c_int as uint32_t,
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x326 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x219 as libc::c_int as uint32_t,
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x326 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x21a as libc::c_int as uint32_t,
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x326 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x21b as libc::c_int as uint32_t,
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x326 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x21e as libc::c_int as uint32_t,
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x21f as libc::c_int as uint32_t,
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x30c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x226 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x227 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x228 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x229 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22a as libc::c_int as uint32_t,
            cp1: 0xd6 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22b as libc::c_int as uint32_t,
            cp1: 0xf6 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22c as libc::c_int as uint32_t,
            cp1: 0xd5 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22d as libc::c_int as uint32_t,
            cp1: 0xf5 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22e as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22f as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x230 as libc::c_int as uint32_t,
            cp1: 0x22e as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x231 as libc::c_int as uint32_t,
            cp1: 0x22f as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x232 as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x233 as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x385 as libc::c_int as uint32_t,
            cp1: 0xa8 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x386 as libc::c_int as uint32_t,
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x388 as libc::c_int as uint32_t,
            cp1: 0x395 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x389 as libc::c_int as uint32_t,
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x38a as libc::c_int as uint32_t,
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x38c as libc::c_int as uint32_t,
            cp1: 0x39f as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x38e as libc::c_int as uint32_t,
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x38f as libc::c_int as uint32_t,
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x390 as libc::c_int as uint32_t,
            cp1: 0x3ca as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3aa as libc::c_int as uint32_t,
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3ab as libc::c_int as uint32_t,
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3ac as libc::c_int as uint32_t,
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3ad as libc::c_int as uint32_t,
            cp1: 0x3b5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3ae as libc::c_int as uint32_t,
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3af as libc::c_int as uint32_t,
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3b0 as libc::c_int as uint32_t,
            cp1: 0x3cb as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3ca as libc::c_int as uint32_t,
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3cb as libc::c_int as uint32_t,
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3cc as libc::c_int as uint32_t,
            cp1: 0x3bf as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3cd as libc::c_int as uint32_t,
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3ce as libc::c_int as uint32_t,
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3d3 as libc::c_int as uint32_t,
            cp1: 0x3d2 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3d4 as libc::c_int as uint32_t,
            cp1: 0x3d2 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x400 as libc::c_int as uint32_t,
            cp1: 0x415 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x401 as libc::c_int as uint32_t,
            cp1: 0x415 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x403 as libc::c_int as uint32_t,
            cp1: 0x413 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x407 as libc::c_int as uint32_t,
            cp1: 0x406 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x40c as libc::c_int as uint32_t,
            cp1: 0x41a as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x40d as libc::c_int as uint32_t,
            cp1: 0x418 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x40e as libc::c_int as uint32_t,
            cp1: 0x423 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x419 as libc::c_int as uint32_t,
            cp1: 0x418 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x439 as libc::c_int as uint32_t,
            cp1: 0x438 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x450 as libc::c_int as uint32_t,
            cp1: 0x435 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x451 as libc::c_int as uint32_t,
            cp1: 0x435 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x453 as libc::c_int as uint32_t,
            cp1: 0x433 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x457 as libc::c_int as uint32_t,
            cp1: 0x456 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x45c as libc::c_int as uint32_t,
            cp1: 0x43a as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x45d as libc::c_int as uint32_t,
            cp1: 0x438 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x45e as libc::c_int as uint32_t,
            cp1: 0x443 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x476 as libc::c_int as uint32_t,
            cp1: 0x474 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x477 as libc::c_int as uint32_t,
            cp1: 0x475 as libc::c_int as uint32_t,
            cp2: 0x30f as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4c1 as libc::c_int as uint32_t,
            cp1: 0x416 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4c2 as libc::c_int as uint32_t,
            cp1: 0x436 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4d0 as libc::c_int as uint32_t,
            cp1: 0x410 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4d1 as libc::c_int as uint32_t,
            cp1: 0x430 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4d2 as libc::c_int as uint32_t,
            cp1: 0x410 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4d3 as libc::c_int as uint32_t,
            cp1: 0x430 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4d6 as libc::c_int as uint32_t,
            cp1: 0x415 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4d7 as libc::c_int as uint32_t,
            cp1: 0x435 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4da as libc::c_int as uint32_t,
            cp1: 0x4d8 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4db as libc::c_int as uint32_t,
            cp1: 0x4d9 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4dc as libc::c_int as uint32_t,
            cp1: 0x416 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4dd as libc::c_int as uint32_t,
            cp1: 0x436 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4de as libc::c_int as uint32_t,
            cp1: 0x417 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4df as libc::c_int as uint32_t,
            cp1: 0x437 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4e2 as libc::c_int as uint32_t,
            cp1: 0x418 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4e3 as libc::c_int as uint32_t,
            cp1: 0x438 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4e4 as libc::c_int as uint32_t,
            cp1: 0x418 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4e5 as libc::c_int as uint32_t,
            cp1: 0x438 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4e6 as libc::c_int as uint32_t,
            cp1: 0x41e as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4e7 as libc::c_int as uint32_t,
            cp1: 0x43e as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4ea as libc::c_int as uint32_t,
            cp1: 0x4e8 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4eb as libc::c_int as uint32_t,
            cp1: 0x4e9 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4ec as libc::c_int as uint32_t,
            cp1: 0x42d as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4ed as libc::c_int as uint32_t,
            cp1: 0x44d as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4ee as libc::c_int as uint32_t,
            cp1: 0x423 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4ef as libc::c_int as uint32_t,
            cp1: 0x443 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4f0 as libc::c_int as uint32_t,
            cp1: 0x423 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4f1 as libc::c_int as uint32_t,
            cp1: 0x443 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4f2 as libc::c_int as uint32_t,
            cp1: 0x423 as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4f3 as libc::c_int as uint32_t,
            cp1: 0x443 as libc::c_int as uint32_t,
            cp2: 0x30b as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4f4 as libc::c_int as uint32_t,
            cp1: 0x427 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4f5 as libc::c_int as uint32_t,
            cp1: 0x447 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4f8 as libc::c_int as uint32_t,
            cp1: 0x42b as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x4f9 as libc::c_int as uint32_t,
            cp1: 0x44b as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x622 as libc::c_int as uint32_t,
            cp1: 0x627 as libc::c_int as uint32_t,
            cp2: 0x653 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x623 as libc::c_int as uint32_t,
            cp1: 0x627 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x624 as libc::c_int as uint32_t,
            cp1: 0x648 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x625 as libc::c_int as uint32_t,
            cp1: 0x627 as libc::c_int as uint32_t,
            cp2: 0x655 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x626 as libc::c_int as uint32_t,
            cp1: 0x64a as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x6c0 as libc::c_int as uint32_t,
            cp1: 0x6d5 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x6c2 as libc::c_int as uint32_t,
            cp1: 0x6c1 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x6d3 as libc::c_int as uint32_t,
            cp1: 0x6d2 as libc::c_int as uint32_t,
            cp2: 0x654 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x929 as libc::c_int as uint32_t,
            cp1: 0x928 as libc::c_int as uint32_t,
            cp2: 0x93c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x931 as libc::c_int as uint32_t,
            cp1: 0x930 as libc::c_int as uint32_t,
            cp2: 0x93c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x934 as libc::c_int as uint32_t,
            cp1: 0x933 as libc::c_int as uint32_t,
            cp2: 0x93c as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x9cb as libc::c_int as uint32_t,
            cp1: 0x9c7 as libc::c_int as uint32_t,
            cp2: 0x9be as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x9cc as libc::c_int as uint32_t,
            cp1: 0x9c7 as libc::c_int as uint32_t,
            cp2: 0x9d7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xb48 as libc::c_int as uint32_t,
            cp1: 0xb47 as libc::c_int as uint32_t,
            cp2: 0xb56 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xb4b as libc::c_int as uint32_t,
            cp1: 0xb47 as libc::c_int as uint32_t,
            cp2: 0xb3e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xb4c as libc::c_int as uint32_t,
            cp1: 0xb47 as libc::c_int as uint32_t,
            cp2: 0xb57 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xb94 as libc::c_int as uint32_t,
            cp1: 0xb92 as libc::c_int as uint32_t,
            cp2: 0xbd7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xbca as libc::c_int as uint32_t,
            cp1: 0xbc6 as libc::c_int as uint32_t,
            cp2: 0xbbe as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xbcb as libc::c_int as uint32_t,
            cp1: 0xbc7 as libc::c_int as uint32_t,
            cp2: 0xbbe as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xbcc as libc::c_int as uint32_t,
            cp1: 0xbc6 as libc::c_int as uint32_t,
            cp2: 0xbd7 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xc48 as libc::c_int as uint32_t,
            cp1: 0xc46 as libc::c_int as uint32_t,
            cp2: 0xc56 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xcc0 as libc::c_int as uint32_t,
            cp1: 0xcbf as libc::c_int as uint32_t,
            cp2: 0xcd5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xcc7 as libc::c_int as uint32_t,
            cp1: 0xcc6 as libc::c_int as uint32_t,
            cp2: 0xcd5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xcc8 as libc::c_int as uint32_t,
            cp1: 0xcc6 as libc::c_int as uint32_t,
            cp2: 0xcd6 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xcca as libc::c_int as uint32_t,
            cp1: 0xcc6 as libc::c_int as uint32_t,
            cp2: 0xcc2 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xccb as libc::c_int as uint32_t,
            cp1: 0xcca as libc::c_int as uint32_t,
            cp2: 0xcd5 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd4a as libc::c_int as uint32_t,
            cp1: 0xd46 as libc::c_int as uint32_t,
            cp2: 0xd3e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd4b as libc::c_int as uint32_t,
            cp1: 0xd47 as libc::c_int as uint32_t,
            cp2: 0xd3e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xd4c as libc::c_int as uint32_t,
            cp1: 0xd46 as libc::c_int as uint32_t,
            cp2: 0xd57 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xdda as libc::c_int as uint32_t,
            cp1: 0xdd9 as libc::c_int as uint32_t,
            cp2: 0xdca as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xddc as libc::c_int as uint32_t,
            cp1: 0xdd9 as libc::c_int as uint32_t,
            cp2: 0xdcf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xddd as libc::c_int as uint32_t,
            cp1: 0xddc as libc::c_int as uint32_t,
            cp2: 0xdca as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0xdde as libc::c_int as uint32_t,
            cp1: 0xdd9 as libc::c_int as uint32_t,
            cp2: 0xddf as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1026 as libc::c_int as uint32_t,
            cp1: 0x1025 as libc::c_int as uint32_t,
            cp2: 0x102e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b06 as libc::c_int as uint32_t,
            cp1: 0x1b05 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b08 as libc::c_int as uint32_t,
            cp1: 0x1b07 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b0a as libc::c_int as uint32_t,
            cp1: 0x1b09 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b0c as libc::c_int as uint32_t,
            cp1: 0x1b0b as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b0e as libc::c_int as uint32_t,
            cp1: 0x1b0d as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b12 as libc::c_int as uint32_t,
            cp1: 0x1b11 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b3b as libc::c_int as uint32_t,
            cp1: 0x1b3a as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b3d as libc::c_int as uint32_t,
            cp1: 0x1b3c as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b40 as libc::c_int as uint32_t,
            cp1: 0x1b3e as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b41 as libc::c_int as uint32_t,
            cp1: 0x1b3f as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1b43 as libc::c_int as uint32_t,
            cp1: 0x1b42 as libc::c_int as uint32_t,
            cp2: 0x1b35 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e00 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x325 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e01 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x325 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e02 as libc::c_int as uint32_t,
            cp1: 0x42 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e03 as libc::c_int as uint32_t,
            cp1: 0x62 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e04 as libc::c_int as uint32_t,
            cp1: 0x42 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e05 as libc::c_int as uint32_t,
            cp1: 0x62 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e06 as libc::c_int as uint32_t,
            cp1: 0x42 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e07 as libc::c_int as uint32_t,
            cp1: 0x62 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e08 as libc::c_int as uint32_t,
            cp1: 0xc7 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e09 as libc::c_int as uint32_t,
            cp1: 0xe7 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e0a as libc::c_int as uint32_t,
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e0b as libc::c_int as uint32_t,
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e0c as libc::c_int as uint32_t,
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e0d as libc::c_int as uint32_t,
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e0e as libc::c_int as uint32_t,
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e0f as libc::c_int as uint32_t,
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e10 as libc::c_int as uint32_t,
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e11 as libc::c_int as uint32_t,
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e12 as libc::c_int as uint32_t,
            cp1: 0x44 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e13 as libc::c_int as uint32_t,
            cp1: 0x64 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e14 as libc::c_int as uint32_t,
            cp1: 0x112 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e15 as libc::c_int as uint32_t,
            cp1: 0x113 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e16 as libc::c_int as uint32_t,
            cp1: 0x112 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e17 as libc::c_int as uint32_t,
            cp1: 0x113 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e18 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e19 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e1a as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e1b as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e1c as libc::c_int as uint32_t,
            cp1: 0x228 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e1d as libc::c_int as uint32_t,
            cp1: 0x229 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e1e as libc::c_int as uint32_t,
            cp1: 0x46 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e1f as libc::c_int as uint32_t,
            cp1: 0x66 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e20 as libc::c_int as uint32_t,
            cp1: 0x47 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e21 as libc::c_int as uint32_t,
            cp1: 0x67 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e22 as libc::c_int as uint32_t,
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e23 as libc::c_int as uint32_t,
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e24 as libc::c_int as uint32_t,
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e25 as libc::c_int as uint32_t,
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e26 as libc::c_int as uint32_t,
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e27 as libc::c_int as uint32_t,
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e28 as libc::c_int as uint32_t,
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e29 as libc::c_int as uint32_t,
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x327 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e2a as libc::c_int as uint32_t,
            cp1: 0x48 as libc::c_int as uint32_t,
            cp2: 0x32e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e2b as libc::c_int as uint32_t,
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x32e as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e2c as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e2d as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e2e as libc::c_int as uint32_t,
            cp1: 0xcf as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e2f as libc::c_int as uint32_t,
            cp1: 0xef as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e30 as libc::c_int as uint32_t,
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e31 as libc::c_int as uint32_t,
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e32 as libc::c_int as uint32_t,
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e33 as libc::c_int as uint32_t,
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e34 as libc::c_int as uint32_t,
            cp1: 0x4b as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e35 as libc::c_int as uint32_t,
            cp1: 0x6b as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e36 as libc::c_int as uint32_t,
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e37 as libc::c_int as uint32_t,
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e38 as libc::c_int as uint32_t,
            cp1: 0x1e36 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e39 as libc::c_int as uint32_t,
            cp1: 0x1e37 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e3a as libc::c_int as uint32_t,
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e3b as libc::c_int as uint32_t,
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e3c as libc::c_int as uint32_t,
            cp1: 0x4c as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e3d as libc::c_int as uint32_t,
            cp1: 0x6c as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e3e as libc::c_int as uint32_t,
            cp1: 0x4d as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e3f as libc::c_int as uint32_t,
            cp1: 0x6d as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e40 as libc::c_int as uint32_t,
            cp1: 0x4d as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e41 as libc::c_int as uint32_t,
            cp1: 0x6d as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e42 as libc::c_int as uint32_t,
            cp1: 0x4d as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e43 as libc::c_int as uint32_t,
            cp1: 0x6d as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e44 as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e45 as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e46 as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e47 as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e48 as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e49 as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e4a as libc::c_int as uint32_t,
            cp1: 0x4e as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e4b as libc::c_int as uint32_t,
            cp1: 0x6e as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e4c as libc::c_int as uint32_t,
            cp1: 0xd5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e4d as libc::c_int as uint32_t,
            cp1: 0xf5 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e4e as libc::c_int as uint32_t,
            cp1: 0xd5 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e4f as libc::c_int as uint32_t,
            cp1: 0xf5 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e50 as libc::c_int as uint32_t,
            cp1: 0x14c as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e51 as libc::c_int as uint32_t,
            cp1: 0x14d as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e52 as libc::c_int as uint32_t,
            cp1: 0x14c as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e53 as libc::c_int as uint32_t,
            cp1: 0x14d as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e54 as libc::c_int as uint32_t,
            cp1: 0x50 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e55 as libc::c_int as uint32_t,
            cp1: 0x70 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e56 as libc::c_int as uint32_t,
            cp1: 0x50 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e57 as libc::c_int as uint32_t,
            cp1: 0x70 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e58 as libc::c_int as uint32_t,
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e59 as libc::c_int as uint32_t,
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e5a as libc::c_int as uint32_t,
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e5b as libc::c_int as uint32_t,
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e5c as libc::c_int as uint32_t,
            cp1: 0x1e5a as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e5d as libc::c_int as uint32_t,
            cp1: 0x1e5b as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e5e as libc::c_int as uint32_t,
            cp1: 0x52 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e5f as libc::c_int as uint32_t,
            cp1: 0x72 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e60 as libc::c_int as uint32_t,
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e61 as libc::c_int as uint32_t,
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e62 as libc::c_int as uint32_t,
            cp1: 0x53 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e63 as libc::c_int as uint32_t,
            cp1: 0x73 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e64 as libc::c_int as uint32_t,
            cp1: 0x15a as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e65 as libc::c_int as uint32_t,
            cp1: 0x15b as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e66 as libc::c_int as uint32_t,
            cp1: 0x160 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e67 as libc::c_int as uint32_t,
            cp1: 0x161 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e68 as libc::c_int as uint32_t,
            cp1: 0x1e62 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e69 as libc::c_int as uint32_t,
            cp1: 0x1e63 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e6a as libc::c_int as uint32_t,
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e6b as libc::c_int as uint32_t,
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e6c as libc::c_int as uint32_t,
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e6d as libc::c_int as uint32_t,
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e6e as libc::c_int as uint32_t,
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e6f as libc::c_int as uint32_t,
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e70 as libc::c_int as uint32_t,
            cp1: 0x54 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e71 as libc::c_int as uint32_t,
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e72 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x324 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e73 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x324 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e74 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e75 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x330 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e76 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e77 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x32d as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e78 as libc::c_int as uint32_t,
            cp1: 0x168 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e79 as libc::c_int as uint32_t,
            cp1: 0x169 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e7a as libc::c_int as uint32_t,
            cp1: 0x16a as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e7b as libc::c_int as uint32_t,
            cp1: 0x16b as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e7c as libc::c_int as uint32_t,
            cp1: 0x56 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e7d as libc::c_int as uint32_t,
            cp1: 0x76 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e7e as libc::c_int as uint32_t,
            cp1: 0x56 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e7f as libc::c_int as uint32_t,
            cp1: 0x76 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e80 as libc::c_int as uint32_t,
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e81 as libc::c_int as uint32_t,
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e82 as libc::c_int as uint32_t,
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e83 as libc::c_int as uint32_t,
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e84 as libc::c_int as uint32_t,
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e85 as libc::c_int as uint32_t,
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e86 as libc::c_int as uint32_t,
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e87 as libc::c_int as uint32_t,
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e88 as libc::c_int as uint32_t,
            cp1: 0x57 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e89 as libc::c_int as uint32_t,
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e8a as libc::c_int as uint32_t,
            cp1: 0x58 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e8b as libc::c_int as uint32_t,
            cp1: 0x78 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e8c as libc::c_int as uint32_t,
            cp1: 0x58 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e8d as libc::c_int as uint32_t,
            cp1: 0x78 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e8e as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e8f as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e90 as libc::c_int as uint32_t,
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e91 as libc::c_int as uint32_t,
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e92 as libc::c_int as uint32_t,
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e93 as libc::c_int as uint32_t,
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e94 as libc::c_int as uint32_t,
            cp1: 0x5a as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e95 as libc::c_int as uint32_t,
            cp1: 0x7a as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e96 as libc::c_int as uint32_t,
            cp1: 0x68 as libc::c_int as uint32_t,
            cp2: 0x331 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e97 as libc::c_int as uint32_t,
            cp1: 0x74 as libc::c_int as uint32_t,
            cp2: 0x308 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e98 as libc::c_int as uint32_t,
            cp1: 0x77 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e99 as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x30a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1e9b as libc::c_int as uint32_t,
            cp1: 0x17f as libc::c_int as uint32_t,
            cp2: 0x307 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea0 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea1 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea2 as libc::c_int as uint32_t,
            cp1: 0x41 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea3 as libc::c_int as uint32_t,
            cp1: 0x61 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea4 as libc::c_int as uint32_t,
            cp1: 0xc2 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea5 as libc::c_int as uint32_t,
            cp1: 0xe2 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea6 as libc::c_int as uint32_t,
            cp1: 0xc2 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea7 as libc::c_int as uint32_t,
            cp1: 0xe2 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea8 as libc::c_int as uint32_t,
            cp1: 0xc2 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ea9 as libc::c_int as uint32_t,
            cp1: 0xe2 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eaa as libc::c_int as uint32_t,
            cp1: 0xc2 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eab as libc::c_int as uint32_t,
            cp1: 0xe2 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eac as libc::c_int as uint32_t,
            cp1: 0x1ea0 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ead as libc::c_int as uint32_t,
            cp1: 0x1ea1 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eae as libc::c_int as uint32_t,
            cp1: 0x102 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eaf as libc::c_int as uint32_t,
            cp1: 0x103 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb0 as libc::c_int as uint32_t,
            cp1: 0x102 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb1 as libc::c_int as uint32_t,
            cp1: 0x103 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb2 as libc::c_int as uint32_t,
            cp1: 0x102 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb3 as libc::c_int as uint32_t,
            cp1: 0x103 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb4 as libc::c_int as uint32_t,
            cp1: 0x102 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb5 as libc::c_int as uint32_t,
            cp1: 0x103 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb6 as libc::c_int as uint32_t,
            cp1: 0x1ea0 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb7 as libc::c_int as uint32_t,
            cp1: 0x1ea1 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb8 as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eb9 as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eba as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ebb as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ebc as libc::c_int as uint32_t,
            cp1: 0x45 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ebd as libc::c_int as uint32_t,
            cp1: 0x65 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ebe as libc::c_int as uint32_t,
            cp1: 0xca as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ebf as libc::c_int as uint32_t,
            cp1: 0xea as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec0 as libc::c_int as uint32_t,
            cp1: 0xca as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec1 as libc::c_int as uint32_t,
            cp1: 0xea as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec2 as libc::c_int as uint32_t,
            cp1: 0xca as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec3 as libc::c_int as uint32_t,
            cp1: 0xea as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec4 as libc::c_int as uint32_t,
            cp1: 0xca as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec5 as libc::c_int as uint32_t,
            cp1: 0xea as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec6 as libc::c_int as uint32_t,
            cp1: 0x1eb8 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec7 as libc::c_int as uint32_t,
            cp1: 0x1eb9 as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec8 as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ec9 as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eca as libc::c_int as uint32_t,
            cp1: 0x49 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ecb as libc::c_int as uint32_t,
            cp1: 0x69 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ecc as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ecd as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ece as libc::c_int as uint32_t,
            cp1: 0x4f as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ecf as libc::c_int as uint32_t,
            cp1: 0x6f as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed0 as libc::c_int as uint32_t,
            cp1: 0xd4 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed1 as libc::c_int as uint32_t,
            cp1: 0xf4 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed2 as libc::c_int as uint32_t,
            cp1: 0xd4 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed3 as libc::c_int as uint32_t,
            cp1: 0xf4 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed4 as libc::c_int as uint32_t,
            cp1: 0xd4 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed5 as libc::c_int as uint32_t,
            cp1: 0xf4 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed6 as libc::c_int as uint32_t,
            cp1: 0xd4 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed7 as libc::c_int as uint32_t,
            cp1: 0xf4 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed8 as libc::c_int as uint32_t,
            cp1: 0x1ecc as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ed9 as libc::c_int as uint32_t,
            cp1: 0x1ecd as libc::c_int as uint32_t,
            cp2: 0x302 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eda as libc::c_int as uint32_t,
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1edb as libc::c_int as uint32_t,
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1edc as libc::c_int as uint32_t,
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1edd as libc::c_int as uint32_t,
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ede as libc::c_int as uint32_t,
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1edf as libc::c_int as uint32_t,
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee0 as libc::c_int as uint32_t,
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee1 as libc::c_int as uint32_t,
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee2 as libc::c_int as uint32_t,
            cp1: 0x1a0 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee3 as libc::c_int as uint32_t,
            cp1: 0x1a1 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee4 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee5 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee6 as libc::c_int as uint32_t,
            cp1: 0x55 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee7 as libc::c_int as uint32_t,
            cp1: 0x75 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee8 as libc::c_int as uint32_t,
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ee9 as libc::c_int as uint32_t,
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eea as libc::c_int as uint32_t,
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eeb as libc::c_int as uint32_t,
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eec as libc::c_int as uint32_t,
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eed as libc::c_int as uint32_t,
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eee as libc::c_int as uint32_t,
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1eef as libc::c_int as uint32_t,
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef0 as libc::c_int as uint32_t,
            cp1: 0x1af as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef1 as libc::c_int as uint32_t,
            cp1: 0x1b0 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef2 as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef3 as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef4 as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef5 as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x323 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef6 as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef7 as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x309 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef8 as libc::c_int as uint32_t,
            cp1: 0x59 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ef9 as libc::c_int as uint32_t,
            cp1: 0x79 as libc::c_int as uint32_t,
            cp2: 0x303 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f00 as libc::c_int as uint32_t,
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f01 as libc::c_int as uint32_t,
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f02 as libc::c_int as uint32_t,
            cp1: 0x1f00 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f03 as libc::c_int as uint32_t,
            cp1: 0x1f01 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f04 as libc::c_int as uint32_t,
            cp1: 0x1f00 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f05 as libc::c_int as uint32_t,
            cp1: 0x1f01 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f06 as libc::c_int as uint32_t,
            cp1: 0x1f00 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f07 as libc::c_int as uint32_t,
            cp1: 0x1f01 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f08 as libc::c_int as uint32_t,
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f09 as libc::c_int as uint32_t,
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f0a as libc::c_int as uint32_t,
            cp1: 0x1f08 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f0b as libc::c_int as uint32_t,
            cp1: 0x1f09 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f0c as libc::c_int as uint32_t,
            cp1: 0x1f08 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f0d as libc::c_int as uint32_t,
            cp1: 0x1f09 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f0e as libc::c_int as uint32_t,
            cp1: 0x1f08 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f0f as libc::c_int as uint32_t,
            cp1: 0x1f09 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f10 as libc::c_int as uint32_t,
            cp1: 0x3b5 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f11 as libc::c_int as uint32_t,
            cp1: 0x3b5 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f12 as libc::c_int as uint32_t,
            cp1: 0x1f10 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f13 as libc::c_int as uint32_t,
            cp1: 0x1f11 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f14 as libc::c_int as uint32_t,
            cp1: 0x1f10 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f15 as libc::c_int as uint32_t,
            cp1: 0x1f11 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f18 as libc::c_int as uint32_t,
            cp1: 0x395 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f19 as libc::c_int as uint32_t,
            cp1: 0x395 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f1a as libc::c_int as uint32_t,
            cp1: 0x1f18 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f1b as libc::c_int as uint32_t,
            cp1: 0x1f19 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f1c as libc::c_int as uint32_t,
            cp1: 0x1f18 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f1d as libc::c_int as uint32_t,
            cp1: 0x1f19 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f20 as libc::c_int as uint32_t,
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f21 as libc::c_int as uint32_t,
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f22 as libc::c_int as uint32_t,
            cp1: 0x1f20 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f23 as libc::c_int as uint32_t,
            cp1: 0x1f21 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f24 as libc::c_int as uint32_t,
            cp1: 0x1f20 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f25 as libc::c_int as uint32_t,
            cp1: 0x1f21 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f26 as libc::c_int as uint32_t,
            cp1: 0x1f20 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f27 as libc::c_int as uint32_t,
            cp1: 0x1f21 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f28 as libc::c_int as uint32_t,
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f29 as libc::c_int as uint32_t,
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f2a as libc::c_int as uint32_t,
            cp1: 0x1f28 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f2b as libc::c_int as uint32_t,
            cp1: 0x1f29 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f2c as libc::c_int as uint32_t,
            cp1: 0x1f28 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f2d as libc::c_int as uint32_t,
            cp1: 0x1f29 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f2e as libc::c_int as uint32_t,
            cp1: 0x1f28 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f2f as libc::c_int as uint32_t,
            cp1: 0x1f29 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f30 as libc::c_int as uint32_t,
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f31 as libc::c_int as uint32_t,
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f32 as libc::c_int as uint32_t,
            cp1: 0x1f30 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f33 as libc::c_int as uint32_t,
            cp1: 0x1f31 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f34 as libc::c_int as uint32_t,
            cp1: 0x1f30 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f35 as libc::c_int as uint32_t,
            cp1: 0x1f31 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f36 as libc::c_int as uint32_t,
            cp1: 0x1f30 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f37 as libc::c_int as uint32_t,
            cp1: 0x1f31 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f38 as libc::c_int as uint32_t,
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f39 as libc::c_int as uint32_t,
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f3a as libc::c_int as uint32_t,
            cp1: 0x1f38 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f3b as libc::c_int as uint32_t,
            cp1: 0x1f39 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f3c as libc::c_int as uint32_t,
            cp1: 0x1f38 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f3d as libc::c_int as uint32_t,
            cp1: 0x1f39 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f3e as libc::c_int as uint32_t,
            cp1: 0x1f38 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f3f as libc::c_int as uint32_t,
            cp1: 0x1f39 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f40 as libc::c_int as uint32_t,
            cp1: 0x3bf as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f41 as libc::c_int as uint32_t,
            cp1: 0x3bf as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f42 as libc::c_int as uint32_t,
            cp1: 0x1f40 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f43 as libc::c_int as uint32_t,
            cp1: 0x1f41 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f44 as libc::c_int as uint32_t,
            cp1: 0x1f40 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f45 as libc::c_int as uint32_t,
            cp1: 0x1f41 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f48 as libc::c_int as uint32_t,
            cp1: 0x39f as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f49 as libc::c_int as uint32_t,
            cp1: 0x39f as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f4a as libc::c_int as uint32_t,
            cp1: 0x1f48 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f4b as libc::c_int as uint32_t,
            cp1: 0x1f49 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f4c as libc::c_int as uint32_t,
            cp1: 0x1f48 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f4d as libc::c_int as uint32_t,
            cp1: 0x1f49 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f50 as libc::c_int as uint32_t,
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f51 as libc::c_int as uint32_t,
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f52 as libc::c_int as uint32_t,
            cp1: 0x1f50 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f53 as libc::c_int as uint32_t,
            cp1: 0x1f51 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f54 as libc::c_int as uint32_t,
            cp1: 0x1f50 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f55 as libc::c_int as uint32_t,
            cp1: 0x1f51 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f56 as libc::c_int as uint32_t,
            cp1: 0x1f50 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f57 as libc::c_int as uint32_t,
            cp1: 0x1f51 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f59 as libc::c_int as uint32_t,
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f5b as libc::c_int as uint32_t,
            cp1: 0x1f59 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f5d as libc::c_int as uint32_t,
            cp1: 0x1f59 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f5f as libc::c_int as uint32_t,
            cp1: 0x1f59 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f60 as libc::c_int as uint32_t,
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f61 as libc::c_int as uint32_t,
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f62 as libc::c_int as uint32_t,
            cp1: 0x1f60 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f63 as libc::c_int as uint32_t,
            cp1: 0x1f61 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f64 as libc::c_int as uint32_t,
            cp1: 0x1f60 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f65 as libc::c_int as uint32_t,
            cp1: 0x1f61 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f66 as libc::c_int as uint32_t,
            cp1: 0x1f60 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f67 as libc::c_int as uint32_t,
            cp1: 0x1f61 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f68 as libc::c_int as uint32_t,
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f69 as libc::c_int as uint32_t,
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f6a as libc::c_int as uint32_t,
            cp1: 0x1f68 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f6b as libc::c_int as uint32_t,
            cp1: 0x1f69 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f6c as libc::c_int as uint32_t,
            cp1: 0x1f68 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f6d as libc::c_int as uint32_t,
            cp1: 0x1f69 as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f6e as libc::c_int as uint32_t,
            cp1: 0x1f68 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f6f as libc::c_int as uint32_t,
            cp1: 0x1f69 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f70 as libc::c_int as uint32_t,
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f72 as libc::c_int as uint32_t,
            cp1: 0x3b5 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f74 as libc::c_int as uint32_t,
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f76 as libc::c_int as uint32_t,
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f78 as libc::c_int as uint32_t,
            cp1: 0x3bf as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f7a as libc::c_int as uint32_t,
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f7c as libc::c_int as uint32_t,
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f80 as libc::c_int as uint32_t,
            cp1: 0x1f00 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f81 as libc::c_int as uint32_t,
            cp1: 0x1f01 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f82 as libc::c_int as uint32_t,
            cp1: 0x1f02 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f83 as libc::c_int as uint32_t,
            cp1: 0x1f03 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f84 as libc::c_int as uint32_t,
            cp1: 0x1f04 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f85 as libc::c_int as uint32_t,
            cp1: 0x1f05 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f86 as libc::c_int as uint32_t,
            cp1: 0x1f06 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f87 as libc::c_int as uint32_t,
            cp1: 0x1f07 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f88 as libc::c_int as uint32_t,
            cp1: 0x1f08 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f89 as libc::c_int as uint32_t,
            cp1: 0x1f09 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f8a as libc::c_int as uint32_t,
            cp1: 0x1f0a as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f8b as libc::c_int as uint32_t,
            cp1: 0x1f0b as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f8c as libc::c_int as uint32_t,
            cp1: 0x1f0c as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f8d as libc::c_int as uint32_t,
            cp1: 0x1f0d as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f8e as libc::c_int as uint32_t,
            cp1: 0x1f0e as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f8f as libc::c_int as uint32_t,
            cp1: 0x1f0f as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f90 as libc::c_int as uint32_t,
            cp1: 0x1f20 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f91 as libc::c_int as uint32_t,
            cp1: 0x1f21 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f92 as libc::c_int as uint32_t,
            cp1: 0x1f22 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f93 as libc::c_int as uint32_t,
            cp1: 0x1f23 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f94 as libc::c_int as uint32_t,
            cp1: 0x1f24 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f95 as libc::c_int as uint32_t,
            cp1: 0x1f25 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f96 as libc::c_int as uint32_t,
            cp1: 0x1f26 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f97 as libc::c_int as uint32_t,
            cp1: 0x1f27 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f98 as libc::c_int as uint32_t,
            cp1: 0x1f28 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f99 as libc::c_int as uint32_t,
            cp1: 0x1f29 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f9a as libc::c_int as uint32_t,
            cp1: 0x1f2a as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f9b as libc::c_int as uint32_t,
            cp1: 0x1f2b as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f9c as libc::c_int as uint32_t,
            cp1: 0x1f2c as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f9d as libc::c_int as uint32_t,
            cp1: 0x1f2d as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f9e as libc::c_int as uint32_t,
            cp1: 0x1f2e as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1f9f as libc::c_int as uint32_t,
            cp1: 0x1f2f as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa0 as libc::c_int as uint32_t,
            cp1: 0x1f60 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa1 as libc::c_int as uint32_t,
            cp1: 0x1f61 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa2 as libc::c_int as uint32_t,
            cp1: 0x1f62 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa3 as libc::c_int as uint32_t,
            cp1: 0x1f63 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa4 as libc::c_int as uint32_t,
            cp1: 0x1f64 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa5 as libc::c_int as uint32_t,
            cp1: 0x1f65 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa6 as libc::c_int as uint32_t,
            cp1: 0x1f66 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa7 as libc::c_int as uint32_t,
            cp1: 0x1f67 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa8 as libc::c_int as uint32_t,
            cp1: 0x1f68 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fa9 as libc::c_int as uint32_t,
            cp1: 0x1f69 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1faa as libc::c_int as uint32_t,
            cp1: 0x1f6a as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fab as libc::c_int as uint32_t,
            cp1: 0x1f6b as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fac as libc::c_int as uint32_t,
            cp1: 0x1f6c as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fad as libc::c_int as uint32_t,
            cp1: 0x1f6d as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fae as libc::c_int as uint32_t,
            cp1: 0x1f6e as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1faf as libc::c_int as uint32_t,
            cp1: 0x1f6f as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb0 as libc::c_int as uint32_t,
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb1 as libc::c_int as uint32_t,
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb2 as libc::c_int as uint32_t,
            cp1: 0x1f70 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb3 as libc::c_int as uint32_t,
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb4 as libc::c_int as uint32_t,
            cp1: 0x3ac as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb6 as libc::c_int as uint32_t,
            cp1: 0x3b1 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb7 as libc::c_int as uint32_t,
            cp1: 0x1fb6 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb8 as libc::c_int as uint32_t,
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fb9 as libc::c_int as uint32_t,
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fba as libc::c_int as uint32_t,
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fbc as libc::c_int as uint32_t,
            cp1: 0x391 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fc1 as libc::c_int as uint32_t,
            cp1: 0xa8 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fc2 as libc::c_int as uint32_t,
            cp1: 0x1f74 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fc3 as libc::c_int as uint32_t,
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fc4 as libc::c_int as uint32_t,
            cp1: 0x3ae as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fc6 as libc::c_int as uint32_t,
            cp1: 0x3b7 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fc7 as libc::c_int as uint32_t,
            cp1: 0x1fc6 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fc8 as libc::c_int as uint32_t,
            cp1: 0x395 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fca as libc::c_int as uint32_t,
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fcc as libc::c_int as uint32_t,
            cp1: 0x397 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fcd as libc::c_int as uint32_t,
            cp1: 0x1fbf as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fce as libc::c_int as uint32_t,
            cp1: 0x1fbf as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fcf as libc::c_int as uint32_t,
            cp1: 0x1fbf as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fd0 as libc::c_int as uint32_t,
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fd1 as libc::c_int as uint32_t,
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fd2 as libc::c_int as uint32_t,
            cp1: 0x3ca as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fd6 as libc::c_int as uint32_t,
            cp1: 0x3b9 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fd7 as libc::c_int as uint32_t,
            cp1: 0x3ca as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fd8 as libc::c_int as uint32_t,
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fd9 as libc::c_int as uint32_t,
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fda as libc::c_int as uint32_t,
            cp1: 0x399 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fdd as libc::c_int as uint32_t,
            cp1: 0x1ffe as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fde as libc::c_int as uint32_t,
            cp1: 0x1ffe as libc::c_int as uint32_t,
            cp2: 0x301 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fdf as libc::c_int as uint32_t,
            cp1: 0x1ffe as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe0 as libc::c_int as uint32_t,
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe1 as libc::c_int as uint32_t,
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe2 as libc::c_int as uint32_t,
            cp1: 0x3cb as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe4 as libc::c_int as uint32_t,
            cp1: 0x3c1 as libc::c_int as uint32_t,
            cp2: 0x313 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe5 as libc::c_int as uint32_t,
            cp1: 0x3c1 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe6 as libc::c_int as uint32_t,
            cp1: 0x3c5 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe7 as libc::c_int as uint32_t,
            cp1: 0x3cb as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe8 as libc::c_int as uint32_t,
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x306 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fe9 as libc::c_int as uint32_t,
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x304 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fea as libc::c_int as uint32_t,
            cp1: 0x3a5 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fec as libc::c_int as uint32_t,
            cp1: 0x3a1 as libc::c_int as uint32_t,
            cp2: 0x314 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1fed as libc::c_int as uint32_t,
            cp1: 0xa8 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ff2 as libc::c_int as uint32_t,
            cp1: 0x1f7c as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ff3 as libc::c_int as uint32_t,
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ff4 as libc::c_int as uint32_t,
            cp1: 0x3ce as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ff6 as libc::c_int as uint32_t,
            cp1: 0x3c9 as libc::c_int as uint32_t,
            cp2: 0x342 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ff7 as libc::c_int as uint32_t,
            cp1: 0x1ff6 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ff8 as libc::c_int as uint32_t,
            cp1: 0x39f as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ffa as libc::c_int as uint32_t,
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x300 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1ffc as libc::c_int as uint32_t,
            cp1: 0x3a9 as libc::c_int as uint32_t,
            cp2: 0x345 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x219a as libc::c_int as uint32_t,
            cp1: 0x2190 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x219b as libc::c_int as uint32_t,
            cp1: 0x2192 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x21ae as libc::c_int as uint32_t,
            cp1: 0x2194 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x21cd as libc::c_int as uint32_t,
            cp1: 0x21d0 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x21ce as libc::c_int as uint32_t,
            cp1: 0x21d4 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x21cf as libc::c_int as uint32_t,
            cp1: 0x21d2 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2204 as libc::c_int as uint32_t,
            cp1: 0x2203 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2209 as libc::c_int as uint32_t,
            cp1: 0x2208 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x220c as libc::c_int as uint32_t,
            cp1: 0x220b as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2224 as libc::c_int as uint32_t,
            cp1: 0x2223 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2226 as libc::c_int as uint32_t,
            cp1: 0x2225 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2241 as libc::c_int as uint32_t,
            cp1: 0x223c as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2244 as libc::c_int as uint32_t,
            cp1: 0x2243 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2247 as libc::c_int as uint32_t,
            cp1: 0x2245 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2249 as libc::c_int as uint32_t,
            cp1: 0x2248 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2260 as libc::c_int as uint32_t,
            cp1: 0x3d as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2262 as libc::c_int as uint32_t,
            cp1: 0x2261 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x226d as libc::c_int as uint32_t,
            cp1: 0x224d as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x226e as libc::c_int as uint32_t,
            cp1: 0x3c as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x226f as libc::c_int as uint32_t,
            cp1: 0x3e as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2270 as libc::c_int as uint32_t,
            cp1: 0x2264 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2271 as libc::c_int as uint32_t,
            cp1: 0x2265 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2274 as libc::c_int as uint32_t,
            cp1: 0x2272 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2275 as libc::c_int as uint32_t,
            cp1: 0x2273 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2278 as libc::c_int as uint32_t,
            cp1: 0x2276 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2279 as libc::c_int as uint32_t,
            cp1: 0x2277 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2280 as libc::c_int as uint32_t,
            cp1: 0x227a as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2281 as libc::c_int as uint32_t,
            cp1: 0x227b as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2284 as libc::c_int as uint32_t,
            cp1: 0x2282 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2285 as libc::c_int as uint32_t,
            cp1: 0x2283 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2288 as libc::c_int as uint32_t,
            cp1: 0x2286 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x2289 as libc::c_int as uint32_t,
            cp1: 0x2287 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22ac as libc::c_int as uint32_t,
            cp1: 0x22a2 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22ad as libc::c_int as uint32_t,
            cp1: 0x22a8 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22ae as libc::c_int as uint32_t,
            cp1: 0x22a9 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22af as libc::c_int as uint32_t,
            cp1: 0x22ab as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22e0 as libc::c_int as uint32_t,
            cp1: 0x227c as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22e1 as libc::c_int as uint32_t,
            cp1: 0x227d as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22e2 as libc::c_int as uint32_t,
            cp1: 0x2291 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22e3 as libc::c_int as uint32_t,
            cp1: 0x2292 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22ea as libc::c_int as uint32_t,
            cp1: 0x22b2 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22eb as libc::c_int as uint32_t,
            cp1: 0x22b3 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22ec as libc::c_int as uint32_t,
            cp1: 0x22b4 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x22ed as libc::c_int as uint32_t,
            cp1: 0x22b5 as libc::c_int as uint32_t,
            cp2: 0x338 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x304c as libc::c_int as uint32_t,
            cp1: 0x304b as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x304e as libc::c_int as uint32_t,
            cp1: 0x304d as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3050 as libc::c_int as uint32_t,
            cp1: 0x304f as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3052 as libc::c_int as uint32_t,
            cp1: 0x3051 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3054 as libc::c_int as uint32_t,
            cp1: 0x3053 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3056 as libc::c_int as uint32_t,
            cp1: 0x3055 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3058 as libc::c_int as uint32_t,
            cp1: 0x3057 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x305a as libc::c_int as uint32_t,
            cp1: 0x3059 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x305c as libc::c_int as uint32_t,
            cp1: 0x305b as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x305e as libc::c_int as uint32_t,
            cp1: 0x305d as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3060 as libc::c_int as uint32_t,
            cp1: 0x305f as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3062 as libc::c_int as uint32_t,
            cp1: 0x3061 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3065 as libc::c_int as uint32_t,
            cp1: 0x3064 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3067 as libc::c_int as uint32_t,
            cp1: 0x3066 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3069 as libc::c_int as uint32_t,
            cp1: 0x3068 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3070 as libc::c_int as uint32_t,
            cp1: 0x306f as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3071 as libc::c_int as uint32_t,
            cp1: 0x306f as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3073 as libc::c_int as uint32_t,
            cp1: 0x3072 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3074 as libc::c_int as uint32_t,
            cp1: 0x3072 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3076 as libc::c_int as uint32_t,
            cp1: 0x3075 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3077 as libc::c_int as uint32_t,
            cp1: 0x3075 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3079 as libc::c_int as uint32_t,
            cp1: 0x3078 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x307a as libc::c_int as uint32_t,
            cp1: 0x3078 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x307c as libc::c_int as uint32_t,
            cp1: 0x307b as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x307d as libc::c_int as uint32_t,
            cp1: 0x307b as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x3094 as libc::c_int as uint32_t,
            cp1: 0x3046 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x309e as libc::c_int as uint32_t,
            cp1: 0x309d as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30ac as libc::c_int as uint32_t,
            cp1: 0x30ab as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30ae as libc::c_int as uint32_t,
            cp1: 0x30ad as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30b0 as libc::c_int as uint32_t,
            cp1: 0x30af as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30b2 as libc::c_int as uint32_t,
            cp1: 0x30b1 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30b4 as libc::c_int as uint32_t,
            cp1: 0x30b3 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30b6 as libc::c_int as uint32_t,
            cp1: 0x30b5 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30b8 as libc::c_int as uint32_t,
            cp1: 0x30b7 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30ba as libc::c_int as uint32_t,
            cp1: 0x30b9 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30bc as libc::c_int as uint32_t,
            cp1: 0x30bb as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30be as libc::c_int as uint32_t,
            cp1: 0x30bd as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30c0 as libc::c_int as uint32_t,
            cp1: 0x30bf as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30c2 as libc::c_int as uint32_t,
            cp1: 0x30c1 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30c5 as libc::c_int as uint32_t,
            cp1: 0x30c4 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30c7 as libc::c_int as uint32_t,
            cp1: 0x30c6 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30c9 as libc::c_int as uint32_t,
            cp1: 0x30c8 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30d0 as libc::c_int as uint32_t,
            cp1: 0x30cf as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30d1 as libc::c_int as uint32_t,
            cp1: 0x30cf as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30d3 as libc::c_int as uint32_t,
            cp1: 0x30d2 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30d4 as libc::c_int as uint32_t,
            cp1: 0x30d2 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30d6 as libc::c_int as uint32_t,
            cp1: 0x30d5 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30d7 as libc::c_int as uint32_t,
            cp1: 0x30d5 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30d9 as libc::c_int as uint32_t,
            cp1: 0x30d8 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30da as libc::c_int as uint32_t,
            cp1: 0x30d8 as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30dc as libc::c_int as uint32_t,
            cp1: 0x30db as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30dd as libc::c_int as uint32_t,
            cp1: 0x30db as libc::c_int as uint32_t,
            cp2: 0x309a as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30f4 as libc::c_int as uint32_t,
            cp1: 0x30a6 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30f7 as libc::c_int as uint32_t,
            cp1: 0x30ef as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30f8 as libc::c_int as uint32_t,
            cp1: 0x30f0 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30f9 as libc::c_int as uint32_t,
            cp1: 0x30f1 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30fa as libc::c_int as uint32_t,
            cp1: 0x30f2 as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x30fe as libc::c_int as uint32_t,
            cp1: 0x30fd as libc::c_int as uint32_t,
            cp2: 0x3099 as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1109a as libc::c_int as uint32_t,
            cp1: 0x11099 as libc::c_int as uint32_t,
            cp2: 0x110ba as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x1109c as libc::c_int as uint32_t,
            cp1: 0x1109b as libc::c_int as uint32_t,
            cp2: 0x110ba as libc::c_int as uint32_t,
        };
        init
    },
    {
        let mut init = unicode_decomposition_table {
            nfc: 0x110ab as libc::c_int as uint32_t,
            cp1: 0x110a5 as libc::c_int as uint32_t,
            cp2: 0x110ba as libc::c_int as uint32_t,
        };
        init
    },
];
pub const SCONV_TO_CHARSET: libc::c_int = 1 as libc::c_int;
pub const SCONV_FROM_CHARSET: libc::c_int = (1 as libc::c_int) << 1 as libc::c_int;
pub const SCONV_BEST_EFFORT: libc::c_int = (1 as libc::c_int) << 2 as libc::c_int;
pub const SCONV_WIN_CP: libc::c_int = (1 as libc::c_int) << 3 as libc::c_int;
pub const SCONV_UTF8_LIBARCHIVE_2: libc::c_int = (1 as libc::c_int) << 4 as libc::c_int;
pub const SCONV_NORMALIZATION_C: libc::c_int = (1 as libc::c_int) << 6 as libc::c_int;
pub const SCONV_NORMALIZATION_D: libc::c_int = (1 as libc::c_int) << 7 as libc::c_int;
pub const SCONV_TO_UTF8: libc::c_int = (1 as libc::c_int) << 8 as libc::c_int;
pub const SCONV_FROM_UTF8: libc::c_int = (1 as libc::c_int) << 9 as libc::c_int;
pub const SCONV_TO_UTF16BE: libc::c_int = (1 as libc::c_int) << 10 as libc::c_int;
pub const SCONV_FROM_UTF16BE: libc::c_int = (1 as libc::c_int) << 11 as libc::c_int;
pub const SCONV_TO_UTF16LE: libc::c_int = (1 as libc::c_int) << 12 as libc::c_int;
pub const SCONV_FROM_UTF16LE: libc::c_int = (1 as libc::c_int) << 13 as libc::c_int;
pub const SCONV_TO_UTF16: libc::c_int = SCONV_TO_UTF16BE | SCONV_TO_UTF16LE;
pub const SCONV_FROM_UTF16: libc::c_int = SCONV_FROM_UTF16BE | SCONV_FROM_UTF16LE;
pub const UNICODE_MAX: libc::c_int = 0x10ffff as libc::c_int;
pub const UNICODE_R_CHAR: libc::c_int = 0xfffd as libc::c_int;
/* Replacement character. */
/* Set U+FFFD(Replacement character) in UTF-8. */
static mut utf8_replacement_char: [libc::c_char; 3] = [
    0xef as libc::c_int as libc::c_char,
    0xbf as libc::c_int as libc::c_char,
    0xbd as libc::c_int as libc::c_char,
];
unsafe extern "C" fn archive_string_append(
    mut as_0: *mut archive_string,
    mut p: *const libc::c_char,
    mut s: size_t,
) -> *mut archive_string {
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(s)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        return 0 as *mut archive_string;
    }
    if s != 0 {
        memmove(
            (*as_0).s.offset((*as_0).length as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            s,
        );
    }
    (*as_0).length = ((*as_0).length as libc::c_ulong).wrapping_add(s) as size_t as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = 0 as libc::c_int as libc::c_char;
    return as_0;
}
unsafe extern "C" fn archive_wstring_append(
    mut as_0: *mut archive_wstring,
    mut p: *const wchar_t,
    mut s: size_t,
) -> *mut archive_wstring {
    if archive_wstring_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(s)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        return 0 as *mut archive_wstring;
    }
    if s != 0 {
        wmemmove((*as_0).s.offset((*as_0).length as isize), p, s);
    }
    (*as_0).length = ((*as_0).length as libc::c_ulong).wrapping_add(s) as size_t as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = 0 as libc::c_int;
    return as_0;
}
#[no_mangle]
pub unsafe extern "C" fn archive_array_append(
    mut as_0: *mut archive_string,
    mut p: *const libc::c_char,
    mut s: size_t,
) -> *mut archive_string {
    return archive_string_append(as_0, p, s);
}
#[no_mangle]
pub unsafe extern "C" fn archive_string_concat(
    mut dest: *mut archive_string,
    mut src: *mut archive_string,
) {
    if archive_string_append(dest, (*src).s, (*src).length).is_null() {
        __archive_errx(
            1 as libc::c_int,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_wstring_concat(
    mut dest: *mut archive_wstring,
    mut src: *mut archive_wstring,
) {
    if archive_wstring_append(dest, (*src).s, (*src).length).is_null() {
        __archive_errx(
            1 as libc::c_int,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn archive_string_free(mut as_0: *mut archive_string) {
    (*as_0).length = 0 as libc::c_int as size_t;
    (*as_0).buffer_length = 0 as libc::c_int as size_t;
    free((*as_0).s as *mut libc::c_void);
    (*as_0).s = NULL as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn archive_wstring_free(mut as_0: *mut archive_wstring) {
    (*as_0).length = 0 as libc::c_int as size_t;
    (*as_0).buffer_length = 0 as libc::c_int as size_t;
    free((*as_0).s as *mut libc::c_void);
    (*as_0).s = NULL as *mut wchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn archive_wstring_ensure(
    mut as_0: *mut archive_wstring,
    mut s: size_t,
) -> *mut archive_wstring {
    return archive_string_ensure(
        as_0 as *mut archive_string,
        s.wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong),
    ) as *mut archive_wstring;
}
/* Returns NULL on any allocation failure. */
#[no_mangle]
pub unsafe extern "C" fn archive_string_ensure(
    mut as_0: *mut archive_string,
    mut s: size_t,
) -> *mut archive_string {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_length: size_t = 0;
    /* If buffer is already big enough, don't reallocate. */
    if !(*as_0).s.is_null() && s <= (*as_0).buffer_length {
        return as_0;
    }
    /*
     * Growing the buffer at least exponentially ensures that
     * append operations are always linear in the number of
     * characters appended.  Using a smaller growth rate for
     * larger buffers reduces memory waste somewhat at the cost of
     * a larger constant factor.
     */
    if (*as_0).buffer_length < 32 as libc::c_int as libc::c_ulong {
        /* Start with a minimum 32-character buffer. */
        new_length = 32 as libc::c_int as size_t
    } else if (*as_0).buffer_length < 8192 as libc::c_int as libc::c_ulong {
        /* Buffers under 8k are doubled for speed. */
        new_length = (*as_0).buffer_length.wrapping_add((*as_0).buffer_length)
    } else {
        /* Buffers 8k and over grow by at least 25% each time. */
        new_length = (*as_0).buffer_length.wrapping_add(
            (*as_0)
                .buffer_length
                .wrapping_div(4 as libc::c_int as libc::c_ulong),
        );
        /* Be safe: If size wraps, fail. */
        if new_length < (*as_0).buffer_length {
            /* On failure, wipe the string and return NULL. */
            archive_string_free(as_0); /* Make sure errno has ENOMEM. */
            errno = ENOMEM;
            return 0 as *mut archive_string;
        }
    }
    /*
     * The computation above is a lower limit to how much we'll
     * grow the buffer.  In any case, we have to grow it enough to
     * hold the request.
     */
    if new_length < s {
        new_length = s
    }
    /* Now we can reallocate the buffer. */
    p = realloc((*as_0).s as *mut libc::c_void, new_length) as *mut libc::c_char;
    if p.is_null() {
        /* On failure, wipe the string and return NULL. */
        archive_string_free(as_0); /* Make sure errno has ENOMEM. */
        errno = ENOMEM;
        return 0 as *mut archive_string;
    }
    (*as_0).s = p;
    (*as_0).buffer_length = new_length;
    return as_0;
}
/*
 * TODO: See if there's a way to avoid scanning
 * the source string twice.  Then test to see
 * if it actually helps (remember that we're almost
 * always called with pretty short arguments, so
 * such an optimization might not help).
 */
#[no_mangle]
pub unsafe extern "C" fn archive_strncat(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut n: size_t,
) -> *mut archive_string {
    let mut s: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut pp: *const libc::c_char = 0 as *const libc::c_char;
    p = _p as *const libc::c_char;
    /* Like strlen(p), except won't examine positions beyond p[n]. */
    s = 0 as libc::c_int as size_t;
    pp = p;
    while s < n && *pp as libc::c_int != 0 {
        pp = pp.offset(1);
        s = s.wrapping_add(1)
    }
    as_0 = archive_string_append(as_0, p, s);
    if as_0.is_null() {
        __archive_errx(
            1 as libc::c_int,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return as_0;
}
#[no_mangle]
pub unsafe extern "C" fn archive_wstrncat(
    mut as_0: *mut archive_wstring,
    mut p: *const wchar_t,
    mut n: size_t,
) -> *mut archive_wstring {
    let mut s: size_t = 0;
    let mut pp: *const wchar_t = 0 as *const wchar_t;
    /* Like strlen(p), except won't examine positions beyond p[n]. */
    s = 0 as libc::c_int as size_t;
    pp = p;
    while s < n && *pp != 0 {
        pp = pp.offset(1);
        s = s.wrapping_add(1)
    }
    as_0 = archive_wstring_append(as_0, p, s);
    if as_0.is_null() {
        __archive_errx(
            1 as libc::c_int,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return as_0;
}
#[no_mangle]
pub unsafe extern "C" fn archive_strcat(
    mut as_0: *mut archive_string,
    mut p: *const libc::c_void,
) -> *mut archive_string {
    /* strcat is just strncat without an effective limit.
     * Assert that we'll never get called with a source
     * string over 16MB.
     * TODO: Review all uses of strcat in the source
     * and try to replace them with strncat().
     */
    return archive_strncat(as_0, p, 0x1000000 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn archive_wstrcat(
    mut as_0: *mut archive_wstring,
    mut p: *const wchar_t,
) -> *mut archive_wstring {
    /* Ditto. */
    return archive_wstrncat(as_0, p, 0x1000000 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn archive_strappend_char(
    mut as_0: *mut archive_string,
    mut c: libc::c_char,
) -> *mut archive_string {
    as_0 = archive_string_append(as_0, &mut c, 1 as libc::c_int as size_t);
    if as_0.is_null() {
        __archive_errx(
            1 as libc::c_int,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return as_0;
}
#[no_mangle]
pub unsafe extern "C" fn archive_wstrappend_wchar(
    mut as_0: *mut archive_wstring,
    mut c: wchar_t,
) -> *mut archive_wstring {
    as_0 = archive_wstring_append(as_0, &mut c, 1 as libc::c_int as size_t);
    if as_0.is_null() {
        __archive_errx(
            1 as libc::c_int,
            b"Out of memory\x00" as *const u8 as *const libc::c_char,
        );
    }
    return as_0;
}
/*
 * Get the "current character set" name to use with iconv.
 * On FreeBSD, the empty character set name "" chooses
 * the correct character encoding for the current locale,
 * so this isn't necessary.
 * But iconv on Mac OS 10.6 doesn't seem to handle this correctly;
 * on that system, we have to explicitly call nl_langinfo()
 * to get the right name.  Not sure about other platforms.
 *
 * NOTE: GNU libiconv does not recognize the character-set name
 * which some platform nl_langinfo(CODESET) returns, so we should
 * use locale_charset() instead of nl_langinfo(CODESET) for GNU libiconv.
 */
unsafe extern "C" fn default_iconv_charset(
    mut charset: *const libc::c_char,
) -> *const libc::c_char {
    if !charset.is_null()
        && *charset.offset(0 as libc::c_int as isize) as libc::c_int != '\u{0}' as i32
    {
        return charset;
    }
    return nl_langinfo(CODESET_0);
}
/*
 * Convert MBS to WCS.
 * Note: returns -1 if conversion fails.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_wstring_append_from_mbs(
    mut dest: *mut archive_wstring,
    mut p: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut r: size_t = 0;
    let mut ret_val: libc::c_int = 0 as libc::c_int;
    /*
     * No single byte will be more than one wide character,
     * so this length estimate will always be big enough.
     */
    // size_t wcs_length = len;
    let mut mbs_length: size_t = len;
    let mut mbs: *const libc::c_char = p;
    let mut wcs: *mut wchar_t = 0 as *mut wchar_t;
    let mut shift_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed_0 { __wch: 0 },
    };
    memset(
        &mut shift_state as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    /*
     * As we decided to have wcs_length == mbs_length == len
     * we can use len here instead of wcs_length
     */
    if archive_wstring_ensure(
        dest,
        (*dest)
            .length
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    wcs = (*dest).s.offset((*dest).length as isize);
    /*
     * We cannot use mbsrtowcs/mbstowcs here because those may convert
     * extra MBS when strlen(p) > len and one wide character consists of
     * multi bytes.
     */
    while *mbs as libc::c_int != 0 && mbs_length > 0 as libc::c_int as libc::c_ulong {
        /*
         * The buffer we allocated is always big enough.
         * Keep this code path in a comment if we decide to choose
         * smaller wcs_length in the future
         */
        /*
                if (wcs_length == 0) {
                    dest->length = wcs - dest->s;
                    dest->s[dest->length] = L'\0';
                    wcs_length = mbs_length;
                    if (NULL == archive_wstring_ensure(dest,
                        dest->length + wcs_length + 1))
                        return (-1);
                    wcs = dest->s + dest->length;
                }
        */
        r = mbrtowc(wcs, mbs, mbs_length, &mut shift_state);
        if r == -(1 as libc::c_int) as size_t || r == -(2 as libc::c_int) as size_t {
            ret_val = -(1 as libc::c_int);
            break;
        } else {
            if r == 0 as libc::c_int as libc::c_ulong || r > mbs_length {
                break;
            }
            wcs = wcs.offset(1);
            // wcs_length--;
            mbs = mbs.offset(r as isize);
            mbs_length = (mbs_length as libc::c_ulong).wrapping_sub(r) as size_t as size_t
        }
    }
    (*dest).length = wcs.offset_from((*dest).s) as libc::c_long as size_t;
    *(*dest).s.offset((*dest).length as isize) = '\u{0}' as i32;
    return ret_val;
}
/*
 * Translates a wide character string into current locale character set
 * and appends to the archive_string.  Note: returns -1 if conversion
 * fails.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_string_append_from_wcs(
    mut as_0: *mut archive_string,
    mut w: *const wchar_t,
    mut len: size_t,
) -> libc::c_int {
    /* We cannot use the standard wcstombs() here because it
     * cannot tell us how big the output buffer should be.  So
     * I've built a loop around wcrtomb() or wctomb() that
     * converts a character at a time and resizes the string as
     * needed.  We prefer wcrtomb() when it's available because
     * it's thread-safe. */
    let mut n: libc::c_int = 0;
    let mut ret_val: libc::c_int = 0 as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut shift_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed_0 { __wch: 0 },
    };
    memset(
        &mut shift_state as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    /*
     * Allocate buffer for MBS.
     * We need this allocation here since it is possible that
     * as->s is still NULL.
     */
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    p = (*as_0).s.offset((*as_0).length as isize);
    end = (*as_0)
        .s
        .offset((*as_0).buffer_length as isize)
        .offset(-(MB_CUR_MAX as isize))
        .offset(-(1 as libc::c_int as isize));
    while *w != '\u{0}' as i32 && len > 0 as libc::c_int as libc::c_ulong {
        if p >= end {
            (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
            *(*as_0).s.offset((*as_0).length as isize) = '\u{0}' as i32 as libc::c_char;
            /* Re-allocate buffer for MBS. */
            if archive_string_ensure(
                as_0,
                (*as_0)
                    .length
                    .wrapping_add(
                        (if len.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            > __ctype_get_mb_cur_max()
                        {
                            len.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        } else {
                            __ctype_get_mb_cur_max()
                        }),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            .is_null()
            {
                return -(1 as libc::c_int);
            }
            p = (*as_0).s.offset((*as_0).length as isize);
            end = (*as_0)
                .s
                .offset((*as_0).buffer_length as isize)
                .offset(-(MB_CUR_MAX as isize))
                .offset(-(1 as libc::c_int as isize))
        }
        let fresh0 = w;
        w = w.offset(1);
        n = wcrtomb(p, *fresh0, &mut shift_state) as libc::c_int;
        if n == -(1 as libc::c_int) {
            if errno == EILSEQ {
                /* Skip an illegal wide char. */
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = '?' as i32 as libc::c_char;
                ret_val = -(1 as libc::c_int)
            } else {
                ret_val = -(1 as libc::c_int);
                break;
            }
        } else {
            p = p.offset(n as isize)
        }
        len = len.wrapping_sub(1)
    }
    (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = '\u{0}' as i32 as libc::c_char;
    return ret_val;
}
/* HAVE_WCTOMB || HAVE_WCRTOMB */
/* HAVE_WCTOMB || HAVE_WCRTOMB */
/*
 * Find a string conversion object by a pair of 'from' charset name
 * and 'to' charset name from an archive object.
 * Return NULL if not found.
 */
unsafe extern "C" fn find_sconv_object(
    mut a: *mut archive,
    mut fc: *const libc::c_char,
    mut tc: *const libc::c_char,
) -> *mut archive_string_conv {
    let mut sc: *mut archive_string_conv = 0 as *mut archive_string_conv;
    if a.is_null() {
        return 0 as *mut archive_string_conv;
    }
    sc = (*a).sconv;
    while !sc.is_null() {
        if strcmp((*sc).from_charset, fc) == 0 as libc::c_int
            && strcmp((*sc).to_charset, tc) == 0 as libc::c_int
        {
            break;
        }
        sc = (*sc).next
    }
    return sc;
}
/*
 * Register a string object to an archive object.
 */
unsafe extern "C" fn add_sconv_object(mut a: *mut archive, mut sc: *mut archive_string_conv) {
    let mut psc: *mut *mut archive_string_conv = 0 as *mut *mut archive_string_conv;
    /* Add a new sconv to sconv list. */
    psc = &mut (*a).sconv;
    while !(*psc).is_null() {
        psc = &mut (**psc).next
    }
    *psc = sc;
}
unsafe extern "C" fn add_converter(
    mut sc: *mut archive_string_conv,
    mut converter: Option<
        unsafe extern "C" fn(
            _: *mut archive_string,
            _: *const libc::c_void,
            _: size_t,
            _: *mut archive_string_conv,
        ) -> libc::c_int,
    >,
) {
    if sc.is_null() || (*sc).nconverter >= 2 as libc::c_int {
        __archive_errx(
            1 as libc::c_int,
            b"Programming error\x00" as *const u8 as *const libc::c_char,
        );
    }
    let fresh2 = (*sc).nconverter;
    (*sc).nconverter = (*sc).nconverter + 1;
    (*sc).converter[fresh2 as usize] = converter;
}
unsafe extern "C" fn setup_converter(mut sc: *mut archive_string_conv) {
    /* Reset. */
    (*sc).nconverter = 0 as libc::c_int;
    /*
     * Perform special sequence for the incorrect UTF-8 filenames
     * made by libarchive2.x.
     */
    if (*sc).flag & SCONV_UTF8_LIBARCHIVE_2 != 0 {
        add_converter(
            sc,
            Some(
                strncat_from_utf8_libarchive2
                    as unsafe extern "C" fn(
                        _: *mut archive_string,
                        _: *const libc::c_void,
                        _: size_t,
                        _: *mut archive_string_conv,
                    ) -> libc::c_int,
            ),
        );
        return;
    }
    /*
     * Convert a string to UTF-16BE/LE.
     */
    if (*sc).flag & SCONV_TO_UTF16 != 0 {
        /*
         * If the current locale is UTF-8, we can translate
         * a UTF-8 string into a UTF-16BE string.
         */
        if (*sc).flag & SCONV_FROM_UTF8 != 0 {
            add_converter(
                sc,
                Some(
                    archive_string_append_unicode
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
            return;
        }
        if (*sc).cd != -(1 as libc::c_int) as iconv_t {
            add_converter(
                sc,
                Some(
                    iconv_strncat_in_locale
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
            return;
        }
        if (*sc).flag & SCONV_BEST_EFFORT != 0 {
            if (*sc).flag & SCONV_TO_UTF16BE != 0 {
                add_converter(
                    sc,
                    Some(
                        best_effort_strncat_to_utf16be
                            as unsafe extern "C" fn(
                                _: *mut archive_string,
                                _: *const libc::c_void,
                                _: size_t,
                                _: *mut archive_string_conv,
                            ) -> libc::c_int,
                    ),
                );
            } else {
                add_converter(
                    sc,
                    Some(
                        best_effort_strncat_to_utf16le
                            as unsafe extern "C" fn(
                                _: *mut archive_string,
                                _: *const libc::c_void,
                                _: size_t,
                                _: *mut archive_string_conv,
                            ) -> libc::c_int,
                    ),
                );
            }
        } else {
            /* Make sure we have no converter. */
            (*sc).nconverter = 0 as libc::c_int
        }
        return;
    }
    /*
     * Convert a string from UTF-16BE/LE.
     */
    if (*sc).flag & SCONV_FROM_UTF16 != 0 {
        /*
         * At least we should normalize a UTF-16BE string.
         */
        if (*sc).flag & SCONV_NORMALIZATION_D != 0 {
            add_converter(
                sc,
                Some(
                    archive_string_normalize_D
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
        } else if (*sc).flag & SCONV_NORMALIZATION_C != 0 {
            add_converter(
                sc,
                Some(
                    archive_string_normalize_C
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
        }
        if (*sc).flag & SCONV_TO_UTF8 != 0 {
            /*
             * If the current locale is UTF-8, we can translate
             * a UTF-16BE/LE string into a UTF-8 string directly.
             */
            if (*sc).flag & (SCONV_NORMALIZATION_D | SCONV_NORMALIZATION_C) == 0 {
                add_converter(
                    sc,
                    Some(
                        archive_string_append_unicode
                            as unsafe extern "C" fn(
                                _: *mut archive_string,
                                _: *const libc::c_void,
                                _: size_t,
                                _: *mut archive_string_conv,
                            ) -> libc::c_int,
                    ),
                );
            }
            return;
        }
        if (*sc).cd != -(1 as libc::c_int) as iconv_t {
            add_converter(
                sc,
                Some(
                    iconv_strncat_in_locale
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
            return;
        }
        if (*sc).flag & (SCONV_BEST_EFFORT | SCONV_FROM_UTF16BE)
            == SCONV_BEST_EFFORT | SCONV_FROM_UTF16BE
        {
            add_converter(
                sc,
                Some(
                    best_effort_strncat_from_utf16be
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
        } else if (*sc).flag & (SCONV_BEST_EFFORT | SCONV_FROM_UTF16LE)
            == SCONV_BEST_EFFORT | SCONV_FROM_UTF16LE
        {
            add_converter(
                sc,
                Some(
                    best_effort_strncat_from_utf16le
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
        } else {
            /* Make sure we have no converter. */
            (*sc).nconverter = 0 as libc::c_int
        }
        return;
    }
    if (*sc).flag & SCONV_FROM_UTF8 != 0 {
        /*
         * At least we should normalize a UTF-8 string.
         */
        if (*sc).flag & SCONV_NORMALIZATION_D != 0 {
            add_converter(
                sc,
                Some(
                    archive_string_normalize_D
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
        } else if (*sc).flag & SCONV_NORMALIZATION_C != 0 {
            add_converter(
                sc,
                Some(
                    archive_string_normalize_C
                        as unsafe extern "C" fn(
                            _: *mut archive_string,
                            _: *const libc::c_void,
                            _: size_t,
                            _: *mut archive_string_conv,
                        ) -> libc::c_int,
                ),
            );
        }
        /*
         * Copy UTF-8 string with a check of CESU-8.
         * Apparently, iconv does not check surrogate pairs in UTF-8
         * when both from-charset and to-charset are UTF-8, and then
         * we use our UTF-8 copy code.
         */
        if (*sc).flag & SCONV_TO_UTF8 != 0 {
            /*
             * If the current locale is UTF-8, we can translate
             * a UTF-16BE string into a UTF-8 string directly.
             */
            if (*sc).flag & (SCONV_NORMALIZATION_D | SCONV_NORMALIZATION_C) == 0 {
                add_converter(
                    sc,
                    Some(
                        strncat_from_utf8_to_utf8
                            as unsafe extern "C" fn(
                                _: *mut archive_string,
                                _: *const libc::c_void,
                                _: size_t,
                                _: *mut archive_string_conv,
                            ) -> libc::c_int,
                    ),
                );
            }
            return;
        }
    }
    if (*sc).cd != -(1 as libc::c_int) as iconv_t {
        add_converter(
            sc,
            Some(
                iconv_strncat_in_locale
                    as unsafe extern "C" fn(
                        _: *mut archive_string,
                        _: *const libc::c_void,
                        _: size_t,
                        _: *mut archive_string_conv,
                    ) -> libc::c_int,
            ),
        );
        /*
         * iconv generally does not support UTF-8-MAC and so
         * we have to the output of iconv from NFC to NFD if
         * need.
         */
        if (*sc).flag & SCONV_FROM_CHARSET != 0 && (*sc).flag & SCONV_TO_UTF8 != 0 {
            if (*sc).flag & SCONV_NORMALIZATION_D != 0 {
                add_converter(
                    sc,
                    Some(
                        archive_string_normalize_D
                            as unsafe extern "C" fn(
                                _: *mut archive_string,
                                _: *const libc::c_void,
                                _: size_t,
                                _: *mut archive_string_conv,
                            ) -> libc::c_int,
                    ),
                );
            }
        }
        return;
    }
    /*
     * Try conversion in the best effort or no conversion.
     */
    if (*sc).flag & SCONV_BEST_EFFORT != 0 || (*sc).same != 0 {
        add_converter(
            sc,
            Some(
                best_effort_strncat_in_locale
                    as unsafe extern "C" fn(
                        _: *mut archive_string,
                        _: *const libc::c_void,
                        _: size_t,
                        _: *mut archive_string_conv,
                    ) -> libc::c_int,
            ),
        );
    } else {
        /* Make sure we have no converter. */
        (*sc).nconverter = 0 as libc::c_int
    };
}
/*
 * Return canonicalized charset-name but this supports just UTF-8, UTF-16BE
 * and CP932 which are referenced in create_sconv_object().
 */
unsafe extern "C" fn canonical_charset_name(
    mut charset: *const libc::c_char,
) -> *const libc::c_char {
    let mut cs: [libc::c_char; 16] = [0; 16];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if charset.is_null()
        || *charset.offset(0 as libc::c_int as isize) as libc::c_int == '\u{0}' as i32
        || strlen(charset) > 15 as libc::c_int as libc::c_ulong
    {
        return charset;
    }
    /* Copy name to uppercase. */
    p = cs.as_mut_ptr();
    s = charset;
    while *s != 0 {
        let fresh3 = s;
        s = s.offset(1);
        let mut c: libc::c_char = *fresh3;
        if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
            c = (c as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char
        }
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = c
    }
    let fresh5 = p;
    p = p.offset(1);
    *fresh5 = '\u{0}' as i32 as libc::c_char;
    if strcmp(
        cs.as_mut_ptr(),
        b"UTF-8\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            cs.as_mut_ptr(),
            b"UTF8\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        return b"UTF-8\x00" as *const u8 as *const libc::c_char;
    }
    if strcmp(
        cs.as_mut_ptr(),
        b"UTF-16BE\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            cs.as_mut_ptr(),
            b"UTF16BE\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        return b"UTF-16BE\x00" as *const u8 as *const libc::c_char;
    }
    if strcmp(
        cs.as_mut_ptr(),
        b"UTF-16LE\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            cs.as_mut_ptr(),
            b"UTF16LE\x00" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        return b"UTF-16LE\x00" as *const u8 as *const libc::c_char;
    }
    if strcmp(
        cs.as_mut_ptr(),
        b"CP932\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        return b"CP932\x00" as *const u8 as *const libc::c_char;
    }
    return charset;
}
/*
 * Create a string conversion object.
 */
unsafe extern "C" fn create_sconv_object(
    mut fc: *const libc::c_char,
    mut tc: *const libc::c_char,
    mut current_codepage: libc::c_uint,
    mut flag: libc::c_int,
) -> *mut archive_string_conv {
    let mut sc: *mut archive_string_conv = 0 as *mut archive_string_conv;
    sc = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<archive_string_conv>() as libc::c_ulong,
    ) as *mut archive_string_conv;
    if sc.is_null() {
        return 0 as *mut archive_string_conv;
    }
    (*sc).next = NULL as *mut archive_string_conv;
    (*sc).from_charset = strdup(fc);
    if (*sc).from_charset.is_null() {
        free(sc as *mut libc::c_void);
        return 0 as *mut archive_string_conv;
    }
    (*sc).to_charset = strdup(tc);
    if (*sc).to_charset.is_null() {
        free((*sc).from_charset as *mut libc::c_void);
        free(sc as *mut libc::c_void);
        return 0 as *mut archive_string_conv;
    }
    (*sc).utftmp.s = NULL as *mut libc::c_char;
    (*sc).utftmp.length = 0 as libc::c_int as size_t;
    (*sc).utftmp.buffer_length = 0 as libc::c_int as size_t;
    if flag & SCONV_TO_CHARSET != 0 {
        /*
         * Convert characters from the current locale charset to
         * a specified charset.
         */
        (*sc).from_cp = current_codepage;
        (*sc).to_cp = make_codepage_from_charset(tc)
    } else if flag & SCONV_FROM_CHARSET != 0 {
        /*
         * Convert characters from a specified charset to
         * the current locale charset.
         */
        (*sc).to_cp = current_codepage;
        (*sc).from_cp = make_codepage_from_charset(fc)
    }
    /*
     * Check if "from charset" and "to charset" are the same.
     */
    if strcmp(fc, tc) == 0 as libc::c_int
        || (*sc).from_cp != -(1 as libc::c_int) as libc::c_uint && (*sc).from_cp == (*sc).to_cp
    {
        (*sc).same = 1 as libc::c_int
    } else {
        (*sc).same = 0 as libc::c_int
    }
    /*
     * Mark if "from charset" or "to charset" are UTF-8 or UTF-16BE/LE.
     */
    if strcmp(tc, b"UTF-8\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        flag |= SCONV_TO_UTF8
    } else if strcmp(tc, b"UTF-16BE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        flag |= SCONV_TO_UTF16BE
    } else if strcmp(tc, b"UTF-16LE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        flag |= SCONV_TO_UTF16LE
    }
    if strcmp(fc, b"UTF-8\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        flag |= SCONV_FROM_UTF8
    } else if strcmp(fc, b"UTF-16BE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        flag |= SCONV_FROM_UTF16BE
    } else if strcmp(fc, b"UTF-16LE\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        flag |= SCONV_FROM_UTF16LE
    }
    /*
     * Set a flag for Unicode NFD. Usually iconv cannot correctly
     * handle it. So we have to translate NFD characters to NFC ones
     * ourselves before iconv handles. Another reason is to prevent
     * that the same sight of two filenames, one is NFC and other
     * is NFD, would be in its directory.
     * On Mac OS X, although its filesystem layer automatically
     * convert filenames to NFD, it would be useful for filename
     * comparing to find out the same filenames that we normalize
     * that to be NFD ourselves.
     */
    if flag & SCONV_FROM_CHARSET != 0 && flag & (SCONV_FROM_UTF16 | SCONV_FROM_UTF8) != 0 {
        flag |= SCONV_NORMALIZATION_C
    }
    (*sc).cd_w = -(1 as libc::c_int) as iconv_t;
    /*
     * Create an iconv object.
     */
    if flag & (SCONV_TO_UTF8 | SCONV_TO_UTF16) != 0
        && flag & (SCONV_FROM_UTF8 | SCONV_FROM_UTF16) != 0
        || flag & SCONV_WIN_CP != 0
    {
        /* This case we won't use iconv. */
        (*sc).cd = -(1 as libc::c_int) as iconv_t
    } else {
        (*sc).cd = iconv_open(tc, fc);
        if (*sc).cd == -(1 as libc::c_int) as iconv_t && (*sc).flag & SCONV_BEST_EFFORT != 0 {
            /* _WIN32 && !__CYGWIN__ */
            /*
             * Unfortunately, all of iconv implements do support
             * "CP932" character-set, so we should use "SJIS"
             * instead if iconv_open failed.
             */
            if strcmp(tc, b"CP932\x00" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
                (*sc).cd = iconv_open(b"SJIS\x00" as *const u8 as *const libc::c_char, fc)
            } else if strcmp(fc, b"CP932\x00" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                (*sc).cd = iconv_open(tc, b"SJIS\x00" as *const u8 as *const libc::c_char)
            }
        }
    }
    /* HAVE_ICONV */
    (*sc).flag = flag;
    /*
     * Set up converters.
     */
    setup_converter(sc);
    return sc;
}
/*
 * Free a string conversion object.
 */
unsafe extern "C" fn free_sconv_object(mut sc: *mut archive_string_conv) {
    free((*sc).from_charset as *mut libc::c_void);
    free((*sc).to_charset as *mut libc::c_void);
    archive_string_free(&mut (*sc).utftmp);
    if (*sc).cd != -(1 as libc::c_int) as iconv_t {
        iconv_close((*sc).cd);
    }
    if (*sc).cd_w != -(1 as libc::c_int) as iconv_t {
        iconv_close((*sc).cd_w);
    }
    free(sc as *mut libc::c_void);
}
/*
 * POSIX platform does not use CodePage.
 */
unsafe extern "C" fn get_current_codepage() -> libc::c_uint {
    return -(1 as libc::c_int) as libc::c_uint;
    /* Unknown */
}
unsafe extern "C" fn make_codepage_from_charset(mut charset: *const libc::c_char) -> libc::c_uint {
    /* UNUSED */
    return -(1 as libc::c_int) as libc::c_uint;
    /* Unknown */
}
unsafe extern "C" fn get_current_oemcp() -> libc::c_uint {
    return -(1 as libc::c_int) as libc::c_uint;
    /* Unknown */
}
/* defined(_WIN32) && !defined(__CYGWIN__) */
/*
 * Return a string conversion object.
 */
unsafe extern "C" fn get_sconv_object(
    mut a: *mut archive,
    mut fc: *const libc::c_char,
    mut tc: *const libc::c_char,
    mut flag: libc::c_int,
) -> *mut archive_string_conv {
    let mut sc: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut current_codepage: libc::c_uint = 0;
    /* Check if we have made the sconv object. */
    sc = find_sconv_object(a, fc, tc);
    if !sc.is_null() {
        return sc;
    }
    if a.is_null() {
        current_codepage = get_current_codepage()
    } else {
        current_codepage = (*a).current_codepage
    }
    sc = create_sconv_object(
        canonical_charset_name(fc),
        canonical_charset_name(tc),
        current_codepage,
        flag,
    );
    if sc.is_null() {
        if !a.is_null() {
            archive_set_error(
                a,
                ENOMEM,
                b"Could not allocate memory for a string conversion object\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as *mut archive_string_conv;
    }
    /*
     * If there is no converter for current string conversion object,
     * we cannot handle this conversion.
     */
    if (*sc).nconverter == 0 as libc::c_int {
        if !a.is_null() {
            archive_set_error(
                a,
                ARCHIVE_ERRNO_MISC,
                b"iconv_open failed : Cannot handle ``%s\'\'\x00" as *const u8
                    as *const libc::c_char,
                if flag & SCONV_TO_CHARSET != 0 { tc } else { fc },
            );
        }
        /* Failed; free a sconv object. */
        free_sconv_object(sc);
        return 0 as *mut archive_string_conv;
    }
    /*
     * Success!
     */
    if !a.is_null() {
        add_sconv_object(a, sc);
    }
    return sc;
}
unsafe extern "C" fn get_current_charset(mut a: *mut archive) -> *const libc::c_char {
    let mut cur_charset: *const libc::c_char = 0 as *const libc::c_char;
    if a.is_null() {
        cur_charset = default_iconv_charset(b"\x00" as *const u8 as *const libc::c_char)
    } else {
        cur_charset = default_iconv_charset((*a).current_code);
        if (*a).current_code.is_null() {
            (*a).current_code = strdup(cur_charset);
            (*a).current_codepage = get_current_codepage();
            (*a).current_oemcp = get_current_oemcp()
        }
    }
    return cur_charset;
}
/*
 * Make and Return a string conversion object.
 * Return NULL if the platform does not support the specified conversion
 * and best_effort is 0.
 * If best_effort is set, A string conversion object must be returned
 * unless memory allocation for the object fails, but the conversion
 * might fail when non-ASCII code is found.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_string_conversion_to_charset(
    mut a: *mut archive,
    mut charset: *const libc::c_char,
    mut best_effort: libc::c_int,
) -> *mut archive_string_conv {
    let mut flag: libc::c_int = SCONV_TO_CHARSET;
    if best_effort != 0 {
        flag |= SCONV_BEST_EFFORT
    }
    return get_sconv_object(a, get_current_charset(a), charset, flag);
}
#[no_mangle]
pub unsafe extern "C" fn archive_string_conversion_from_charset(
    mut a: *mut archive,
    mut charset: *const libc::c_char,
    mut best_effort: libc::c_int,
) -> *mut archive_string_conv {
    let mut flag: libc::c_int = SCONV_FROM_CHARSET;
    if best_effort != 0 {
        flag |= SCONV_BEST_EFFORT
    }
    return get_sconv_object(a, charset, get_current_charset(a), flag);
}
/*
 * archive_string_default_conversion_*_archive() are provided for Windows
 * platform because other archiver application use CP_OEMCP for
 * MultiByteToWideChar() and WideCharToMultiByte() for the filenames
 * in tar or zip files. But mbstowcs/wcstombs(CRT) usually use CP_ACP
 * unless you use setlocale(LC_ALL, ".OCP")(specify CP_OEMCP).
 * So we should make a string conversion between CP_ACP and CP_OEMCP
 * for compatibility.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_string_default_conversion_for_read(
    mut a: *mut archive,
) -> *mut archive_string_conv {
    /* UNUSED */
    return 0 as *mut archive_string_conv;
}
#[no_mangle]
pub unsafe extern "C" fn archive_string_default_conversion_for_write(
    mut a: *mut archive,
) -> *mut archive_string_conv {
    /* UNUSED */
    return 0 as *mut archive_string_conv;
}
/*
 * Dispose of all character conversion objects in the archive object.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_string_conversion_free(mut a: *mut archive) {
    let mut sc: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut sc_next: *mut archive_string_conv = 0 as *mut archive_string_conv;
    sc = (*a).sconv;
    while !sc.is_null() {
        sc_next = (*sc).next;
        free_sconv_object(sc);
        sc = sc_next
    }
    (*a).sconv = NULL as *mut archive_string_conv;
    free((*a).current_code as *mut libc::c_void);
    (*a).current_code = NULL as *mut libc::c_char;
}
/*
 * Return a conversion charset name.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_string_conversion_charset_name(
    mut sc: *mut archive_string_conv,
) -> *const libc::c_char {
    if (*sc).flag & SCONV_TO_CHARSET != 0 {
        return (*sc).to_charset;
    } else {
        return (*sc).from_charset;
    };
}
/*
 * Change the behavior of a string conversion.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_string_conversion_set_opt(
    mut sc: *mut archive_string_conv,
    mut opt: libc::c_int,
) {
    match opt {
        SCONV_SET_OPT_UTF8_LIBARCHIVE2X => {}
        SCONV_SET_OPT_NORMALIZATION_C => {
            if (*sc).flag & SCONV_NORMALIZATION_C == 0 as libc::c_int {
                (*sc).flag |= SCONV_NORMALIZATION_C;
                (*sc).flag &= !SCONV_NORMALIZATION_D;
                /* Set up string converters. */
                setup_converter(sc);
            }
        }
        SCONV_SET_OPT_NORMALIZATION_D => {
            /*
             * If iconv will take the string, do not change the
             * setting of the normalization.
             */
            if !((*sc).flag & SCONV_WIN_CP == 0
                && (*sc).flag & (SCONV_FROM_UTF16 | SCONV_FROM_UTF8) != 0
                && (*sc).flag & (SCONV_TO_UTF16 | SCONV_TO_UTF8) == 0)
            {
                if (*sc).flag & SCONV_NORMALIZATION_D == 0 as libc::c_int {
                    (*sc).flag |= SCONV_NORMALIZATION_D;
                    (*sc).flag &= !SCONV_NORMALIZATION_C;
                    /* Set up string converters. */
                    setup_converter(sc);
                }
            }
        }
        _ => {}
    };
}
/*
 *
 * Copy one archive_string to another in locale conversion.
 *
 *	archive_strncat_l();
 *	archive_strncpy_l();
 *
 */
unsafe extern "C" fn mbsnbytes(mut _p: *const libc::c_void, mut n: size_t) -> size_t {
    let mut s: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut pp: *const libc::c_char = 0 as *const libc::c_char;
    if _p == NULL as *const libc::c_void {
        return 0 as libc::c_int as size_t;
    }
    p = _p as *const libc::c_char;
    /* Like strlen(p), except won't examine positions beyond p[n]. */
    s = 0 as libc::c_int as size_t;
    pp = p;
    while s < n && *pp as libc::c_int != 0 {
        pp = pp.offset(1);
        s = s.wrapping_add(1)
    }
    return s;
}
unsafe extern "C" fn utf16nbytes(mut _p: *const libc::c_void, mut n: size_t) -> size_t {
    let mut s: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut pp: *const libc::c_char = 0 as *const libc::c_char;
    if _p == NULL as *const libc::c_void {
        return 0 as libc::c_int as size_t;
    }
    p = _p as *const libc::c_char;
    /* Like strlen(p), except won't examine positions beyond p[n]. */
    s = 0 as libc::c_int as size_t;
    pp = p;
    n >>= 1 as libc::c_int;
    while s < n
        && (*pp.offset(0 as libc::c_int as isize) as libc::c_int != 0
            || *pp.offset(1 as libc::c_int as isize) as libc::c_int != 0)
    {
        pp = pp.offset(2 as libc::c_int as isize);
        s = s.wrapping_add(1)
    }
    return s << 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_strncpy_l(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut n: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    (*as_0).length = 0 as libc::c_int as size_t;
    return archive_strncat_l(as_0, _p, n, sc);
}
#[no_mangle]
pub unsafe extern "C" fn archive_strncat_l(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut n: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut s: *const libc::c_void = 0 as *const libc::c_void;
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut r2: libc::c_int = 0;
    if _p != NULL as *const libc::c_void && n > 0 as libc::c_int as libc::c_ulong {
        if !sc.is_null() && (*sc).flag & SCONV_FROM_UTF16 != 0 {
            length = utf16nbytes(_p, n)
        } else {
            length = mbsnbytes(_p, n)
        }
    }
    /* We must allocate memory even if there is no data for conversion
     * or copy. This simulates archive_string_append behavior. */
    if length == 0 as libc::c_int as libc::c_ulong {
        let mut tn: libc::c_int = 1 as libc::c_int;
        if !sc.is_null() && (*sc).flag & SCONV_TO_UTF16 != 0 {
            tn = 2 as libc::c_int
        }
        if archive_string_ensure(as_0, (*as_0).length.wrapping_add(tn as libc::c_ulong)).is_null() {
            return -(1 as libc::c_int);
        }
        *(*as_0).s.offset((*as_0).length as isize) = 0 as libc::c_int as libc::c_char;
        if tn == 2 as libc::c_int {
            *(*as_0).s.offset(
                (*as_0)
                    .length
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char
        }
        return 0 as libc::c_int;
    }
    /*
     * If sc is NULL, we just make a copy.
     */
    if sc.is_null() {
        if archive_string_append(as_0, _p as *const libc::c_char, length).is_null() {
            return -(1 as libc::c_int);
        } /* No memory */
        return 0 as libc::c_int;
    }
    s = _p;
    i = 0 as libc::c_int;
    if (*sc).nconverter > 1 as libc::c_int {
        (*sc).utftmp.length = 0 as libc::c_int as size_t;
        r2 = (*sc).converter[0 as libc::c_int as usize].expect("non-null function pointer")(
            &mut (*sc).utftmp,
            s,
            length,
            sc,
        );
        if r2 != 0 as libc::c_int && errno == ENOMEM {
            return r2;
        }
        if r > r2 {
            r = r2
        }
        s = (*sc).utftmp.s as *const libc::c_void;
        length = (*sc).utftmp.length;
        i += 1
    }
    r2 = (*sc).converter[i as usize].expect("non-null function pointer")(as_0, s, length, sc);
    if r > r2 {
        r = r2
    }
    return r;
}
/*
 * Return -1 if conversion fails.
 */
unsafe extern "C" fn iconv_strncat_in_locale(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut length: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut itp: *mut libc::c_char = 0 as *mut libc::c_char; /* success */
    let mut remaining: size_t = 0; /* Conversion completed. */
    let mut cd: iconv_t = 0 as *mut libc::c_void;
    let mut outp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut avail: size_t = 0;
    let mut bs: size_t = 0;
    let mut return_value: libc::c_int = 0 as libc::c_int;
    let mut to_size: libc::c_int = 0;
    let mut from_size: libc::c_int = 0;
    if (*sc).flag & SCONV_TO_UTF16 != 0 {
        to_size = 2 as libc::c_int
    } else {
        to_size = 1 as libc::c_int
    }
    if (*sc).flag & SCONV_FROM_UTF16 != 0 {
        from_size = 2 as libc::c_int
    } else {
        from_size = 1 as libc::c_int
    }
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(length.wrapping_mul(2 as libc::c_int as libc::c_ulong))
            .wrapping_add(to_size as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    cd = (*sc).cd;
    itp = _p as uintptr_t as *mut libc::c_char;
    remaining = length;
    outp = (*as_0).s.offset((*as_0).length as isize);
    avail = (*as_0)
        .buffer_length
        .wrapping_sub((*as_0).length)
        .wrapping_sub(to_size as libc::c_ulong);
    while remaining >= from_size as size_t {
        let mut result: size_t = iconv(cd, &mut itp, &mut remaining, &mut outp, &mut avail);
        if result != -(1 as libc::c_int) as size_t {
            break;
        }
        if errno == EILSEQ || errno == EINVAL {
            /*
             * If an output charset is UTF-8 or UTF-16BE/LE,
             * unknown character should be U+FFFD
             * (replacement character).
             */
            if (*sc).flag & (SCONV_TO_UTF8 | SCONV_TO_UTF16) != 0 {
                let mut rbytes: size_t = 0;
                if (*sc).flag & SCONV_TO_UTF8 != 0 {
                    rbytes = ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                } else {
                    rbytes = 2 as libc::c_int as size_t
                }
                if avail < rbytes {
                    (*as_0).length = outp.offset_from((*as_0).s) as libc::c_long as size_t;
                    bs = (*as_0)
                        .buffer_length
                        .wrapping_add(remaining.wrapping_mul(to_size as libc::c_ulong))
                        .wrapping_add(rbytes);
                    if archive_string_ensure(as_0, bs).is_null() {
                        return -(1 as libc::c_int);
                    }
                    outp = (*as_0).s.offset((*as_0).length as isize);
                    avail = (*as_0)
                        .buffer_length
                        .wrapping_sub((*as_0).length)
                        .wrapping_sub(to_size as libc::c_ulong)
                }
                if (*sc).flag & SCONV_TO_UTF8 != 0 {
                    memcpy(
                        outp as *mut libc::c_void,
                        utf8_replacement_char.as_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
                    );
                } else if (*sc).flag & SCONV_TO_UTF16BE != 0 {
                    archive_be16enc(outp as *mut libc::c_void, UNICODE_R_CHAR as uint16_t);
                } else {
                    archive_le16enc(outp as *mut libc::c_void, UNICODE_R_CHAR as uint16_t);
                }
                outp = outp.offset(rbytes as isize);
                avail = (avail as libc::c_ulong).wrapping_sub(rbytes) as size_t as size_t
            } else {
                /* Skip the illegal input bytes. */
                let fresh6 = outp;
                outp = outp.offset(1);
                *fresh6 = '?' as i32 as libc::c_char;
                avail = avail.wrapping_sub(1)
            }
            itp = itp.offset(from_size as isize);
            remaining = (remaining as libc::c_ulong).wrapping_sub(from_size as libc::c_ulong)
                as size_t as size_t;
            return_value = -(1 as libc::c_int)
        /* failure */
        } else {
            /* E2BIG no output buffer,
             * Increase an output buffer.  */
            (*as_0).length = outp.offset_from((*as_0).s) as libc::c_long as size_t;
            bs = (*as_0)
                .buffer_length
                .wrapping_add(remaining.wrapping_mul(2 as libc::c_int as libc::c_ulong));
            if archive_string_ensure(as_0, bs).is_null() {
                return -(1 as libc::c_int);
            }
            outp = (*as_0).s.offset((*as_0).length as isize);
            avail = (*as_0)
                .buffer_length
                .wrapping_sub((*as_0).length)
                .wrapping_sub(to_size as libc::c_ulong)
        }
    }
    (*as_0).length = outp.offset_from((*as_0).s) as libc::c_long as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = 0 as libc::c_int as libc::c_char;
    if to_size == 2 as libc::c_int {
        *(*as_0).s.offset(
            (*as_0)
                .length
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as libc::c_char
    }
    return return_value;
}
/* HAVE_ICONV */
/*
 * Test whether MBS ==> WCS is okay.
 */
unsafe extern "C" fn invalid_mbs(
    mut _p: *const libc::c_void,
    mut n: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut p: *const libc::c_char = _p as *const libc::c_char; /* Invalid. */
    let mut r: size_t = 0;
    let mut shift_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed_0 { __wch: 0 },
    };
    memset(
        &mut shift_state as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    while n != 0 {
        let mut wc: wchar_t = 0;
        r = mbrtowc(&mut wc, p, n, &mut shift_state);
        if r == -(1 as libc::c_int) as size_t || r == -(2 as libc::c_int) as size_t {
            return -(1 as libc::c_int);
        }
        if r == 0 as libc::c_int as libc::c_ulong {
            break;
        }
        p = p.offset(r as isize);
        n = (n as libc::c_ulong).wrapping_sub(r) as size_t as size_t
    }
    /* UNUSED */
    return 0 as libc::c_int;
    /* All Okey. */
}
/* defined(_WIN32) && !defined(__CYGWIN__) */
/*
 * Basically returns -1 because we cannot make a conversion of charset
 * without iconv but in some cases this would return 0.
 * Returns 0 if all copied characters are ASCII.
 * Returns 0 if both from-locale and to-locale are the same and those
 * can be WCS with no error.
 */
unsafe extern "C" fn best_effort_strncat_in_locale(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut length: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut remaining: size_t = 0; /* success */
    let mut itp: *const uint8_t = 0 as *const uint8_t;
    let mut return_value: libc::c_int = 0 as libc::c_int;
    /*
     * If both from-locale and to-locale is the same, this makes a copy.
     * And then this checks all copied MBS can be WCS if so returns 0.
     */
    if (*sc).same != 0 {
        if archive_string_append(as_0, _p as *const libc::c_char, length).is_null() {
            return -(1 as libc::c_int);
        } /* No memory */
        return invalid_mbs(_p, length, sc);
    }
    /*
     * If a character is ASCII, this just copies it. If not, this
     * assigns '?' character instead but in UTF-8 locale this assigns
     * byte sequence 0xEF 0xBD 0xBD, which are code point U+FFFD,
     * a Replacement Character in Unicode.
     */
    remaining = length;
    itp = _p as *const uint8_t;
    while *itp as libc::c_int != 0 && remaining > 0 as libc::c_int as libc::c_ulong {
        if *itp as libc::c_int > 127 as libc::c_int {
            // Non-ASCII: Substitute with suitable replacement
            if (*sc).flag & SCONV_TO_UTF8 != 0 {
                if archive_string_append(
                    as_0,
                    utf8_replacement_char.as_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong,
                )
                .is_null()
                {
                    __archive_errx(
                        1 as libc::c_int,
                        b"Out of memory\x00" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                archive_strappend_char(as_0, '?' as i32 as libc::c_char);
            }
            return_value = -(1 as libc::c_int)
        } else {
            archive_strappend_char(as_0, *itp as libc::c_char);
        }
        itp = itp.offset(1)
    }
    return return_value;
}
/*
 * Unicode conversion functions.
 *   - UTF-8 <===> UTF-8 in removing surrogate pairs.
 *   - UTF-8 NFD ===> UTF-8 NFC in removing surrogate pairs.
 *   - UTF-8 made by libarchive 2.x ===> UTF-8.
 *   - UTF-16BE <===> UTF-8.
 *
 */
/*
 * Utility to convert a single UTF-8 sequence.
 *
 * Usually return used bytes, return used byte in negative value when
 * a unicode character is replaced with U+FFFD.
 * See also http://unicode.org/review/pr-121.html Public Review Issue #121
 * Recommended Practice for Replacement Characters.
 */
unsafe extern "C" fn _utf8_to_unicode(
    mut pwc: *mut uint32_t,
    mut s: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    static mut utf8_count: [libc::c_char; 256] = [
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        1 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        2 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        3 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        4 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut wc: uint32_t = 0;
    /* Sanity check. */
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    /*
     * Decode 1-4 bytes depending on the value of the first byte.
     */
    ch = *s as libc::c_uchar as libc::c_int; /* Standard:  return 0 for end-of-string. */
    if ch == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    cnt = utf8_count[ch as usize] as libc::c_int;
    /* Invalid sequence or there are not plenty bytes. */
    if (n as libc::c_int) < cnt {
        cnt = n as libc::c_int;
        i = 1 as libc::c_int;
        while i < cnt {
            if *s.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int != 0x80 as libc::c_int {
                cnt = i;
                break;
            } else {
                i += 1
            }
        }
    } else {
        /* Make a Unicode code point from a single UTF-8 sequence. */
        match cnt {
            1 => {
                /* 1 byte sequence. */
                *pwc = (ch & 0x7f as libc::c_int) as uint32_t;
                return cnt;
            }
            2 => {
                /* 2 bytes sequence. */
                if *s.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    cnt = 1 as libc::c_int
                } else {
                    *pwc = ((ch & 0x1f as libc::c_int) << 6 as libc::c_int
                        | *s.offset(1 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int)
                        as uint32_t;
                    return cnt;
                }
                current_block = 14387958397301732898;
            }
            3 => {
                /* 3 bytes sequence. */
                if *s.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    cnt = 1 as libc::c_int; /* Overlong sequence. */
                    current_block = 14387958397301732898;
                } else if *s.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    cnt = 2 as libc::c_int;
                    current_block = 14387958397301732898;
                } else {
                    wc = ((ch & 0xf as libc::c_int) << 12 as libc::c_int
                        | (*s.offset(1 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int)
                            << 6 as libc::c_int
                        | *s.offset(2 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int)
                        as uint32_t;
                    if wc < 0x800 as libc::c_int as libc::c_uint {
                        current_block = 14387958397301732898;
                    } else {
                        current_block = 2520131295878969859;
                    }
                }
            }
            4 => {
                /* 4 bytes sequence. */
                if *s.offset(1 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    cnt = 1 as libc::c_int; /* Overlong sequence. */
                    current_block = 14387958397301732898;
                } else if *s.offset(2 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    cnt = 2 as libc::c_int;
                    current_block = 14387958397301732898;
                } else if *s.offset(3 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
                    != 0x80 as libc::c_int
                {
                    cnt = 3 as libc::c_int;
                    current_block = 14387958397301732898;
                } else {
                    wc = ((ch & 0x7 as libc::c_int) << 18 as libc::c_int
                        | (*s.offset(1 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int)
                            << 12 as libc::c_int
                        | (*s.offset(2 as libc::c_int as isize) as libc::c_int
                            & 0x3f as libc::c_int)
                            << 6 as libc::c_int
                        | *s.offset(3 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int)
                        as uint32_t;
                    if wc < 0x10000 as libc::c_int as libc::c_uint {
                        current_block = 14387958397301732898;
                    } else {
                        current_block = 2520131295878969859;
                    }
                }
            }
            _ => {
                /* Others are all invalid sequence. */
                if ch == 0xc0 as libc::c_int || ch == 0xc1 as libc::c_int {
                    cnt = 2 as libc::c_int
                } else if ch >= 0xf5 as libc::c_int && ch <= 0xf7 as libc::c_int {
                    cnt = 4 as libc::c_int
                } else if ch >= 0xf8 as libc::c_int && ch <= 0xfb as libc::c_int {
                    cnt = 5 as libc::c_int
                } else if ch == 0xfc as libc::c_int || ch == 0xfd as libc::c_int {
                    cnt = 6 as libc::c_int
                } else {
                    cnt = 1 as libc::c_int
                }
                if (n as libc::c_int) < cnt {
                    cnt = n as libc::c_int
                }
                i = 1 as libc::c_int;
                while i < cnt {
                    if *s.offset(i as isize) as libc::c_int & 0xc0 as libc::c_int
                        != 0x80 as libc::c_int
                    {
                        cnt = i;
                        break;
                    } else {
                        i += 1
                    }
                }
                current_block = 14387958397301732898;
            }
        }
        match current_block {
            14387958397301732898 => {}
            _ =>
            /* The code point larger than 0x10FFFF is not legal
             * Unicode values. */
            {
                if !(wc > UNICODE_MAX as libc::c_uint) {
                    /* Correctly gets a Unicode, returns used bytes. */
                    *pwc = wc; /* set the Replacement Character instead. */
                    return cnt;
                }
            }
        }
    }
    *pwc = UNICODE_R_CHAR as uint32_t;
    return cnt * -(1 as libc::c_int);
}
unsafe extern "C" fn utf8_to_unicode(
    mut pwc: *mut uint32_t,
    mut s: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    let mut cnt: libc::c_int = 0;
    cnt = _utf8_to_unicode(pwc, s, n);
    /* Any of Surrogate pair is not legal Unicode values. */
    if cnt == 3 as libc::c_int
        && (*pwc >= 0xd800 as libc::c_int as libc::c_uint
            && *pwc <= 0xdfff as libc::c_int as libc::c_uint)
    {
        return -(3 as libc::c_int);
    }
    return cnt;
}
#[inline]
unsafe extern "C" fn combine_surrogate_pair(mut uc: uint32_t, mut uc2: uint32_t) -> uint32_t {
    uc = (uc as libc::c_uint).wrapping_sub(0xd800 as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    uc = (uc as libc::c_uint).wrapping_mul(0x400 as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    uc = (uc as libc::c_uint).wrapping_add(uc2.wrapping_sub(0xdc00 as libc::c_int as libc::c_uint))
        as uint32_t as uint32_t;
    uc = (uc as libc::c_uint).wrapping_add(0x10000 as libc::c_int as libc::c_uint) as uint32_t
        as uint32_t;
    return uc;
}
/*
 * Convert a single UTF-8/CESU-8 sequence to a Unicode code point in
 * removing surrogate pairs.
 *
 * CESU-8: The Compatibility Encoding Scheme for UTF-16.
 *
 * Usually return used bytes, return used byte in negative value when
 * a unicode character is replaced with U+FFFD.
 */
unsafe extern "C" fn cesu8_to_unicode(
    mut pwc: *mut uint32_t,
    mut s: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut wc: uint32_t = 0 as libc::c_int as uint32_t;
    let mut cnt: libc::c_int = 0;
    cnt = _utf8_to_unicode(&mut wc, s, n);
    if cnt == 3 as libc::c_int
        && (wc >= 0xd800 as libc::c_int as libc::c_uint
            && wc <= 0xdbff as libc::c_int as libc::c_uint)
    {
        let mut wc2: uint32_t = 0 as libc::c_int as uint32_t;
        if n.wrapping_sub(3 as libc::c_int as libc::c_ulong) < 3 as libc::c_int as libc::c_ulong {
            /* Invalid byte sequence. */
            current_block = 16073591882049499585;
        } else {
            cnt = _utf8_to_unicode(
                &mut wc2,
                s.offset(3 as libc::c_int as isize),
                n.wrapping_sub(3 as libc::c_int as libc::c_ulong),
            );
            if cnt != 3 as libc::c_int
                || !(wc2 >= 0xdc00 as libc::c_int as libc::c_uint
                    && wc2 <= 0xdfff as libc::c_int as libc::c_uint)
            {
                /* Invalid byte sequence. */
                current_block = 16073591882049499585;
            } else {
                wc = combine_surrogate_pair(wc, wc2);
                cnt = 6 as libc::c_int;
                current_block = 12209867499936983673;
            }
        }
    } else if cnt == 3 as libc::c_int
        && (wc >= 0xdc00 as libc::c_int as libc::c_uint
            && wc <= 0xdfff as libc::c_int as libc::c_uint)
    {
        /* Invalid byte sequence. */
        current_block = 16073591882049499585; /* set the Replacement Character instead. */
    } else {
        current_block = 12209867499936983673;
    }
    match current_block {
        16073591882049499585 => {
            *pwc = UNICODE_R_CHAR as uint32_t;
            if cnt > 0 as libc::c_int {
                cnt *= -(1 as libc::c_int)
            }
            return cnt;
        }
        _ => {
            *pwc = wc;
            return cnt;
        }
    };
}
/*
 * Convert a Unicode code point to a single UTF-8 sequence.
 *
 * NOTE:This function does not check if the Unicode is legal or not.
 * Please you definitely check it before calling this.
 */
unsafe extern "C" fn unicode_to_utf8(
    mut p: *mut libc::c_char,
    mut remaining: size_t,
    mut uc: uint32_t,
) -> size_t {
    let mut _p: *mut libc::c_char = p;
    /* Invalid Unicode char maps to Replacement character */
    if uc > UNICODE_MAX as libc::c_uint {
        uc = UNICODE_R_CHAR as uint32_t
    }
    /* Translate code point to UTF8 */
    if uc <= 0x7f as libc::c_int as libc::c_uint {
        if remaining == 0 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = uc as libc::c_char
    } else if uc <= 0x7ff as libc::c_int as libc::c_uint {
        if remaining < 2 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        let fresh8 = p;
        p = p.offset(1);
        *fresh8 = (0xc0 as libc::c_int as libc::c_uint
            | uc >> 6 as libc::c_int & 0x1f as libc::c_int as libc::c_uint)
            as libc::c_char;
        let fresh9 = p;
        p = p.offset(1);
        *fresh9 = (0x80 as libc::c_int as libc::c_uint | uc & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_char
    } else if uc <= 0xffff as libc::c_int as libc::c_uint {
        if remaining < 3 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = (0xe0 as libc::c_int as libc::c_uint
            | uc >> 12 as libc::c_int & 0xf as libc::c_int as libc::c_uint)
            as libc::c_char;
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = (0x80 as libc::c_int as libc::c_uint
            | uc >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_char;
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = (0x80 as libc::c_int as libc::c_uint | uc & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_char
    } else {
        if remaining < 4 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        let fresh13 = p;
        p = p.offset(1);
        *fresh13 = (0xf0 as libc::c_int as libc::c_uint
            | uc >> 18 as libc::c_int & 0x7 as libc::c_int as libc::c_uint)
            as libc::c_char;
        let fresh14 = p;
        p = p.offset(1);
        *fresh14 = (0x80 as libc::c_int as libc::c_uint
            | uc >> 12 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_char;
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = (0x80 as libc::c_int as libc::c_uint
            | uc >> 6 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_char;
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = (0x80 as libc::c_int as libc::c_uint | uc & 0x3f as libc::c_int as libc::c_uint)
            as libc::c_char
    }
    return p.offset_from(_p) as libc::c_long as size_t;
}
unsafe extern "C" fn utf16be_to_unicode(
    mut pwc: *mut uint32_t,
    mut s: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    return utf16_to_unicode(pwc, s, n, 1 as libc::c_int);
}
unsafe extern "C" fn utf16le_to_unicode(
    mut pwc: *mut uint32_t,
    mut s: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    return utf16_to_unicode(pwc, s, n, 0 as libc::c_int);
}
unsafe extern "C" fn utf16_to_unicode(
    mut pwc: *mut uint32_t,
    mut s: *const libc::c_char,
    mut n: size_t,
    mut be: libc::c_int,
) -> libc::c_int {
    let mut utf16: *const libc::c_char = s;
    let mut uc: libc::c_uint = 0;
    if n == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if n == 1 as libc::c_int as libc::c_ulong {
        /* set the Replacement Character instead. */
        *pwc = UNICODE_R_CHAR as uint32_t;
        return -(1 as libc::c_int);
    }
    if be != 0 {
        uc = archive_be16dec(utf16 as *const libc::c_void) as libc::c_uint
    } else {
        uc = archive_le16dec(utf16 as *const libc::c_void) as libc::c_uint
    }
    utf16 = utf16.offset(2 as libc::c_int as isize);
    /* If this is a surrogate pair, assemble the full code point.*/
    if uc >= 0xd800 as libc::c_int as libc::c_uint && uc <= 0xdbff as libc::c_int as libc::c_uint {
        let mut uc2: libc::c_uint = 0;
        if n >= 4 as libc::c_int as libc::c_ulong {
            if be != 0 {
                uc2 = archive_be16dec(utf16 as *const libc::c_void) as libc::c_uint
            } else {
                uc2 = archive_le16dec(utf16 as *const libc::c_void) as libc::c_uint
            }
        } else {
            uc2 = 0 as libc::c_int as libc::c_uint
        }
        if uc2 >= 0xdc00 as libc::c_int as libc::c_uint
            && uc2 <= 0xdfff as libc::c_int as libc::c_uint
        {
            uc = combine_surrogate_pair(uc, uc2);
            utf16 = utf16.offset(2 as libc::c_int as isize)
        } else {
            /* Undescribed code point should be U+FFFD
            	* (replacement character). */
            *pwc = UNICODE_R_CHAR as uint32_t;
            return -(2 as libc::c_int);
        }
    }
    /*
     * Surrogate pair values(0xd800 through 0xdfff) are only
     * used by UTF-16, so, after above calculation, the code
     * must not be surrogate values, and Unicode has no codes
     * larger than 0x10ffff. Thus, those are not legal Unicode
     * values.
     */
    if uc >= 0xd800 as libc::c_int as libc::c_uint && uc <= 0xdfff as libc::c_int as libc::c_uint
        || uc > UNICODE_MAX as libc::c_uint
    {
        /* Undescribed code point should be U+FFFD
        	* (replacement character). */
        *pwc = UNICODE_R_CHAR as uint32_t;
        return utf16.offset_from(s) as libc::c_long as libc::c_int * -(1 as libc::c_int);
    }
    *pwc = uc;
    return utf16.offset_from(s) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn unicode_to_utf16be(
    mut p: *mut libc::c_char,
    mut remaining: size_t,
    mut uc: uint32_t,
) -> size_t {
    let mut utf16: *mut libc::c_char = p;
    if uc > 0xffff as libc::c_int as libc::c_uint {
        /* We have a code point that won't fit into a
         * wchar_t; convert it to a surrogate pair. */
        if remaining < 4 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        uc = (uc as libc::c_uint).wrapping_sub(0x10000 as libc::c_int as libc::c_uint) as uint32_t
            as uint32_t;
        archive_be16enc(
            utf16 as *mut libc::c_void,
            (uc >> 10 as libc::c_int & 0x3ff as libc::c_int as libc::c_uint)
                .wrapping_add(0xd800 as libc::c_int as libc::c_uint) as uint16_t,
        );
        archive_be16enc(
            utf16.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            (uc & 0x3ff as libc::c_int as libc::c_uint)
                .wrapping_add(0xdc00 as libc::c_int as libc::c_uint) as uint16_t,
        );
        return 4 as libc::c_int as size_t;
    } else {
        if remaining < 2 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        archive_be16enc(utf16 as *mut libc::c_void, uc as uint16_t);
        return 2 as libc::c_int as size_t;
    };
}
unsafe extern "C" fn unicode_to_utf16le(
    mut p: *mut libc::c_char,
    mut remaining: size_t,
    mut uc: uint32_t,
) -> size_t {
    let mut utf16: *mut libc::c_char = p;
    if uc > 0xffff as libc::c_int as libc::c_uint {
        /* We have a code point that won't fit into a
         * wchar_t; convert it to a surrogate pair. */
        if remaining < 4 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        uc = (uc as libc::c_uint).wrapping_sub(0x10000 as libc::c_int as libc::c_uint) as uint32_t
            as uint32_t;
        archive_le16enc(
            utf16 as *mut libc::c_void,
            (uc >> 10 as libc::c_int & 0x3ff as libc::c_int as libc::c_uint)
                .wrapping_add(0xd800 as libc::c_int as libc::c_uint) as uint16_t,
        );
        archive_le16enc(
            utf16.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            (uc & 0x3ff as libc::c_int as libc::c_uint)
                .wrapping_add(0xdc00 as libc::c_int as libc::c_uint) as uint16_t,
        );
        return 4 as libc::c_int as size_t;
    } else {
        if remaining < 2 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int as size_t;
        }
        archive_le16enc(utf16 as *mut libc::c_void, uc as uint16_t);
        return 2 as libc::c_int as size_t;
    };
}
/*
 * Copy UTF-8 string in checking surrogate pair.
 * If any surrogate pair are found, it would be canonicalized.
 */
unsafe extern "C" fn strncat_from_utf8_to_utf8(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    /* UNUSED */
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    s = _p as *const libc::c_char;
    p = (*as_0).s.offset((*as_0).length as isize);
    endp = (*as_0)
        .s
        .offset((*as_0).buffer_length as isize)
        .offset(-(1 as libc::c_int as isize));
    loop {
        let mut uc: uint32_t = 0;
        let mut ss: *const libc::c_char = s;
        let mut w: size_t = 0;
        loop
        /*
         * Forward byte sequence until a conversion of that is needed.
         */
        {
            n = utf8_to_unicode(&mut uc, s, len);
            if !(n > 0 as libc::c_int) {
                break;
            }
            s = s.offset(n as isize);
            len = (len as libc::c_ulong).wrapping_sub(n as libc::c_ulong) as size_t as size_t
        }
        if ss < s {
            if p.offset(s.offset_from(ss) as libc::c_long as isize) > endp {
                (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
                if archive_string_ensure(
                    as_0,
                    (*as_0)
                        .buffer_length
                        .wrapping_add(len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                )
                .is_null()
                {
                    return -(1 as libc::c_int);
                }
                p = (*as_0).s.offset((*as_0).length as isize);
                endp = (*as_0)
                    .s
                    .offset((*as_0).buffer_length as isize)
                    .offset(-(1 as libc::c_int as isize))
            }
            memcpy(
                p as *mut libc::c_void,
                ss as *const libc::c_void,
                s.offset_from(ss) as libc::c_long as libc::c_ulong,
            );
            p = p.offset(s.offset_from(ss) as libc::c_long as isize)
        }
        /*
         * If n is negative, current byte sequence needs a replacement.
         */
        if n < 0 as libc::c_int {
            if n == -(3 as libc::c_int)
                && (uc >= 0xd800 as libc::c_int as libc::c_uint
                    && uc <= 0xdfff as libc::c_int as libc::c_uint)
            {
                /* Current byte sequence may be CESU-8. */
                n = cesu8_to_unicode(&mut uc, s, len)
            }
            if n < 0 as libc::c_int {
                ret = -(1 as libc::c_int);
                n *= -(1 as libc::c_int)
                /* Use a replaced unicode character. */
            }
            loop
            /* Rebuild UTF-8 byte sequence. */
            {
                w = unicode_to_utf8(
                    p,
                    endp.offset_from(p) as libc::c_long as size_t,
                    uc,
                );
                if !(w == 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
                (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
                if archive_string_ensure(
                    as_0,
                    (*as_0)
                        .buffer_length
                        .wrapping_add(len)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                )
                .is_null()
                {
                    return -(1 as libc::c_int);
                }
                p = (*as_0).s.offset((*as_0).length as isize);
                endp = (*as_0)
                    .s
                    .offset((*as_0).buffer_length as isize)
                    .offset(-(1 as libc::c_int as isize))
            }
            p = p.offset(w as isize);
            s = s.offset(n as isize);
            len = (len as libc::c_ulong).wrapping_sub(n as libc::c_ulong) as size_t as size_t
        }
        if !(n > 0 as libc::c_int) {
            break;
        }
    }
    (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = '\u{0}' as i32 as libc::c_char;
    return ret;
}
unsafe extern "C" fn archive_string_append_unicode(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uc: uint32_t = 0;
    let mut w: size_t = 0;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ts: libc::c_int = 0;
    let mut tm: libc::c_int = 0;
    let mut parse: Option<
        unsafe extern "C" fn(_: *mut uint32_t, _: *const libc::c_char, _: size_t) -> libc::c_int,
    > = None;
    let mut unparse: Option<
        unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
    > = None;
    if (*sc).flag & SCONV_TO_UTF16BE != 0 {
        unparse = Some(
            unicode_to_utf16be
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 2 as libc::c_int
    } else if (*sc).flag & SCONV_TO_UTF16LE != 0 {
        unparse = Some(
            unicode_to_utf16le
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 2 as libc::c_int
    } else if (*sc).flag & SCONV_TO_UTF8 != 0 {
        unparse = Some(
            unicode_to_utf8
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 1 as libc::c_int
    } else if (*sc).flag & SCONV_FROM_UTF16BE != 0 {
        unparse = Some(
            unicode_to_utf16be
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 2 as libc::c_int
    } else if (*sc).flag & SCONV_FROM_UTF16LE != 0 {
        unparse = Some(
            unicode_to_utf16le
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 2 as libc::c_int
    } else {
        unparse = Some(
            unicode_to_utf8
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 1 as libc::c_int
    }
    if (*sc).flag & SCONV_FROM_UTF16BE != 0 {
        parse = Some(
            utf16be_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = 1 as libc::c_int
    } else if (*sc).flag & SCONV_FROM_UTF16LE != 0 {
        parse = Some(
            utf16le_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = 1 as libc::c_int
    } else {
        parse = Some(
            cesu8_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = ts
    }
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
            .wrapping_add(ts as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    s = _p as *const libc::c_char;
    p = (*as_0).s.offset((*as_0).length as isize);
    endp = (*as_0)
        .s
        .offset((*as_0).buffer_length as isize)
        .offset(-(ts as isize));
    loop {
        n = parse.expect("non-null function pointer")(&mut uc, s, len);
        if !(n != 0 as libc::c_int) {
            break;
        }
        if n < 0 as libc::c_int {
            /*
             * This case is going to be converted to another
             * character-set through iconv.
             */
            /* Use a replaced unicode character. */
            n *= -(1 as libc::c_int);
            ret = -(1 as libc::c_int)
        }
        s = s.offset(n as isize);
        len = (len as libc::c_ulong).wrapping_sub(n as libc::c_ulong) as size_t as size_t;
        loop {
            w = unparse.expect("non-null function pointer")(
                p,
                endp.offset_from(p) as libc::c_long as size_t,
                uc,
            );
            if !(w == 0 as libc::c_int as libc::c_ulong) {
                break;
            }
            /* There is not enough output buffer so
             * we have to expand it. */
            (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
            if archive_string_ensure(
                as_0,
                (*as_0)
                    .buffer_length
                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                    .wrapping_add(ts as libc::c_ulong),
            )
            .is_null()
            {
                return -(1 as libc::c_int);
            }
            p = (*as_0).s.offset((*as_0).length as isize);
            endp = (*as_0)
                .s
                .offset((*as_0).buffer_length as isize)
                .offset(-(ts as isize))
        }
        p = p.offset(w as isize)
    }
    (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = '\u{0}' as i32 as libc::c_char;
    if ts == 2 as libc::c_int {
        *(*as_0).s.offset(
            (*as_0)
                .length
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\u{0}' as i32 as libc::c_char
    }
    return ret;
}
/*
 * Following Constants for Hangul compositions this information comes from
 * Unicode Standard Annex #15  http://unicode.org/reports/tr15/
 */
pub const HC_SBASE: libc::c_int = 0xac00 as libc::c_int;
pub const HC_LBASE: libc::c_int = 0x1100 as libc::c_int;
pub const HC_VBASE: libc::c_int = 0x1161 as libc::c_int;
pub const HC_TBASE: libc::c_int = 0x11a7 as libc::c_int;
pub const HC_LCOUNT: libc::c_int = 19 as libc::c_int;
pub const HC_VCOUNT: libc::c_int = 21 as libc::c_int;
pub const HC_TCOUNT: libc::c_int = 28 as libc::c_int;
pub const HC_NCOUNT: libc::c_int = HC_VCOUNT * HC_TCOUNT;
pub const HC_SCOUNT: libc::c_int = HC_LCOUNT * HC_NCOUNT;
unsafe extern "C" fn get_nfc(mut uc: uint32_t, mut uc2: uint32_t) -> uint32_t {
    let mut t: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    t = 0 as libc::c_int;
    b = (::std::mem::size_of::<[unicode_composition_table; 931]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<unicode_composition_table>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while b >= t {
        let mut m: libc::c_int = (t + b) / 2 as libc::c_int;
        if u_composition_table[m as usize].cp1 < uc {
            t = m + 1 as libc::c_int
        } else if u_composition_table[m as usize].cp1 > uc {
            b = m - 1 as libc::c_int
        } else if u_composition_table[m as usize].cp2 < uc2 {
            t = m + 1 as libc::c_int
        } else if u_composition_table[m as usize].cp2 > uc2 {
            b = m - 1 as libc::c_int
        } else {
            return u_composition_table[m as usize].nfc;
        }
    }
    return 0 as libc::c_int as uint32_t;
}
pub const FDC_MAX: libc::c_int = 10 as libc::c_int;
/* The maximum number of Following Decomposable
 * Characters. */
/*
 * Update first code point.
 */
/*
 * Replace first code point with second code point.
 */
/*
 * Write first code point.
 * If the code point has not be changed from its original code,
 * this just copies it from its original buffer pointer.
 * If not, this converts it to UTF-8 byte sequence and copies it.
 */
/* FALL THROUGH */
/* FALL THROUGH */
/* FALL THROUGH */
/*
 * Collect following decomposable code points.
 */
/*
 * Normalize UTF-8/UTF-16BE characters to Form C and copy the result.
 *
 * TODO: Convert composition exclusions, which are never converted
 * from NFC,NFD,NFKC and NFKD, to Form C.
 */
unsafe extern "C" fn archive_string_normalize_C(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut s: *const libc::c_char = _p as *const libc::c_char; /* text size. */
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uc: uint32_t = 0;
    let mut uc2: uint32_t = 0;
    let mut w: size_t = 0;
    let mut always_replace: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut spair: libc::c_int = 0;
    let mut ts: libc::c_int = 0;
    let mut tm: libc::c_int = 0;
    let mut parse: Option<
        unsafe extern "C" fn(_: *mut uint32_t, _: *const libc::c_char, _: size_t) -> libc::c_int,
    > = None;
    let mut unparse: Option<
        unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
    > = None;
    always_replace = 1 as libc::c_int;
    ts = 1 as libc::c_int;
    if (*sc).flag & SCONV_TO_UTF16BE != 0 {
        unparse = Some(
            unicode_to_utf16be
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 2 as libc::c_int;
        if (*sc).flag & SCONV_FROM_UTF16BE != 0 {
            always_replace = 0 as libc::c_int
        }
    } else if (*sc).flag & SCONV_TO_UTF16LE != 0 {
        unparse = Some(
            unicode_to_utf16le
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 2 as libc::c_int;
        if (*sc).flag & SCONV_FROM_UTF16LE != 0 {
            always_replace = 0 as libc::c_int
        }
    } else if (*sc).flag & SCONV_TO_UTF8 != 0 {
        unparse = Some(
            unicode_to_utf8
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        if (*sc).flag & SCONV_FROM_UTF8 != 0 {
            always_replace = 0 as libc::c_int
        }
    } else {
        /*
         * This case is going to be converted to another
         * character-set through iconv.
         */
        always_replace = 0 as libc::c_int;
        if (*sc).flag & SCONV_FROM_UTF16BE != 0 {
            unparse = Some(
                unicode_to_utf16be
                    as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
            );
            ts = 2 as libc::c_int
        } else if (*sc).flag & SCONV_FROM_UTF16LE != 0 {
            unparse = Some(
                unicode_to_utf16le
                    as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
            );
            ts = 2 as libc::c_int
        } else {
            unparse = Some(
                unicode_to_utf8
                    as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
            )
        }
    }
    if (*sc).flag & SCONV_FROM_UTF16BE != 0 {
        parse = Some(
            utf16be_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = 1 as libc::c_int;
        spair = 4 as libc::c_int
    /* surrogate pair size in UTF-16. */
    } else if (*sc).flag & SCONV_FROM_UTF16LE != 0 {
        parse = Some(
            utf16le_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = 1 as libc::c_int;
        spair = 4 as libc::c_int
    /* surrogate pair size in UTF-16. */
    } else {
        parse = Some(
            cesu8_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = ts;
        spair = 6 as libc::c_int
        /* surrogate pair size in UTF-8. */
    }
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
            .wrapping_add(ts as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    p = (*as_0).s.offset((*as_0).length as isize);
    endp = (*as_0)
        .s
        .offset((*as_0).buffer_length as isize)
        .offset(-(ts as isize));
    loop {
        n = parse.expect("non-null function pointer")(&mut uc, s, len);
        if !(n != 0 as libc::c_int) {
            break;
        }
        let mut ucptr: *const libc::c_char = 0 as *const libc::c_char;
        let mut uc2ptr: *const libc::c_char = 0 as *const libc::c_char;
        if n < 0 as libc::c_int {
            /* Use a replaced unicode character. */
            loop {
                w = unparse.expect("non-null function pointer")(
                    p,
                    endp.offset_from(p) as libc::c_long as size_t,
                    uc,
                );
                if !(w == 0 as libc::c_int as libc::c_ulong) {
                    break;
                }
                (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
                if archive_string_ensure(
                    as_0,
                    (*as_0)
                        .buffer_length
                        .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                        .wrapping_add(ts as libc::c_ulong),
                )
                .is_null()
                {
                    return -(1 as libc::c_int);
                }
                p = (*as_0).s.offset((*as_0).length as isize);
                endp = (*as_0)
                    .s
                    .offset((*as_0).buffer_length as isize)
                    .offset(-(ts as isize))
            }
            p = p.offset(w as isize);
            s = s.offset((n * -(1 as libc::c_int)) as isize);
            len = (len as libc::c_ulong).wrapping_sub((n * -(1 as libc::c_int)) as libc::c_ulong)
                as size_t as size_t;
            ret = -(1 as libc::c_int)
        } else {
            if n == spair || always_replace != 0 {
                /* uc is converted from a surrogate pair.
                 * this should be treated as a changed code. */
                ucptr = NULL as *const libc::c_char
            } else {
                ucptr = s
            }
            s = s.offset(n as isize);
            len = (len as libc::c_ulong).wrapping_sub(n as libc::c_ulong) as size_t as size_t;
            loop
            /* Read second code point. */
            {
                n2 = parse.expect("non-null function pointer")(&mut uc2, s, len);
                if !(n2 > 0 as libc::c_int) {
                    break;
                }
                let mut ucx: [uint32_t; 10] = [0; 10];
                let mut ccx: [libc::c_int; 10] = [0; 10];
                let mut cl: libc::c_int = 0;
                let mut cx: libc::c_int = 0;
                let mut i: libc::c_int = 0;
                let mut nx: libc::c_int = 0;
                let mut ucx_size: libc::c_int = 0;
                let mut LIndex: libc::c_int = 0;
                let mut SIndex: libc::c_int = 0;
                let mut nfc: uint32_t = 0;
                if n2 == spair || always_replace != 0 {
                    /* uc2 is converted from a surrogate pair.
                     * this should be treated as a changed code. */
                    uc2ptr = NULL as *const libc::c_char
                } else {
                    uc2ptr = s
                }
                s = s.offset(n2 as isize);
                len = (len as libc::c_ulong).wrapping_sub(n2 as libc::c_ulong) as size_t as size_t;
                /*
                 * If current second code point is out of decomposable
                 * code points, finding compositions is unneeded.
                 */
                if !(uc2 >> 8 as libc::c_int <= 0x1d2 as libc::c_int as libc::c_uint
                    && u_decomposable_blocks[(uc2 >> 8 as libc::c_int) as usize] as libc::c_int
                        != 0)
                {
                    if !ucptr.is_null() {
                        if p.offset(n as isize) > endp {
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        let mut current_block_85: u64;
                        match n {
                            4 => {
                                let fresh17 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh18 = p;
                                p = p.offset(1);
                                *fresh18 = *fresh17;
                                current_block_85 = 11836324135612123818;
                            }
                            3 => {
                                current_block_85 = 11836324135612123818;
                            }
                            2 => {
                                current_block_85 = 8859835090198054363;
                            }
                            1 => {
                                current_block_85 = 3422898060693054794;
                            }
                            _ => {
                                current_block_85 = 7301440000599063274;
                            }
                        }
                        match current_block_85 {
                            11836324135612123818 => {
                                let fresh19 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh20 = p;
                                p = p.offset(1);
                                *fresh20 = *fresh19;
                                current_block_85 = 8859835090198054363;
                            }
                            _ => {}
                        }
                        match current_block_85 {
                            8859835090198054363 => {
                                let fresh21 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh22 = p;
                                p = p.offset(1);
                                *fresh22 = *fresh21;
                                current_block_85 = 3422898060693054794;
                            }
                            _ => {}
                        }
                        match current_block_85 {
                            3422898060693054794 => {
                                let fresh23 = p;
                                p = p.offset(1);
                                *fresh23 = *ucptr
                            }
                            _ => {}
                        }
                        ucptr = NULL as *const libc::c_char
                    } else {
                        loop {
                            w = unparse.expect("non-null function pointer")(
                                p,
                                endp.offset_from(p) as libc::c_long as size_t,
                                uc,
                            );
                            if !(w == 0 as libc::c_int as libc::c_ulong) {
                                break;
                            }
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        p = p.offset(w as isize)
                    }
                    uc = uc2;
                    ucptr = uc2ptr;
                    n = n2
                } else {
                    /*
                     * Try to combine current code points.
                     */
                    /*
                     * We have to combine Hangul characters according to
                     * http://uniicode.org/reports/tr15/#Hangul
                     */
                    LIndex = uc.wrapping_sub(HC_LBASE as libc::c_uint) as libc::c_int;
                    if 0 as libc::c_int <= LIndex && LIndex < HC_LCOUNT {
                        /*
                         * Hangul Composition.
                         * 1. Two current code points are L and V.
                         */
                        let mut VIndex: libc::c_int =
                            uc2.wrapping_sub(HC_VBASE as libc::c_uint) as libc::c_int;
                        if 0 as libc::c_int <= VIndex && VIndex < HC_VCOUNT {
                            /* Make syllable of form LV. */
                            uc = (0xac00 as libc::c_int
                                + (LIndex * 21 as libc::c_int + VIndex) * 28 as libc::c_int)
                                as uint32_t;
                            ucptr = NULL as *const libc::c_char
                        } else {
                            if !ucptr.is_null() {
                                if p.offset(n as isize) > endp {
                                    (*as_0).length =
                                        p.offset_from((*as_0).s) as libc::c_long as size_t;
                                    if archive_string_ensure(
                                        as_0,
                                        (*as_0)
                                            .buffer_length
                                            .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                            .wrapping_add(ts as libc::c_ulong),
                                    )
                                    .is_null()
                                    {
                                        return -(1 as libc::c_int);
                                    }
                                    p = (*as_0).s.offset((*as_0).length as isize);
                                    endp = (*as_0)
                                        .s
                                        .offset((*as_0).buffer_length as isize)
                                        .offset(-(ts as isize))
                                }
                                let mut current_block_126: u64;
                                match n {
                                    4 => {
                                        let fresh24 = ucptr;
                                        ucptr = ucptr.offset(1);
                                        let fresh25 = p;
                                        p = p.offset(1);
                                        *fresh25 = *fresh24;
                                        current_block_126 = 15924199146436505371;
                                    }
                                    3 => {
                                        current_block_126 = 15924199146436505371;
                                    }
                                    2 => {
                                        current_block_126 = 1100012244865001999;
                                    }
                                    1 => {
                                        current_block_126 = 9052778393660674436;
                                    }
                                    _ => {
                                        current_block_126 = 7293850626974290116;
                                    }
                                }
                                match current_block_126 {
                                    15924199146436505371 => {
                                        let fresh26 = ucptr;
                                        ucptr = ucptr.offset(1);
                                        let fresh27 = p;
                                        p = p.offset(1);
                                        *fresh27 = *fresh26;
                                        current_block_126 = 1100012244865001999;
                                    }
                                    _ => {}
                                }
                                match current_block_126 {
                                    1100012244865001999 => {
                                        let fresh28 = ucptr;
                                        ucptr = ucptr.offset(1);
                                        let fresh29 = p;
                                        p = p.offset(1);
                                        *fresh29 = *fresh28;
                                        current_block_126 = 9052778393660674436;
                                    }
                                    _ => {}
                                }
                                match current_block_126 {
                                    9052778393660674436 => {
                                        let fresh30 = p;
                                        p = p.offset(1);
                                        *fresh30 = *ucptr
                                    }
                                    _ => {}
                                }
                                ucptr = NULL as *const libc::c_char
                            } else {
                                loop {
                                    w = unparse.expect("non-null function pointer")(
                                        p,
                                        endp.offset_from(p) as libc::c_long as size_t,
                                        uc,
                                    );
                                    if !(w == 0 as libc::c_int as libc::c_ulong) {
                                        break;
                                    }
                                    (*as_0).length =
                                        p.offset_from((*as_0).s) as libc::c_long as size_t;
                                    if archive_string_ensure(
                                        as_0,
                                        (*as_0)
                                            .buffer_length
                                            .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                            .wrapping_add(ts as libc::c_ulong),
                                    )
                                    .is_null()
                                    {
                                        return -(1 as libc::c_int);
                                    }
                                    p = (*as_0).s.offset((*as_0).length as isize);
                                    endp = (*as_0)
                                        .s
                                        .offset((*as_0).buffer_length as isize)
                                        .offset(-(ts as isize))
                                }
                                p = p.offset(w as isize)
                            }
                            uc = uc2;
                            ucptr = uc2ptr;
                            n = n2
                        }
                    } else {
                        SIndex = uc.wrapping_sub(HC_SBASE as libc::c_uint) as libc::c_int;
                        if 0 as libc::c_int <= SIndex
                            && SIndex < HC_SCOUNT
                            && SIndex % HC_TCOUNT == 0 as libc::c_int
                        {
                            /*
                             * Hangul Composition.
                             * 2. Two current code points are LV and T.
                             */
                            let mut TIndex: libc::c_int =
                                uc2.wrapping_sub(HC_TBASE as libc::c_uint) as libc::c_int;
                            if (0 as libc::c_int) < TIndex && TIndex < HC_TCOUNT {
                                /* Make syllable of form LVT. */
                                uc = uc.wrapping_add(TIndex as libc::c_uint);
                                ucptr = NULL as *const libc::c_char
                            } else {
                                if !ucptr.is_null() {
                                    if p.offset(n as isize) > endp {
                                        (*as_0).length = p.offset_from((*as_0).s)
                                            as libc::c_long
                                            as size_t;
                                        if archive_string_ensure(
                                            as_0,
                                            (*as_0)
                                                .buffer_length
                                                .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                                .wrapping_add(ts as libc::c_ulong),
                                        )
                                        .is_null()
                                        {
                                            return -(1 as libc::c_int);
                                        }
                                        p = (*as_0).s.offset((*as_0).length as isize);
                                        endp = (*as_0)
                                            .s
                                            .offset((*as_0).buffer_length as isize)
                                            .offset(-(ts as isize))
                                    }
                                    let mut current_block_169: u64;
                                    match n {
                                        4 => {
                                            let fresh31 = ucptr;
                                            ucptr = ucptr.offset(1);
                                            let fresh32 = p;
                                            p = p.offset(1);
                                            *fresh32 = *fresh31;
                                            current_block_169 = 7018216100678426810;
                                        }
                                        3 => {
                                            current_block_169 = 7018216100678426810;
                                        }
                                        2 => {
                                            current_block_169 = 13110120122054567791;
                                        }
                                        1 => {
                                            current_block_169 = 3993379603071744588;
                                        }
                                        _ => {
                                            current_block_169 = 857031028540284188;
                                        }
                                    }
                                    match current_block_169 {
                                        7018216100678426810 => {
                                            let fresh33 = ucptr;
                                            ucptr = ucptr.offset(1);
                                            let fresh34 = p;
                                            p = p.offset(1);
                                            *fresh34 = *fresh33;
                                            current_block_169 = 13110120122054567791;
                                        }
                                        _ => {}
                                    }
                                    match current_block_169 {
                                        13110120122054567791 => {
                                            let fresh35 = ucptr;
                                            ucptr = ucptr.offset(1);
                                            let fresh36 = p;
                                            p = p.offset(1);
                                            *fresh36 = *fresh35;
                                            current_block_169 = 3993379603071744588;
                                        }
                                        _ => {}
                                    }
                                    match current_block_169 {
                                        3993379603071744588 => {
                                            let fresh37 = p;
                                            p = p.offset(1);
                                            *fresh37 = *ucptr
                                        }
                                        _ => {}
                                    }
                                    ucptr = NULL as *const libc::c_char
                                } else {
                                    loop {
                                        w = unparse.expect("non-null function pointer")(
                                            p,
                                            endp.offset_from(p) as libc::c_long as size_t,
                                            uc,
                                        );
                                        if !(w == 0 as libc::c_int as libc::c_ulong) {
                                            break;
                                        }
                                        (*as_0).length = p.offset_from((*as_0).s)
                                            as libc::c_long
                                            as size_t;
                                        if archive_string_ensure(
                                            as_0,
                                            (*as_0)
                                                .buffer_length
                                                .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                                .wrapping_add(ts as libc::c_ulong),
                                        )
                                        .is_null()
                                        {
                                            return -(1 as libc::c_int);
                                        }
                                        p = (*as_0).s.offset((*as_0).length as isize);
                                        endp = (*as_0)
                                            .s
                                            .offset((*as_0).buffer_length as isize)
                                            .offset(-(ts as isize))
                                    }
                                    p = p.offset(w as isize)
                                }
                                uc = uc2;
                                ucptr = uc2ptr;
                                n = n2
                            }
                        } else {
                            nfc = get_nfc(uc, uc2);
                            if nfc != 0 as libc::c_int as libc::c_uint {
                                /* A composition to current code points
                                 * is found. */
                                uc = nfc;
                                ucptr = NULL as *const libc::c_char
                            } else {
                                cl = (if uc2 > 0x1d244 as libc::c_int as libc::c_uint {
                                    0 as libc::c_int
                                } else {
                                    ccc_val[ccc_val_index
                                        [ccc_index[(uc2 >> 8 as libc::c_int) as usize] as usize]
                                        [(uc2 >> 4 as libc::c_int
                                            & 0xf as libc::c_int as libc::c_uint)
                                            as usize]
                                        as usize]
                                        [(uc2 & 0xf as libc::c_int as libc::c_uint) as usize]
                                        as libc::c_int
                                });
                                if cl == 0 as libc::c_int {
                                    /* Clearly 'uc2' the second code point is not
                                     * a decomposable code. */
                                    if !ucptr.is_null() {
                                        if p.offset(n as isize) > endp {
                                            (*as_0).length = p.offset_from((*as_0).s)
                                                as libc::c_long
                                                as size_t;
                                            if archive_string_ensure(
                                                as_0,
                                                (*as_0)
                                                    .buffer_length
                                                    .wrapping_add(
                                                        len.wrapping_mul(tm as libc::c_ulong),
                                                    )
                                                    .wrapping_add(ts as libc::c_ulong),
                                            )
                                            .is_null()
                                            {
                                                return -(1 as libc::c_int);
                                            }
                                            p = (*as_0).s.offset((*as_0).length as isize);
                                            endp = (*as_0)
                                                .s
                                                .offset((*as_0).buffer_length as isize)
                                                .offset(-(ts as isize))
                                        }
                                        let mut current_block_211: u64;
                                        match n {
                                            4 => {
                                                let fresh38 = ucptr;
                                                ucptr = ucptr.offset(1);
                                                let fresh39 = p;
                                                p = p.offset(1);
                                                *fresh39 = *fresh38;
                                                current_block_211 = 4247167652607022228;
                                            }
                                            3 => {
                                                current_block_211 = 4247167652607022228;
                                            }
                                            2 => {
                                                current_block_211 = 12634553267770736951;
                                            }
                                            1 => {
                                                current_block_211 = 926775400160495934;
                                            }
                                            _ => {
                                                current_block_211 = 10644040035716118461;
                                            }
                                        }
                                        match current_block_211 {
                                            4247167652607022228 => {
                                                let fresh40 = ucptr;
                                                ucptr = ucptr.offset(1);
                                                let fresh41 = p;
                                                p = p.offset(1);
                                                *fresh41 = *fresh40;
                                                current_block_211 = 12634553267770736951;
                                            }
                                            _ => {}
                                        }
                                        match current_block_211 {
                                            12634553267770736951 => {
                                                let fresh42 = ucptr;
                                                ucptr = ucptr.offset(1);
                                                let fresh43 = p;
                                                p = p.offset(1);
                                                *fresh43 = *fresh42;
                                                current_block_211 = 926775400160495934;
                                            }
                                            _ => {}
                                        }
                                        match current_block_211 {
                                            926775400160495934 => {
                                                let fresh44 = p;
                                                p = p.offset(1);
                                                *fresh44 = *ucptr
                                            }
                                            _ => {}
                                        }
                                        ucptr = NULL as *const libc::c_char
                                    } else {
                                        loop {
                                            w = unparse.expect("non-null function pointer")(
                                                p,
                                                endp.offset_from(p) as libc::c_long
                                                    as size_t,
                                                uc,
                                            );
                                            if !(w == 0 as libc::c_int as libc::c_ulong) {
                                                break;
                                            }
                                            (*as_0).length = p.offset_from((*as_0).s)
                                                as libc::c_long
                                                as size_t;
                                            if archive_string_ensure(
                                                as_0,
                                                (*as_0)
                                                    .buffer_length
                                                    .wrapping_add(
                                                        len.wrapping_mul(tm as libc::c_ulong),
                                                    )
                                                    .wrapping_add(ts as libc::c_ulong),
                                            )
                                            .is_null()
                                            {
                                                return -(1 as libc::c_int);
                                            }
                                            p = (*as_0).s.offset((*as_0).length as isize);
                                            endp = (*as_0)
                                                .s
                                                .offset((*as_0).buffer_length as isize)
                                                .offset(-(ts as isize))
                                        }
                                        p = p.offset(w as isize)
                                    }
                                    uc = uc2;
                                    ucptr = uc2ptr;
                                    n = n2
                                } else {
                                    /*
                                     * Collect following decomposable code points.
                                     */
                                    cx = 0 as libc::c_int;
                                    ucx[0 as libc::c_int as usize] = uc2;
                                    ccx[0 as libc::c_int as usize] = cl;
                                    let mut _i: libc::c_int = 0;
                                    _i = 1 as libc::c_int;
                                    while _i < FDC_MAX {
                                        nx = parse.expect("non-null function pointer")(
                                            &mut *ucx.as_mut_ptr().offset(_i as isize),
                                            s,
                                            len,
                                        );
                                        if nx <= 0 as libc::c_int {
                                            break;
                                        }
                                        cx = if ucx[_i as usize]
                                            > 0x1d244 as libc::c_int as libc::c_uint
                                        {
                                            0 as libc::c_int
                                        } else {
                                            ccc_val[ccc_val_index[ccc_index
                                                [(ucx[_i as usize] >> 8 as libc::c_int) as usize]
                                                as usize]
                                                [(ucx[_i as usize] >> 4 as libc::c_int
                                                    & 0xf as libc::c_int as libc::c_uint)
                                                    as usize]
                                                as usize]
                                                [(ucx[_i as usize]
                                                    & 0xf as libc::c_int as libc::c_uint)
                                                    as usize]
                                                as libc::c_int
                                        };
                                        if cl >= cx
                                            && cl != 228 as libc::c_int
                                            && cx != 228 as libc::c_int
                                        {
                                            break;
                                        }
                                        s = s.offset(nx as isize);
                                        len = (len as libc::c_ulong)
                                            .wrapping_sub(nx as libc::c_ulong)
                                            as size_t
                                            as size_t;
                                        cl = cx;
                                        ccx[_i as usize] = cx;
                                        _i += 1
                                    }
                                    if _i >= FDC_MAX {
                                        ret = -(1 as libc::c_int);
                                        ucx_size = FDC_MAX
                                    } else {
                                        ucx_size = _i
                                    }
                                    /*
                                     * Find a composed code in the collected code points.
                                     */
                                    i = 1 as libc::c_int;
                                    while i < ucx_size {
                                        let mut j: libc::c_int = 0;
                                        nfc = get_nfc(uc, ucx[i as usize]);
                                        if nfc == 0 as libc::c_int as libc::c_uint {
                                            i += 1
                                        } else {
                                            /*
                                             * nfc is composed of uc and ucx[i].
                                             */
                                            uc = nfc;
                                            ucptr = NULL as *const libc::c_char;
                                            /*
                                             * Remove ucx[i] by shifting
                                             * following code points.
                                             */
                                            j = i;
                                            while (j + 1 as libc::c_int) < ucx_size {
                                                ucx[j as usize] =
                                                    ucx[(j + 1 as libc::c_int) as usize];
                                                ccx[j as usize] =
                                                    ccx[(j + 1 as libc::c_int) as usize];
                                                j += 1
                                            }
                                            ucx_size -= 1;
                                            /*
                                             * Collect following code points blocked
                                             * by ucx[i] the removed code point.
                                             */
                                            if ucx_size > 0 as libc::c_int
                                                && i == ucx_size
                                                && nx > 0 as libc::c_int
                                                && cx == cl
                                            {
                                                cl = ccx[(ucx_size - 1 as libc::c_int) as usize];
                                                let mut _i_0: libc::c_int = 0;
                                                _i_0 = ucx_size;
                                                while _i_0 < FDC_MAX {
                                                    nx = parse.expect("non-null function pointer")(
                                                        &mut *ucx
                                                            .as_mut_ptr()
                                                            .offset(_i_0 as isize),
                                                        s,
                                                        len,
                                                    );
                                                    if nx <= 0 as libc::c_int {
                                                        break;
                                                    }
                                                    cx = if ucx[_i_0 as usize]
                                                        > 0x1d244 as libc::c_int as libc::c_uint
                                                    {
                                                        0 as libc::c_int
                                                    } else {
                                                        ccc_val[ccc_val_index[ccc_index[(ucx
                                                            [_i_0 as usize]
                                                            >> 8 as libc::c_int)
                                                            as usize]
                                                            as usize]
                                                            [(ucx[_i_0 as usize]
                                                                >> 4 as libc::c_int
                                                                & 0xf as libc::c_int
                                                                    as libc::c_uint)
                                                                as usize]
                                                            as usize]
                                                            [(ucx[_i_0 as usize]
                                                                & 0xf as libc::c_int
                                                                    as libc::c_uint)
                                                                as usize]
                                                            as libc::c_int
                                                    };
                                                    if cl >= cx
                                                        && cl != 228 as libc::c_int
                                                        && cx != 228 as libc::c_int
                                                    {
                                                        break;
                                                    }
                                                    s = s.offset(nx as isize);
                                                    len = (len as libc::c_ulong)
                                                        .wrapping_sub(nx as libc::c_ulong)
                                                        as size_t
                                                        as size_t;
                                                    cl = cx;
                                                    ccx[_i_0 as usize] = cx;
                                                    _i_0 += 1
                                                }
                                                if _i_0 >= FDC_MAX {
                                                    ret = -(1 as libc::c_int);
                                                    ucx_size = FDC_MAX
                                                } else {
                                                    ucx_size = _i_0
                                                }
                                            }
                                            /*
                                             * Restart finding a composed code with
                                             * the updated uc from the top of the
                                             * collected code points.
                                             */
                                            i = 0 as libc::c_int
                                        }
                                    }
                                    /*
                                     * Apparently the current code points are not
                                     * decomposed characters or already composed.
                                     */
                                    if !ucptr.is_null() {
                                        if p.offset(n as isize) > endp {
                                            (*as_0).length = p.offset_from((*as_0).s)
                                                as libc::c_long
                                                as size_t;
                                            if archive_string_ensure(
                                                as_0,
                                                (*as_0)
                                                    .buffer_length
                                                    .wrapping_add(
                                                        len.wrapping_mul(tm as libc::c_ulong),
                                                    )
                                                    .wrapping_add(ts as libc::c_ulong),
                                            )
                                            .is_null()
                                            {
                                                return -(1 as libc::c_int);
                                            }
                                            p = (*as_0).s.offset((*as_0).length as isize);
                                            endp = (*as_0)
                                                .s
                                                .offset((*as_0).buffer_length as isize)
                                                .offset(-(ts as isize))
                                        }
                                        let mut current_block_297: u64;
                                        match n {
                                            4 => {
                                                let fresh45 = ucptr;
                                                ucptr = ucptr.offset(1);
                                                let fresh46 = p;
                                                p = p.offset(1);
                                                *fresh46 = *fresh45;
                                                current_block_297 = 11798945521115977539;
                                            }
                                            3 => {
                                                current_block_297 = 11798945521115977539;
                                            }
                                            2 => {
                                                current_block_297 = 17787851924190067911;
                                            }
                                            1 => {
                                                current_block_297 = 2842964501748019089;
                                            }
                                            _ => {
                                                current_block_297 = 6632235551759984864;
                                            }
                                        }
                                        match current_block_297 {
                                            11798945521115977539 => {
                                                let fresh47 = ucptr;
                                                ucptr = ucptr.offset(1);
                                                let fresh48 = p;
                                                p = p.offset(1);
                                                *fresh48 = *fresh47;
                                                current_block_297 = 17787851924190067911;
                                            }
                                            _ => {}
                                        }
                                        match current_block_297 {
                                            17787851924190067911 => {
                                                let fresh49 = ucptr;
                                                ucptr = ucptr.offset(1);
                                                let fresh50 = p;
                                                p = p.offset(1);
                                                *fresh50 = *fresh49;
                                                current_block_297 = 2842964501748019089;
                                            }
                                            _ => {}
                                        }
                                        match current_block_297 {
                                            2842964501748019089 => {
                                                let fresh51 = p;
                                                p = p.offset(1);
                                                *fresh51 = *ucptr
                                            }
                                            _ => {}
                                        }
                                        ucptr = NULL as *const libc::c_char
                                    } else {
                                        loop {
                                            w = unparse.expect("non-null function pointer")(
                                                p,
                                                endp.offset_from(p) as libc::c_long
                                                    as size_t,
                                                uc,
                                            );
                                            if !(w == 0 as libc::c_int as libc::c_ulong) {
                                                break;
                                            }
                                            (*as_0).length = p.offset_from((*as_0).s)
                                                as libc::c_long
                                                as size_t;
                                            if archive_string_ensure(
                                                as_0,
                                                (*as_0)
                                                    .buffer_length
                                                    .wrapping_add(
                                                        len.wrapping_mul(tm as libc::c_ulong),
                                                    )
                                                    .wrapping_add(ts as libc::c_ulong),
                                            )
                                            .is_null()
                                            {
                                                return -(1 as libc::c_int);
                                            }
                                            p = (*as_0).s.offset((*as_0).length as isize);
                                            endp = (*as_0)
                                                .s
                                                .offset((*as_0).buffer_length as isize)
                                                .offset(-(ts as isize))
                                        }
                                        p = p.offset(w as isize)
                                    }
                                    i = 0 as libc::c_int;
                                    while i < ucx_size {
                                        loop {
                                            w = unparse.expect("non-null function pointer")(
                                                p,
                                                endp.offset_from(p) as libc::c_long
                                                    as size_t,
                                                ucx[i as usize],
                                            );
                                            if !(w == 0 as libc::c_int as libc::c_ulong) {
                                                break;
                                            }
                                            (*as_0).length = p.offset_from((*as_0).s)
                                                as libc::c_long
                                                as size_t;
                                            if archive_string_ensure(
                                                as_0,
                                                (*as_0)
                                                    .buffer_length
                                                    .wrapping_add(
                                                        len.wrapping_mul(tm as libc::c_ulong),
                                                    )
                                                    .wrapping_add(ts as libc::c_ulong),
                                            )
                                            .is_null()
                                            {
                                                return -(1 as libc::c_int);
                                            }
                                            p = (*as_0).s.offset((*as_0).length as isize);
                                            endp = (*as_0)
                                                .s
                                                .offset((*as_0).buffer_length as isize)
                                                .offset(-(ts as isize))
                                        }
                                        p = p.offset(w as isize);
                                        i += 1
                                    }
                                    /*
                                     * Flush out remaining canonical combining characters.
                                     */
                                    if nx > 0 as libc::c_int
                                        && cx == cl
                                        && len > 0 as libc::c_int as libc::c_ulong
                                    {
                                        loop {
                                            nx = parse.expect("non-null function pointer")(
                                                &mut *ucx
                                                    .as_mut_ptr()
                                                    .offset(0 as libc::c_int as isize),
                                                s,
                                                len,
                                            );
                                            if !(nx > 0 as libc::c_int) {
                                                break;
                                            }
                                            cx = if ucx[0 as libc::c_int as usize]
                                                > 0x1d244 as libc::c_int as libc::c_uint
                                            {
                                                0 as libc::c_int
                                            } else {
                                                ccc_val[ccc_val_index[ccc_index[(ucx
                                                    [0 as libc::c_int as usize]
                                                    >> 8 as libc::c_int)
                                                    as usize]
                                                    as usize]
                                                    [(ucx[0 as libc::c_int as usize]
                                                        >> 4 as libc::c_int
                                                        & 0xf as libc::c_int as libc::c_uint)
                                                        as usize]
                                                    as usize]
                                                    [(ucx[0 as libc::c_int as usize]
                                                        & 0xf as libc::c_int as libc::c_uint)
                                                        as usize]
                                                    as libc::c_int
                                            };
                                            if cl > cx {
                                                break;
                                            }
                                            s = s.offset(nx as isize);
                                            len = (len as libc::c_ulong)
                                                .wrapping_sub(nx as libc::c_ulong)
                                                as size_t
                                                as size_t;
                                            cl = cx;
                                            loop {
                                                w = unparse.expect("non-null function pointer")(
                                                    p,
                                                    endp.offset_from(p) as libc::c_long
                                                        as size_t,
                                                    ucx[0 as libc::c_int as usize],
                                                );
                                                if !(w == 0 as libc::c_int as libc::c_ulong) {
                                                    break;
                                                }
                                                (*as_0).length = p.offset_from((*as_0).s)
                                                    as libc::c_long
                                                    as size_t;
                                                if archive_string_ensure(
                                                    as_0,
                                                    (*as_0)
                                                        .buffer_length
                                                        .wrapping_add(
                                                            len.wrapping_mul(tm as libc::c_ulong),
                                                        )
                                                        .wrapping_add(ts as libc::c_ulong),
                                                )
                                                .is_null()
                                                {
                                                    return -(1 as libc::c_int);
                                                }
                                                p = (*as_0).s.offset((*as_0).length as isize);
                                                endp = (*as_0)
                                                    .s
                                                    .offset((*as_0).buffer_length as isize)
                                                    .offset(-(ts as isize))
                                            }
                                            p = p.offset(w as isize)
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            if n2 < 0 as libc::c_int {
                if !ucptr.is_null() {
                    if p.offset(n as isize) > endp {
                        (*as_0).length =
                            p.offset_from((*as_0).s) as libc::c_long as size_t;
                        if archive_string_ensure(
                            as_0,
                            (*as_0)
                                .buffer_length
                                .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                .wrapping_add(ts as libc::c_ulong),
                        )
                        .is_null()
                        {
                            return -(1 as libc::c_int);
                        }
                        p = (*as_0).s.offset((*as_0).length as isize);
                        endp = (*as_0)
                            .s
                            .offset((*as_0).buffer_length as isize)
                            .offset(-(ts as isize))
                    }
                    let mut current_block_362: u64;
                    match n {
                        4 => {
                            let fresh52 = ucptr;
                            ucptr = ucptr.offset(1);
                            let fresh53 = p;
                            p = p.offset(1);
                            *fresh53 = *fresh52;
                            current_block_362 = 16868400320782070032;
                        }
                        3 => {
                            current_block_362 = 16868400320782070032;
                        }
                        2 => {
                            current_block_362 = 16997017939309801789;
                        }
                        1 => {
                            current_block_362 = 15139127810538190319;
                        }
                        _ => {
                            current_block_362 = 16953886395775657100;
                        }
                    }
                    match current_block_362 {
                        16868400320782070032 => {
                            let fresh54 = ucptr;
                            ucptr = ucptr.offset(1);
                            let fresh55 = p;
                            p = p.offset(1);
                            *fresh55 = *fresh54;
                            current_block_362 = 16997017939309801789;
                        }
                        _ => {}
                    }
                    match current_block_362 {
                        16997017939309801789 => {
                            let fresh56 = ucptr;
                            ucptr = ucptr.offset(1);
                            let fresh57 = p;
                            p = p.offset(1);
                            *fresh57 = *fresh56;
                            current_block_362 = 15139127810538190319;
                        }
                        _ => {}
                    }
                    match current_block_362 {
                        15139127810538190319 => {
                            let fresh58 = p;
                            p = p.offset(1);
                            *fresh58 = *ucptr
                        }
                        _ => {}
                    }
                    ucptr = NULL as *const libc::c_char
                } else {
                    loop {
                        w = unparse.expect("non-null function pointer")(
                            p,
                            endp.offset_from(p) as libc::c_long as size_t,
                            uc,
                        );
                        if !(w == 0 as libc::c_int as libc::c_ulong) {
                            break;
                        }
                        (*as_0).length =
                            p.offset_from((*as_0).s) as libc::c_long as size_t;
                        if archive_string_ensure(
                            as_0,
                            (*as_0)
                                .buffer_length
                                .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                .wrapping_add(ts as libc::c_ulong),
                        )
                        .is_null()
                        {
                            return -(1 as libc::c_int);
                        }
                        p = (*as_0).s.offset((*as_0).length as isize);
                        endp = (*as_0)
                            .s
                            .offset((*as_0).buffer_length as isize)
                            .offset(-(ts as isize))
                    }
                    p = p.offset(w as isize)
                }
                /* Use a replaced unicode character. */
                loop {
                    w = unparse.expect("non-null function pointer")(
                        p,
                        endp.offset_from(p) as libc::c_long as size_t,
                        uc2,
                    );
                    if !(w == 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                    (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
                    if archive_string_ensure(
                        as_0,
                        (*as_0)
                            .buffer_length
                            .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                            .wrapping_add(ts as libc::c_ulong),
                    )
                    .is_null()
                    {
                        return -(1 as libc::c_int);
                    }
                    p = (*as_0).s.offset((*as_0).length as isize);
                    endp = (*as_0)
                        .s
                        .offset((*as_0).buffer_length as isize)
                        .offset(-(ts as isize))
                }
                p = p.offset(w as isize);
                s = s.offset((n2 * -(1 as libc::c_int)) as isize);
                len = (len as libc::c_ulong)
                    .wrapping_sub((n2 * -(1 as libc::c_int)) as libc::c_ulong)
                    as size_t as size_t;
                ret = -(1 as libc::c_int)
            } else {
                if !(n2 == 0 as libc::c_int) {
                    continue;
                }
                if !ucptr.is_null() {
                    if p.offset(n as isize) > endp {
                        (*as_0).length =
                            p.offset_from((*as_0).s) as libc::c_long as size_t;
                        if archive_string_ensure(
                            as_0,
                            (*as_0)
                                .buffer_length
                                .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                .wrapping_add(ts as libc::c_ulong),
                        )
                        .is_null()
                        {
                            return -(1 as libc::c_int);
                        }
                        p = (*as_0).s.offset((*as_0).length as isize);
                        endp = (*as_0)
                            .s
                            .offset((*as_0).buffer_length as isize)
                            .offset(-(ts as isize))
                    }
                    let mut current_block_408: u64;
                    match n {
                        4 => {
                            let fresh59 = ucptr;
                            ucptr = ucptr.offset(1);
                            let fresh60 = p;
                            p = p.offset(1);
                            *fresh60 = *fresh59;
                            current_block_408 = 1197512327513536685;
                        }
                        3 => {
                            current_block_408 = 1197512327513536685;
                        }
                        2 => {
                            current_block_408 = 303600079375861751;
                        }
                        1 => {
                            current_block_408 = 2892376829882225545;
                        }
                        _ => {
                            current_block_408 = 4637513635194184052;
                        }
                    }
                    match current_block_408 {
                        1197512327513536685 => {
                            let fresh61 = ucptr;
                            ucptr = ucptr.offset(1);
                            let fresh62 = p;
                            p = p.offset(1);
                            *fresh62 = *fresh61;
                            current_block_408 = 303600079375861751;
                        }
                        _ => {}
                    }
                    match current_block_408 {
                        303600079375861751 => {
                            let fresh63 = ucptr;
                            ucptr = ucptr.offset(1);
                            let fresh64 = p;
                            p = p.offset(1);
                            *fresh64 = *fresh63;
                            current_block_408 = 2892376829882225545;
                        }
                        _ => {}
                    }
                    match current_block_408 {
                        2892376829882225545 => {
                            let fresh65 = p;
                            p = p.offset(1);
                            *fresh65 = *ucptr
                        }
                        _ => {}
                    }
                    ucptr = NULL as *const libc::c_char
                } else {
                    loop {
                        w = unparse.expect("non-null function pointer")(
                            p,
                            endp.offset_from(p) as libc::c_long as size_t,
                            uc,
                        );
                        if !(w == 0 as libc::c_int as libc::c_ulong) {
                            break;
                        }
                        (*as_0).length =
                            p.offset_from((*as_0).s) as libc::c_long as size_t;
                        if archive_string_ensure(
                            as_0,
                            (*as_0)
                                .buffer_length
                                .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                .wrapping_add(ts as libc::c_ulong),
                        )
                        .is_null()
                        {
                            return -(1 as libc::c_int);
                        }
                        p = (*as_0).s.offset((*as_0).length as isize);
                        endp = (*as_0)
                            .s
                            .offset((*as_0).buffer_length as isize)
                            .offset(-(ts as isize))
                    }
                    p = p.offset(w as isize)
                }
                break;
            }
        }
    }
    (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = '\u{0}' as i32 as libc::c_char;
    if ts == 2 as libc::c_int {
        *(*as_0).s.offset(
            (*as_0)
                .length
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\u{0}' as i32 as libc::c_char
    }
    return ret;
}
unsafe extern "C" fn get_nfd(
    mut cp1: *mut uint32_t,
    mut cp2: *mut uint32_t,
    mut uc: uint32_t,
) -> libc::c_int {
    let mut t: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    /*
     * These are not converted to NFD on Mac OS.
     */
    if uc >= 0x2000 as libc::c_int as libc::c_uint && uc <= 0x2fff as libc::c_int as libc::c_uint
        || uc >= 0xf900 as libc::c_int as libc::c_uint
            && uc <= 0xfaff as libc::c_int as libc::c_uint
        || uc >= 0x2f800 as libc::c_int as libc::c_uint
            && uc <= 0x2faff as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    /*
     * Those code points are not converted to NFD on Mac OS.
     * I do not know the reason because it is undocumented.
     *   NFC        NFD
     *   1109A  ==> 11099 110BA
     *   1109C  ==> 1109B 110BA
     *   110AB  ==> 110A5 110BA
     */
    if uc == 0x1109a as libc::c_int as libc::c_uint
        || uc == 0x1109c as libc::c_int as libc::c_uint
        || uc == 0x110ab as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    t = 0 as libc::c_int;
    b = (::std::mem::size_of::<[unicode_decomposition_table; 931]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<unicode_decomposition_table>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    while b >= t {
        let mut m: libc::c_int = (t + b) / 2 as libc::c_int;
        if u_decomposition_table[m as usize].nfc < uc {
            t = m + 1 as libc::c_int
        } else if u_decomposition_table[m as usize].nfc > uc {
            b = m - 1 as libc::c_int
        } else {
            *cp1 = u_decomposition_table[m as usize].cp1;
            *cp2 = u_decomposition_table[m as usize].cp2;
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
/*
 * Normalize UTF-8 characters to Form D and copy the result.
 */
unsafe extern "C" fn archive_string_normalize_D(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut s: *const libc::c_char = _p as *const libc::c_char; /* text size. */
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uc: uint32_t = 0;
    let mut uc2: uint32_t = 0;
    let mut w: size_t = 0;
    let mut always_replace: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut spair: libc::c_int = 0;
    let mut ts: libc::c_int = 0;
    let mut tm: libc::c_int = 0;
    let mut parse: Option<
        unsafe extern "C" fn(_: *mut uint32_t, _: *const libc::c_char, _: size_t) -> libc::c_int,
    > = None;
    let mut unparse: Option<
        unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
    > = None;
    always_replace = 1 as libc::c_int;
    ts = 1 as libc::c_int;
    if (*sc).flag & SCONV_TO_UTF16BE != 0 {
        unparse = Some(
            unicode_to_utf16be
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 2 as libc::c_int;
        if (*sc).flag & SCONV_FROM_UTF16BE != 0 {
            always_replace = 0 as libc::c_int
        }
    } else if (*sc).flag & SCONV_TO_UTF16LE != 0 {
        unparse = Some(
            unicode_to_utf16le
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        ts = 2 as libc::c_int;
        if (*sc).flag & SCONV_FROM_UTF16LE != 0 {
            always_replace = 0 as libc::c_int
        }
    } else if (*sc).flag & SCONV_TO_UTF8 != 0 {
        unparse = Some(
            unicode_to_utf8
                as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
        );
        if (*sc).flag & SCONV_FROM_UTF8 != 0 {
            always_replace = 0 as libc::c_int
        }
    } else {
        /*
         * This case is going to be converted to another
         * character-set through iconv.
         */
        always_replace = 0 as libc::c_int;
        if (*sc).flag & SCONV_FROM_UTF16BE != 0 {
            unparse = Some(
                unicode_to_utf16be
                    as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
            );
            ts = 2 as libc::c_int
        } else if (*sc).flag & SCONV_FROM_UTF16LE != 0 {
            unparse = Some(
                unicode_to_utf16le
                    as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
            );
            ts = 2 as libc::c_int
        } else {
            unparse = Some(
                unicode_to_utf8
                    as unsafe extern "C" fn(_: *mut libc::c_char, _: size_t, _: uint32_t) -> size_t,
            )
        }
    }
    if (*sc).flag & SCONV_FROM_UTF16BE != 0 {
        parse = Some(
            utf16be_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = 1 as libc::c_int;
        spair = 4 as libc::c_int
    /* surrogate pair size in UTF-16. */
    } else if (*sc).flag & SCONV_FROM_UTF16LE != 0 {
        parse = Some(
            utf16le_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = 1 as libc::c_int;
        spair = 4 as libc::c_int
    /* surrogate pair size in UTF-16. */
    } else {
        parse = Some(
            cesu8_to_unicode
                as unsafe extern "C" fn(
                    _: *mut uint32_t,
                    _: *const libc::c_char,
                    _: size_t,
                ) -> libc::c_int,
        );
        tm = ts;
        spair = 6 as libc::c_int
        /* surrogate pair size in UTF-8. */
    }
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
            .wrapping_add(ts as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    p = (*as_0).s.offset((*as_0).length as isize);
    endp = (*as_0)
        .s
        .offset((*as_0).buffer_length as isize)
        .offset(-(ts as isize));
    's_239: loop {
        n = parse.expect("non-null function pointer")(&mut uc, s, len);
        if !(n != 0 as libc::c_int) {
            break;
        }
        let mut ucptr: *const libc::c_char = 0 as *const libc::c_char;
        let mut cp1: uint32_t = 0;
        let mut cp2: uint32_t = 0;
        let mut SIndex: libc::c_int = 0;
        let mut fdc: [C2RustUnnamed_1; 10] = [C2RustUnnamed_1 { uc: 0, ccc: 0 }; 10];
        let mut fdi: libc::c_int = 0;
        let mut fdj: libc::c_int = 0;
        let mut ccc: libc::c_int = 0;
        loop {
            if n < 0 as libc::c_int {
                /* Use a replaced unicode character. */
                loop {
                    w = unparse.expect("non-null function pointer")(
                        p,
                        endp.offset_from(p) as libc::c_long as size_t,
                        uc,
                    );
                    if !(w == 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                    (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
                    if archive_string_ensure(
                        as_0,
                        (*as_0)
                            .buffer_length
                            .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                            .wrapping_add(ts as libc::c_ulong),
                    )
                    .is_null()
                    {
                        return -(1 as libc::c_int);
                    }
                    p = (*as_0).s.offset((*as_0).length as isize);
                    endp = (*as_0)
                        .s
                        .offset((*as_0).buffer_length as isize)
                        .offset(-(ts as isize))
                }
                p = p.offset(w as isize);
                s = s.offset((n * -(1 as libc::c_int)) as isize);
                len = (len as libc::c_ulong)
                    .wrapping_sub((n * -(1 as libc::c_int)) as libc::c_ulong)
                    as size_t as size_t;
                ret = -(1 as libc::c_int);
                break;
            } else {
                if n == spair || always_replace != 0 {
                    /* uc is converted from a surrogate pair.
                     * this should be treated as a changed code. */
                    ucptr = NULL as *const libc::c_char
                } else {
                    ucptr = s
                }
                s = s.offset(n as isize);
                len = (len as libc::c_ulong).wrapping_sub(n as libc::c_ulong) as size_t as size_t;
                /* Hangul Decomposition. */
                SIndex = uc.wrapping_sub(HC_SBASE as libc::c_uint) as libc::c_int;
                if SIndex >= 0 as libc::c_int && SIndex < HC_SCOUNT {
                    let mut L: libc::c_int = HC_LBASE + SIndex / HC_NCOUNT;
                    let mut V: libc::c_int = HC_VBASE + SIndex % HC_NCOUNT / HC_TCOUNT;
                    let mut T: libc::c_int = HC_TBASE + SIndex % HC_TCOUNT;
                    uc = L as uint32_t;
                    ucptr = NULL as *const libc::c_char;
                    if !ucptr.is_null() {
                        if p.offset(n as isize) > endp {
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        let mut current_block_84: u64;
                        match n {
                            4 => {
                                let fresh66 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh67 = p;
                                p = p.offset(1);
                                *fresh67 = *fresh66;
                                current_block_84 = 17471177608285378829;
                            }
                            3 => {
                                current_block_84 = 17471177608285378829;
                            }
                            2 => {
                                current_block_84 = 4328682382543395768;
                            }
                            1 => {
                                current_block_84 = 12680612309660809340;
                            }
                            _ => {
                                current_block_84 = 16313536926714486912;
                            }
                        }
                        match current_block_84 {
                            17471177608285378829 => {
                                let fresh68 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh69 = p;
                                p = p.offset(1);
                                *fresh69 = *fresh68;
                                current_block_84 = 4328682382543395768;
                            }
                            _ => {}
                        }
                        match current_block_84 {
                            4328682382543395768 => {
                                let fresh70 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh71 = p;
                                p = p.offset(1);
                                *fresh71 = *fresh70;
                                current_block_84 = 12680612309660809340;
                            }
                            _ => {}
                        }
                        match current_block_84 {
                            12680612309660809340 => {
                                let fresh72 = p;
                                p = p.offset(1);
                                *fresh72 = *ucptr
                            }
                            _ => {}
                        }
                        ucptr = NULL as *const libc::c_char
                    } else {
                        loop {
                            w = unparse.expect("non-null function pointer")(
                                p,
                                endp.offset_from(p) as libc::c_long as size_t,
                                uc,
                            );
                            if !(w == 0 as libc::c_int as libc::c_ulong) {
                                break;
                            }
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        p = p.offset(w as isize)
                    }
                    uc = V as uint32_t;
                    ucptr = NULL as *const libc::c_char;
                    if !ucptr.is_null() {
                        if p.offset(n as isize) > endp {
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        let mut current_block_119: u64;
                        match n {
                            4 => {
                                let fresh73 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh74 = p;
                                p = p.offset(1);
                                *fresh74 = *fresh73;
                                current_block_119 = 5676425471993221197;
                            }
                            3 => {
                                current_block_119 = 5676425471993221197;
                            }
                            2 => {
                                current_block_119 = 7260900127117304468;
                            }
                            1 => {
                                current_block_119 = 4466282900189035683;
                            }
                            _ => {
                                current_block_119 = 17212496701767205014;
                            }
                        }
                        match current_block_119 {
                            5676425471993221197 => {
                                let fresh75 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh76 = p;
                                p = p.offset(1);
                                *fresh76 = *fresh75;
                                current_block_119 = 7260900127117304468;
                            }
                            _ => {}
                        }
                        match current_block_119 {
                            7260900127117304468 => {
                                let fresh77 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh78 = p;
                                p = p.offset(1);
                                *fresh78 = *fresh77;
                                current_block_119 = 4466282900189035683;
                            }
                            _ => {}
                        }
                        match current_block_119 {
                            4466282900189035683 => {
                                let fresh79 = p;
                                p = p.offset(1);
                                *fresh79 = *ucptr
                            }
                            _ => {}
                        }
                        ucptr = NULL as *const libc::c_char
                    } else {
                        loop {
                            w = unparse.expect("non-null function pointer")(
                                p,
                                endp.offset_from(p) as libc::c_long as size_t,
                                uc,
                            );
                            if !(w == 0 as libc::c_int as libc::c_ulong) {
                                break;
                            }
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        p = p.offset(w as isize)
                    }
                    if T != HC_TBASE {
                        uc = T as uint32_t;
                        ucptr = NULL as *const libc::c_char;
                        if !ucptr.is_null() {
                            if p.offset(n as isize) > endp {
                                (*as_0).length =
                                    p.offset_from((*as_0).s) as libc::c_long as size_t;
                                if archive_string_ensure(
                                    as_0,
                                    (*as_0)
                                        .buffer_length
                                        .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                        .wrapping_add(ts as libc::c_ulong),
                                )
                                .is_null()
                                {
                                    return -(1 as libc::c_int);
                                }
                                p = (*as_0).s.offset((*as_0).length as isize);
                                endp = (*as_0)
                                    .s
                                    .offset((*as_0).buffer_length as isize)
                                    .offset(-(ts as isize))
                            }
                            let mut current_block_154: u64;
                            match n {
                                4 => {
                                    let fresh80 = ucptr;
                                    ucptr = ucptr.offset(1);
                                    let fresh81 = p;
                                    p = p.offset(1);
                                    *fresh81 = *fresh80;
                                    current_block_154 = 15277245382882665200;
                                }
                                3 => {
                                    current_block_154 = 15277245382882665200;
                                }
                                2 => {
                                    current_block_154 = 16948214300005970184;
                                }
                                1 => {
                                    current_block_154 = 14602048341576980508;
                                }
                                _ => {
                                    current_block_154 = 4183419379601546972;
                                }
                            }
                            match current_block_154 {
                                15277245382882665200 => {
                                    let fresh82 = ucptr;
                                    ucptr = ucptr.offset(1);
                                    let fresh83 = p;
                                    p = p.offset(1);
                                    *fresh83 = *fresh82;
                                    current_block_154 = 16948214300005970184;
                                }
                                _ => {}
                            }
                            match current_block_154 {
                                16948214300005970184 => {
                                    let fresh84 = ucptr;
                                    ucptr = ucptr.offset(1);
                                    let fresh85 = p;
                                    p = p.offset(1);
                                    *fresh85 = *fresh84;
                                    current_block_154 = 14602048341576980508;
                                }
                                _ => {}
                            }
                            match current_block_154 {
                                14602048341576980508 => {
                                    let fresh86 = p;
                                    p = p.offset(1);
                                    *fresh86 = *ucptr
                                }
                                _ => {}
                            }
                            ucptr = NULL as *const libc::c_char
                        } else {
                            loop {
                                w = unparse.expect("non-null function pointer")(
                                    p,
                                    endp.offset_from(p) as libc::c_long as size_t,
                                    uc,
                                );
                                if !(w == 0 as libc::c_int as libc::c_ulong) {
                                    break;
                                }
                                (*as_0).length =
                                    p.offset_from((*as_0).s) as libc::c_long as size_t;
                                if archive_string_ensure(
                                    as_0,
                                    (*as_0)
                                        .buffer_length
                                        .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                        .wrapping_add(ts as libc::c_ulong),
                                )
                                .is_null()
                                {
                                    return -(1 as libc::c_int);
                                }
                                p = (*as_0).s.offset((*as_0).length as isize);
                                endp = (*as_0)
                                    .s
                                    .offset((*as_0).buffer_length as isize)
                                    .offset(-(ts as isize))
                            }
                            p = p.offset(w as isize)
                        }
                    }
                    break;
                } else if uc >> 8 as libc::c_int <= 0x1d2 as libc::c_int as libc::c_uint
                    && u_decomposable_blocks[(uc >> 8 as libc::c_int) as usize] as libc::c_int != 0
                    && (if uc > 0x1d244 as libc::c_int as libc::c_uint {
                        0 as libc::c_int
                    } else {
                        ccc_val[ccc_val_index[ccc_index[(uc >> 8 as libc::c_int) as usize] as usize]
                            [(uc >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as usize]
                            as usize]
                            [(uc & 0xf as libc::c_int as libc::c_uint) as usize]
                            as libc::c_int
                    }) != 0 as libc::c_int
                {
                    if !ucptr.is_null() {
                        if p.offset(n as isize) > endp {
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        let mut current_block_187: u64;
                        match n {
                            4 => {
                                let fresh87 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh88 = p;
                                p = p.offset(1);
                                *fresh88 = *fresh87;
                                current_block_187 = 17332659574749544158;
                            }
                            3 => {
                                current_block_187 = 17332659574749544158;
                            }
                            2 => {
                                current_block_187 = 13006060414296805144;
                            }
                            1 => {
                                current_block_187 = 13213222958757291311;
                            }
                            _ => {
                                current_block_187 = 11577926782275222206;
                            }
                        }
                        match current_block_187 {
                            17332659574749544158 => {
                                let fresh89 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh90 = p;
                                p = p.offset(1);
                                *fresh90 = *fresh89;
                                current_block_187 = 13006060414296805144;
                            }
                            _ => {}
                        }
                        match current_block_187 {
                            13006060414296805144 => {
                                let fresh91 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh92 = p;
                                p = p.offset(1);
                                *fresh92 = *fresh91;
                                current_block_187 = 13213222958757291311;
                            }
                            _ => {}
                        }
                        match current_block_187 {
                            13213222958757291311 => {
                                let fresh93 = p;
                                p = p.offset(1);
                                *fresh93 = *ucptr
                            }
                            _ => {}
                        }
                        ucptr = NULL as *const libc::c_char
                    } else {
                        loop {
                            w = unparse.expect("non-null function pointer")(
                                p,
                                endp.offset_from(p) as libc::c_long as size_t,
                                uc,
                            );
                            if !(w == 0 as libc::c_int as libc::c_ulong) {
                                break;
                            }
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        p = p.offset(w as isize)
                    }
                    break;
                } else {
                    fdi = 0 as libc::c_int;
                    while get_nfd(&mut cp1, &mut cp2, uc) != 0 && fdi < FDC_MAX {
                        let mut k: libc::c_int = 0;
                        k = fdi;
                        while k > 0 as libc::c_int {
                            fdc[k as usize] = fdc[(k - 1 as libc::c_int) as usize];
                            k -= 1
                        }
                        fdc[0 as libc::c_int as usize].ccc =
                            if cp2 > 0x1d244 as libc::c_int as libc::c_uint {
                                0 as libc::c_int
                            } else {
                                ccc_val[ccc_val_index
                                    [ccc_index[(cp2 >> 8 as libc::c_int) as usize] as usize]
                                    [(cp2 >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint)
                                        as usize] as usize]
                                    [(cp2 & 0xf as libc::c_int as libc::c_uint) as usize]
                                    as libc::c_int
                            };
                        fdc[0 as libc::c_int as usize].uc = cp2;
                        fdi += 1;
                        uc = cp1;
                        ucptr = NULL as *const libc::c_char
                    }
                    loop
                    /* Read following code points. */
                    {
                        n2 = parse.expect("non-null function pointer")(&mut uc2, s, len);
                        if !(n2 > 0 as libc::c_int
                            && {
                                ccc = (if uc2 > 0x1d244 as libc::c_int as libc::c_uint {
                                    0 as libc::c_int
                                } else {
                                    ccc_val[ccc_val_index
                                        [ccc_index[(uc2 >> 8 as libc::c_int) as usize] as usize]
                                        [(uc2 >> 4 as libc::c_int
                                            & 0xf as libc::c_int as libc::c_uint)
                                            as usize]
                                        as usize]
                                        [(uc2 & 0xf as libc::c_int as libc::c_uint) as usize]
                                        as libc::c_int
                                });
                                (ccc) != 0 as libc::c_int
                            }
                            && fdi < FDC_MAX)
                        {
                            break;
                        }
                        let mut j: libc::c_int = 0;
                        let mut k_0: libc::c_int = 0;
                        s = s.offset(n2 as isize);
                        len = (len as libc::c_ulong).wrapping_sub(n2 as libc::c_ulong) as size_t
                            as size_t;
                        j = 0 as libc::c_int;
                        while j < fdi {
                            if fdc[j as usize].ccc > ccc {
                                break;
                            }
                            j += 1
                        }
                        if j < fdi {
                            k_0 = fdi;
                            while k_0 > j {
                                fdc[k_0 as usize] = fdc[(k_0 - 1 as libc::c_int) as usize];
                                k_0 -= 1
                            }
                            fdc[j as usize].ccc = ccc;
                            fdc[j as usize].uc = uc2
                        } else {
                            fdc[fdi as usize].ccc = ccc;
                            fdc[fdi as usize].uc = uc2
                        }
                        fdi += 1
                    }
                    if !ucptr.is_null() {
                        if p.offset(n as isize) > endp {
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        let mut current_block_248: u64;
                        match n {
                            4 => {
                                let fresh94 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh95 = p;
                                p = p.offset(1);
                                *fresh95 = *fresh94;
                                current_block_248 = 5207448745478133187;
                            }
                            3 => {
                                current_block_248 = 5207448745478133187;
                            }
                            2 => {
                                current_block_248 = 1425033589847473459;
                            }
                            1 => {
                                current_block_248 = 5363848836628713517;
                            }
                            _ => {
                                current_block_248 = 5564518856185825108;
                            }
                        }
                        match current_block_248 {
                            5207448745478133187 => {
                                let fresh96 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh97 = p;
                                p = p.offset(1);
                                *fresh97 = *fresh96;
                                current_block_248 = 1425033589847473459;
                            }
                            _ => {}
                        }
                        match current_block_248 {
                            1425033589847473459 => {
                                let fresh98 = ucptr;
                                ucptr = ucptr.offset(1);
                                let fresh99 = p;
                                p = p.offset(1);
                                *fresh99 = *fresh98;
                                current_block_248 = 5363848836628713517;
                            }
                            _ => {}
                        }
                        match current_block_248 {
                            5363848836628713517 => {
                                let fresh100 = p;
                                p = p.offset(1);
                                *fresh100 = *ucptr
                            }
                            _ => {}
                        }
                        ucptr = NULL as *const libc::c_char
                    } else {
                        loop {
                            w = unparse.expect("non-null function pointer")(
                                p,
                                endp.offset_from(p) as libc::c_long as size_t,
                                uc,
                            );
                            if !(w == 0 as libc::c_int as libc::c_ulong) {
                                break;
                            }
                            (*as_0).length =
                                p.offset_from((*as_0).s) as libc::c_long as size_t;
                            if archive_string_ensure(
                                as_0,
                                (*as_0)
                                    .buffer_length
                                    .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                    .wrapping_add(ts as libc::c_ulong),
                            )
                            .is_null()
                            {
                                return -(1 as libc::c_int);
                            }
                            p = (*as_0).s.offset((*as_0).length as isize);
                            endp = (*as_0)
                                .s
                                .offset((*as_0).buffer_length as isize)
                                .offset(-(ts as isize))
                        }
                        p = p.offset(w as isize)
                    }
                    fdj = 0 as libc::c_int;
                    while fdj < fdi {
                        uc = fdc[fdj as usize].uc;
                        ucptr = NULL as *const libc::c_char;
                        if !ucptr.is_null() {
                            if p.offset(n as isize) > endp {
                                (*as_0).length =
                                    p.offset_from((*as_0).s) as libc::c_long as size_t;
                                if archive_string_ensure(
                                    as_0,
                                    (*as_0)
                                        .buffer_length
                                        .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                        .wrapping_add(ts as libc::c_ulong),
                                )
                                .is_null()
                                {
                                    return -(1 as libc::c_int);
                                }
                                p = (*as_0).s.offset((*as_0).length as isize);
                                endp = (*as_0)
                                    .s
                                    .offset((*as_0).buffer_length as isize)
                                    .offset(-(ts as isize))
                            }
                            let mut current_block_284: u64;
                            match n {
                                4 => {
                                    let fresh101 = ucptr;
                                    ucptr = ucptr.offset(1);
                                    let fresh102 = p;
                                    p = p.offset(1);
                                    *fresh102 = *fresh101;
                                    current_block_284 = 17778196055652068863;
                                }
                                3 => {
                                    current_block_284 = 17778196055652068863;
                                }
                                2 => {
                                    current_block_284 = 9711374677867188652;
                                }
                                1 => {
                                    current_block_284 = 14725529575698383794;
                                }
                                _ => {
                                    current_block_284 = 16070719095729554596;
                                }
                            }
                            match current_block_284 {
                                17778196055652068863 => {
                                    let fresh103 = ucptr;
                                    ucptr = ucptr.offset(1);
                                    let fresh104 = p;
                                    p = p.offset(1);
                                    *fresh104 = *fresh103;
                                    current_block_284 = 9711374677867188652;
                                }
                                _ => {}
                            }
                            match current_block_284 {
                                9711374677867188652 => {
                                    let fresh105 = ucptr;
                                    ucptr = ucptr.offset(1);
                                    let fresh106 = p;
                                    p = p.offset(1);
                                    *fresh106 = *fresh105;
                                    current_block_284 = 14725529575698383794;
                                }
                                _ => {}
                            }
                            match current_block_284 {
                                14725529575698383794 => {
                                    let fresh107 = p;
                                    p = p.offset(1);
                                    *fresh107 = *ucptr
                                }
                                _ => {}
                            }
                            ucptr = NULL as *const libc::c_char
                        } else {
                            loop {
                                w = unparse.expect("non-null function pointer")(
                                    p,
                                    endp.offset_from(p) as libc::c_long as size_t,
                                    uc,
                                );
                                if !(w == 0 as libc::c_int as libc::c_ulong) {
                                    break;
                                }
                                (*as_0).length =
                                    p.offset_from((*as_0).s) as libc::c_long as size_t;
                                if archive_string_ensure(
                                    as_0,
                                    (*as_0)
                                        .buffer_length
                                        .wrapping_add(len.wrapping_mul(tm as libc::c_ulong))
                                        .wrapping_add(ts as libc::c_ulong),
                                )
                                .is_null()
                                {
                                    return -(1 as libc::c_int);
                                }
                                p = (*as_0).s.offset((*as_0).length as isize);
                                endp = (*as_0)
                                    .s
                                    .offset((*as_0).buffer_length as isize)
                                    .offset(-(ts as isize))
                            }
                            p = p.offset(w as isize)
                        }
                        fdj += 1
                    }
                    if n2 == 0 as libc::c_int {
                        break 's_239;
                    }
                    uc = uc2;
                    ucptr = NULL as *const libc::c_char;
                    n = n2
                }
            }
        }
    }
    (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = '\u{0}' as i32 as libc::c_char;
    if ts == 2 as libc::c_int {
        *(*as_0).s.offset(
            (*as_0)
                .length
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
        ) = '\u{0}' as i32 as libc::c_char
    }
    return ret;
}
/*
 * libarchive 2.x made incorrect UTF-8 strings in the wrong assumption
 * that WCS is Unicode. It is true for several platforms but some are false.
 * And then people who did not use UTF-8 locale on the non Unicode WCS
 * platform and made a tar file with libarchive(mostly bsdtar) 2.x. Those
 * now cannot get right filename from libarchive 3.x and later since we
 * fixed the wrong assumption and it is incompatible to older its versions.
 * So we provide special option, "compat-2x.x", for resolving it.
 * That option enable the string conversion of libarchive 2.x.
 *
 * Translates the wrong UTF-8 string made by libarchive 2.x into current
 * locale character set and appends to the archive_string.
 * Note: returns -1 if conversion fails.
 */
unsafe extern "C" fn strncat_from_utf8_libarchive2(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unicode: uint32_t = 0;
    let mut shift_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed_0 { __wch: 0 },
    };
    memset(
        &mut shift_state as *mut mbstate_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
    /* UNUSED */
    /*
     * Allocate buffer for MBS.
     * We need this allocation here since it is possible that
     * as->s is still NULL.
     */
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    s = _p as *const libc::c_char;
    p = (*as_0).s.offset((*as_0).length as isize);
    end = (*as_0)
        .s
        .offset((*as_0).buffer_length as isize)
        .offset(-(MB_CUR_MAX as isize))
        .offset(-(1 as libc::c_int as isize));
    loop {
        n = _utf8_to_unicode(&mut unicode, s, len);
        if !(n != 0 as libc::c_int) {
            break;
        }
        let mut wc: wchar_t = 0;
        if p >= end {
            (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
            /* Re-allocate buffer for MBS. */
            if archive_string_ensure(
                as_0,
                (*as_0)
                    .length
                    .wrapping_add(
                        (if len.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            > __ctype_get_mb_cur_max()
                        {
                            len.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        } else {
                            __ctype_get_mb_cur_max()
                        }),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            )
            .is_null()
            {
                return -(1 as libc::c_int);
            }
            p = (*as_0).s.offset((*as_0).length as isize);
            end = (*as_0)
                .s
                .offset((*as_0).buffer_length as isize)
                .offset(-(MB_CUR_MAX as isize))
                .offset(-(1 as libc::c_int as isize))
        }
        /*
         * As libarchive 2.x, translates the UTF-8 characters into
         * wide-characters in the assumption that WCS is Unicode.
         */
        if n < 0 as libc::c_int {
            n *= -(1 as libc::c_int);
            wc = '?' as i32
        } else {
            wc = unicode as wchar_t
        }
        s = s.offset(n as isize);
        len = (len as libc::c_ulong).wrapping_sub(n as libc::c_ulong) as size_t as size_t;
        /*
         * Translates the wide-character into the current locale MBS.
         */
        n = wcrtomb(p, wc, &mut shift_state) as libc::c_int;
        if n == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        p = p.offset(n as isize)
    }
    (*as_0).length = p.offset_from((*as_0).s) as libc::c_long as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = '\u{0}' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
/*
 * Conversion functions between current locale dependent MBS and UTF-16BE.
 *   strncat_from_utf16be() : UTF-16BE --> MBS
 *   strncat_to_utf16be()   : MBS --> UTF16BE
 */
/* _WIN32 && !__CYGWIN__ */
/*
 * Do the best effort for conversions.
 * We cannot handle UTF-16BE character-set without such iconv,
 * but there is a chance if a string consists just ASCII code or
 * a current locale is UTF-8.
 */
/*
 * Convert a UTF-16BE string to current locale and copy the result.
 * Return -1 if conversion fails.
 */
unsafe extern "C" fn best_effort_strncat_from_utf16(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut bytes: size_t,
    mut sc: *mut archive_string_conv,
    mut be: libc::c_int,
) -> libc::c_int {
    let mut utf16: *const libc::c_char = _p as *const libc::c_char;
    let mut mbs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uc: uint32_t = 0;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    /* UNUSED */
    /*
     * Other case, we should do the best effort.
     * If all character are ASCII(<0x7f), we can convert it.
     * if not , we set a alternative character and return -1.
     */
    ret = 0 as libc::c_int;
    if archive_string_ensure(
        as_0,
        (*as_0)
            .length
            .wrapping_add(bytes)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    mbs = (*as_0).s.offset((*as_0).length as isize);
    loop {
        n = utf16_to_unicode(&mut uc, utf16, bytes, be);
        if !(n != 0 as libc::c_int) {
            break;
        }
        if n < 0 as libc::c_int {
            n *= -(1 as libc::c_int);
            ret = -(1 as libc::c_int)
        }
        bytes = (bytes as libc::c_ulong).wrapping_sub(n as libc::c_ulong) as size_t as size_t;
        utf16 = utf16.offset(n as isize);
        if uc > 127 as libc::c_int as libc::c_uint {
            /* We cannot handle it. */
            let fresh108 = mbs;
            mbs = mbs.offset(1);
            *fresh108 = '?' as i32 as libc::c_char;
            ret = -(1 as libc::c_int)
        } else {
            let fresh109 = mbs;
            mbs = mbs.offset(1);
            *fresh109 = uc as libc::c_char
        }
    }
    (*as_0).length = mbs.offset_from((*as_0).s) as libc::c_long as size_t;
    *(*as_0).s.offset((*as_0).length as isize) = '\u{0}' as i32 as libc::c_char;
    return ret;
}
unsafe extern "C" fn best_effort_strncat_from_utf16be(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut bytes: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return best_effort_strncat_from_utf16(as_0, _p, bytes, sc, 1 as libc::c_int);
}
unsafe extern "C" fn best_effort_strncat_from_utf16le(
    mut as_0: *mut archive_string,
    mut _p: *const libc::c_void,
    mut bytes: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return best_effort_strncat_from_utf16(as_0, _p, bytes, sc, 0 as libc::c_int);
}
/*
 * Convert a current locale string to UTF-16BE/LE and copy the result.
 * Return -1 if conversion fails.
 */
unsafe extern "C" fn best_effort_strncat_to_utf16(
    mut as16: *mut archive_string,
    mut _p: *const libc::c_void,
    mut length: size_t,
    mut sc: *mut archive_string_conv,
    mut bigendian: libc::c_int,
) -> libc::c_int {
    let mut s: *const libc::c_char = _p as *const libc::c_char;
    let mut utf16: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut remaining: size_t = 0;
    let mut ret: libc::c_int = 0;
    /* UNUSED */
    /*
     * Other case, we should do the best effort.
     * If all character are ASCII(<0x7f), we can convert it.
     * if not , we set a alternative character and return -1.
     */
    ret = 0 as libc::c_int;
    remaining = length;
    if archive_string_ensure(
        as16,
        (*as16).length.wrapping_add(
            length
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ),
    )
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    utf16 = (*as16).s.offset((*as16).length as isize);
    loop {
        let fresh110 = remaining;
        remaining = remaining.wrapping_sub(1);
        if !(fresh110 != 0) {
            break;
        }
        let fresh111 = s;
        s = s.offset(1);
        let mut c: libc::c_uint = *fresh111 as libc::c_uint;
        if c > 127 as libc::c_int as libc::c_uint {
            /* We cannot handle it. */
            c = UNICODE_R_CHAR as libc::c_uint;
            ret = -(1 as libc::c_int)
        }
        if bigendian != 0 {
            archive_be16enc(utf16 as *mut libc::c_void, c as uint16_t);
        } else {
            archive_le16enc(utf16 as *mut libc::c_void, c as uint16_t);
        }
        utf16 = utf16.offset(2 as libc::c_int as isize)
    }
    (*as16).length = utf16.offset_from((*as16).s) as libc::c_long as size_t;
    *(*as16).s.offset((*as16).length as isize) = 0 as libc::c_int as libc::c_char;
    *(*as16).s.offset(
        (*as16)
            .length
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
    ) = 0 as libc::c_int as libc::c_char;
    return ret;
}
unsafe extern "C" fn best_effort_strncat_to_utf16be(
    mut as16: *mut archive_string,
    mut _p: *const libc::c_void,
    mut length: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return best_effort_strncat_to_utf16(as16, _p, length, sc, 1 as libc::c_int);
}
unsafe extern "C" fn best_effort_strncat_to_utf16le(
    mut as16: *mut archive_string,
    mut _p: *const libc::c_void,
    mut length: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    return best_effort_strncat_to_utf16(as16, _p, length, sc, 0 as libc::c_int);
}
/*
 * Multistring operations.
 */
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_clean(mut aes: *mut archive_mstring) {
    archive_wstring_free(&mut (*aes).aes_wcs);
    archive_string_free(&mut (*aes).aes_mbs);
    archive_string_free(&mut (*aes).aes_utf8);
    archive_string_free(&mut (*aes).aes_mbs_in_locale);
    (*aes).aes_set = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_copy(
    mut dest: *mut archive_mstring,
    mut src: *mut archive_mstring,
) {
    (*dest).aes_set = (*src).aes_set;
    (*dest).aes_mbs.length = 0 as libc::c_int as size_t;
    archive_string_concat(&mut (*dest).aes_mbs, &mut (*src).aes_mbs);
    (*dest).aes_utf8.length = 0 as libc::c_int as size_t;
    archive_string_concat(&mut (*dest).aes_utf8, &mut (*src).aes_utf8);
    (*dest).aes_wcs.length = 0 as libc::c_int as size_t;
    archive_wstring_concat(&mut (*dest).aes_wcs, &mut (*src).aes_wcs);
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_get_utf8(
    mut a: *mut archive,
    mut aes: *mut archive_mstring,
    mut p: *mut *const libc::c_char,
) -> libc::c_int {
    let mut sc: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut r: libc::c_int = 0;
    /* If we already have a UTF8 form, return that immediately. */
    if (*aes).aes_set & AES_SET_UTF8 != 0 {
        *p = (*aes).aes_utf8.s;
        return 0 as libc::c_int;
    }
    *p = NULL as *const libc::c_char;
    if (*aes).aes_set & AES_SET_MBS != 0 {
        sc = archive_string_conversion_to_charset(
            a,
            b"UTF-8\x00" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        /* failure. */
        if sc.is_null() {
            return -(1 as libc::c_int);
        } /* Couldn't allocate memory for sc. */
        r = archive_strncpy_l(
            &mut (*aes).aes_utf8,
            (*aes).aes_mbs.s as *const libc::c_void,
            (*aes).aes_mbs.length,
            sc,
        );
        if a.is_null() {
            free_sconv_object(sc);
        }
        if r == 0 as libc::c_int {
            (*aes).aes_set |= AES_SET_UTF8;
            *p = (*aes).aes_utf8.s;
            return 0 as libc::c_int;
        /* success. */
        } else {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
    /* success. */
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_get_mbs(
    mut a: *mut archive,
    mut aes: *mut archive_mstring,
    mut p: *mut *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    /* UNUSED */
    /* If we already have an MBS form, return that immediately. */
    if (*aes).aes_set & AES_SET_MBS != 0 {
        *p = (*aes).aes_mbs.s;
        return ret;
    }
    *p = NULL as *const libc::c_char;
    /* If there's a WCS form, try converting with the native locale. */
    if (*aes).aes_set & AES_SET_WCS != 0 {
        (*aes).aes_mbs.length = 0 as libc::c_int as size_t;
        r = archive_string_append_from_wcs(
            &mut (*aes).aes_mbs,
            (*aes).aes_wcs.s,
            (*aes).aes_wcs.length,
        );
        *p = (*aes).aes_mbs.s;
        if r == 0 as libc::c_int {
            (*aes).aes_set |= AES_SET_MBS;
            return ret;
        } else {
            ret = -(1 as libc::c_int)
        }
    }
    /*
     * Only a UTF-8 form cannot avail because its conversion already
     * failed at archive_mstring_update_utf8().
     */
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_get_wcs(
    mut a: *mut archive,
    mut aes: *mut archive_mstring,
    mut wp: *mut *const wchar_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    /* UNUSED */
    /* Return WCS form if we already have it. */
    if (*aes).aes_set & AES_SET_WCS != 0 {
        *wp = (*aes).aes_wcs.s;
        return ret;
    }
    *wp = NULL as *const wchar_t;
    /* Try converting MBS to WCS using native locale. */
    if (*aes).aes_set & AES_SET_MBS != 0 {
        (*aes).aes_wcs.length = 0 as libc::c_int as size_t;
        r = archive_wstring_append_from_mbs(
            &mut (*aes).aes_wcs,
            (*aes).aes_mbs.s,
            (*aes).aes_mbs.length,
        );
        if r == 0 as libc::c_int {
            (*aes).aes_set |= AES_SET_WCS;
            *wp = (*aes).aes_wcs.s
        } else {
            ret = -(1 as libc::c_int)
        }
        /* failure. */
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_get_mbs_l(
    mut aes: *mut archive_mstring,
    mut p: *mut *const libc::c_char,
    mut length: *mut size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    /* If there is not an MBS form but is a WCS form, try converting
     * with the native locale to be used for translating it to specified
     * character-set. */
    if (*aes).aes_set & AES_SET_MBS == 0 as libc::c_int
        && (*aes).aes_set & AES_SET_WCS != 0 as libc::c_int
    {
        (*aes).aes_mbs.length = 0 as libc::c_int as size_t;
        r = archive_string_append_from_wcs(
            &mut (*aes).aes_mbs,
            (*aes).aes_wcs.s,
            (*aes).aes_wcs.length,
        );
        if r == 0 as libc::c_int {
            (*aes).aes_set |= AES_SET_MBS
        } else if errno == ENOMEM {
            return -(1 as libc::c_int);
        } else {
            ret = -(1 as libc::c_int)
        }
    }
    /* If we already have an MBS form, use it to be translated to
     * specified character-set. */
    if (*aes).aes_set & AES_SET_MBS != 0 {
        if sc.is_null() {
            /* Conversion is unneeded. */
            *p = (*aes).aes_mbs.s; /* Only MBS form is set now. */
            if !length.is_null() {
                *length = (*aes).aes_mbs.length
            } /* Only WCS form set. */
            return 0 as libc::c_int;
        } /* Only MBS form is set now. */
        ret = archive_strncpy_l(
            &mut (*aes).aes_mbs_in_locale,
            (*aes).aes_mbs.s as *const libc::c_void,
            (*aes).aes_mbs.length,
            sc,
        );
        *p = (*aes).aes_mbs_in_locale.s;
        if !length.is_null() {
            *length = (*aes).aes_mbs_in_locale.length
        }
    } else {
        *p = NULL as *const libc::c_char;
        if !length.is_null() {
            *length = 0 as libc::c_int as size_t
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_copy_mbs(
    mut aes: *mut archive_mstring,
    mut mbs: *const libc::c_char,
) -> libc::c_int {
    if mbs.is_null() {
        (*aes).aes_set = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    return archive_mstring_copy_mbs_len(aes, mbs, strlen(mbs));
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_copy_mbs_len(
    mut aes: *mut archive_mstring,
    mut mbs: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if mbs.is_null() {
        (*aes).aes_set = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*aes).aes_set = AES_SET_MBS;
    (*aes).aes_mbs.length = 0 as libc::c_int as size_t;
    archive_strncat(&mut (*aes).aes_mbs, mbs as *const libc::c_void, len);
    (*aes).aes_utf8.length = 0 as libc::c_int as size_t;
    (*aes).aes_wcs.length = 0 as libc::c_int as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_copy_wcs(
    mut aes: *mut archive_mstring,
    mut wcs: *const wchar_t,
) -> libc::c_int {
    return archive_mstring_copy_wcs_len(
        aes,
        wcs,
        if wcs.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            wcslen(wcs)
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_copy_utf8(
    mut aes: *mut archive_mstring,
    mut utf8: *const libc::c_char,
) -> libc::c_int {
    if utf8.is_null() {
        (*aes).aes_set = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*aes).aes_set = AES_SET_UTF8;
    (*aes).aes_mbs.length = 0 as libc::c_int as size_t;
    (*aes).aes_wcs.length = 0 as libc::c_int as size_t;
    (*aes).aes_utf8.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*aes).aes_utf8,
        utf8 as *const libc::c_void,
        strlen(utf8),
    );
    return strlen(utf8) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_copy_wcs_len(
    mut aes: *mut archive_mstring,
    mut wcs: *const wchar_t,
    mut len: size_t,
) -> libc::c_int {
    if wcs.is_null() {
        (*aes).aes_set = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*aes).aes_set = AES_SET_WCS;
    (*aes).aes_mbs.length = 0 as libc::c_int as size_t;
    (*aes).aes_utf8.length = 0 as libc::c_int as size_t;
    (*aes).aes_wcs.length = 0 as libc::c_int as size_t;
    archive_wstrncat(&mut (*aes).aes_wcs, wcs, len);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_copy_mbs_len_l(
    mut aes: *mut archive_mstring,
    mut mbs: *const libc::c_char,
    mut len: size_t,
    mut sc: *mut archive_string_conv,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if mbs.is_null() {
        (*aes).aes_set = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    (*aes).aes_mbs.length = 0 as libc::c_int as size_t;
    (*aes).aes_wcs.length = 0 as libc::c_int as size_t;
    (*aes).aes_utf8.length = 0 as libc::c_int as size_t;
    r = archive_strncpy_l(&mut (*aes).aes_mbs, mbs as *const libc::c_void, len, sc);
    if r == 0 as libc::c_int {
        (*aes).aes_set = AES_SET_MBS
    } else {
        (*aes).aes_set = 0 as libc::c_int
    }
    return r;
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
 * $FreeBSD: head/lib/libarchive/archive_string.h 201092 2009-12-28 02:26:06Z kientzle $
 *
 */
/* required for wchar_t on some systems */
/*
 * Basic resizable/reusable string support similar to Java's "StringBuffer."
 *
 * Unlike sbuf(9), the buffers here are fully reusable and track the
 * length throughout.
 */
/* Pointer to the storage */
/* Length of 's' in characters */
/* Length of malloc-ed storage in bytes. */
/* Pointer to the storage */
/* Length of 's' in characters */
/* Length of malloc-ed storage in bytes. */
/* Initialize an archive_string object on the stack or elsewhere. */
/* Append a C char to an archive_string, resizing as necessary. */
/* Ditto for a wchar_t and an archive_wstring. */
/* Append a raw array to an archive_string, resizing as necessary */
/* Convert a Unicode string to current locale and append the result. */
/* Returns -1 if conversion fails. */
/* Create a string conversion object.
 * Return NULL and set a error message if the conversion is not supported
 * on the platform. */
/* Create the default string conversion object for reading/writing an archive.
 * Return NULL if the conversion is unneeded.
 * Note: On non Windows platform this always returns NULL.
 */
/* Dispose of a string conversion object. */
/* Copy one archive_string to another in locale conversion.
 * Return -1 if conversion fails. */
/* Copy one archive_string to another in locale conversion.
 * Return -1 if conversion fails. */
/* Copy one archive_string to another */
/* Concatenate one archive_string to another */
/* Ensure that the underlying buffer is at least as large as the request. */
/* Append C string, which may lack trailing \0. */
/* The source is declared void * here because this gets used with
 * "signed char *", "unsigned char *" and "char *" arguments.
 * Declaring it "char *" as with some of the other functions just
 * leads to a lot of extra casts. */
/* Append a C string to an archive_string, resizing as necessary. */
/* Copy a C string to an archive_string, resizing as necessary. */
/* Copy a C string to an archive_string with limit, resizing as necessary. */
/* Return length of string. */
/* Set string length to zero. */
/* Release any allocated storage resources. */
/* Like 'vsprintf', but resizes the underlying string as necessary. */
/* Note: This only implements a small subset of standard printf functionality. */
/* Translates from MBS to Unicode. */
/* Returns non-zero if conversion failed in any way. */
/* A "multistring" can hold Unicode, UTF8, or MBS versions of
 * the string.  If you set and read the same version, no translation
 * is done.  If you set and read different versions, the library
 * will attempt to transparently convert.
 */
/* Bitmap of which of the above are valid.  Because we're lazy
 * about malloc-ing and reusing the underlying storage, we
 * can't rely on NULL pointers to indicate whether a string
 * has been set. */
/*
 * The 'update' form tries to proactively update all forms of
 * this string (WCS and MBS) and returns an error if any of
 * them fail.  This is used by the 'pax' handler, for instance,
 * to detect and report character-conversion failures early while
 * still allowing clients to get potentially useful values from
 * the more tolerant lazy conversions.  (get_mbs and get_wcs will
 * strive to give the user something useful, so you can get hopefully
 * usable values even if some of the character conversions are failing.)
 */
#[no_mangle]
pub unsafe extern "C" fn archive_mstring_update_utf8(
    mut a: *mut archive,
    mut aes: *mut archive_mstring,
    mut utf8: *const libc::c_char,
) -> libc::c_int {
    let mut sc: *mut archive_string_conv = 0 as *mut archive_string_conv;
    let mut r: libc::c_int = 0;
    if utf8.is_null() {
        (*aes).aes_set = 0 as libc::c_int;
        return 0 as libc::c_int;
        /* Succeeded in clearing everything. */
    }
    /* Save the UTF8 string. */
    (*aes).aes_utf8.length = 0 as libc::c_int as size_t;
    archive_strncat(
        &mut (*aes).aes_utf8,
        utf8 as *const libc::c_void,
        (if utf8.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(utf8)
        }),
    );
    /* Empty the mbs and wcs strings. */
    (*aes).aes_mbs.length = 0 as libc::c_int as size_t; /* Only UTF8 is set now. */
    (*aes).aes_wcs.length = 0 as libc::c_int as size_t;
    (*aes).aes_set = AES_SET_UTF8;
    /* Try converting UTF-8 to MBS, return false on failure. */
    sc = archive_string_conversion_from_charset(
        a,
        b"UTF-8\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ); /* Couldn't allocate memory for sc. */
    if sc.is_null() {
        return -(1 as libc::c_int);
    } /* Both UTF8 and MBS set. */
    r = archive_strncpy_l(
        &mut (*aes).aes_mbs,
        utf8 as *const libc::c_void,
        if utf8.is_null() {
            0 as libc::c_int as libc::c_ulong
        } else {
            strlen(utf8)
        },
        sc,
    );
    if a.is_null() {
        free_sconv_object(sc);
    }
    if r != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*aes).aes_set = AES_SET_UTF8 | AES_SET_MBS;
    /* Try converting MBS to WCS, return false on failure. */
    if archive_wstring_append_from_mbs(&mut (*aes).aes_wcs, (*aes).aes_mbs.s, (*aes).aes_mbs.length)
        != 0
    {
        return -(1 as libc::c_int);
    }
    (*aes).aes_set = AES_SET_UTF8 | AES_SET_WCS | AES_SET_MBS;
    /* All conversions succeeded. */
    return 0 as libc::c_int;
}
