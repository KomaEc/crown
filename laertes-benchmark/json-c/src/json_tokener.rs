#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, extern_types,
           linkage, ptr_offset_from, register_tool)]
extern "C" {
    pub type lh_table;
    #[no_mangle]
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn __maskrune(_: __darwin_ct_rune_t, _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: std::os::raw::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
               _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn strdup(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strncasecmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                   _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn mc_debug(msg: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn printbuf_new() -> *mut printbuf;
    #[no_mangle]
    fn printbuf_memappend(p: *mut printbuf, buf: *const std::os::raw::c_char,
                          size: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn printbuf_reset(p: *mut printbuf);
    #[no_mangle]
    fn printbuf_free(p: *mut printbuf);
    #[no_mangle]
    fn json_object_get(obj: *mut json_object) -> *mut json_object;
    #[no_mangle]
    fn json_object_put(obj: *mut json_object) -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_new_object() -> *mut json_object;
    #[no_mangle]
    fn json_object_object_add(obj: *mut json_object, key: *const std::os::raw::c_char,
                              val: *mut json_object) -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_new_array() -> *mut json_object;
    #[no_mangle]
    fn json_object_array_add(obj: *mut json_object, val: *mut json_object)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_new_boolean(b: json_bool) -> *mut json_object;
    #[no_mangle]
    fn json_object_new_int64(i: int64_t) -> *mut json_object;
    #[no_mangle]
    fn json_object_new_double(d: std::os::raw::c_double) -> *mut json_object;
    #[no_mangle]
    fn json_object_new_double_s(d: std::os::raw::c_double, ds: *const std::os::raw::c_char)
     -> *mut json_object;
    #[no_mangle]
    fn json_object_new_string_len(s: *const std::os::raw::c_char, len: std::os::raw::c_int)
     -> *mut json_object;
    #[no_mangle]
    static mut json_number_chars: *const std::os::raw::c_char;
    #[no_mangle]
    static mut json_hex_chars: *const std::os::raw::c_char;
    #[no_mangle]
    fn json_parse_int64(buf: *const std::os::raw::c_char, retval: *mut int64_t)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn json_parse_double(buf: *const std::os::raw::c_char,
                         retval: *mut std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn setlocale(_: std::os::raw::c_int, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __darwin_ct_rune_t = std::os::raw::c_int;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_wchar_t = std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type int64_t = std::os::raw::c_longlong;
pub type size_t = __darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: std::os::raw::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [std::os::raw::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [std::os::raw::c_char; 8],
    pub __encoding: [std::os::raw::c_char; 32],
    pub __sgetrune: Option<unsafe extern "C" fn(_: *const std::os::raw::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *const std::os::raw::c_char)
                               -> __darwin_rune_t>,
    pub __sputrune: Option<unsafe extern "C" fn(_: __darwin_rune_t,
                                                _: *mut std::os::raw::c_char,
                                                _: __darwin_size_t,
                                                _: *mut *mut std::os::raw::c_char)
                               -> std::os::raw::c_int>,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: std::os::raw::c_int,
    pub __ncharclasses: std::os::raw::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut std::os::raw::c_char,
    pub bpos: std::os::raw::c_int,
    pub size: std::os::raw::c_int,
}
pub type array_list_free_fn
    =
    unsafe extern "C" fn(_: *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_list {
    pub array: *mut *mut libc::c_void,
    pub length: size_t,
    pub size: size_t,
    pub free_fn: Option<array_list_free_fn>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object {
    pub o_type: json_type,
    pub _delete: Option<json_object_private_delete_fn>,
    pub _to_json_string: Option<json_object_to_json_string_fn>,
    pub _ref_count: std::os::raw::c_int,
    pub _pb: *mut printbuf,
    pub o: data,
    pub _user_delete: Option<json_object_delete_fn>,
    pub _userdata: *mut libc::c_void,
}
pub type json_object_delete_fn
    =
    unsafe extern "C" fn(_: *mut json_object, _: *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub union data {
    pub c_boolean: json_bool,
    pub c_double: std::os::raw::c_double,
    pub c_int64: int64_t,
    pub c_object: *mut lh_table,
    pub c_array: *mut array_list,
    pub c_string: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub str_0: C2RustUnnamed_0,
    pub len: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ptr: *mut std::os::raw::c_char,
    pub data: [std::os::raw::c_char; 32],
}
pub type json_bool = std::os::raw::c_int;
pub type json_object_to_json_string_fn
    =
    unsafe extern "C" fn(_: *mut json_object, _: *mut printbuf,
                         _: std::os::raw::c_int, _: std::os::raw::c_int) -> std::os::raw::c_int;
pub type json_object_private_delete_fn
    =
    unsafe extern "C" fn(_: *mut json_object) -> ();
pub type json_type = std::os::raw::c_uint;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
pub type json_tokener_error = std::os::raw::c_uint;
pub const json_tokener_error_size: json_tokener_error = 14;
pub const json_tokener_error_parse_comment: json_tokener_error = 13;
pub const json_tokener_error_parse_string: json_tokener_error = 12;
pub const json_tokener_error_parse_object_value_sep: json_tokener_error = 11;
pub const json_tokener_error_parse_object_key_sep: json_tokener_error = 10;
pub const json_tokener_error_parse_object_key_name: json_tokener_error = 9;
pub const json_tokener_error_parse_array: json_tokener_error = 8;
pub const json_tokener_error_parse_number: json_tokener_error = 7;
pub const json_tokener_error_parse_boolean: json_tokener_error = 6;
pub const json_tokener_error_parse_null: json_tokener_error = 5;
pub const json_tokener_error_parse_unexpected: json_tokener_error = 4;
pub const json_tokener_error_parse_eof: json_tokener_error = 3;
pub const json_tokener_error_depth: json_tokener_error = 2;
pub const json_tokener_continue: json_tokener_error = 1;
pub const json_tokener_success: json_tokener_error = 0;
pub type json_tokener_state = std::os::raw::c_uint;
pub const json_tokener_state_inf: json_tokener_state = 24;
pub const json_tokener_state_object_field_start_after_sep: json_tokener_state
          =
    23;
pub const json_tokener_state_array_after_sep: json_tokener_state = 22;
pub const json_tokener_state_object_sep: json_tokener_state = 21;
pub const json_tokener_state_object_value_add: json_tokener_state = 20;
pub const json_tokener_state_object_value: json_tokener_state = 19;
pub const json_tokener_state_object_field_end: json_tokener_state = 18;
pub const json_tokener_state_object_field: json_tokener_state = 17;
pub const json_tokener_state_object_field_start: json_tokener_state = 16;
pub const json_tokener_state_array_sep: json_tokener_state = 15;
pub const json_tokener_state_array_add: json_tokener_state = 14;
pub const json_tokener_state_array: json_tokener_state = 13;
pub const json_tokener_state_number: json_tokener_state = 12;
pub const json_tokener_state_boolean: json_tokener_state = 11;
pub const json_tokener_state_escape_unicode: json_tokener_state = 10;
pub const json_tokener_state_string_escape: json_tokener_state = 9;
pub const json_tokener_state_string: json_tokener_state = 8;
pub const json_tokener_state_comment_end: json_tokener_state = 7;
pub const json_tokener_state_comment_eol: json_tokener_state = 6;
pub const json_tokener_state_comment: json_tokener_state = 5;
pub const json_tokener_state_comment_start: json_tokener_state = 4;
pub const json_tokener_state_null: json_tokener_state = 3;
pub const json_tokener_state_finish: json_tokener_state = 2;
pub const json_tokener_state_start: json_tokener_state = 1;
pub const json_tokener_state_eatws: json_tokener_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener_srec {
    pub state: json_tokener_state,
    pub saved_state: json_tokener_state,
    pub obj: *mut json_object,
    pub current: *mut json_object,
    pub obj_field_name: *mut std::os::raw::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_tokener {
    pub str_0: *mut std::os::raw::c_char,
    pub pb: *mut printbuf,
    pub max_depth: std::os::raw::c_int,
    pub depth: std::os::raw::c_int,
    pub is_double: std::os::raw::c_int,
    pub st_pos: std::os::raw::c_int,
    pub char_offset: std::os::raw::c_int,
    pub err: json_tokener_error,
    pub ucs_char: std::os::raw::c_uint,
    pub quote_char: std::os::raw::c_char,
    pub stack: *mut json_tokener_srec,
    pub flags: std::os::raw::c_int,
}
#[inline]
unsafe extern "C" fn isascii(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (_c & !(0x7f as std::os::raw::c_int) == 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t,
                              mut _f: std::os::raw::c_ulong) -> std::os::raw::c_int {
    return if isascii(_c) != 0 {
               (_DefaultRuneLocale.__runetype[_c as usize] as std::os::raw::c_ulong &
                    _f != 0) as std::os::raw::c_int
           } else { (__maskrune(_c, _f) != 0) as std::os::raw::c_int };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isspace(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __istype(_c, 0x4000 as std::os::raw::c_long as std::os::raw::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __tolower(_c);
}
/* HAVE_STRNCASECMP */
/* Use C99 NAN by default; if not available, nan("") should work too. */
/* !NAN */
static mut json_null_str: [std::os::raw::c_char; 5] =
    unsafe {
        *::std::mem::transmute::<&[u8; 5], &[std::os::raw::c_char; 5]>(b"null\x00")
    };
// Initialized in run_static_initializers
static mut json_null_str_len: std::os::raw::c_int = 0;
static mut json_inf_str: [std::os::raw::c_char; 9] =
    unsafe {
        *::std::mem::transmute::<&[u8; 9],
                                 &[std::os::raw::c_char; 9]>(b"Infinity\x00")
    };
static mut json_inf_str_lower: [std::os::raw::c_char; 9] =
    unsafe {
        *::std::mem::transmute::<&[u8; 9],
                                 &[std::os::raw::c_char; 9]>(b"infinity\x00")
    };
// Initialized in run_static_initializers
static mut json_inf_str_len: std::os::raw::c_uint = 0;
static mut json_nan_str: [std::os::raw::c_char; 4] =
    unsafe {
        *::std::mem::transmute::<&[u8; 4], &[std::os::raw::c_char; 4]>(b"NaN\x00")
    };
// Initialized in run_static_initializers
static mut json_nan_str_len: std::os::raw::c_int = 0;
static mut json_true_str: [std::os::raw::c_char; 5] =
    unsafe {
        *::std::mem::transmute::<&[u8; 5], &[std::os::raw::c_char; 5]>(b"true\x00")
    };
// Initialized in run_static_initializers
static mut json_true_str_len: std::os::raw::c_int = 0;
static mut json_false_str: [std::os::raw::c_char; 6] =
    unsafe {
        *::std::mem::transmute::<&[u8; 6], &[std::os::raw::c_char; 6]>(b"false\x00")
    };
// Initialized in run_static_initializers
static mut json_false_str_len: std::os::raw::c_int = 0;
static mut json_tokener_errors: [*const std::os::raw::c_char; 15] =
    [b"success\x00" as *const u8 as *const std::os::raw::c_char,
     b"continue\x00" as *const u8 as *const std::os::raw::c_char,
     b"nesting too deep\x00" as *const u8 as *const std::os::raw::c_char,
     b"unexpected end of data\x00" as *const u8 as *const std::os::raw::c_char,
     b"unexpected character\x00" as *const u8 as *const std::os::raw::c_char,
     b"null expected\x00" as *const u8 as *const std::os::raw::c_char,
     b"boolean expected\x00" as *const u8 as *const std::os::raw::c_char,
     b"number expected\x00" as *const u8 as *const std::os::raw::c_char,
     b"array value separator \',\' expected\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"quoted object property name expected\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"object property name separator \':\' expected\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"object value separator \',\' expected\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"invalid string sequence\x00" as *const u8 as *const std::os::raw::c_char,
     b"expected comment\x00" as *const u8 as *const std::os::raw::c_char,
     b"buffer size overflow\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn json_tokener_error_desc(mut jerr: json_tokener_error)
 -> *const std::os::raw::c_char {
    let mut jerr_int: std::os::raw::c_int = jerr as std::os::raw::c_int;
    if jerr_int < 0 as std::os::raw::c_int ||
           jerr_int >=
               (::std::mem::size_of::<[*const std::os::raw::c_char; 15]>() as
                    std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<*const std::os::raw::c_char>()
                                                    as std::os::raw::c_ulong) as
                   std::os::raw::c_int {
        return b"Unknown error, invalid json_tokener_error value passed to json_tokener_error_desc()\x00"
                   as *const u8 as *const std::os::raw::c_char
    }
    return json_tokener_errors[jerr as usize];
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_get_error(mut tok: *mut json_tokener)
 -> json_tokener_error {
    return (*tok).err;
}
static mut utf8_replacement_char: [std::os::raw::c_uchar; 3] =
    [0xef as std::os::raw::c_int as std::os::raw::c_uchar,
     0xbf as std::os::raw::c_int as std::os::raw::c_uchar,
     0xbd as std::os::raw::c_int as std::os::raw::c_uchar];
#[no_mangle]
pub unsafe extern "C" fn json_tokener_new_ex(mut depth: std::os::raw::c_int)
 -> *mut json_tokener {
    let mut tok: *mut json_tokener = 0 as *mut json_tokener;
    tok =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<json_tokener>() as std::os::raw::c_ulong) as
            *mut json_tokener;
    if tok.is_null() { return 0 as *mut json_tokener }
    (*tok).stack =
        calloc(depth as std::os::raw::c_ulong,
               ::std::mem::size_of::<json_tokener_srec>() as std::os::raw::c_ulong) as
            *mut json_tokener_srec;
    if (*tok).stack.is_null() {
        free(tok as *mut libc::c_void);
        return 0 as *mut json_tokener
    }
    (*tok).pb = printbuf_new();
    (*tok).max_depth = depth;
    json_tokener_reset(tok);
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_new() -> *mut json_tokener {
    return json_tokener_new_ex(32 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_free(mut tok: *mut json_tokener) {
    json_tokener_reset(tok);
    if !(*tok).pb.is_null() { printbuf_free((*tok).pb); }
    free((*tok).stack as *mut libc::c_void);
    free(tok as *mut libc::c_void);
}
unsafe extern "C" fn json_tokener_reset_level(mut tok: *mut json_tokener,
                                              mut depth: std::os::raw::c_int) {
    (*(*tok).stack.offset(depth as isize)).state = json_tokener_state_eatws;
    (*(*tok).stack.offset(depth as isize)).saved_state =
        json_tokener_state_start;
    json_object_put((*(*tok).stack.offset(depth as isize)).current);
    let ref mut fresh0 = (*(*tok).stack.offset(depth as isize)).current;
    *fresh0 = 0 as *mut json_object;
    free((*(*tok).stack.offset(depth as isize)).obj_field_name as
             *mut libc::c_void);
    let ref mut fresh1 =
        (*(*tok).stack.offset(depth as isize)).obj_field_name;
    *fresh1 = 0 as *mut std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_reset(mut tok: *mut json_tokener) {
    let mut i: std::os::raw::c_int = 0;
    if tok.is_null() { return }
    i = (*tok).depth;
    while i >= 0 as std::os::raw::c_int { json_tokener_reset_level(tok, i); i -= 1 }
    (*tok).depth = 0 as std::os::raw::c_int;
    (*tok).err = json_tokener_success;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_parse(mut str: *const std::os::raw::c_char)
 -> *mut json_object {
    let mut jerr_ignored: json_tokener_error = json_tokener_success;
    let mut obj: *mut json_object = 0 as *mut json_object;
    obj = json_tokener_parse_verbose(str, &mut jerr_ignored);
    return obj;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_parse_verbose(mut str:
                                                        *const std::os::raw::c_char,
                                                    mut error:
                                                        *mut json_tokener_error)
 -> *mut json_object {
    let mut tok: *mut json_tokener = 0 as *mut json_tokener;
    let mut obj: *mut json_object = 0 as *mut json_object;
    tok = json_tokener_new();
    if tok.is_null() { return 0 as *mut json_object }
    obj = json_tokener_parse_ex(tok, str, -(1 as std::os::raw::c_int));
    *error = (*tok).err;
    if (*tok).err as std::os::raw::c_uint !=
           json_tokener_success as std::os::raw::c_int as std::os::raw::c_uint {
        if !obj.is_null() { json_object_put(obj); }
        obj = 0 as *mut json_object
    }
    json_tokener_free(tok);
    return obj;
}
/* End optimization macro defs */
#[no_mangle]
pub unsafe extern "C" fn json_tokener_parse_ex(mut tok: *mut json_tokener,
                                               mut str: *const std::os::raw::c_char,
                                               mut len: std::os::raw::c_int)
 -> *mut json_object {
    let mut case_start_3: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut got_hi_surrogate: std::os::raw::c_uint = 0;
    let mut case_start_1: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut case_start_0: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut case_start: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut size: std::os::raw::c_int = 0;
    let mut size_nan: std::os::raw::c_int = 0;
    let mut size1: std::os::raw::c_int = 0;
    let mut size2: std::os::raw::c_int = 0;
    let mut current_block: u64;
    let mut obj: *mut json_object = 0 as *mut json_object;
    let mut c: std::os::raw::c_char = '\u{1}' as i32 as std::os::raw::c_char;
    let mut oldlocale: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    (*tok).char_offset = 0 as std::os::raw::c_int;
    (*tok).err = json_tokener_success;
    /* this interface is presently not 64-bit clean due to the int len argument
     and the internal printbuf interface that takes 32-bit int len arguments
     so the function limits the maximum string size to INT32_MAX (2GB).
     If the function is called with len == -1 then strlen is called to check
     the string length is less than INT32_MAX (2GB) */
    if len < -(1 as std::os::raw::c_int) ||
           len == -(1 as std::os::raw::c_int) &&
               strlen(str) > 2147483647 as std::os::raw::c_int as std::os::raw::c_ulong {
        (*tok).err = json_tokener_error_size; /* while(PEEK_CHAR) */
        return 0 as *mut json_object
    }
    let mut tmplocale: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    tmplocale = setlocale(4 as std::os::raw::c_int, 0 as *const std::os::raw::c_char);
    if !tmplocale.is_null() { oldlocale = strdup(tmplocale) }
    setlocale(4 as std::os::raw::c_int, b"C\x00" as *const u8 as *const std::os::raw::c_char);
    's_60:
        while if (*tok).char_offset == len {
                  if (*tok).depth == 0 as std::os::raw::c_int &&
                         (*(*tok).stack.offset((*tok).depth as isize)).state
                             as std::os::raw::c_uint ==
                             json_tokener_state_eatws as std::os::raw::c_int as
                                 std::os::raw::c_uint &&
                         (*(*tok).stack.offset((*tok).depth as
                                                   isize)).saved_state as
                             std::os::raw::c_uint ==
                             json_tokener_state_finish as std::os::raw::c_int as
                                 std::os::raw::c_uint {
                      (*tok).err = json_tokener_success;
                      0 as std::os::raw::c_int
                  } else {
                      (*tok).err = json_tokener_continue;
                      0 as std::os::raw::c_int
                  }
              } else { c = *str; 1 as std::os::raw::c_int } != 0 {
            loop  {
                match (*(*tok).stack.offset((*tok).depth as isize)).state as
                          std::os::raw::c_uint {
                    0 => {
                        /* Advance until we change state */
                        while isspace(c as std::os::raw::c_int) != 0 {
                            str = str.offset(1); // or NaN
                            (*tok).char_offset += 1;
                            if c == 0 ||
                                   (if (*tok).char_offset == len {
                                        (if (*tok).depth == 0 as std::os::raw::c_int
                                                &&
                                                (*(*tok).stack.offset((*tok).depth
                                                                          as
                                                                          isize)).state
                                                    as std::os::raw::c_uint ==
                                                    json_tokener_state_eatws
                                                        as std::os::raw::c_int as
                                                        std::os::raw::c_uint &&
                                                (*(*tok).stack.offset((*tok).depth
                                                                          as
                                                                          isize)).saved_state
                                                    as std::os::raw::c_uint ==
                                                    json_tokener_state_finish
                                                        as std::os::raw::c_int as
                                                        std::os::raw::c_uint {
                                             (*tok).err =
                                                 json_tokener_success;
                                             0 as std::os::raw::c_int
                                         } else {
                                             (*tok).err =
                                                 json_tokener_continue;
                                             0 as std::os::raw::c_int
                                         })
                                    } else { c = *str; 1 as std::os::raw::c_int }) ==
                                       0 {
                                break 's_60 ;
                            }
                        }
                        if c as std::os::raw::c_int == '/' as i32 &&
                               (*tok).flags & 0x1 as std::os::raw::c_int == 0 {
                            printbuf_reset((*tok).pb);
                            if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                   1 as std::os::raw::c_int {
                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                   as isize)
                                           as *mut libc::c_void,
                                       &mut c as *mut std::os::raw::c_char as
                                           *const libc::c_void,
                                       1 as std::os::raw::c_int as std::os::raw::c_ulong);
                                (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                             isize) =
                                    '\u{0}' as i32 as std::os::raw::c_char
                            } else {
                                printbuf_memappend((*tok).pb, &mut c,
                                                   1 as std::os::raw::c_int);
                            }
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_comment_start;
                            current_block = 8178439852521326633;
                            break ;
                        } else {
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).saved_state
                        }
                    }
                    1 => {
                        match c as std::os::raw::c_int {
                            123 => {
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).state =
                                    json_tokener_state_eatws;
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).saved_state
                                    = json_tokener_state_object_field_start;
                                let ref mut fresh2 =
                                    (*(*tok).stack.offset((*tok).depth as
                                                              isize)).current;
                                *fresh2 = json_object_new_object();
                                if (*(*tok).stack.offset((*tok).depth as
                                                             isize)).current.is_null()
                                   {
                                    break 's_60 ;
                                } else {
                                    current_block = 8178439852521326633;
                                    break ;
                                }
                            }
                            91 => {
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).state =
                                    json_tokener_state_eatws;
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).saved_state
                                    = json_tokener_state_array;
                                let ref mut fresh3 =
                                    (*(*tok).stack.offset((*tok).depth as
                                                              isize)).current;
                                *fresh3 = json_object_new_array();
                                if (*(*tok).stack.offset((*tok).depth as
                                                             isize)).current.is_null()
                                   {
                                    break 's_60 ;
                                } else {
                                    current_block = 8178439852521326633;
                                    break ;
                                }
                            }
                            73 | 105 => {
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).state =
                                    json_tokener_state_inf;
                                printbuf_reset((*tok).pb);
                                (*tok).st_pos = 0 as std::os::raw::c_int
                            }
                            78 | 110 => {
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).state =
                                    json_tokener_state_null;
                                printbuf_reset((*tok).pb);
                                (*tok).st_pos = 0 as std::os::raw::c_int
                            }
                            39 => {
                                if (*tok).flags & 0x1 as std::os::raw::c_int != 0 {
                                    current_block = 11763295167351361500;
                                    break ;
                                } else {
                                    current_block = 8892199680835157453;
                                    break ;
                                }
                            }
                            34 => {
                                current_block = 8892199680835157453;
                                break ;
                            }
                            84 | 116 | 70 | 102 => {
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).state =
                                    json_tokener_state_boolean;
                                printbuf_reset((*tok).pb);
                                (*tok).st_pos = 0 as std::os::raw::c_int
                            }
                            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 |
                            45 => {
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).state =
                                    json_tokener_state_number;
                                printbuf_reset((*tok).pb);
                                (*tok).is_double = 0 as std::os::raw::c_int
                            }
                            _ => {
                                (*tok).err =
                                    json_tokener_error_parse_unexpected;
                                break 's_60 ;
                            }
                        }
                    }
                    2 => {
                        if (*tok).depth == 0 as std::os::raw::c_int { break 's_60 ; }
                        obj =
                            json_object_get((*(*tok).stack.offset((*tok).depth
                                                                      as
                                                                      isize)).current);
                        json_tokener_reset_level(tok, (*tok).depth);
                        (*tok).depth -= 1
                    }
                    24 => {
                        /* aka starts with 'i' (or 'I', or "-i", or "-I") */
                        /* If we were guaranteed to have len set, then we could (usually) handle
	 * the entire "Infinity" check in a single strncmp (strncasecmp), but
	 * since len might be -1 (i.e. "read until \0"), we need to check it
	 * a character at a time.
	 * Trying to handle it both ways would make this code considerably more
	 * complicated with likely little performance benefit.
	 */
                        let mut is_negative: std::os::raw::c_int = 0 as std::os::raw::c_int;
                        let mut _json_inf_str: *const std::os::raw::c_char =
                            json_inf_str.as_ptr();
                        if (*tok).flags & 0x1 as std::os::raw::c_int == 0 {
                            _json_inf_str = json_inf_str_lower.as_ptr()
                        }
                        /* Note: tok->st_pos must be 0 when state is set to json_tokener_state_inf */
                        while (*tok).st_pos < json_inf_str_len as std::os::raw::c_int
                              {
                            let mut inf_char: std::os::raw::c_char = *str;
                            if (*tok).flags & 0x1 as std::os::raw::c_int == 0 {
                                inf_char =
                                    tolower(*str as std::os::raw::c_int) as
                                        std::os::raw::c_char
                            }
                            if inf_char as std::os::raw::c_int !=
                                   *_json_inf_str.offset((*tok).st_pos as
                                                             isize) as
                                       std::os::raw::c_int {
                                (*tok).err =
                                    json_tokener_error_parse_unexpected;
                                break 's_60 ;
                            } else {
                                (*tok).st_pos += 1;
                                str = str.offset(1);
                                (*tok).char_offset += 1;
                                if if (*tok).char_offset == len {
                                       if (*tok).depth == 0 as std::os::raw::c_int &&
                                              (*(*tok).stack.offset((*tok).depth
                                                                        as
                                                                        isize)).state
                                                  as std::os::raw::c_uint ==
                                                  json_tokener_state_eatws as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint &&
                                              (*(*tok).stack.offset((*tok).depth
                                                                        as
                                                                        isize)).saved_state
                                                  as std::os::raw::c_uint ==
                                                  json_tokener_state_finish as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint {
                                           (*tok).err = json_tokener_success;
                                           0 as std::os::raw::c_int
                                       } else {
                                           (*tok).err = json_tokener_continue;
                                           0 as std::os::raw::c_int
                                       }
                                   } else { c = *str; 1 as std::os::raw::c_int } == 0
                                   {
                                    break 's_60 ;
                                }
                            }
                        }
                        /* We checked the full length of "Infinity", so create the object.
	 * When handling -Infinity, the number parsing code will have dropped
	 * the "-" into tok->pb for us, so check it now.
	 */
                        if (*(*tok).pb).bpos > 0 as std::os::raw::c_int &&
                               *(*(*tok).pb).buf as std::os::raw::c_int == '-' as i32
                           {
                            is_negative = 1 as std::os::raw::c_int
                        }
                        let ref mut fresh4 =
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).current;
                        *fresh4 =
                            json_object_new_double(if is_negative != 0 {
                                                       -::std::f32::INFINITY
                                                   } else {
                                                       ::std::f32::INFINITY
                                                   } as std::os::raw::c_double);
                        if (*(*tok).stack.offset((*tok).depth as
                                                     isize)).current.is_null()
                           {
                            break 's_60 ;
                        }
                        (*(*tok).stack.offset((*tok).depth as
                                                  isize)).saved_state =
                            json_tokener_state_finish;
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws
                    }
                    3 => {
                        /* aka starts with 'n' */
                        size = 0;
                        size_nan = 0;
                        if (*(*tok).pb).size - (*(*tok).pb).bpos >
                               1 as std::os::raw::c_int {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   &mut c as *mut std::os::raw::c_char as
                                       *const libc::c_void,
                                   1 as std::os::raw::c_int as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb, &mut c,
                                               1 as std::os::raw::c_int);
                        }
                        size =
                            if ((*tok).st_pos + 1 as std::os::raw::c_int) <
                                   json_null_str_len {
                                ((*tok).st_pos) + 1 as std::os::raw::c_int
                            } else { json_null_str_len };
                        size_nan =
                            if ((*tok).st_pos + 1 as std::os::raw::c_int) <
                                   json_nan_str_len {
                                ((*tok).st_pos) + 1 as std::os::raw::c_int
                            } else { json_nan_str_len };
                        if (*tok).flags & 0x1 as std::os::raw::c_int == 0 &&
                               strncasecmp(json_null_str.as_ptr(),
                                           (*(*tok).pb).buf,
                                           size as std::os::raw::c_ulong) ==
                                   0 as std::os::raw::c_int ||
                               strncmp(json_null_str.as_ptr(),
                                       (*(*tok).pb).buf,
                                       size as std::os::raw::c_ulong) ==
                                   0 as std::os::raw::c_int {
                            if !((*tok).st_pos == json_null_str_len) {
                                current_block = 9508502311343125869;
                                break ;
                            }
                            let ref mut fresh5 =
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).current;
                            *fresh5 = 0 as *mut json_object;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_finish;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_eatws
                        } else if (*tok).flags & 0x1 as std::os::raw::c_int == 0 &&
                                      strncasecmp(json_nan_str.as_ptr(),
                                                  (*(*tok).pb).buf,
                                                  size_nan as std::os::raw::c_ulong)
                                          == 0 as std::os::raw::c_int ||
                                      strncmp(json_nan_str.as_ptr(),
                                              (*(*tok).pb).buf,
                                              size_nan as std::os::raw::c_ulong) ==
                                          0 as std::os::raw::c_int {
                            if !((*tok).st_pos == json_nan_str_len) {
                                current_block = 9508502311343125869;
                                break ;
                            }
                            let ref mut fresh6 =
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).current;
                            *fresh6 =
                                json_object_new_double(::std::f32::NAN as
                                                           std::os::raw::c_double);
                            if (*(*tok).stack.offset((*tok).depth as
                                                         isize)).current.is_null()
                               {
                                break 's_60 ;
                            }
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_finish;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_eatws
                        } else {
                            (*tok).err = json_tokener_error_parse_null;
                            break 's_60 ;
                        }
                    }
                    4 => {
                        if c as std::os::raw::c_int == '*' as i32 {
                            current_block = 1604201581803946138;
                            break ;
                        } else {
                            current_block = 15417752026496523887;
                            break ;
                        }
                    }
                    5 => {
                        /* Advance until we change state */
                        case_start = str;
                        current_block = 15321816652064063775;
                        break ;
                    }
                    6 => {
                        /* Advance until we change state */
                        case_start_0 = str;
                        current_block = 10253503901371725554;
                        break ;
                    }
                    7 => {
                        if (*(*tok).pb).size - (*(*tok).pb).bpos >
                               1 as std::os::raw::c_int {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   &mut c as *mut std::os::raw::c_char as
                                       *const libc::c_void,
                                   1 as std::os::raw::c_int as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb, &mut c,
                                               1 as std::os::raw::c_int);
                        }
                        if c as std::os::raw::c_int == '/' as i32 {
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_eatws
                        } else {
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_comment
                        }
                        current_block = 8178439852521326633;
                        break ;
                    }
                    8 => {
                        /* Advance until we change state */
                        case_start_1 = str;
                        current_block = 13434751124187322381;
                        break ;
                    }
                    9 => {
                        match c as std::os::raw::c_int {
                            34 | 92 | 47 => {
                                current_block = 3988677119214552796;
                                break ;
                            }
                            98 | 110 | 114 | 116 | 102 => {
                                current_block = 484703659935464883;
                                break ;
                            }
                            117 => {
                                current_block = 7391758747053199081;
                                break ;
                            }
                            _ => {
                                current_block = 7415988906898333153;
                                break ;
                            }
                        }
                    }
                    10 => {
                        got_hi_surrogate = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                        current_block = 15462824429697920828;
                        break ;
                    }
                    11 => {
                        size1 = 0;
                        size2 = 0;
                        if (*(*tok).pb).size - (*(*tok).pb).bpos >
                               1 as std::os::raw::c_int {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   &mut c as *mut std::os::raw::c_char as
                                       *const libc::c_void,
                                   1 as std::os::raw::c_int as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb, &mut c,
                                               1 as std::os::raw::c_int);
                        }
                        size1 =
                            if ((*tok).st_pos + 1 as std::os::raw::c_int) <
                                   json_true_str_len {
                                ((*tok).st_pos) + 1 as std::os::raw::c_int
                            } else { json_true_str_len };
                        size2 =
                            if ((*tok).st_pos + 1 as std::os::raw::c_int) <
                                   json_false_str_len {
                                ((*tok).st_pos) + 1 as std::os::raw::c_int
                            } else { json_false_str_len };
                        if (*tok).flags & 0x1 as std::os::raw::c_int == 0 &&
                               strncasecmp(json_true_str.as_ptr(),
                                           (*(*tok).pb).buf,
                                           size1 as std::os::raw::c_ulong) ==
                                   0 as std::os::raw::c_int ||
                               strncmp(json_true_str.as_ptr(),
                                       (*(*tok).pb).buf,
                                       size1 as std::os::raw::c_ulong) ==
                                   0 as std::os::raw::c_int {
                            if !((*tok).st_pos == json_true_str_len) {
                                current_block = 6143943233158298354;
                                break ;
                            }
                            let ref mut fresh9 =
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).current;
                            *fresh9 =
                                json_object_new_boolean(1 as std::os::raw::c_int);
                            if (*(*tok).stack.offset((*tok).depth as
                                                         isize)).current.is_null()
                               {
                                break 's_60 ;
                            }
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_finish;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_eatws
                        } else if (*tok).flags & 0x1 as std::os::raw::c_int == 0 &&
                                      strncasecmp(json_false_str.as_ptr(),
                                                  (*(*tok).pb).buf,
                                                  size2 as std::os::raw::c_ulong) ==
                                          0 as std::os::raw::c_int ||
                                      strncmp(json_false_str.as_ptr(),
                                              (*(*tok).pb).buf,
                                              size2 as std::os::raw::c_ulong) ==
                                          0 as std::os::raw::c_int {
                            if !((*tok).st_pos == json_false_str_len) {
                                current_block = 6143943233158298354;
                                break ;
                            }
                            let ref mut fresh10 =
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).current;
                            *fresh10 =
                                json_object_new_boolean(0 as std::os::raw::c_int);
                            if (*(*tok).stack.offset((*tok).depth as
                                                         isize)).current.is_null()
                               {
                                break 's_60 ;
                            }
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_finish;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_eatws
                        } else {
                            (*tok).err = json_tokener_error_parse_boolean;
                            break 's_60 ;
                        }
                    }
                    12 => {
                        /* Advance until we change state */
                        let mut case_start_2: *const std::os::raw::c_char = str;
                        let mut case_len: std::os::raw::c_int = 0 as std::os::raw::c_int;
                        let mut is_exponent: std::os::raw::c_int = 0 as std::os::raw::c_int;
                        let mut negativesign_next_possible_location:
                                std::os::raw::c_int = 1 as std::os::raw::c_int;
                        while c as std::os::raw::c_int != 0 &&
                                  !strchr(json_number_chars,
                                          c as std::os::raw::c_int).is_null() {
                            case_len += 1;
                            /* non-digit characters checks */
	  /* note: since the main loop condition to get here was
	           an input starting with 0-9 or '-', we are
	           protected from input starting with '.' or
	           e/E. */
                            if c as std::os::raw::c_int == '.' as i32 {
                                if (*tok).is_double != 0 as std::os::raw::c_int {
                                    /* '.' can only be found once, and out of the exponent part.
	         Thus, if the input is already flagged as double, it
	         is invalid. */
                                    (*tok).err =
                                        json_tokener_error_parse_number;
                                    break 's_60 ;
                                } else { (*tok).is_double = 1 as std::os::raw::c_int }
                            }
                            if c as std::os::raw::c_int == 'e' as i32 ||
                                   c as std::os::raw::c_int == 'E' as i32 {
                                if is_exponent != 0 as std::os::raw::c_int {
                                    /* only one exponent possible */
                                    (*tok).err =
                                        json_tokener_error_parse_number;
                                    break 's_60 ;
                                } else {
                                    is_exponent = 1 as std::os::raw::c_int;
                                    (*tok).is_double = 1 as std::os::raw::c_int;
                                    /* the exponent part can begin with a negative sign */
                                    negativesign_next_possible_location =
                                        case_len + 1 as std::os::raw::c_int
                                }
                            }
                            if c as std::os::raw::c_int == '-' as i32 &&
                                   case_len !=
                                       negativesign_next_possible_location {
                                /* If the negative sign is not where expected (ie
	       start of input or start of exponent part), the
	       input is invalid. */
                                (*tok).err = json_tokener_error_parse_number;
                                break 's_60 ;
                            } else {
                                str = str.offset(1);
                                (*tok).char_offset += 1;
                                if !(c == 0 ||
                                         (if (*tok).char_offset == len {
                                              (if (*tok).depth ==
                                                      0 as std::os::raw::c_int &&
                                                      (*(*tok).stack.offset((*tok).depth
                                                                                as
                                                                                isize)).state
                                                          as std::os::raw::c_uint ==
                                                          json_tokener_state_eatws
                                                              as std::os::raw::c_int
                                                              as std::os::raw::c_uint
                                                      &&
                                                      (*(*tok).stack.offset((*tok).depth
                                                                                as
                                                                                isize)).saved_state
                                                          as std::os::raw::c_uint ==
                                                          json_tokener_state_finish
                                                              as std::os::raw::c_int
                                                              as std::os::raw::c_uint
                                                  {
                                                   (*tok).err =
                                                       json_tokener_success;
                                                   0 as std::os::raw::c_int
                                               } else {
                                                   (*tok).err =
                                                       json_tokener_continue;
                                                   0 as std::os::raw::c_int
                                               })
                                          } else {
                                              c = *str;
                                              1 as std::os::raw::c_int
                                          }) == 0) {
                                    continue ;
                                }
                                if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                       case_len {
                                    memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                       as
                                                                       isize)
                                               as *mut libc::c_void,
                                           case_start_2 as
                                               *const libc::c_void,
                                           case_len as std::os::raw::c_ulong);
                                    (*(*tok).pb).bpos += case_len;
                                    *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                 as isize) =
                                        '\u{0}' as i32 as std::os::raw::c_char
                                } else {
                                    printbuf_memappend((*tok).pb,
                                                       case_start_2,
                                                       case_len);
                                }
                                break 's_60 ;
                            }
                        }
                        if case_len > 0 as std::os::raw::c_int {
                            if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                   case_len {
                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                   as isize)
                                           as *mut libc::c_void,
                                       case_start_2 as *const libc::c_void,
                                       case_len as std::os::raw::c_ulong);
                                (*(*tok).pb).bpos += case_len;
                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                             isize) =
                                    '\u{0}' as i32 as std::os::raw::c_char
                            } else {
                                printbuf_memappend((*tok).pb, case_start_2,
                                                   case_len);
                            }
                        }
                        // Check for -Infinity
                        if *(*(*tok).pb).buf.offset(0 as std::os::raw::c_int as isize)
                               as std::os::raw::c_int == '-' as i32 &&
                               case_len <= 1 as std::os::raw::c_int &&
                               (c as std::os::raw::c_int == 'i' as i32 ||
                                    c as std::os::raw::c_int == 'I' as i32) {
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_inf;
                            (*tok).st_pos = 0 as std::os::raw::c_int
                        } else {
                            let mut num64: int64_t = 0;
                            let mut numd: std::os::raw::c_double = 0.;
                            if (*tok).is_double == 0 &&
                                   json_parse_int64((*(*tok).pb).buf,
                                                    &mut num64) ==
                                       0 as std::os::raw::c_int {
                                if num64 != 0 &&
                                       *(*(*tok).pb).buf.offset(0 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                           as std::os::raw::c_int == '0' as i32 &&
                                       (*tok).flags & 0x1 as std::os::raw::c_int != 0
                                   {
                                    /* in strict mode, number must not start with 0 */
                                    (*tok).err =
                                        json_tokener_error_parse_number;
                                    break 's_60 ;
                                } else {
                                    let ref mut fresh11 =
                                        (*(*tok).stack.offset((*tok).depth as
                                                                  isize)).current;
                                    *fresh11 = json_object_new_int64(num64);
                                    if (*(*tok).stack.offset((*tok).depth as
                                                                 isize)).current.is_null()
                                       {
                                        break 's_60 ;
                                    }
                                }
                            } else if (*tok).is_double != 0 &&
                                          json_parse_double((*(*tok).pb).buf,
                                                            &mut numd) ==
                                              0 as std::os::raw::c_int {
                                let ref mut fresh12 =
                                    (*(*tok).stack.offset((*tok).depth as
                                                              isize)).current;
                                *fresh12 =
                                    json_object_new_double_s(numd,
                                                             (*(*tok).pb).buf);
                                if (*(*tok).stack.offset((*tok).depth as
                                                             isize)).current.is_null()
                                   {
                                    break 's_60 ;
                                }
                            } else {
                                (*tok).err = json_tokener_error_parse_number;
                                break 's_60 ;
                            }
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_finish;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_eatws
                        }
                    }
                    22 | 13 => {
                        if c as std::os::raw::c_int == ']' as i32 {
                            if (*(*tok).stack.offset((*tok).depth as
                                                         isize)).state as
                                   std::os::raw::c_uint ==
                                   json_tokener_state_array_after_sep as
                                       std::os::raw::c_int as std::os::raw::c_uint &&
                                   (*tok).flags & 0x1 as std::os::raw::c_int != 0 {
                                current_block = 7885421652216951207;
                                break ;
                            } else {
                                current_block = 10393664800653117347;
                                break ;
                            }
                        } else if (*tok).depth >=
                                      (*tok).max_depth - 1 as std::os::raw::c_int {
                            (*tok).err = json_tokener_error_depth;
                            break 's_60 ;
                        } else {
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_array_add;
                            (*tok).depth += 1;
                            json_tokener_reset_level(tok, (*tok).depth);
                        }
                    }
                    14 => {
                        if json_object_array_add((*(*tok).stack.offset((*tok).depth
                                                                           as
                                                                           isize)).current,
                                                 obj) != 0 as std::os::raw::c_int {
                            break 's_60 ;
                        }
                        (*(*tok).stack.offset((*tok).depth as
                                                  isize)).saved_state =
                            json_tokener_state_array_sep;
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws
                    }
                    15 => {
                        if c as std::os::raw::c_int == ']' as i32 {
                            current_block = 5717195992839464149;
                            break ;
                        } else {
                            current_block = 13722339513228526441;
                            break ;
                        }
                    }
                    16 | 23 => {
                        if c as std::os::raw::c_int == '}' as i32 {
                            current_block = 4985811629903272821;
                            break ;
                        } else {
                            current_block = 13712166503846696363;
                            break ;
                        }
                    }
                    17 => {
                        /* Advance until we change state */
                        case_start_3 = str;
                        current_block = 18400086188140786992;
                        break ;
                    }
                    18 => {
                        if c as std::os::raw::c_int == ':' as i32 {
                            current_block = 1795228135880752216;
                            break ;
                        } else {
                            current_block = 17038009265789171769;
                            break ;
                        }
                    }
                    19 => {
                        if (*tok).depth >= (*tok).max_depth - 1 as std::os::raw::c_int
                           {
                            (*tok).err = json_tokener_error_depth;
                            break 's_60 ;
                        } else {
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_object_value_add;
                            (*tok).depth += 1;
                            json_tokener_reset_level(tok, (*tok).depth);
                        }
                    }
                    20 => {
                        json_object_object_add((*(*tok).stack.offset((*tok).depth
                                                                         as
                                                                         isize)).current,
                                               (*(*tok).stack.offset((*tok).depth
                                                                         as
                                                                         isize)).obj_field_name,
                                               obj);
                        free((*(*tok).stack.offset((*tok).depth as
                                                       isize)).obj_field_name
                                 as *mut libc::c_void);
                        let ref mut fresh14 =
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).obj_field_name;
                        *fresh14 = 0 as *mut std::os::raw::c_char;
                        (*(*tok).stack.offset((*tok).depth as
                                                  isize)).saved_state =
                            json_tokener_state_object_sep;
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws
                    }
                    21 => {
                        /* { */
                        if c as std::os::raw::c_int == '}' as i32 {
                            current_block = 15340942182342897967;
                            break ;
                        } else {
                            current_block = 17487020865977263557;
                            break ;
                        }
                    }
                    _ => { current_block = 8178439852521326633; break ; }
                }
            }
            match current_block {
                18400086188140786992 => {
                    loop  {
                        if c as std::os::raw::c_int ==
                               (*tok).quote_char as std::os::raw::c_int {
                            if ((*(*tok).pb).size - (*(*tok).pb).bpos) as
                                   std::os::raw::c_long >
                                   str.offset_from(case_start_3) as
                                       std::os::raw::c_long {
                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                   as isize)
                                           as *mut libc::c_void,
                                       case_start_3 as *const libc::c_void,
                                       str.offset_from(case_start_3)
                                           as std::os::raw::c_long as std::os::raw::c_ulong);
                                (*(*tok).pb).bpos =
                                    ((*(*tok).pb).bpos as std::os::raw::c_long +
                                         str.offset_from(case_start_3)
                                             as std::os::raw::c_long) as std::os::raw::c_int;
                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                             isize) =
                                    '\u{0}' as i32 as std::os::raw::c_char
                            } else {
                                printbuf_memappend((*tok).pb, case_start_3,
                                                   str.offset_from(case_start_3)
                                                       as std::os::raw::c_long as
                                                       std::os::raw::c_int);
                            }
                            let ref mut fresh13 =
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).obj_field_name;
                            *fresh13 = strdup((*(*tok).pb).buf);
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_object_field_end;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_eatws;
                            break ;
                        } else if c as std::os::raw::c_int == '\\' as i32 {
                            if ((*(*tok).pb).size - (*(*tok).pb).bpos) as
                                   std::os::raw::c_long >
                                   str.offset_from(case_start_3) as
                                       std::os::raw::c_long {
                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                   as isize)
                                           as *mut libc::c_void,
                                       case_start_3 as *const libc::c_void,
                                       str.offset_from(case_start_3)
                                           as std::os::raw::c_long as std::os::raw::c_ulong);
                                (*(*tok).pb).bpos =
                                    ((*(*tok).pb).bpos as std::os::raw::c_long +
                                         str.offset_from(case_start_3)
                                             as std::os::raw::c_long) as std::os::raw::c_int;
                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                             isize) =
                                    '\u{0}' as i32 as std::os::raw::c_char
                            } else {
                                printbuf_memappend((*tok).pb, case_start_3,
                                                   str.offset_from(case_start_3)
                                                       as std::os::raw::c_long as
                                                       std::os::raw::c_int);
                            }
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_object_field;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_string_escape;
                            break ;
                        } else {
                            str = str.offset(1);
                            (*tok).char_offset += 1;
                            if !(c == 0 ||
                                     (if (*tok).char_offset == len {
                                          (if (*tok).depth == 0 as std::os::raw::c_int
                                                  &&
                                                  (*(*tok).stack.offset((*tok).depth
                                                                            as
                                                                            isize)).state
                                                      as std::os::raw::c_uint ==
                                                      json_tokener_state_eatws
                                                          as std::os::raw::c_int as
                                                          std::os::raw::c_uint &&
                                                  (*(*tok).stack.offset((*tok).depth
                                                                            as
                                                                            isize)).saved_state
                                                      as std::os::raw::c_uint ==
                                                      json_tokener_state_finish
                                                          as std::os::raw::c_int as
                                                          std::os::raw::c_uint {
                                               (*tok).err =
                                                   json_tokener_success;
                                               0 as std::os::raw::c_int
                                           } else {
                                               (*tok).err =
                                                   json_tokener_continue;
                                               0 as std::os::raw::c_int
                                           })
                                      } else { c = *str; 1 as std::os::raw::c_int })
                                         == 0) {
                                continue ;
                            }
                            if ((*(*tok).pb).size - (*(*tok).pb).bpos) as
                                   std::os::raw::c_long >
                                   str.offset_from(case_start_3) as
                                       std::os::raw::c_long {
                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                   as isize)
                                           as *mut libc::c_void,
                                       case_start_3 as *const libc::c_void,
                                       str.offset_from(case_start_3)
                                           as std::os::raw::c_long as std::os::raw::c_ulong);
                                (*(*tok).pb).bpos =
                                    ((*(*tok).pb).bpos as std::os::raw::c_long +
                                         str.offset_from(case_start_3)
                                             as std::os::raw::c_long) as std::os::raw::c_int;
                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                             isize) =
                                    '\u{0}' as i32 as std::os::raw::c_char
                            } else {
                                printbuf_memappend((*tok).pb, case_start_3,
                                                   str.offset_from(case_start_3)
                                                       as std::os::raw::c_long as
                                                       std::os::raw::c_int);
                            }
                            break 's_60 ;
                        }
                    }
                    current_block = 8178439852521326633;
                }
                17487020865977263557 => {
                    if c as std::os::raw::c_int == ',' as i32 {
                        (*(*tok).stack.offset((*tok).depth as
                                                  isize)).saved_state =
                            json_tokener_state_object_field_start_after_sep;
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws
                    } else {
                        (*tok).err =
                            json_tokener_error_parse_object_value_sep;
                        break ;
                    }
                    current_block = 8178439852521326633;
                }
                13712166503846696363 => {
                    if c as std::os::raw::c_int == '\"' as i32 ||
                           c as std::os::raw::c_int == '\'' as i32 {
                        (*tok).quote_char = c;
                        printbuf_reset((*tok).pb);
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_object_field
                    } else {
                        (*tok).err = json_tokener_error_parse_object_key_name;
                        break ;
                    }
                    current_block = 8178439852521326633;
                }
                4985811629903272821 => {
                    if (*(*tok).stack.offset((*tok).depth as isize)).state as
                           std::os::raw::c_uint ==
                           json_tokener_state_object_field_start_after_sep as
                               std::os::raw::c_int as std::os::raw::c_uint &&
                           (*tok).flags & 0x1 as std::os::raw::c_int != 0 {
                        (*tok).err = json_tokener_error_parse_unexpected;
                        break ;
                    } else {
                        (*(*tok).stack.offset((*tok).depth as
                                                  isize)).saved_state =
                            json_tokener_state_finish;
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws
                    }
                    current_block = 8178439852521326633;
                }
                13722339513228526441 => {
                    if c as std::os::raw::c_int == ',' as i32 {
                        (*(*tok).stack.offset((*tok).depth as
                                                  isize)).saved_state =
                            json_tokener_state_array_after_sep;
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws
                    } else {
                        (*tok).err = json_tokener_error_parse_array;
                        break ;
                    }
                    current_block = 8178439852521326633;
                }
                10393664800653117347 => {
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                        = json_tokener_state_finish;
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws;
                    current_block = 8178439852521326633;
                }
                15462824429697920828 => {
                    loop 
                         /* Handle a 4-byte sequence, or two sequences if a surrogate pair */
                         {
                        if c as std::os::raw::c_int != 0 &&
                               !strchr(json_hex_chars,
                                       c as std::os::raw::c_int).is_null() {
                            let fresh8 = (*tok).st_pos;
                            (*tok).st_pos = (*tok).st_pos + 1;
                            (*tok).ucs_char =
                                (*tok).ucs_char.wrapping_add(((if c as
                                                                      std::os::raw::c_int
                                                                      <=
                                                                      '9' as
                                                                          i32
                                                                  {
                                                                   (c as
                                                                        std::os::raw::c_int)
                                                                       -
                                                                       '0' as
                                                                           i32
                                                               } else {
                                                                   (c as
                                                                        std::os::raw::c_int
                                                                        &
                                                                        7 as
                                                                            std::os::raw::c_int)
                                                                       +
                                                                       9 as
                                                                           std::os::raw::c_int
                                                               }) as
                                                                  std::os::raw::c_uint)
                                                                 <<
                                                                 (3 as
                                                                      std::os::raw::c_int
                                                                      -
                                                                      fresh8)
                                                                     *
                                                                     4 as
                                                                         std::os::raw::c_int);
                            if (*tok).st_pos == 4 as std::os::raw::c_int {
                                let mut unescaped_utf: [std::os::raw::c_uchar; 4] =
                                    [0; 4];
                                if got_hi_surrogate != 0 {
                                    if (*tok).ucs_char &
                                           0xfc00 as std::os::raw::c_int as
                                               std::os::raw::c_uint ==
                                           0xdc00 as std::os::raw::c_int as
                                               std::os::raw::c_uint {
                                        /* Recalculate the ucs_char, then fall thru to process normally */
                                        (*tok).ucs_char =
                                            ((got_hi_surrogate &
                                                  0x3ff as std::os::raw::c_int as
                                                      std::os::raw::c_uint) <<
                                                 10 as
                                                     std::os::raw::c_int).wrapping_add((*tok).ucs_char
                                                                                   &
                                                                                   0x3ff
                                                                                       as
                                                                                       std::os::raw::c_int
                                                                                       as
                                                                                       std::os::raw::c_uint).wrapping_add(0x10000
                                                                                                                      as
                                                                                                                      std::os::raw::c_int
                                                                                                                      as
                                                                                                                      std::os::raw::c_uint)
                                    } else if (*(*tok).pb).size -
                                                  (*(*tok).pb).bpos >
                                                  3 as std::os::raw::c_int {
                                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                           as
                                                                           isize)
                                                   as *mut libc::c_void,
                                               utf8_replacement_char.as_mut_ptr()
                                                   as *mut std::os::raw::c_char as
                                                   *const libc::c_void,
                                               3 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
                                        (*(*tok).pb).bpos += 3 as std::os::raw::c_int;
                                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                     as isize)
                                            = '\u{0}' as i32 as std::os::raw::c_char
                                    } else {
                                        printbuf_memappend((*tok).pb,
                                                           utf8_replacement_char.as_mut_ptr()
                                                               as
                                                               *mut std::os::raw::c_char,
                                                           3 as std::os::raw::c_int);
                                    }
                                    got_hi_surrogate =
                                        0 as std::os::raw::c_int as std::os::raw::c_uint
                                }
                                if (*tok).ucs_char <
                                       0x80 as std::os::raw::c_int as std::os::raw::c_uint {
                                    unescaped_utf[0 as std::os::raw::c_int as usize] =
                                        (*tok).ucs_char as std::os::raw::c_uchar;
                                    if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                           1 as std::os::raw::c_int {
                                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                           as
                                                                           isize)
                                                   as *mut libc::c_void,
                                               unescaped_utf.as_mut_ptr() as
                                                   *mut std::os::raw::c_char as
                                                   *const libc::c_void,
                                               1 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
                                        (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                     as isize)
                                            = '\u{0}' as i32 as std::os::raw::c_char
                                    } else {
                                        printbuf_memappend((*tok).pb,
                                                           unescaped_utf.as_mut_ptr()
                                                               as
                                                               *mut std::os::raw::c_char,
                                                           1 as std::os::raw::c_int);
                                    }
                                } else if (*tok).ucs_char <
                                              0x800 as std::os::raw::c_int as
                                                  std::os::raw::c_uint {
                                    unescaped_utf[0 as std::os::raw::c_int as usize] =
                                        (0xc0 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char >>
                                                 6 as std::os::raw::c_int) as
                                            std::os::raw::c_uchar;
                                    unescaped_utf[1 as std::os::raw::c_int as usize] =
                                        (0x80 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char &
                                                 0x3f as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as
                                            std::os::raw::c_uchar;
                                    if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                           2 as std::os::raw::c_int {
                                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                           as
                                                                           isize)
                                                   as *mut libc::c_void,
                                               unescaped_utf.as_mut_ptr() as
                                                   *mut std::os::raw::c_char as
                                                   *const libc::c_void,
                                               2 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
                                        (*(*tok).pb).bpos += 2 as std::os::raw::c_int;
                                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                     as isize)
                                            = '\u{0}' as i32 as std::os::raw::c_char
                                    } else {
                                        printbuf_memappend((*tok).pb,
                                                           unescaped_utf.as_mut_ptr()
                                                               as
                                                               *mut std::os::raw::c_char,
                                                           2 as std::os::raw::c_int);
                                    }
                                } else if (*tok).ucs_char &
                                              0xfc00 as std::os::raw::c_int as
                                                  std::os::raw::c_uint ==
                                              0xd800 as std::os::raw::c_int as
                                                  std::os::raw::c_uint {
                                    /* Hi surrogate was not followed by a low surrogate */
                    /* Replace the hi and process the rest normally */
                                    /* Got a high surrogate.  Remember it and look for the
                   * the beginning of another sequence, which should be the
                   * low surrogate.
                   */
                                    got_hi_surrogate = (*tok).ucs_char;
                                    /* Not at end, and the next two chars should be "\u" */
                                    if (len == -(1 as std::os::raw::c_int) ||
                                            len >
                                                (*tok).char_offset +
                                                    2 as std::os::raw::c_int) &&
                                           *str.offset(1 as std::os::raw::c_int as
                                                           isize) as
                                               std::os::raw::c_int == '\\' as i32 &&
                                           *str.offset(2 as std::os::raw::c_int as
                                                           isize) as
                                               std::os::raw::c_int == 'u' as i32 {
                                        /* Advance through the 16 bit surrogate, and move on to the
                 * next sequence. The next step is to process the following
                 * characters.
                 */
                                        str = str.offset(1);
                                        (*tok).char_offset += 1;
                                        if c == 0 ||
                                               {
                                                   str = str.offset(1);
                                                   (*tok).char_offset += 1;
                                                   (c) == 0
                                               } {
                                            if (*(*tok).pb).size -
                                                   (*(*tok).pb).bpos >
                                                   3 as std::os::raw::c_int {
                                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                                   as
                                                                                   isize)
                                                           as
                                                           *mut libc::c_void,
                                                       utf8_replacement_char.as_mut_ptr()
                                                           as
                                                           *mut std::os::raw::c_char
                                                           as
                                                           *const libc::c_void,
                                                       3 as std::os::raw::c_int as
                                                           std::os::raw::c_ulong);
                                                (*(*tok).pb).bpos +=
                                                    3 as std::os::raw::c_int;
                                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                             as
                                                                             isize)
                                                    =
                                                    '\u{0}' as i32 as
                                                        std::os::raw::c_char
                                            } else {
                                                printbuf_memappend((*tok).pb,
                                                                   utf8_replacement_char.as_mut_ptr()
                                                                       as
                                                                       *mut std::os::raw::c_char,
                                                                   3 as
                                                                       std::os::raw::c_int);
                                            }
                                        }
                                        /* other json_tokener_state_escape_unicode */
                                        /* Advance to the first char of the next sequence and
                     * continue processing with the next sequence.
                     */
                                        str = str.offset(1);
                                        (*tok).char_offset += 1;
                                        if c == 0 ||
                                               (if (*tok).char_offset == len {
                                                    (if (*tok).depth ==
                                                            0 as std::os::raw::c_int
                                                            &&
                                                            (*(*tok).stack.offset((*tok).depth
                                                                                      as
                                                                                      isize)).state
                                                                as
                                                                std::os::raw::c_uint
                                                                ==
                                                                json_tokener_state_eatws
                                                                    as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint
                                                            &&
                                                            (*(*tok).stack.offset((*tok).depth
                                                                                      as
                                                                                      isize)).saved_state
                                                                as
                                                                std::os::raw::c_uint
                                                                ==
                                                                json_tokener_state_finish
                                                                    as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint
                                                        {
                                                         (*tok).err =
                                                             json_tokener_success;
                                                         0 as std::os::raw::c_int
                                                     } else {
                                                         (*tok).err =
                                                             json_tokener_continue;
                                                         0 as std::os::raw::c_int
                                                     })
                                                } else {
                                                    c = *str;
                                                    1 as std::os::raw::c_int
                                                }) == 0 {
                                            if (*(*tok).pb).size -
                                                   (*(*tok).pb).bpos >
                                                   3 as std::os::raw::c_int {
                                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                                   as
                                                                                   isize)
                                                           as
                                                           *mut libc::c_void,
                                                       utf8_replacement_char.as_mut_ptr()
                                                           as
                                                           *mut std::os::raw::c_char
                                                           as
                                                           *const libc::c_void,
                                                       3 as std::os::raw::c_int as
                                                           std::os::raw::c_ulong);
                                                (*(*tok).pb).bpos +=
                                                    3 as std::os::raw::c_int;
                                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                             as
                                                                             isize)
                                                    =
                                                    '\u{0}' as i32 as
                                                        std::os::raw::c_char
                                            } else {
                                                printbuf_memappend((*tok).pb,
                                                                   utf8_replacement_char.as_mut_ptr()
                                                                       as
                                                                       *mut std::os::raw::c_char,
                                                                   3 as
                                                                       std::os::raw::c_int);
                                            }
                                            break 's_60 ;
                                        } else {
                                            (*tok).ucs_char =
                                                0 as std::os::raw::c_int as
                                                    std::os::raw::c_uint;
                                            (*tok).st_pos = 0 as std::os::raw::c_int;
                                            continue ;
                                        }
                                    } else if (*(*tok).pb).size -
                                                  (*(*tok).pb).bpos >
                                                  3 as std::os::raw::c_int {
                                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                           as
                                                                           isize)
                                                   as *mut libc::c_void,
                                               utf8_replacement_char.as_mut_ptr()
                                                   as *mut std::os::raw::c_char as
                                                   *const libc::c_void,
                                               3 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
                                        (*(*tok).pb).bpos += 3 as std::os::raw::c_int;
                                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                     as isize)
                                            = '\u{0}' as i32 as std::os::raw::c_char
                                    } else {
                                        printbuf_memappend((*tok).pb,
                                                           utf8_replacement_char.as_mut_ptr()
                                                               as
                                                               *mut std::os::raw::c_char,
                                                           3 as std::os::raw::c_int);
                                    }
                                } else if (*tok).ucs_char &
                                              0xfc00 as std::os::raw::c_int as
                                                  std::os::raw::c_uint ==
                                              0xdc00 as std::os::raw::c_int as
                                                  std::os::raw::c_uint {
                                    /* Got a high surrogate without another sequence following
                     * it.  Put a replacement char in for the hi surrogate
                     * and pretend we finished.
                     */
                                    /* Got a low surrogate not preceded by a high */
                                    if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                           3 as std::os::raw::c_int {
                                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                           as
                                                                           isize)
                                                   as *mut libc::c_void,
                                               utf8_replacement_char.as_mut_ptr()
                                                   as *mut std::os::raw::c_char as
                                                   *const libc::c_void,
                                               3 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
                                        (*(*tok).pb).bpos += 3 as std::os::raw::c_int;
                                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                     as isize)
                                            = '\u{0}' as i32 as std::os::raw::c_char
                                    } else {
                                        printbuf_memappend((*tok).pb,
                                                           utf8_replacement_char.as_mut_ptr()
                                                               as
                                                               *mut std::os::raw::c_char,
                                                           3 as std::os::raw::c_int);
                                    }
                                } else if (*tok).ucs_char <
                                              0x10000 as std::os::raw::c_int as
                                                  std::os::raw::c_uint {
                                    unescaped_utf[0 as std::os::raw::c_int as usize] =
                                        (0xe0 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char >>
                                                 12 as std::os::raw::c_int) as
                                            std::os::raw::c_uchar;
                                    unescaped_utf[1 as std::os::raw::c_int as usize] =
                                        (0x80 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char >>
                                                 6 as std::os::raw::c_int &
                                                 0x3f as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as
                                            std::os::raw::c_uchar;
                                    unescaped_utf[2 as std::os::raw::c_int as usize] =
                                        (0x80 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char &
                                                 0x3f as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as
                                            std::os::raw::c_uchar;
                                    if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                           3 as std::os::raw::c_int {
                                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                           as
                                                                           isize)
                                                   as *mut libc::c_void,
                                               unescaped_utf.as_mut_ptr() as
                                                   *mut std::os::raw::c_char as
                                                   *const libc::c_void,
                                               3 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
                                        (*(*tok).pb).bpos += 3 as std::os::raw::c_int;
                                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                     as isize)
                                            = '\u{0}' as i32 as std::os::raw::c_char
                                    } else {
                                        printbuf_memappend((*tok).pb,
                                                           unescaped_utf.as_mut_ptr()
                                                               as
                                                               *mut std::os::raw::c_char,
                                                           3 as std::os::raw::c_int);
                                    }
                                } else if (*tok).ucs_char <
                                              0x110000 as std::os::raw::c_int as
                                                  std::os::raw::c_uint {
                                    unescaped_utf[0 as std::os::raw::c_int as usize] =
                                        (0xf0 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char >>
                                                 18 as std::os::raw::c_int &
                                                 0x7 as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as
                                            std::os::raw::c_uchar;
                                    unescaped_utf[1 as std::os::raw::c_int as usize] =
                                        (0x80 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char >>
                                                 12 as std::os::raw::c_int &
                                                 0x3f as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as
                                            std::os::raw::c_uchar;
                                    unescaped_utf[2 as std::os::raw::c_int as usize] =
                                        (0x80 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char >>
                                                 6 as std::os::raw::c_int &
                                                 0x3f as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as
                                            std::os::raw::c_uchar;
                                    unescaped_utf[3 as std::os::raw::c_int as usize] =
                                        (0x80 as std::os::raw::c_int as std::os::raw::c_uint |
                                             (*tok).ucs_char &
                                                 0x3f as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as
                                            std::os::raw::c_uchar;
                                    if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                           4 as std::os::raw::c_int {
                                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                           as
                                                                           isize)
                                                   as *mut libc::c_void,
                                               unescaped_utf.as_mut_ptr() as
                                                   *mut std::os::raw::c_char as
                                                   *const libc::c_void,
                                               4 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
                                        (*(*tok).pb).bpos += 4 as std::os::raw::c_int;
                                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                     as isize)
                                            = '\u{0}' as i32 as std::os::raw::c_char
                                    } else {
                                        printbuf_memappend((*tok).pb,
                                                           unescaped_utf.as_mut_ptr()
                                                               as
                                                               *mut std::os::raw::c_char,
                                                           4 as std::os::raw::c_int);
                                    }
                                } else if (*(*tok).pb).size -
                                              (*(*tok).pb).bpos >
                                              3 as std::os::raw::c_int {
                                    memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                       as
                                                                       isize)
                                               as *mut libc::c_void,
                                           utf8_replacement_char.as_mut_ptr()
                                               as *mut std::os::raw::c_char as
                                               *const libc::c_void,
                                           3 as std::os::raw::c_int as std::os::raw::c_ulong);
                                    (*(*tok).pb).bpos += 3 as std::os::raw::c_int;
                                    *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                 as isize) =
                                        '\u{0}' as i32 as std::os::raw::c_char
                                } else {
                                    printbuf_memappend((*tok).pb,
                                                       utf8_replacement_char.as_mut_ptr()
                                                           as
                                                           *mut std::os::raw::c_char,
                                                       3 as std::os::raw::c_int);
                                }
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).state =
                                    (*(*tok).stack.offset((*tok).depth as
                                                              isize)).saved_state;
                                break ;
                            } else {
                                str = str.offset(1);
                                (*tok).char_offset += 1;
                                if !(c == 0 ||
                                         (if (*tok).char_offset == len {
                                              (if (*tok).depth ==
                                                      0 as std::os::raw::c_int &&
                                                      (*(*tok).stack.offset((*tok).depth
                                                                                as
                                                                                isize)).state
                                                          as std::os::raw::c_uint ==
                                                          json_tokener_state_eatws
                                                              as std::os::raw::c_int
                                                              as std::os::raw::c_uint
                                                      &&
                                                      (*(*tok).stack.offset((*tok).depth
                                                                                as
                                                                                isize)).saved_state
                                                          as std::os::raw::c_uint ==
                                                          json_tokener_state_finish
                                                              as std::os::raw::c_int
                                                              as std::os::raw::c_uint
                                                  {
                                                   (*tok).err =
                                                       json_tokener_success;
                                                   0 as std::os::raw::c_int
                                               } else {
                                                   (*tok).err =
                                                       json_tokener_continue;
                                                   0 as std::os::raw::c_int
                                               })
                                          } else {
                                              c = *str;
                                              1 as std::os::raw::c_int
                                          }) == 0) {
                                    continue ;
                                }
                                if got_hi_surrogate != 0 {
                                    /* Don't know what we got--insert the replacement char */
                                    /* Clean up any pending chars */
                                    if (*(*tok).pb).size - (*(*tok).pb).bpos >
                                           3 as std::os::raw::c_int {
                                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                           as
                                                                           isize)
                                                   as *mut libc::c_void,
                                               utf8_replacement_char.as_mut_ptr()
                                                   as *mut std::os::raw::c_char as
                                                   *const libc::c_void,
                                               3 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong);
                                        (*(*tok).pb).bpos += 3 as std::os::raw::c_int;
                                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                     as isize)
                                            = '\u{0}' as i32 as std::os::raw::c_char
                                    } else {
                                        printbuf_memappend((*tok).pb,
                                                           utf8_replacement_char.as_mut_ptr()
                                                               as
                                                               *mut std::os::raw::c_char,
                                                           3 as std::os::raw::c_int);
                                    }
                                }
                                break 's_60 ;
                            }
                        } else {
                            (*tok).err = json_tokener_error_parse_string;
                            break 's_60 ;
                        }
                    }
                    current_block = 8178439852521326633;
                }
                7391758747053199081 => {
                    (*tok).ucs_char = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                    (*tok).st_pos = 0 as std::os::raw::c_int;
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_escape_unicode;
                    current_block = 8178439852521326633;
                }
                484703659935464883 => {
                    if c as std::os::raw::c_int == 'b' as i32 {
                        if (*(*tok).pb).size - (*(*tok).pb).bpos >
                               1 as std::os::raw::c_int {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   b"\x08\x00" as *const u8 as
                                       *const std::os::raw::c_char as
                                       *const libc::c_void,
                                   1 as std::os::raw::c_int as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb,
                                               b"\x08\x00" as *const u8 as
                                                   *const std::os::raw::c_char,
                                               1 as std::os::raw::c_int);
                        }
                    } else if c as std::os::raw::c_int == 'n' as i32 {
                        if (*(*tok).pb).size - (*(*tok).pb).bpos >
                               1 as std::os::raw::c_int {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   b"\n\x00" as *const u8 as
                                       *const std::os::raw::c_char as
                                       *const libc::c_void,
                                   1 as std::os::raw::c_int as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb,
                                               b"\n\x00" as *const u8 as
                                                   *const std::os::raw::c_char,
                                               1 as std::os::raw::c_int);
                        }
                    } else if c as std::os::raw::c_int == 'r' as i32 {
                        if (*(*tok).pb).size - (*(*tok).pb).bpos >
                               1 as std::os::raw::c_int {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   b"\r\x00" as *const u8 as
                                       *const std::os::raw::c_char as
                                       *const libc::c_void,
                                   1 as std::os::raw::c_int as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb,
                                               b"\r\x00" as *const u8 as
                                                   *const std::os::raw::c_char,
                                               1 as std::os::raw::c_int);
                        }
                    } else if c as std::os::raw::c_int == 't' as i32 {
                        if (*(*tok).pb).size - (*(*tok).pb).bpos >
                               1 as std::os::raw::c_int {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   b"\t\x00" as *const u8 as
                                       *const std::os::raw::c_char as
                                       *const libc::c_void,
                                   1 as std::os::raw::c_int as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb,
                                               b"\t\x00" as *const u8 as
                                                   *const std::os::raw::c_char,
                                               1 as std::os::raw::c_int);
                        }
                    } else if c as std::os::raw::c_int == 'f' as i32 {
                        if (*(*tok).pb).size - (*(*tok).pb).bpos >
                               1 as std::os::raw::c_int {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   b"\x0c\x00" as *const u8 as
                                       *const std::os::raw::c_char as
                                       *const libc::c_void,
                                   1 as std::os::raw::c_int as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb,
                                               b"\x0c\x00" as *const u8 as
                                                   *const std::os::raw::c_char,
                                               1 as std::os::raw::c_int);
                        }
                    }
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        (*(*tok).stack.offset((*tok).depth as
                                                  isize)).saved_state;
                    current_block = 8178439852521326633;
                }
                3988677119214552796 => {
                    if (*(*tok).pb).size - (*(*tok).pb).bpos >
                           1 as std::os::raw::c_int {
                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                           isize) as
                                   *mut libc::c_void,
                               &mut c as *mut std::os::raw::c_char as
                                   *const libc::c_void,
                               1 as std::os::raw::c_int as std::os::raw::c_ulong);
                        (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as std::os::raw::c_char
                    } else {
                        printbuf_memappend((*tok).pb, &mut c,
                                           1 as std::os::raw::c_int);
                    }
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        (*(*tok).stack.offset((*tok).depth as
                                                  isize)).saved_state;
                    current_block = 8178439852521326633;
                }
                13434751124187322381 => {
                    loop  {
                        if c as std::os::raw::c_int ==
                               (*tok).quote_char as std::os::raw::c_int {
                            if ((*(*tok).pb).size - (*(*tok).pb).bpos) as
                                   std::os::raw::c_long >
                                   str.offset_from(case_start_1) as
                                       std::os::raw::c_long {
                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                   as isize)
                                           as *mut libc::c_void,
                                       case_start_1 as *const libc::c_void,
                                       str.offset_from(case_start_1)
                                           as std::os::raw::c_long as std::os::raw::c_ulong);
                                (*(*tok).pb).bpos =
                                    ((*(*tok).pb).bpos as std::os::raw::c_long +
                                         str.offset_from(case_start_1)
                                             as std::os::raw::c_long) as std::os::raw::c_int;
                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                             isize) =
                                    '\u{0}' as i32 as std::os::raw::c_char
                            } else {
                                printbuf_memappend((*tok).pb, case_start_1,
                                                   str.offset_from(case_start_1)
                                                       as std::os::raw::c_long as
                                                       std::os::raw::c_int);
                            }
                            let ref mut fresh7 =
                                (*(*tok).stack.offset((*tok).depth as
                                                          isize)).current;
                            *fresh7 =
                                json_object_new_string_len((*(*tok).pb).buf,
                                                           (*(*tok).pb).bpos);
                            if (*(*tok).stack.offset((*tok).depth as
                                                         isize)).current.is_null()
                               {
                                break 's_60 ;
                            }
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_finish;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_eatws;
                            break ;
                        } else if c as std::os::raw::c_int == '\\' as i32 {
                            if ((*(*tok).pb).size - (*(*tok).pb).bpos) as
                                   std::os::raw::c_long >
                                   str.offset_from(case_start_1) as
                                       std::os::raw::c_long {
                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                   as isize)
                                           as *mut libc::c_void,
                                       case_start_1 as *const libc::c_void,
                                       str.offset_from(case_start_1)
                                           as std::os::raw::c_long as std::os::raw::c_ulong);
                                (*(*tok).pb).bpos =
                                    ((*(*tok).pb).bpos as std::os::raw::c_long +
                                         str.offset_from(case_start_1)
                                             as std::os::raw::c_long) as std::os::raw::c_int;
                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                             isize) =
                                    '\u{0}' as i32 as std::os::raw::c_char
                            } else {
                                printbuf_memappend((*tok).pb, case_start_1,
                                                   str.offset_from(case_start_1)
                                                       as std::os::raw::c_long as
                                                       std::os::raw::c_int);
                            }
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).saved_state =
                                json_tokener_state_string;
                            (*(*tok).stack.offset((*tok).depth as
                                                      isize)).state =
                                json_tokener_state_string_escape;
                            break ;
                        } else {
                            str = str.offset(1);
                            (*tok).char_offset += 1;
                            if !(c == 0 ||
                                     (if (*tok).char_offset == len {
                                          (if (*tok).depth == 0 as std::os::raw::c_int
                                                  &&
                                                  (*(*tok).stack.offset((*tok).depth
                                                                            as
                                                                            isize)).state
                                                      as std::os::raw::c_uint ==
                                                      json_tokener_state_eatws
                                                          as std::os::raw::c_int as
                                                          std::os::raw::c_uint &&
                                                  (*(*tok).stack.offset((*tok).depth
                                                                            as
                                                                            isize)).saved_state
                                                      as std::os::raw::c_uint ==
                                                      json_tokener_state_finish
                                                          as std::os::raw::c_int as
                                                          std::os::raw::c_uint {
                                               (*tok).err =
                                                   json_tokener_success;
                                               0 as std::os::raw::c_int
                                           } else {
                                               (*tok).err =
                                                   json_tokener_continue;
                                               0 as std::os::raw::c_int
                                           })
                                      } else { c = *str; 1 as std::os::raw::c_int })
                                         == 0) {
                                continue ;
                            }
                            if ((*(*tok).pb).size - (*(*tok).pb).bpos) as
                                   std::os::raw::c_long >
                                   str.offset_from(case_start_1) as
                                       std::os::raw::c_long {
                                memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                                   as isize)
                                           as *mut libc::c_void,
                                       case_start_1 as *const libc::c_void,
                                       str.offset_from(case_start_1)
                                           as std::os::raw::c_long as std::os::raw::c_ulong);
                                (*(*tok).pb).bpos =
                                    ((*(*tok).pb).bpos as std::os::raw::c_long +
                                         str.offset_from(case_start_1)
                                             as std::os::raw::c_long) as std::os::raw::c_int;
                                *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                             isize) =
                                    '\u{0}' as i32 as std::os::raw::c_char
                            } else {
                                printbuf_memappend((*tok).pb, case_start_1,
                                                   str.offset_from(case_start_1)
                                                       as std::os::raw::c_long as
                                                       std::os::raw::c_int);
                            }
                            break 's_60 ;
                        }
                    }
                    current_block = 8178439852521326633;
                }
                10253503901371725554 => {
                    while c as std::os::raw::c_int != '\n' as i32 {
                        str = str.offset(1);
                        (*tok).char_offset += 1;
                        if !(c == 0 ||
                                 (if (*tok).char_offset == len {
                                      (if (*tok).depth == 0 as std::os::raw::c_int &&
                                              (*(*tok).stack.offset((*tok).depth
                                                                        as
                                                                        isize)).state
                                                  as std::os::raw::c_uint ==
                                                  json_tokener_state_eatws as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint &&
                                              (*(*tok).stack.offset((*tok).depth
                                                                        as
                                                                        isize)).saved_state
                                                  as std::os::raw::c_uint ==
                                                  json_tokener_state_finish as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint {
                                           (*tok).err = json_tokener_success;
                                           0 as std::os::raw::c_int
                                       } else {
                                           (*tok).err = json_tokener_continue;
                                           0 as std::os::raw::c_int
                                       })
                                  } else { c = *str; 1 as std::os::raw::c_int }) == 0)
                           {
                            continue ;
                        }
                        if ((*(*tok).pb).size - (*(*tok).pb).bpos) as
                               std::os::raw::c_long >
                               str.offset_from(case_start_0) as
                                   std::os::raw::c_long {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   case_start_0 as *const libc::c_void,
                                   str.offset_from(case_start_0) as
                                       std::os::raw::c_long as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos =
                                ((*(*tok).pb).bpos as std::os::raw::c_long +
                                     str.offset_from(case_start_0) as
                                         std::os::raw::c_long) as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb, case_start_0,
                                               str.offset_from(case_start_0)
                                                   as std::os::raw::c_long as
                                                   std::os::raw::c_int);
                        }
                        break 's_60 ;
                    }
                    if ((*(*tok).pb).size - (*(*tok).pb).bpos) as std::os::raw::c_long
                           >
                           str.offset_from(case_start_0) as
                               std::os::raw::c_long {
                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                           isize) as
                                   *mut libc::c_void,
                               case_start_0 as *const libc::c_void,
                               str.offset_from(case_start_0) as
                                   std::os::raw::c_long as std::os::raw::c_ulong);
                        (*(*tok).pb).bpos =
                            ((*(*tok).pb).bpos as std::os::raw::c_long +
                                 str.offset_from(case_start_0) as
                                     std::os::raw::c_long) as std::os::raw::c_int;
                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as std::os::raw::c_char
                    } else {
                        printbuf_memappend((*tok).pb, case_start_0,
                                           str.offset_from(case_start_0)
                                               as std::os::raw::c_long as
                                               std::os::raw::c_int);
                    }
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws;
                    current_block = 8178439852521326633;
                }
                15321816652064063775 => {
                    while c as std::os::raw::c_int != '*' as i32 {
                        str = str.offset(1);
                        (*tok).char_offset += 1;
                        if !(c == 0 ||
                                 (if (*tok).char_offset == len {
                                      (if (*tok).depth == 0 as std::os::raw::c_int &&
                                              (*(*tok).stack.offset((*tok).depth
                                                                        as
                                                                        isize)).state
                                                  as std::os::raw::c_uint ==
                                                  json_tokener_state_eatws as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint &&
                                              (*(*tok).stack.offset((*tok).depth
                                                                        as
                                                                        isize)).saved_state
                                                  as std::os::raw::c_uint ==
                                                  json_tokener_state_finish as
                                                      std::os::raw::c_int as
                                                      std::os::raw::c_uint {
                                           (*tok).err = json_tokener_success;
                                           0 as std::os::raw::c_int
                                       } else {
                                           (*tok).err = json_tokener_continue;
                                           0 as std::os::raw::c_int
                                       })
                                  } else { c = *str; 1 as std::os::raw::c_int }) == 0)
                           {
                            continue ;
                        }
                        if ((*(*tok).pb).size - (*(*tok).pb).bpos) as
                               std::os::raw::c_long >
                               str.offset_from(case_start) as
                                   std::os::raw::c_long {
                            memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos
                                                               as isize) as
                                       *mut libc::c_void,
                                   case_start as *const libc::c_void,
                                   str.offset_from(case_start) as
                                       std::os::raw::c_long as std::os::raw::c_ulong);
                            (*(*tok).pb).bpos =
                                ((*(*tok).pb).bpos as std::os::raw::c_long +
                                     str.offset_from(case_start) as
                                         std::os::raw::c_long) as std::os::raw::c_int;
                            *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                         isize) =
                                '\u{0}' as i32 as std::os::raw::c_char
                        } else {
                            printbuf_memappend((*tok).pb, case_start,
                                               str.offset_from(case_start)
                                                   as std::os::raw::c_long as
                                                   std::os::raw::c_int);
                        }
                        break 's_60 ;
                    }
                    if ((*(*tok).pb).size - (*(*tok).pb).bpos) as std::os::raw::c_long
                           >
                           str.offset(1 as std::os::raw::c_int as
                                          isize).offset_from(case_start)
                               as std::os::raw::c_long {
                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                           isize) as
                                   *mut libc::c_void,
                               case_start as *const libc::c_void,
                               str.offset(1 as std::os::raw::c_int as
                                              isize).offset_from(case_start)
                                   as std::os::raw::c_long as std::os::raw::c_ulong);
                        (*(*tok).pb).bpos =
                            ((*(*tok).pb).bpos as std::os::raw::c_long +
                                 str.offset(1 as std::os::raw::c_int as
                                                isize).offset_from(case_start)
                                     as std::os::raw::c_long) as std::os::raw::c_int;
                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as std::os::raw::c_char
                    } else {
                        printbuf_memappend((*tok).pb, case_start,
                                           str.offset(1 as std::os::raw::c_int as
                                                          isize).offset_from(case_start)
                                               as std::os::raw::c_long as
                                               std::os::raw::c_int);
                    }
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_comment_end;
                    current_block = 8178439852521326633;
                }
                15417752026496523887 => {
                    if c as std::os::raw::c_int == '/' as i32 {
                        (*(*tok).stack.offset((*tok).depth as isize)).state =
                            json_tokener_state_comment_eol
                    } else {
                        (*tok).err = json_tokener_error_parse_comment;
                        break ;
                    }
                    current_block = 10468276026569382870;
                }
                8892199680835157453 =>
                /* FALLTHRU */
                {
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_string;
                    printbuf_reset((*tok).pb);
                    (*tok).quote_char = c;
                    current_block = 8178439852521326633;
                }
                7885421652216951207 => {
                    (*tok).err = json_tokener_error_parse_unexpected;
                    break ;
                }
                1604201581803946138 => {
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_comment;
                    current_block = 10468276026569382870;
                }
                7415988906898333153 => {
                    (*tok).err = json_tokener_error_parse_string;
                    break ;
                }
                17038009265789171769 => {
                    (*tok).err = json_tokener_error_parse_object_key_sep;
                    break ;
                }
                15340942182342897967 => {
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                        = json_tokener_state_finish;
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws;
                    current_block = 8178439852521326633;
                }
                1795228135880752216 => {
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                        = json_tokener_state_object_value;
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws;
                    current_block = 8178439852521326633;
                }
                11763295167351361500 => {
                    /* in STRICT mode only double-quote are allowed */
                    (*tok).err = json_tokener_error_parse_unexpected;
                    break ;
                }
                5717195992839464149 => {
                    (*(*tok).stack.offset((*tok).depth as isize)).saved_state
                        = json_tokener_state_finish;
                    (*(*tok).stack.offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws;
                    current_block = 8178439852521326633;
                }
                9508502311343125869 => {
                    (*tok).st_pos += 1;
                    current_block = 8178439852521326633;
                }
                6143943233158298354 => {
                    (*tok).st_pos += 1;
                    current_block = 8178439852521326633;
                }
                _ => { }
            }
            match current_block {
                10468276026569382870 => {
                    if (*(*tok).pb).size - (*(*tok).pb).bpos >
                           1 as std::os::raw::c_int {
                        memcpy((*(*tok).pb).buf.offset((*(*tok).pb).bpos as
                                                           isize) as
                                   *mut libc::c_void,
                               &mut c as *mut std::os::raw::c_char as
                                   *const libc::c_void,
                               1 as std::os::raw::c_int as std::os::raw::c_ulong);
                        (*(*tok).pb).bpos += 1 as std::os::raw::c_int;
                        *(*(*tok).pb).buf.offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as std::os::raw::c_char
                    } else {
                        printbuf_memappend((*tok).pb, &mut c,
                                           1 as std::os::raw::c_int);
                    }
                }
                _ => { }
            }
            str = str.offset(1);
            (*tok).char_offset += 1;
            if c == 0 { break ; }
        }
    /* out of input chars, for now at least */
    if c as std::os::raw::c_int != 0 &&
           (*(*tok).stack.offset((*tok).depth as isize)).state as std::os::raw::c_uint
               == json_tokener_state_finish as std::os::raw::c_int as std::os::raw::c_uint &&
           (*tok).depth == 0 as std::os::raw::c_int &&
           (*tok).flags & 0x1 as std::os::raw::c_int != 0 {
        /* unexpected char after JSON data */
        (*tok).err = json_tokener_error_parse_unexpected
    }
    if c == 0 {
        /* We hit an eof char (0) */
        if (*(*tok).stack.offset((*tok).depth as isize)).state as std::os::raw::c_uint
               != json_tokener_state_finish as std::os::raw::c_int as std::os::raw::c_uint &&
               (*(*tok).stack.offset((*tok).depth as isize)).saved_state as
                   std::os::raw::c_uint !=
                   json_tokener_state_finish as std::os::raw::c_int as std::os::raw::c_uint {
            (*tok).err = json_tokener_error_parse_eof
        }
    }
    setlocale(4 as std::os::raw::c_int, oldlocale);
    free(oldlocale as *mut libc::c_void);
    if (*tok).err as std::os::raw::c_uint ==
           json_tokener_success as std::os::raw::c_int as std::os::raw::c_uint {
        let mut ret: *mut json_object =
            json_object_get((*(*tok).stack.offset((*tok).depth as
                                                      isize)).current);
        let mut ii: std::os::raw::c_int = 0;
        /* Partially reset, so we parse additional objects on subsequent calls. */
        ii = (*tok).depth;
        while ii >= 0 as std::os::raw::c_int {
            json_tokener_reset_level(tok, ii);
            ii -= 1
        }
        return ret
    }
    return 0 as *mut json_object;
}
#[no_mangle]
pub unsafe extern "C" fn json_tokener_set_flags(mut tok: *mut json_tokener,
                                                mut flags: std::os::raw::c_int) {
    (*tok).flags = flags;
}
unsafe extern "C" fn run_static_initializers() {
    json_null_str_len =
        (::std::mem::size_of::<[std::os::raw::c_char; 5]>() as
             std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int;
    json_inf_str_len =
        (::std::mem::size_of::<[std::os::raw::c_char; 9]>() as
             std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_uint;
    json_nan_str_len =
        (::std::mem::size_of::<[std::os::raw::c_char; 4]>() as
             std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int;
    json_true_str_len =
        (::std::mem::size_of::<[std::os::raw::c_char; 5]>() as
             std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int;
    json_false_str_len =
        (::std::mem::size_of::<[std::os::raw::c_char; 6]>() as
             std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
