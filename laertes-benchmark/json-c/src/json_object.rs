#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, linkage,
           ptr_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn isdigit(_c: std::os::raw::c_int) -> std::os::raw::c_int ;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: std::os::raw::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcat(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strdup(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn printbuf_new() -> *mut printbuf;
    #[no_mangle]
    fn printbuf_memappend(p: *mut printbuf, buf: *const std::os::raw::c_char,
                          size: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn printbuf_memset(pb: *mut printbuf, offset: std::os::raw::c_int,
                       charvalue: std::os::raw::c_int, len: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn printbuf_reset(p: *mut printbuf);
    #[no_mangle]
    fn printbuf_free(p: *mut printbuf);
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
    #[no_mangle]
    fn snprintf(_: *mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strtod(_: *const std::os::raw::c_char, _: *mut *mut std::os::raw::c_char)
     -> std::os::raw::c_double;
    #[no_mangle]
    fn lh_kchar_table_new(size: std::os::raw::c_int,
                          free_fn: Option<lh_entry_free_fn>) -> *mut lh_table;
    #[no_mangle]
    fn lh_table_free(t: *mut lh_table);
    #[no_mangle]
    fn lh_table_insert_w_hash(t: *mut lh_table, k: *const libc::c_void,
                              v: *const libc::c_void, h: std::os::raw::c_ulong,
                              opts: std::os::raw::c_uint) -> std::os::raw::c_int;
    #[no_mangle]
    fn lh_table_lookup_entry_w_hash(t: *mut lh_table, k: *const libc::c_void,
                                    h: std::os::raw::c_ulong) -> *mut lh_entry;
    #[no_mangle]
    fn lh_table_lookup_ex(t: *mut lh_table, k: *const libc::c_void,
                          v: *mut *mut libc::c_void) -> json_bool;
    #[no_mangle]
    fn lh_table_delete(t: *mut lh_table, k: *const libc::c_void)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn lh_table_length(t: *mut lh_table) -> std::os::raw::c_int;
    #[no_mangle]
    fn array_list_new(free_fn: Option<array_list_free_fn>) -> *mut array_list;
    #[no_mangle]
    fn array_list_free(al: *mut array_list);
    #[no_mangle]
    fn array_list_get_idx(al: *mut array_list, i: size_t)
     -> *mut libc::c_void;
    #[no_mangle]
    fn array_list_put_idx(al: *mut array_list, i: size_t,
                          data: *mut libc::c_void) -> std::os::raw::c_int;
    #[no_mangle]
    fn array_list_add(al: *mut array_list, data: *mut libc::c_void)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn array_list_length(al: *mut array_list) -> size_t;
    #[no_mangle]
    fn array_list_sort(arr: *mut array_list,
                       compar:
                           Option<unsafe extern "C" fn(_: *const libc::c_void,
                                                       _: *const libc::c_void)
                                      -> std::os::raw::c_int>);
    #[no_mangle]
    fn array_list_bsearch(key: *mut *const libc::c_void, arr: *mut array_list,
                          sort_fn:
                              Option<unsafe extern "C" fn(_:
                                                              *const libc::c_void,
                                                          _:
                                                              *const libc::c_void)
                                         -> std::os::raw::c_int>)
     -> *mut libc::c_void;
    #[no_mangle]
    fn array_list_del_idx(arr: *mut array_list, idx: size_t, count: size_t)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn _json_c_set_last_err(err_fmt: *const std::os::raw::c_char, _: ...);
    #[no_mangle]
    fn json_parse_int64(buf: *const std::os::raw::c_char, retval: *mut int64_t)
     -> std::os::raw::c_int;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __darwin_ct_rune_t = std::os::raw::c_int;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_wchar_t = std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type int32_t = std::os::raw::c_int;
pub type int64_t = std::os::raw::c_longlong;
pub type uintptr_t = std::os::raw::c_ulong;
pub type size_t = __darwin_size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut std::os::raw::c_char,
    pub bpos: std::os::raw::c_int,
    pub size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_iter {
    pub key: *mut std::os::raw::c_char,
    pub val: *mut json_object,
    pub entry: *mut lh_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_entry {
    pub k: *const libc::c_void,
    pub k_is_constant: std::os::raw::c_int,
    pub v: *const libc::c_void,
    pub next: *mut lh_entry,
    pub prev: *mut lh_entry,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_list {
    pub array: *mut *mut libc::c_void,
    pub length: size_t,
    pub size: size_t,
    pub free_fn: Option<array_list_free_fn>,
}
pub type array_list_free_fn
    =
    unsafe extern "C" fn(_: *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_table {
    pub size: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
    pub head: *mut lh_entry,
    pub tail: *mut lh_entry,
    pub table: *mut lh_entry,
    pub free_fn: Option<lh_entry_free_fn>,
    pub hash_fn: Option<lh_hash_fn>,
    pub equal_fn: Option<lh_equal_fn>,
}
pub type lh_equal_fn
    =
    unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void)
        -> std::os::raw::c_int;
pub type lh_hash_fn
    =
    unsafe extern "C" fn(_: *const libc::c_void) -> std::os::raw::c_ulong;
pub type lh_entry_free_fn = unsafe extern "C" fn(_: *mut lh_entry) -> ();
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
pub struct _RuneCharClass {
    pub __name: [std::os::raw::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: std::os::raw::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
pub type json_c_shallow_copy_fn
    =
    unsafe extern "C" fn(_: *mut json_object, _: *mut json_object,
                         _: *const std::os::raw::c_char, _: size_t,
                         _: *mut *mut json_object) -> std::os::raw::c_int;
#[inline]
unsafe extern "C" fn __isctype(mut _c: __darwin_ct_rune_t,
                               mut _f: std::os::raw::c_ulong) -> __darwin_ct_rune_t {
    return if _c < 0 as std::os::raw::c_int ||
                  _c >= (1 as std::os::raw::c_int) << 8 as std::os::raw::c_int {
               0 as std::os::raw::c_int
           } else {
               (_DefaultRuneLocale.__runetype[_c as usize] as std::os::raw::c_ulong &
                    _f != 0) as std::os::raw::c_int
           };
}
#[inline(always)]
unsafe extern "C" fn __inline_isinff(mut __x: std::os::raw::c_float) -> std::os::raw::c_int {
    return (__x.abs() == ::std::f32::INFINITY) as std::os::raw::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isinfd(mut __x: std::os::raw::c_double) -> std::os::raw::c_int {
    return (__x.abs() == ::std::f64::INFINITY) as std::os::raw::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnanf(mut __x: std::os::raw::c_float) -> std::os::raw::c_int {
    return (__x != __x) as std::os::raw::c_int;
}
#[inline(always)]
unsafe extern "C" fn __inline_isnand(mut __x: std::os::raw::c_double) -> std::os::raw::c_int {
    return (__x != __x) as std::os::raw::c_int;
}
#[inline]
unsafe extern "C" fn lh_get_hash(mut t: *const lh_table,
                                 mut k: *const libc::c_void)
 -> std::os::raw::c_ulong {
    return (*t).hash_fn.expect("non-null function pointer")(k);
}
/*
 * $Id: json_object.c,v 1.17 2006/07/25 03:24:50 mclark Exp $
 *
 * Copyright (c) 2004, 2005 Metaparadigm Pte. Ltd.
 * Michael Clark <michael@metaparadigm.com>
 * Copyright (c) 2009 Hewlett-Packard Development Company, L.P.
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 *
 */
// Don't define this.  It's not thread-safe.
/* #define REFCOUNT_DEBUG 1 */
#[no_mangle]
pub static mut json_number_chars: *const std::os::raw::c_char =
    b"0123456789.+-eE\x00" as *const u8 as *const std::os::raw::c_char;
#[no_mangle]
pub static mut json_hex_chars: *const std::os::raw::c_char =
    b"0123456789abcdefABCDEF\x00" as *const u8 as *const std::os::raw::c_char;
/* ref count debugging */
/* REFCOUNT_DEBUG */
/* helper for accessing the optimized string data component in json_object
 */
unsafe extern "C" fn get_string_component(mut jso: *const json_object)
 -> *const std::os::raw::c_char {
    return if (*jso).o.c_string.len < 32 as std::os::raw::c_int {
               (*jso).o.c_string.str_0.data.as_ptr()
           } else { (*jso).o.c_string.str_0.ptr as *const std::os::raw::c_char };
}
/* string escaping */
unsafe extern "C" fn json_escape_str(mut pb: *mut printbuf,
                                     mut str: *const std::os::raw::c_char,
                                     mut len: std::os::raw::c_int,
                                     mut flags: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut pos: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut start_offset: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut c: std::os::raw::c_uchar = 0;
    loop  {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 != 0) { break ; }
        c = *str.offset(pos as isize) as std::os::raw::c_uchar;
        match c as std::os::raw::c_int {
            8 | 10 | 13 | 9 | 12 | 34 | 92 | 47 => {
                if flags & (1 as std::os::raw::c_int) << 4 as std::os::raw::c_int != 0 &&
                       c as std::os::raw::c_int == '/' as i32 {
                    pos += 1
                } else {
                    if pos - start_offset > 0 as std::os::raw::c_int {
                        printbuf_memappend(pb,
                                           str.offset(start_offset as isize),
                                           pos - start_offset);
                    }
                    if c as std::os::raw::c_int == '\u{8}' as i32 {
                        printbuf_memappend(pb,
                                           b"\\b\x00" as *const u8 as
                                               *const std::os::raw::c_char,
                                           2 as std::os::raw::c_int);
                    } else if c as std::os::raw::c_int == '\n' as i32 {
                        printbuf_memappend(pb,
                                           b"\\n\x00" as *const u8 as
                                               *const std::os::raw::c_char,
                                           2 as std::os::raw::c_int);
                    } else if c as std::os::raw::c_int == '\r' as i32 {
                        printbuf_memappend(pb,
                                           b"\\r\x00" as *const u8 as
                                               *const std::os::raw::c_char,
                                           2 as std::os::raw::c_int);
                    } else if c as std::os::raw::c_int == '\t' as i32 {
                        printbuf_memappend(pb,
                                           b"\\t\x00" as *const u8 as
                                               *const std::os::raw::c_char,
                                           2 as std::os::raw::c_int);
                    } else if c as std::os::raw::c_int == '\u{c}' as i32 {
                        printbuf_memappend(pb,
                                           b"\\f\x00" as *const u8 as
                                               *const std::os::raw::c_char,
                                           2 as std::os::raw::c_int);
                    } else if c as std::os::raw::c_int == '\"' as i32 {
                        printbuf_memappend(pb,
                                           b"\\\"\x00" as *const u8 as
                                               *const std::os::raw::c_char,
                                           2 as std::os::raw::c_int);
                    } else if c as std::os::raw::c_int == '\\' as i32 {
                        printbuf_memappend(pb,
                                           b"\\\\\x00" as *const u8 as
                                               *const std::os::raw::c_char,
                                           2 as std::os::raw::c_int);
                    } else if c as std::os::raw::c_int == '/' as i32 {
                        printbuf_memappend(pb,
                                           b"\\/\x00" as *const u8 as
                                               *const std::os::raw::c_char,
                                           2 as std::os::raw::c_int);
                    }
                    pos += 1;
                    start_offset = pos
                }
            }
            _ => {
                if (c as std::os::raw::c_int) < ' ' as i32 {
                    let mut sbuf: [std::os::raw::c_char; 7] = [0; 7];
                    if pos - start_offset > 0 as std::os::raw::c_int {
                        printbuf_memappend(pb,
                                           str.offset(start_offset as isize),
                                           pos - start_offset);
                    }
                    snprintf(sbuf.as_mut_ptr(),
                             ::std::mem::size_of::<[std::os::raw::c_char; 7]>() as
                                 std::os::raw::c_ulong,
                             b"\\u00%c%c\x00" as *const u8 as
                                 *const std::os::raw::c_char,
                             *json_hex_chars.offset((c as std::os::raw::c_int >>
                                                         4 as std::os::raw::c_int) as
                                                        isize) as std::os::raw::c_int,
                             *json_hex_chars.offset((c as std::os::raw::c_int &
                                                         0xf as std::os::raw::c_int)
                                                        as isize) as
                                 std::os::raw::c_int);
                    if (*pb).size - (*pb).bpos >
                           ::std::mem::size_of::<[std::os::raw::c_char; 7]>() as
                               std::os::raw::c_ulong as std::os::raw::c_int - 1 as std::os::raw::c_int
                       {
                        memcpy((*pb).buf.offset((*pb).bpos as isize) as
                                   *mut libc::c_void,
                               sbuf.as_mut_ptr() as *const libc::c_void,
                               (::std::mem::size_of::<[std::os::raw::c_char; 7]>() as
                                    std::os::raw::c_ulong as std::os::raw::c_int -
                                    1 as std::os::raw::c_int) as std::os::raw::c_ulong);
                        (*pb).bpos +=
                            ::std::mem::size_of::<[std::os::raw::c_char; 7]>() as
                                std::os::raw::c_ulong as std::os::raw::c_int -
                                1 as std::os::raw::c_int;
                        *(*pb).buf.offset((*pb).bpos as isize) =
                            '\u{0}' as i32 as std::os::raw::c_char
                    } else {
                        printbuf_memappend(pb, sbuf.as_mut_ptr(),
                                           ::std::mem::size_of::<[std::os::raw::c_char; 7]>()
                                               as std::os::raw::c_ulong as std::os::raw::c_int
                                               - 1 as std::os::raw::c_int);
                    }
                    pos += 1;
                    start_offset = pos
                } else { pos += 1 }
            }
        }
    }
    if pos - start_offset > 0 as std::os::raw::c_int {
        printbuf_memappend(pb, str.offset(start_offset as isize),
                           pos - start_offset);
    }
    return 0 as std::os::raw::c_int;
}
/* reference counting */
#[no_mangle]
pub unsafe extern "C" fn json_object_get(mut jso: *mut json_object)
 -> *mut json_object {
    if jso.is_null() { return jso }
    (*jso)._ref_count += 1;
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_put(mut jso: *mut json_object)
 -> std::os::raw::c_int {
    if jso.is_null() { return 0 as std::os::raw::c_int }
    /* Avoid invalid free and crash explicitly instead of (silently)
	 * segfaulting.
	 */
    if !((*jso)._ref_count > 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long
           != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 16],
                                               &[std::os::raw::c_char; 16]>(b"json_object_put\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     189 as std::os::raw::c_int,
                     b"jso->_ref_count > 0\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    (*jso)._ref_count -= 1;
    if (*jso)._ref_count > 0 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    if (*jso)._user_delete.is_some() {
        (*jso)._user_delete.expect("non-null function pointer")(jso,
                                                                (*jso)._userdata);
    }
    (*jso)._delete.expect("non-null function pointer")(jso);
    return 1 as std::os::raw::c_int;
}
/* generic object construction and destruction parts */
unsafe extern "C" fn json_object_generic_delete(mut jso: *mut json_object) {
    /* REFCOUNT_DEBUG */
    printbuf_free((*jso)._pb);
    free(jso as *mut libc::c_void);
}
unsafe extern "C" fn json_object_new(mut o_type: json_type)
 -> *mut json_object {
    let mut jso: *mut json_object = 0 as *mut json_object;
    jso =
        calloc(::std::mem::size_of::<json_object>() as std::os::raw::c_ulong,
               1 as std::os::raw::c_int as std::os::raw::c_ulong) as *mut json_object;
    if jso.is_null() { return 0 as *mut json_object }
    (*jso).o_type = o_type;
    (*jso)._ref_count = 1 as std::os::raw::c_int;
    (*jso)._delete =
        Some(json_object_generic_delete as
                 unsafe extern "C" fn(_: *mut json_object) -> ());
    /* REFCOUNT_DEBUG */
    return jso;
}
/* type checking functions */
#[no_mangle]
pub unsafe extern "C" fn json_object_is_type(mut jso: *const json_object,
                                             mut type_0: json_type)
 -> std::os::raw::c_int {
    if jso.is_null() {
        return (type_0 as std::os::raw::c_uint ==
                    json_type_null as std::os::raw::c_int as std::os::raw::c_uint) as
                   std::os::raw::c_int
    }
    return ((*jso).o_type as std::os::raw::c_uint == type_0 as std::os::raw::c_uint) as
               std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_type(mut jso: *const json_object)
 -> json_type {
    if jso.is_null() { return json_type_null }
    return (*jso).o_type;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_userdata(mut jso: *mut json_object)
 -> *mut libc::c_void {
    return if !jso.is_null() {
               (*jso)._userdata
           } else { 0 as *mut libc::c_void };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_userdata(mut jso: *mut json_object,
                                                  mut userdata:
                                                      *mut libc::c_void,
                                                  mut user_delete:
                                                      Option<json_object_delete_fn>) {
    // Can't return failure, so abort if we can't perform the operation.
    if jso.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 25],
                                               &[std::os::raw::c_char; 25]>(b"json_object_set_userdata\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     265 as std::os::raw::c_int,
                     b"jso != NULL\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    // First, clean up any previously existing user info
    if (*jso)._user_delete.is_some() {
        (*jso)._user_delete.expect("non-null function pointer")(jso,
                                                                (*jso)._userdata);
    }
    (*jso)._userdata = userdata;
    (*jso)._user_delete = user_delete;
}
/* set a custom conversion to string */
#[no_mangle]
pub unsafe extern "C" fn json_object_set_serializer(mut jso: *mut json_object,
                                                    mut to_string_func:
                                                        Option<json_object_to_json_string_fn>,
                                                    mut userdata:
                                                        *mut libc::c_void,
                                                    mut user_delete:
                                                        Option<json_object_delete_fn>) {
    json_object_set_userdata(jso, userdata, user_delete);
    if to_string_func.is_none() {
        // Reset to the standard serialization function
        match (*jso).o_type as std::os::raw::c_uint {
            0 => { (*jso)._to_json_string = None }
            1 => {
                (*jso)._to_json_string =
                    Some(json_object_boolean_to_json_string as
                             json_object_to_json_string_fn)
            }
            2 => {
                (*jso)._to_json_string =
                    Some(json_object_double_to_json_string_default as
                             json_object_to_json_string_fn)
            }
            3 => {
                (*jso)._to_json_string =
                    Some(json_object_int_to_json_string as
                             json_object_to_json_string_fn)
            }
            4 => {
                (*jso)._to_json_string =
                    Some(json_object_object_to_json_string as
                             json_object_to_json_string_fn)
            }
            5 => {
                (*jso)._to_json_string =
                    Some(json_object_array_to_json_string as
                             json_object_to_json_string_fn)
            }
            6 => {
                (*jso)._to_json_string =
                    Some(json_object_string_to_json_string as
                             json_object_to_json_string_fn)
            }
            _ => { }
        }
        return
    }
    (*jso)._to_json_string = to_string_func;
}
/* extended conversion to string */
#[no_mangle]
pub unsafe extern "C" fn json_object_to_json_string_length(mut jso:
                                                               *mut json_object,
                                                           mut flags:
                                                               std::os::raw::c_int,
                                                           mut length:
                                                               *mut size_t)
 -> *const std::os::raw::c_char {
    let mut r: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut s: size_t = 0 as std::os::raw::c_int as size_t;
    if jso.is_null() {
        s = 4 as std::os::raw::c_int as size_t;
        r = b"null\x00" as *const u8 as *const std::os::raw::c_char
    } else if !(*jso)._pb.is_null() ||
                  { (*jso)._pb = printbuf_new(); !(*jso)._pb.is_null() } {
        printbuf_reset((*jso)._pb);
        if (*jso)._to_json_string.expect("non-null function pointer")(jso,
                                                                      (*jso)._pb,
                                                                      0 as
                                                                          std::os::raw::c_int,
                                                                      flags)
               >= 0 as std::os::raw::c_int {
            s = (*(*jso)._pb).bpos as size_t;
            r = (*(*jso)._pb).buf
        }
    }
    if !length.is_null() { *length = s }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_to_json_string_ext(mut jso:
                                                            *mut json_object,
                                                        mut flags:
                                                            std::os::raw::c_int)
 -> *const std::os::raw::c_char {
    return json_object_to_json_string_length(jso, flags, 0 as *mut size_t);
}
/* backwards-compatible conversion to string */
#[no_mangle]
pub unsafe extern "C" fn json_object_to_json_string(mut jso: *mut json_object)
 -> *const std::os::raw::c_char {
    return json_object_to_json_string_ext(jso,
                                          (1 as std::os::raw::c_int) <<
                                              0 as std::os::raw::c_int);
}
unsafe extern "C" fn indent(mut pb: *mut printbuf, mut level: std::os::raw::c_int,
                            mut flags: std::os::raw::c_int) {
    if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
        if flags & (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int != 0 {
            printbuf_memset(pb, -(1 as std::os::raw::c_int), '\t' as i32, level);
        } else {
            printbuf_memset(pb, -(1 as std::os::raw::c_int), ' ' as i32,
                            level * 2 as std::os::raw::c_int);
        }
    };
}
/* json_object_object */
unsafe extern "C" fn json_object_object_to_json_string(mut jso:
                                                           *mut json_object,
                                                       mut pb: *mut printbuf,
                                                       mut level: std::os::raw::c_int,
                                                       mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut had_children: std::os::raw::c_int = 0 as std::os::raw::c_int; /*}*/
    let mut iter: json_object_iter =
        json_object_iter{key: 0 as *mut std::os::raw::c_char,
                         val: 0 as *mut json_object,
                         entry: 0 as *mut lh_entry,};
    printbuf_memappend(pb, b"{\x00" as *const u8 as *const std::os::raw::c_char,
                       (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                            std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                           std::os::raw::c_int);
    if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
        printbuf_memappend(pb, b"\n\x00" as *const u8 as *const std::os::raw::c_char,
                           (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_ulong)
                               as std::os::raw::c_int);
    }
    iter.entry = (*json_object_get_object(jso)).head;
    while !if !iter.entry.is_null() {
               iter.key =
                   (*iter.entry).k as uintptr_t as *mut libc::c_void as
                       *mut std::os::raw::c_char;
               iter.val =
                   (*iter.entry).v as uintptr_t as *mut libc::c_void as
                       *mut json_object;
               iter.entry
           } else { 0 as *mut lh_entry }.is_null() {
        if had_children != 0 {
            printbuf_memappend(pb,
                               b",\x00" as *const u8 as *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
            if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
                printbuf_memappend(pb,
                                   b"\n\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   (::std::mem::size_of::<[std::os::raw::c_char; 2]>()
                                        as
                                        std::os::raw::c_ulong).wrapping_sub(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_ulong)
                                       as std::os::raw::c_int);
            }
        }
        had_children = 1 as std::os::raw::c_int;
        if flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
            printbuf_memappend(pb,
                               b" \x00" as *const u8 as *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
        }
        indent(pb, level + 1 as std::os::raw::c_int, flags);
        printbuf_memappend(pb, b"\"\x00" as *const u8 as *const std::os::raw::c_char,
                           (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_ulong)
                               as std::os::raw::c_int);
        json_escape_str(pb, iter.key, strlen(iter.key) as std::os::raw::c_int, flags);
        if flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
            printbuf_memappend(pb,
                               b"\": \x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 4]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
        } else {
            printbuf_memappend(pb,
                               b"\":\x00" as *const u8 as *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 3]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
        }
        if iter.val.is_null() {
            printbuf_memappend(pb,
                               b"null\x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 5]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
        } else if (*iter.val)._to_json_string.expect("non-null function pointer")(iter.val,
                                                                                  pb,
                                                                                  level
                                                                                      +
                                                                                      1
                                                                                          as
                                                                                          std::os::raw::c_int,
                                                                                  flags)
                      < 0 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
        iter.entry = (*iter.entry).next
    }
    if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
        if had_children != 0 {
            printbuf_memappend(pb,
                               b"\n\x00" as *const u8 as *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
        }
        indent(pb, level, flags);
    }
    if flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
        return printbuf_memappend(pb,
                                  b" }\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  (::std::mem::size_of::<[std::os::raw::c_char; 3]>()
                                       as
                                       std::os::raw::c_ulong).wrapping_sub(1 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       std::os::raw::c_ulong)
                                      as std::os::raw::c_int)
    } else {
        return printbuf_memappend(pb,
                                  b"}\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  (::std::mem::size_of::<[std::os::raw::c_char; 2]>()
                                       as
                                       std::os::raw::c_ulong).wrapping_sub(1 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       std::os::raw::c_ulong)
                                      as std::os::raw::c_int)
    };
}
unsafe extern "C" fn json_object_lh_entry_free(mut ent: *mut lh_entry) {
    if (*ent).k_is_constant == 0 {
        free((*ent).k as uintptr_t as *mut libc::c_void);
    }
    json_object_put((*ent).v as uintptr_t as *mut libc::c_void as
                        *mut json_object);
}
unsafe extern "C" fn json_object_object_delete(mut jso: *mut json_object) {
    lh_table_free((*jso).o.c_object);
    json_object_generic_delete(jso);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_object() -> *mut json_object {
    let mut jso: *mut json_object = json_object_new(json_type_object);
    if jso.is_null() { return 0 as *mut json_object }
    (*jso)._delete =
        Some(json_object_object_delete as
                 unsafe extern "C" fn(_: *mut json_object) -> ());
    (*jso)._to_json_string =
        Some(json_object_object_to_json_string as
                 json_object_to_json_string_fn);
    (*jso).o.c_object =
        lh_kchar_table_new(16 as std::os::raw::c_int,
                           Some(json_object_lh_entry_free as
                                    unsafe extern "C" fn(_: *mut lh_entry)
                                        -> ()));
    if (*jso).o.c_object.is_null() {
        json_object_generic_delete(jso);
        *__error() = 12 as std::os::raw::c_int;
        return 0 as *mut json_object
    }
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_object(mut jso: *const json_object)
 -> *mut lh_table {
    if jso.is_null() { return 0 as *mut lh_table }
    match (*jso).o_type as std::os::raw::c_uint {
        4 => { return (*jso).o.c_object }
        _ => { return 0 as *mut lh_table }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_add_ex(mut jso: *mut json_object,
                                                   key: *const std::os::raw::c_char,
                                                   val: *mut json_object,
                                                   opts: std::os::raw::c_uint)
 -> std::os::raw::c_int {
    let mut existing_value: *mut json_object = 0 as *mut json_object;
    let mut existing_entry: *mut lh_entry = 0 as *mut lh_entry;
    let mut hash: std::os::raw::c_ulong = 0;
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_object as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 26],
                                               &[std::os::raw::c_char; 26]>(b"json_object_object_add_ex\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     476 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_object\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    // We lookup the entry and replace the value, rather than just deleting
	// and re-adding it, so the existing key remains valid.
    hash = lh_get_hash((*jso).o.c_object, key as *const libc::c_void);
    existing_entry =
        if opts & ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int) as std::os::raw::c_uint !=
               0 {
            0 as *mut lh_entry
        } else {
            lh_table_lookup_entry_w_hash((*jso).o.c_object,
                                         key as *const libc::c_void, hash)
        };
    // The caller must avoid creating loops in the object tree, but do a
	// quick check anyway to make sure we're not creating a trivial loop.
    if jso == val { return -(1 as std::os::raw::c_int) }
    if existing_entry.is_null() {
        let k: *const libc::c_void =
            if opts & ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int) as std::os::raw::c_uint
                   != 0 {
                key as *const libc::c_void
            } else { strdup(key) as *const libc::c_void };
        if k.is_null() { return -(1 as std::os::raw::c_int) }
        return lh_table_insert_w_hash((*jso).o.c_object, k,
                                      val as *const libc::c_void, hash, opts)
    }
    existing_value =
        (*existing_entry).v as uintptr_t as *mut libc::c_void as
            *mut json_object;
    if !existing_value.is_null() { json_object_put(existing_value); }
    (*existing_entry).v = val as *const libc::c_void;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_add(mut jso: *mut json_object,
                                                mut key: *const std::os::raw::c_char,
                                                mut val: *mut json_object)
 -> std::os::raw::c_int {
    return json_object_object_add_ex(jso, key, val,
                                     0 as std::os::raw::c_int as std::os::raw::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_length(mut jso:
                                                       *const json_object)
 -> std::os::raw::c_int {
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_object as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 26],
                                               &[std::os::raw::c_char; 26]>(b"json_object_object_length\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     514 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_object\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    return lh_table_length((*jso).o.c_object);
}
#[no_mangle]
pub unsafe extern "C" fn json_c_object_sizeof() -> size_t {
    return ::std::mem::size_of::<json_object>() as std::os::raw::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_get(mut jso: *const json_object,
                                                mut key: *const std::os::raw::c_char)
 -> *mut json_object {
    let mut result: *mut json_object = 0 as *mut json_object;
    json_object_object_get_ex(jso, key, &mut result);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_get_ex(mut jso:
                                                       *const json_object,
                                                   mut key:
                                                       *const std::os::raw::c_char,
                                                   mut value:
                                                       *mut *mut json_object)
 -> json_bool {
    if !value.is_null() { *value = 0 as *mut json_object }
    if jso.is_null() { return 0 as std::os::raw::c_int }
    match (*jso).o_type as std::os::raw::c_uint {
        4 => {
            return lh_table_lookup_ex((*jso).o.c_object,
                                      key as *const libc::c_void,
                                      value as *mut *mut libc::c_void)
        }
        _ => {
            if !value.is_null() { *value = 0 as *mut json_object }
            return 0 as std::os::raw::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_object_del(mut jso: *mut json_object,
                                                mut key:
                                                    *const std::os::raw::c_char) {
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_object as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 23],
                                               &[std::os::raw::c_char; 23]>(b"json_object_object_del\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     554 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_object\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    lh_table_delete((*jso).o.c_object, key as *const libc::c_void);
}
/* json_object_boolean */
unsafe extern "C" fn json_object_boolean_to_json_string(mut jso:
                                                            *mut json_object,
                                                        mut pb: *mut printbuf,
                                                        mut level:
                                                            std::os::raw::c_int,
                                                        mut flags:
                                                            std::os::raw::c_int)
 -> std::os::raw::c_int {
    if (*jso).o.c_boolean != 0 {
        return printbuf_memappend(pb,
                                  b"true\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  (::std::mem::size_of::<[std::os::raw::c_char; 5]>()
                                       as
                                       std::os::raw::c_ulong).wrapping_sub(1 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       std::os::raw::c_ulong)
                                      as std::os::raw::c_int)
    }
    return printbuf_memappend(pb,
                              b"false\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              (::std::mem::size_of::<[std::os::raw::c_char; 6]>() as
                                   std::os::raw::c_ulong).wrapping_sub(1 as
                                                                   std::os::raw::c_int
                                                                   as
                                                                   std::os::raw::c_ulong)
                                  as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_boolean(mut b: json_bool)
 -> *mut json_object {
    let mut jso: *mut json_object = json_object_new(json_type_boolean);
    if jso.is_null() { return 0 as *mut json_object }
    (*jso)._to_json_string =
        Some(json_object_boolean_to_json_string as
                 json_object_to_json_string_fn);
    (*jso).o.c_boolean = b;
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_boolean(mut jso: *const json_object)
 -> json_bool {
    if jso.is_null() { return 0 as std::os::raw::c_int }
    match (*jso).o_type as std::os::raw::c_uint {
        1 => { return (*jso).o.c_boolean }
        3 => {
            return ((*jso).o.c_int64 != 0 as std::os::raw::c_int as std::os::raw::c_longlong)
                       as std::os::raw::c_int
        }
        2 => {
            return ((*jso).o.c_double != 0 as std::os::raw::c_int as std::os::raw::c_double)
                       as std::os::raw::c_int
        }
        6 => {
            return ((*jso).o.c_string.len != 0 as std::os::raw::c_int) as std::os::raw::c_int
        }
        _ => { return 0 as std::os::raw::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_boolean(mut jso: *mut json_object,
                                                 mut new_value: json_bool)
 -> std::os::raw::c_int {
    if jso.is_null() ||
           (*jso).o_type as std::os::raw::c_uint !=
               json_type_boolean as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    (*jso).o.c_boolean = new_value;
    return 1 as std::os::raw::c_int;
}
/* json_object_int */
unsafe extern "C" fn json_object_int_to_json_string(mut jso: *mut json_object,
                                                    mut pb: *mut printbuf,
                                                    mut level: std::os::raw::c_int,
                                                    mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    /* room for 19 digits, the sign char, and a null term */
    let mut sbuf: [std::os::raw::c_char; 21] = [0; 21];
    snprintf(sbuf.as_mut_ptr(),
             ::std::mem::size_of::<[std::os::raw::c_char; 21]>() as std::os::raw::c_ulong,
             b"%lld\x00" as *const u8 as *const std::os::raw::c_char,
             (*jso).o.c_int64);
    return printbuf_memappend(pb, sbuf.as_mut_ptr(),
                              strlen(sbuf.as_mut_ptr()) as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_int(mut i: int32_t)
 -> *mut json_object {
    let mut jso: *mut json_object = json_object_new(json_type_int);
    if jso.is_null() { return 0 as *mut json_object }
    (*jso)._to_json_string =
        Some(json_object_int_to_json_string as json_object_to_json_string_fn);
    (*jso).o.c_int64 = i as int64_t;
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_int(mut jso: *const json_object)
 -> int32_t {
    let mut cint64: int64_t = 0;
    let mut o_type: json_type = json_type_null;
    if jso.is_null() { return 0 as std::os::raw::c_int }
    o_type = (*jso).o_type;
    cint64 = (*jso).o.c_int64;
    if o_type as std::os::raw::c_uint ==
           json_type_string as std::os::raw::c_int as std::os::raw::c_uint {
        /*
	 * Parse strings into 64-bit numbers, then use the
	 * 64-to-32-bit number handling below.
	 */
        if json_parse_int64(get_string_component(jso), &mut cint64) !=
               0 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int
        } /* whoops, it didn't work. */
        o_type = json_type_int
    }
    match o_type as std::os::raw::c_uint {
        3 => {
            /* Make sure we return the correct values for out of range numbers. */
            if cint64 <=
                   (-(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int) as
                       std::os::raw::c_longlong {
                return -(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int
            }
            if cint64 >= 2147483647 as std::os::raw::c_int as std::os::raw::c_longlong {
                return 2147483647 as std::os::raw::c_int
            }
            return cint64 as int32_t
        }
        2 => {
            if (*jso).o.c_double <=
                   (-(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int) as
                       std::os::raw::c_double {
                return -(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int
            }
            if (*jso).o.c_double >=
                   2147483647 as std::os::raw::c_int as std::os::raw::c_double {
                return 2147483647 as std::os::raw::c_int
            }
            return (*jso).o.c_double as int32_t
        }
        1 => { return (*jso).o.c_boolean }
        _ => { return 0 as std::os::raw::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_int(mut jso: *mut json_object,
                                             mut new_value: std::os::raw::c_int)
 -> std::os::raw::c_int {
    if jso.is_null() ||
           (*jso).o_type as std::os::raw::c_uint !=
               json_type_int as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    (*jso).o.c_int64 = new_value as int64_t;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_int64(mut i: int64_t)
 -> *mut json_object {
    let mut jso: *mut json_object = json_object_new(json_type_int);
    if jso.is_null() { return 0 as *mut json_object }
    (*jso)._to_json_string =
        Some(json_object_int_to_json_string as json_object_to_json_string_fn);
    (*jso).o.c_int64 = i;
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_int64(mut jso: *const json_object)
 -> int64_t {
    let mut cint: int64_t = 0;
    if jso.is_null() { return 0 as std::os::raw::c_int as int64_t }
    match (*jso).o_type as std::os::raw::c_uint {
        3 => { return (*jso).o.c_int64 }
        2 => {
            if (*jso).o.c_double >=
                   9223372036854775807 as std::os::raw::c_longlong as std::os::raw::c_double {
                return 9223372036854775807 as std::os::raw::c_longlong
            }
            if (*jso).o.c_double <=
                   (-(9223372036854775807 as std::os::raw::c_longlong) -
                        1 as std::os::raw::c_int as std::os::raw::c_longlong) as
                       std::os::raw::c_double {
                return -(9223372036854775807 as std::os::raw::c_longlong) -
                           1 as std::os::raw::c_int as std::os::raw::c_longlong
            }
            return (*jso).o.c_double as int64_t
        }
        1 => { return (*jso).o.c_boolean as int64_t }
        6 => {
            if json_parse_int64(get_string_component(jso), &mut cint) ==
                   0 as std::os::raw::c_int {
                return cint
            }
        }
        _ => { }
    }
    /* FALLTHRU */
    return 0 as std::os::raw::c_int as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_int64(mut jso: *mut json_object,
                                               mut new_value: int64_t)
 -> std::os::raw::c_int {
    if jso.is_null() ||
           (*jso).o_type as std::os::raw::c_uint !=
               json_type_int as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    (*jso).o.c_int64 = new_value;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_int_inc(mut jso: *mut json_object,
                                             mut val: int64_t)
 -> std::os::raw::c_int {
    if jso.is_null() ||
           (*jso).o_type as std::os::raw::c_uint !=
               json_type_int as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if val > 0 as std::os::raw::c_int as std::os::raw::c_longlong &&
           (*jso).o.c_int64 > 9223372036854775807 as std::os::raw::c_longlong - val {
        (*jso).o.c_int64 = 9223372036854775807 as std::os::raw::c_longlong
    } else if val < 0 as std::os::raw::c_int as std::os::raw::c_longlong &&
                  (*jso).o.c_int64 <
                      -(9223372036854775807 as std::os::raw::c_longlong) -
                          1 as std::os::raw::c_int as std::os::raw::c_longlong - val {
        (*jso).o.c_int64 =
            -(9223372036854775807 as std::os::raw::c_longlong) -
                1 as std::os::raw::c_int as std::os::raw::c_longlong
    } else { (*jso).o.c_int64 += val }
    return 1 as std::os::raw::c_int;
}
/* json_object_double */
static mut global_serialization_float_format: *mut std::os::raw::c_char =
    0 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
#[no_mangle]
pub unsafe extern "C" fn json_c_set_serialization_double_format(mut double_format:
                                                                    *const std::os::raw::c_char,
                                                                mut global_or_thread:
                                                                    std::os::raw::c_int)
 -> std::os::raw::c_int {
    if global_or_thread == 0 as std::os::raw::c_int {
        if !global_serialization_float_format.is_null() {
            free(global_serialization_float_format as *mut libc::c_void);
        }
        global_serialization_float_format =
            if !double_format.is_null() {
                strdup(double_format)
            } else { 0 as *mut std::os::raw::c_char }
    } else if global_or_thread == 1 as std::os::raw::c_int {
        _json_c_set_last_err(b"json_c_set_option: not compiled with __thread support\n\x00"
                                 as *const u8 as *const std::os::raw::c_char);
        return -(1 as std::os::raw::c_int)
    } else {
        _json_c_set_last_err(b"json_c_set_option: invalid global_or_thread value: %d\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             global_or_thread);
        return -(1 as std::os::raw::c_int)
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn json_object_double_to_json_string_format(mut jso:
                                                                  *mut json_object,
                                                              mut pb:
                                                                  *mut printbuf,
                                                              mut level:
                                                                  std::os::raw::c_int,
                                                              mut flags:
                                                                  std::os::raw::c_int,
                                                              mut format:
                                                                  *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut buf: [std::os::raw::c_char; 128] = [0; 128];
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut size: std::os::raw::c_int = 0;
    /* Although JSON RFC does not support
	NaN or Infinity as numeric values
	ECMA 262 section 9.8.1 defines
	how to handle these cases as strings */
    if if ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong ==
              ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong {
           __inline_isnanf((*jso).o.c_double as std::os::raw::c_float)
       } else if ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong ==
                     ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong
        {
           __inline_isnand((*jso).o.c_double)
       } else { __inline_isnand((*jso).o.c_double) } != 0 {
        size =
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                         std::os::raw::c_ulong,
                     b"NaN\x00" as *const u8 as *const std::os::raw::c_char)
    } else if if ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong ==
                     ::std::mem::size_of::<std::os::raw::c_float>() as std::os::raw::c_ulong {
                  __inline_isinff((*jso).o.c_double as std::os::raw::c_float)
              } else if ::std::mem::size_of::<std::os::raw::c_double>() as
                            std::os::raw::c_ulong ==
                            ::std::mem::size_of::<std::os::raw::c_double>() as
                                std::os::raw::c_ulong {
                  __inline_isinfd((*jso).o.c_double)
              } else { __inline_isinfd((*jso).o.c_double) }
                  != 0 {
        if (*jso).o.c_double > 0 as std::os::raw::c_int as std::os::raw::c_double {
            size =
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                             std::os::raw::c_ulong,
                         b"Infinity\x00" as *const u8 as *const std::os::raw::c_char)
        } else {
            size =
                snprintf(buf.as_mut_ptr(),
                         ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                             std::os::raw::c_ulong,
                         b"-Infinity\x00" as *const u8 as *const std::os::raw::c_char)
        }
    } else {
        let mut std_format: *const std::os::raw::c_char =
            b"%.17g\x00" as *const u8 as *const std::os::raw::c_char;
        let mut format_drops_decimals: std::os::raw::c_int = 0 as std::os::raw::c_int;
        if format.is_null() {
            if !global_serialization_float_format.is_null() {
                format = global_serialization_float_format
            } else { format = std_format }
        }
        size =
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                         std::os::raw::c_ulong, format, (*jso).o.c_double);
        if size < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
        p = strchr(buf.as_mut_ptr(), ',' as i32);
        if !p.is_null() {
            *p = '.' as i32 as std::os::raw::c_char
        } else { p = strchr(buf.as_mut_ptr(), '.' as i32) }
        if format == std_format ||
               strstr(format,
                      b".0f\x00" as *const u8 as
                          *const std::os::raw::c_char).is_null() {
            format_drops_decimals = 1 as std::os::raw::c_int
        }
        if size <
               ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as std::os::raw::c_ulong
                   as std::os::raw::c_int - 2 as std::os::raw::c_int &&
               isdigit(buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int) != 0 &&
               p.is_null() && strchr(buf.as_mut_ptr(), 'e' as i32).is_null()
               && format_drops_decimals != 0 {
            // Ensure it looks like a float, even if snprintf didn't,
			//  unless a custom format is set to omit the decimal.
            strcat(buf.as_mut_ptr(),
                   b".0\x00" as *const u8 as *const std::os::raw::c_char);
            size += 2 as std::os::raw::c_int
        }
        if !p.is_null() && flags & (1 as std::os::raw::c_int) << 2 as std::os::raw::c_int != 0
           {
            /* last useful digit, always keep 1 zero */
            p = p.offset(1);
            q = p;
            while *q != 0 {
                if *q as std::os::raw::c_int != '0' as i32 { p = q }
                q = q.offset(1)
            }
            /* drop trailing zeroes */
            p = p.offset(1);
            *p = 0 as std::os::raw::c_int as std::os::raw::c_char;
            size =
                p.offset_from(buf.as_mut_ptr()) as std::os::raw::c_long as
                    std::os::raw::c_int
        }
    }
    // although unlikely, snprintf can fail
    if size < 0 as std::os::raw::c_int { return -(1 as std::os::raw::c_int) }
    if size >=
           ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as std::os::raw::c_ulong as
               std::os::raw::c_int {
        // The standard formats are guaranteed not to overrun the buffer,
		// but if a custom one happens to do so, just silently truncate.
        size =
            (::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                 std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as std::os::raw::c_int
    }
    printbuf_memappend(pb, buf.as_mut_ptr(), size);
    return size;
}
unsafe extern "C" fn json_object_double_to_json_string_default(mut jso:
                                                                   *mut json_object,
                                                               mut pb:
                                                                   *mut printbuf,
                                                               mut level:
                                                                   std::os::raw::c_int,
                                                               mut flags:
                                                                   std::os::raw::c_int)
 -> std::os::raw::c_int {
    return json_object_double_to_json_string_format(jso, pb, level, flags,
                                                    0 as *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_double_to_json_string(mut jso:
                                                               *mut json_object,
                                                           mut pb:
                                                               *mut printbuf,
                                                           mut level:
                                                               std::os::raw::c_int,
                                                           mut flags:
                                                               std::os::raw::c_int)
 -> std::os::raw::c_int {
    return json_object_double_to_json_string_format(jso, pb, level, flags,
                                                    (*jso)._userdata as
                                                        *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_double(mut d: std::os::raw::c_double)
 -> *mut json_object {
    let mut jso: *mut json_object = json_object_new(json_type_double);
    if jso.is_null() { return 0 as *mut json_object }
    (*jso)._to_json_string =
        Some(json_object_double_to_json_string_default as
                 json_object_to_json_string_fn);
    (*jso).o.c_double = d;
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_double_s(mut d: std::os::raw::c_double,
                                                  mut ds: *const std::os::raw::c_char)
 -> *mut json_object {
    let mut new_ds: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut jso: *mut json_object = json_object_new_double(d);
    if jso.is_null() { return 0 as *mut json_object }
    new_ds = strdup(ds);
    if new_ds.is_null() {
        json_object_generic_delete(jso);
        *__error() = 12 as std::os::raw::c_int;
        return 0 as *mut json_object
    }
    json_object_set_serializer(jso,
                               Some(json_object_userdata_to_json_string as
                                        json_object_to_json_string_fn),
                               new_ds as *mut libc::c_void,
                               Some(json_object_free_userdata as
                                        json_object_delete_fn));
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_userdata_to_json_string(mut jso:
                                                                 *mut json_object,
                                                             mut pb:
                                                                 *mut printbuf,
                                                             mut level:
                                                                 std::os::raw::c_int,
                                                             mut flags:
                                                                 std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut userdata_len: std::os::raw::c_int =
        strlen((*jso)._userdata as *const std::os::raw::c_char) as std::os::raw::c_int;
    printbuf_memappend(pb, (*jso)._userdata as *const std::os::raw::c_char,
                       userdata_len);
    return userdata_len;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_free_userdata(mut jso: *mut json_object,
                                                   mut userdata:
                                                       *mut libc::c_void) {
    free(userdata);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_double(mut jso: *const json_object)
 -> std::os::raw::c_double {
    let mut cdouble: std::os::raw::c_double = 0.;
    let mut errPtr: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    if jso.is_null() { return 0.0f64 }
    match (*jso).o_type as std::os::raw::c_uint {
        2 => { return (*jso).o.c_double }
        3 => { return (*jso).o.c_int64 as std::os::raw::c_double }
        1 => { return (*jso).o.c_boolean as std::os::raw::c_double }
        6 => {
            *__error() = 0 as std::os::raw::c_int;
            cdouble = strtod(get_string_component(jso), &mut errPtr);
            /* if conversion stopped at the first character, return 0.0 */
            if errPtr == get_string_component(jso) as *mut std::os::raw::c_char {
                return 0.0f64
            }
            /*
     * Check that the conversion terminated on something sensible
     *
     * For example, { "pay" : 123AB } would parse as 123.
     */
            if *errPtr as std::os::raw::c_int != '\u{0}' as i32 { return 0.0f64 }
            /*
     * If strtod encounters a string which would exceed the
     * capacity of a double, it returns +/- HUGE_VAL and sets
     * errno to ERANGE. But +/- HUGE_VAL is also a valid result
     * from a conversion, so we need to check errno.
     *
     * Underflow also sets errno to ERANGE, but it returns 0 in
     * that case, which is what we will return anyway.
     *
     * See CERT guideline ERR30-C
     */
            if (::std::f64::INFINITY == cdouble ||
                    -::std::f64::INFINITY == cdouble) &&
                   34 as std::os::raw::c_int == *__error() {
                cdouble = 0.0f64
            }
            return cdouble
        }
        _ => { return 0.0f64 }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_double(mut jso: *mut json_object,
                                                mut new_value: std::os::raw::c_double)
 -> std::os::raw::c_int {
    if jso.is_null() ||
           (*jso).o_type as std::os::raw::c_uint !=
               json_type_double as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    (*jso).o.c_double = new_value;
    return 1 as std::os::raw::c_int;
}
/* json_object_string */
unsafe extern "C" fn json_object_string_to_json_string(mut jso:
                                                           *mut json_object,
                                                       mut pb: *mut printbuf,
                                                       mut level: std::os::raw::c_int,
                                                       mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    printbuf_memappend(pb, b"\"\x00" as *const u8 as *const std::os::raw::c_char,
                       (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                            std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                           std::os::raw::c_int);
    json_escape_str(pb, get_string_component(jso), (*jso).o.c_string.len,
                    flags);
    printbuf_memappend(pb, b"\"\x00" as *const u8 as *const std::os::raw::c_char,
                       (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                            std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                           std::os::raw::c_int);
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn json_object_string_delete(mut jso: *mut json_object) {
    if (*jso).o.c_string.len >= 32 as std::os::raw::c_int {
        free((*jso).o.c_string.str_0.ptr as *mut libc::c_void);
    }
    json_object_generic_delete(jso);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_string(mut s: *const std::os::raw::c_char)
 -> *mut json_object {
    let mut jso: *mut json_object = json_object_new(json_type_string);
    if jso.is_null() { return 0 as *mut json_object }
    (*jso)._delete =
        Some(json_object_string_delete as
                 unsafe extern "C" fn(_: *mut json_object) -> ());
    (*jso)._to_json_string =
        Some(json_object_string_to_json_string as
                 json_object_to_json_string_fn);
    (*jso).o.c_string.len = strlen(s) as std::os::raw::c_int;
    if (*jso).o.c_string.len < 32 as std::os::raw::c_int {
        memcpy((*jso).o.c_string.str_0.data.as_mut_ptr() as *mut libc::c_void,
               s as *const libc::c_void,
               (*jso).o.c_string.len as std::os::raw::c_ulong);
    } else {
        (*jso).o.c_string.str_0.ptr = strdup(s);
        if (*jso).o.c_string.str_0.ptr.is_null() {
            json_object_generic_delete(jso);
            *__error() = 12 as std::os::raw::c_int;
            return 0 as *mut json_object
        }
    }
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_string_len(mut s:
                                                        *const std::os::raw::c_char,
                                                    mut len: std::os::raw::c_int)
 -> *mut json_object {
    let mut dstbuf: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut jso: *mut json_object = json_object_new(json_type_string);
    if jso.is_null() { return 0 as *mut json_object }
    (*jso)._delete =
        Some(json_object_string_delete as
                 unsafe extern "C" fn(_: *mut json_object) -> ());
    (*jso)._to_json_string =
        Some(json_object_string_to_json_string as
                 json_object_to_json_string_fn);
    if len < 32 as std::os::raw::c_int {
        dstbuf = (*jso).o.c_string.str_0.data.as_mut_ptr()
    } else {
        (*jso).o.c_string.str_0.ptr =
            malloc((len + 1 as std::os::raw::c_int) as std::os::raw::c_ulong) as
                *mut std::os::raw::c_char;
        if (*jso).o.c_string.str_0.ptr.is_null() {
            json_object_generic_delete(jso);
            *__error() = 12 as std::os::raw::c_int;
            return 0 as *mut json_object
        }
        dstbuf = (*jso).o.c_string.str_0.ptr
    }
    memcpy(dstbuf as *mut libc::c_void, s as *const libc::c_void,
           len as std::os::raw::c_ulong);
    *dstbuf.offset(len as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    (*jso).o.c_string.len = len;
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_string(mut jso: *mut json_object)
 -> *const std::os::raw::c_char {
    if jso.is_null() { return 0 as *const std::os::raw::c_char }
    match (*jso).o_type as std::os::raw::c_uint {
        6 => { return get_string_component(jso) }
        _ => { return json_object_to_json_string(jso) }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_string_len(mut jso:
                                                        *const json_object)
 -> std::os::raw::c_int {
    if jso.is_null() { return 0 as std::os::raw::c_int }
    match (*jso).o_type as std::os::raw::c_uint {
        6 => { return (*jso).o.c_string.len }
        _ => { return 0 as std::os::raw::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_string(mut jso: *mut json_object,
                                                mut s: *const std::os::raw::c_char)
 -> std::os::raw::c_int {
    return json_object_set_string_len(jso, s, strlen(s) as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_set_string_len(mut jso: *mut json_object,
                                                    mut s:
                                                        *const std::os::raw::c_char,
                                                    mut len: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut dstbuf: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    if jso.is_null() ||
           (*jso).o_type as std::os::raw::c_uint !=
               json_type_string as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    if len < 32 as std::os::raw::c_int {
        dstbuf = (*jso).o.c_string.str_0.data.as_mut_ptr();
        if (*jso).o.c_string.len >= 32 as std::os::raw::c_int {
            free((*jso).o.c_string.str_0.ptr as *mut libc::c_void);
        }
    } else {
        dstbuf =
            malloc((len + 1 as std::os::raw::c_int) as std::os::raw::c_ulong) as
                *mut std::os::raw::c_char;
        if dstbuf.is_null() { return 0 as std::os::raw::c_int }
        if (*jso).o.c_string.len >= 32 as std::os::raw::c_int {
            free((*jso).o.c_string.str_0.ptr as *mut libc::c_void);
        }
        (*jso).o.c_string.str_0.ptr = dstbuf
    }
    (*jso).o.c_string.len = len;
    memcpy(dstbuf as *mut libc::c_void, s as *const libc::c_void,
           len as std::os::raw::c_ulong);
    *dstbuf.offset(len as isize) = '\u{0}' as i32 as std::os::raw::c_char;
    return 1 as std::os::raw::c_int;
}
/* json_object_array */
unsafe extern "C" fn json_object_array_to_json_string(mut jso:
                                                          *mut json_object,
                                                      mut pb: *mut printbuf,
                                                      mut level: std::os::raw::c_int,
                                                      mut flags: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut had_children: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ii: size_t = 0;
    printbuf_memappend(pb, b"[\x00" as *const u8 as *const std::os::raw::c_char,
                       (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                            std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                            std::os::raw::c_ulong) as
                           std::os::raw::c_int);
    if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
        printbuf_memappend(pb, b"\n\x00" as *const u8 as *const std::os::raw::c_char,
                           (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_ulong)
                               as std::os::raw::c_int);
    }
    ii = 0 as std::os::raw::c_int as size_t;
    while ii < json_object_array_length(jso) {
        let mut val: *mut json_object = 0 as *mut json_object;
        if had_children != 0 {
            printbuf_memappend(pb,
                               b",\x00" as *const u8 as *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
            if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
                printbuf_memappend(pb,
                                   b"\n\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   (::std::mem::size_of::<[std::os::raw::c_char; 2]>()
                                        as
                                        std::os::raw::c_ulong).wrapping_sub(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_ulong)
                                       as std::os::raw::c_int);
            }
        }
        had_children = 1 as std::os::raw::c_int;
        if flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
            printbuf_memappend(pb,
                               b" \x00" as *const u8 as *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
        }
        indent(pb, level + 1 as std::os::raw::c_int, flags);
        val = json_object_array_get_idx(jso, ii);
        if val.is_null() {
            printbuf_memappend(pb,
                               b"null\x00" as *const u8 as
                                   *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 5]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
        } else if (*val)._to_json_string.expect("non-null function pointer")(val,
                                                                             pb,
                                                                             level
                                                                                 +
                                                                                 1
                                                                                     as
                                                                                     std::os::raw::c_int,
                                                                             flags)
                      < 0 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
        ii = ii.wrapping_add(1)
    }
    if flags & (1 as std::os::raw::c_int) << 1 as std::os::raw::c_int != 0 {
        if had_children != 0 {
            printbuf_memappend(pb,
                               b"\n\x00" as *const u8 as *const std::os::raw::c_char,
                               (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                    std::os::raw::c_ulong).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_ulong)
                                   as std::os::raw::c_int);
        }
        indent(pb, level, flags);
    }
    if flags & (1 as std::os::raw::c_int) << 0 as std::os::raw::c_int != 0 {
        return printbuf_memappend(pb,
                                  b" ]\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  (::std::mem::size_of::<[std::os::raw::c_char; 3]>()
                                       as
                                       std::os::raw::c_ulong).wrapping_sub(1 as
                                                                       std::os::raw::c_int
                                                                       as
                                                                       std::os::raw::c_ulong)
                                      as std::os::raw::c_int)
    }
    return printbuf_memappend(pb,
                              b"]\x00" as *const u8 as *const std::os::raw::c_char,
                              (::std::mem::size_of::<[std::os::raw::c_char; 2]>() as
                                   std::os::raw::c_ulong).wrapping_sub(1 as
                                                                   std::os::raw::c_int
                                                                   as
                                                                   std::os::raw::c_ulong)
                                  as std::os::raw::c_int);
}
unsafe extern "C" fn json_object_array_entry_free(mut data:
                                                      *mut libc::c_void) {
    json_object_put(data as *mut json_object);
}
unsafe extern "C" fn json_object_array_delete(mut jso: *mut json_object) {
    array_list_free((*jso).o.c_array);
    json_object_generic_delete(jso);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_new_array() -> *mut json_object {
    let mut jso: *mut json_object = json_object_new(json_type_array);
    if jso.is_null() { return 0 as *mut json_object }
    (*jso)._delete =
        Some(json_object_array_delete as
                 unsafe extern "C" fn(_: *mut json_object) -> ());
    (*jso)._to_json_string =
        Some(json_object_array_to_json_string as
                 json_object_to_json_string_fn);
    (*jso).o.c_array =
        array_list_new(Some(json_object_array_entry_free as
                                unsafe extern "C" fn(_: *mut libc::c_void)
                                    -> ()));
    if (*jso).o.c_array.is_null() {
        free(jso as *mut libc::c_void);
        return 0 as *mut json_object
    }
    return jso;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_get_array(mut jso: *const json_object)
 -> *mut array_list {
    if jso.is_null() { return 0 as *mut array_list }
    match (*jso).o_type as std::os::raw::c_uint {
        5 => { return (*jso).o.c_array }
        _ => { return 0 as *mut array_list }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_sort(mut jso: *mut json_object,
                                                mut sort_fn:
                                                    Option<unsafe extern "C" fn(_:
                                                                                    *const libc::c_void,
                                                                                _:
                                                                                    *const libc::c_void)
                                                               ->
                                                                   std::os::raw::c_int>) {
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_array as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 23],
                                               &[std::os::raw::c_char; 23]>(b"json_object_array_sort\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1192 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_array\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    array_list_sort((*jso).o.c_array, sort_fn);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_bsearch(mut key:
                                                       *const json_object,
                                                   mut jso:
                                                       *const json_object,
                                                   mut sort_fn:
                                                       Option<unsafe extern "C" fn(_:
                                                                                       *const libc::c_void,
                                                                                   _:
                                                                                       *const libc::c_void)
                                                                  ->
                                                                      std::os::raw::c_int>)
 -> *mut json_object {
    let mut result: *mut *mut json_object = 0 as *mut *mut json_object;
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_array as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 26],
                                               &[std::os::raw::c_char; 26]>(b"json_object_array_bsearch\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1203 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_array\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    result =
        array_list_bsearch(&mut key as *mut *const json_object as
                               *mut libc::c_void as *mut *const libc::c_void,
                           (*jso).o.c_array, sort_fn) as
            *mut *mut json_object;
    if result.is_null() { return 0 as *mut json_object }
    return *result;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_length(mut jso: *const json_object)
 -> size_t {
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_array as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 25],
                                               &[std::os::raw::c_char; 25]>(b"json_object_array_length\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1214 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_array\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    return array_list_length((*jso).o.c_array);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_add(mut jso: *mut json_object,
                                               mut val: *mut json_object)
 -> std::os::raw::c_int {
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_array as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 22],
                                               &[std::os::raw::c_char; 22]>(b"json_object_array_add\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1220 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_array\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    return array_list_add((*jso).o.c_array, val as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_put_idx(mut jso: *mut json_object,
                                                   mut idx: size_t,
                                                   mut val: *mut json_object)
 -> std::os::raw::c_int {
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_array as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 26],
                                               &[std::os::raw::c_char; 26]>(b"json_object_array_put_idx\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1227 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_array\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    return array_list_put_idx((*jso).o.c_array, idx,
                              val as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_del_idx(mut jso: *mut json_object,
                                                   mut idx: size_t,
                                                   mut count: size_t)
 -> std::os::raw::c_int {
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_array as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 26],
                                               &[std::os::raw::c_char; 26]>(b"json_object_array_del_idx\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1233 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_array\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    return array_list_del_idx((*jso).o.c_array, idx, count);
}
#[no_mangle]
pub unsafe extern "C" fn json_object_array_get_idx(mut jso:
                                                       *const json_object,
                                                   mut idx: size_t)
 -> *mut json_object {
    if !(json_object_get_type(jso) as std::os::raw::c_uint ==
             json_type_array as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 26],
                                               &[std::os::raw::c_char; 26]>(b"json_object_array_get_idx\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1240 as std::os::raw::c_int,
                     b"json_object_get_type(jso) == json_type_array\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    return array_list_get_idx((*jso).o.c_array, idx) as *mut json_object;
}
unsafe extern "C" fn json_array_equal(mut jso1: *mut json_object,
                                      mut jso2: *mut json_object)
 -> std::os::raw::c_int {
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    len = json_object_array_length(jso1);
    if len != json_object_array_length(jso2) { return 0 as std::os::raw::c_int }
    i = 0 as std::os::raw::c_int as size_t;
    while i < len {
        if json_object_equal(json_object_array_get_idx(jso1, i),
                             json_object_array_get_idx(jso2, i)) == 0 {
            return 0 as std::os::raw::c_int
        }
        i = i.wrapping_add(1)
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn json_object_all_values_equal(mut jso1: *mut json_object,
                                                  mut jso2: *mut json_object)
 -> std::os::raw::c_int {
    let mut iter: json_object_iter =
        json_object_iter{key: 0 as *mut std::os::raw::c_char,
                         val: 0 as *mut json_object,
                         entry: 0 as *mut lh_entry,};
    let mut sub: *mut json_object = 0 as *mut json_object;
    if !(json_object_get_type(jso1) as std::os::raw::c_uint ==
             json_type_object as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 29],
                                               &[std::os::raw::c_char; 29]>(b"json_object_all_values_equal\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1267 as std::os::raw::c_int,
                     b"json_object_get_type(jso1) == json_type_object\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(json_object_get_type(jso2) as std::os::raw::c_uint ==
             json_type_object as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 29],
                                               &[std::os::raw::c_char; 29]>(b"json_object_all_values_equal\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1268 as std::os::raw::c_int,
                     b"json_object_get_type(jso2) == json_type_object\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
    /* Iterate over jso1 keys and see if they exist and are equal in jso2 */
    iter.entry = (*json_object_get_object(jso1)).head;
    while !if !iter.entry.is_null() {
               iter.key =
                   (*iter.entry).k as uintptr_t as *mut libc::c_void as
                       *mut std::os::raw::c_char;
               iter.val =
                   (*iter.entry).v as uintptr_t as *mut libc::c_void as
                       *mut json_object;
               iter.entry
           } else { 0 as *mut lh_entry }.is_null() {
        if lh_table_lookup_ex((*jso2).o.c_object,
                              iter.key as *mut libc::c_void,
                              &mut sub as *mut *mut json_object as
                                  *mut libc::c_void as *mut *mut libc::c_void)
               == 0 {
            return 0 as std::os::raw::c_int
        }
        if json_object_equal(iter.val, sub) == 0 { return 0 as std::os::raw::c_int }
        iter.entry = (*iter.entry).next
    }
    /* Iterate over jso2 keys to see if any exist that are not in jso1 */
    iter.entry = (*json_object_get_object(jso2)).head;
    while !if !iter.entry.is_null() {
               iter.key =
                   (*iter.entry).k as uintptr_t as *mut libc::c_void as
                       *mut std::os::raw::c_char;
               iter.val =
                   (*iter.entry).v as uintptr_t as *mut libc::c_void as
                       *mut json_object;
               iter.entry
           } else { 0 as *mut lh_entry }.is_null() {
        if lh_table_lookup_ex((*jso1).o.c_object,
                              iter.key as *mut libc::c_void,
                              &mut sub as *mut *mut json_object as
                                  *mut libc::c_void as *mut *mut libc::c_void)
               == 0 {
            return 0 as std::os::raw::c_int
        }
        iter.entry = (*iter.entry).next
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_equal(mut jso1: *mut json_object,
                                           mut jso2: *mut json_object)
 -> std::os::raw::c_int {
    if jso1 == jso2 { return 1 as std::os::raw::c_int }
    if jso1.is_null() || jso2.is_null() { return 0 as std::os::raw::c_int }
    if (*jso1).o_type as std::os::raw::c_uint != (*jso2).o_type as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    match (*jso1).o_type as std::os::raw::c_uint {
        1 => {
            return ((*jso1).o.c_boolean == (*jso2).o.c_boolean) as std::os::raw::c_int
        }
        2 => {
            return ((*jso1).o.c_double == (*jso2).o.c_double) as std::os::raw::c_int
        }
        3 => {
            return ((*jso1).o.c_int64 == (*jso2).o.c_int64) as std::os::raw::c_int
        }
        6 => {
            return ((*jso1).o.c_string.len == (*jso2).o.c_string.len &&
                        memcmp(get_string_component(jso1) as
                                   *const libc::c_void,
                               get_string_component(jso2) as
                                   *const libc::c_void,
                               (*jso1).o.c_string.len as std::os::raw::c_ulong) ==
                            0 as std::os::raw::c_int) as std::os::raw::c_int
        }
        4 => { return json_object_all_values_equal(jso1, jso2) }
        5 => { return json_array_equal(jso1, jso2) }
        0 => { return 1 as std::os::raw::c_int }
        _ => { }
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn json_object_copy_serializer_data(mut src:
                                                          *mut json_object,
                                                      mut dst:
                                                          *mut json_object)
 -> std::os::raw::c_int {
    if (*src)._userdata.is_null() && (*src)._user_delete.is_none() {
        return 0 as std::os::raw::c_int
    }
    if (*dst)._to_json_string ==
           Some(json_object_userdata_to_json_string as
                    json_object_to_json_string_fn) {
        (*dst)._userdata =
            strdup((*src)._userdata as *const std::os::raw::c_char) as
                *mut libc::c_void
    } else {
        // else if ... other supported serializers ...
        _json_c_set_last_err(b"json_object_deep_copy: unable to copy unknown serializer data: %p\n\x00"
                                 as *const u8 as *const std::os::raw::c_char,
                             (*dst)._to_json_string);
        return -(1 as std::os::raw::c_int)
    }
    (*dst)._user_delete = (*src)._user_delete;
    return 0 as std::os::raw::c_int;
}
/* *
 * The default shallow copy implementation.  Simply creates a new object of the same
 * type but does *not* copy over _userdata nor retain any custom serializer.
 * If custom serializers are in use, json_object_deep_copy() must be passed a shallow copy
 * implementation that is aware of how to copy them.
 *
 * This always returns -1 or 1.  It will never return 2 since it does not copy the serializer.
 */
#[no_mangle]
pub unsafe extern "C" fn json_c_shallow_copy_default(mut src:
                                                         *mut json_object,
                                                     mut parent:
                                                         *mut json_object,
                                                     mut key:
                                                         *const std::os::raw::c_char,
                                                     mut index: size_t,
                                                     mut dst:
                                                         *mut *mut json_object)
 -> std::os::raw::c_int {
    match (*src).o_type as std::os::raw::c_uint {
        1 => { *dst = json_object_new_boolean((*src).o.c_boolean) }
        2 => { *dst = json_object_new_double((*src).o.c_double) }
        3 => { *dst = json_object_new_int64((*src).o.c_int64) }
        6 => { *dst = json_object_new_string(get_string_component(src)) }
        4 => { *dst = json_object_new_object() }
        5 => { *dst = json_object_new_array() }
        _ => { *__error() = 22 as std::os::raw::c_int; return -(1 as std::os::raw::c_int) }
    }
    if (*dst).is_null() {
        *__error() = 12 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    (**dst)._to_json_string = (*src)._to_json_string;
    // _userdata and _user_delete are copied later
    return 1 as std::os::raw::c_int;
}
/*
 * The actual guts of json_object_deep_copy(), with a few additional args
 * needed so we can keep track of where we are within the object tree.
 *
 * Note: caller is responsible for freeing *dst if this fails and returns -1.
 */
unsafe extern "C" fn json_object_deep_copy_recursive(mut src:
                                                         *mut json_object,
                                                     mut parent:
                                                         *mut json_object,
                                                     mut key_in_parent:
                                                         *const std::os::raw::c_char,
                                                     mut index_in_parent:
                                                         size_t,
                                                     mut dst:
                                                         *mut *mut json_object,
                                                     mut shallow_copy:
                                                         Option<json_c_shallow_copy_fn>)
 -> std::os::raw::c_int {
    let mut iter: json_object_iter =
        json_object_iter{key: 0 as *mut std::os::raw::c_char,
                         val: 0 as *mut json_object,
                         entry: 0 as *mut lh_entry,};
    let mut src_array_len: size_t = 0;
    let mut ii: size_t = 0;
    let mut shallow_copy_rc: std::os::raw::c_int = 0 as std::os::raw::c_int;
    shallow_copy_rc =
        shallow_copy.expect("non-null function pointer")(src, parent,
                                                         key_in_parent,
                                                         index_in_parent,
                                                         dst);
    /* -1=error, 1=object created ok, 2=userdata set */
    if shallow_copy_rc < 1 as std::os::raw::c_int {
        *__error() = 22 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    if (*dst).is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 32],
                                               &[std::os::raw::c_char; 32]>(b"json_object_deep_copy_recursive\x00")).as_ptr(),
                     b"json_object.c\x00" as *const u8 as *const std::os::raw::c_char,
                     1416 as std::os::raw::c_int,
                     b"*dst != NULL\x00" as *const u8 as *const std::os::raw::c_char);
    } else { };
    match (*src).o_type as std::os::raw::c_uint {
        4 => {
            iter.entry = (*json_object_get_object(src)).head;
            while !if !iter.entry.is_null() {
                       iter.key =
                           (*iter.entry).k as uintptr_t as *mut libc::c_void
                               as *mut std::os::raw::c_char;
                       iter.val =
                           (*iter.entry).v as uintptr_t as *mut libc::c_void
                               as *mut json_object;
                       iter.entry
                   } else { 0 as *mut lh_entry }.is_null() {
                let mut jso: *mut json_object = 0 as *mut json_object;
                /* This handles the `json_type_null` case */
                if iter.val.is_null() {
                    jso = 0 as *mut json_object
                } else if json_object_deep_copy_recursive(iter.val, src,
                                                          iter.key,
                                                          -(1 as std::os::raw::c_int)
                                                              as size_t,
                                                          &mut jso,
                                                          shallow_copy) <
                              0 as std::os::raw::c_int {
                    json_object_put(jso);
                    return -(1 as std::os::raw::c_int)
                }
                if json_object_object_add(*dst, iter.key, jso) <
                       0 as std::os::raw::c_int {
                    json_object_put(jso);
                    return -(1 as std::os::raw::c_int)
                }
                iter.entry = (*iter.entry).next
            }
            /* else, nothing to do, shallow_copy already did. */
        }
        5 => {
            src_array_len = json_object_array_length(src);
            ii = 0 as std::os::raw::c_int as size_t;
            while ii < src_array_len {
                let mut jso_0: *mut json_object = 0 as *mut json_object;
                let mut jso1: *mut json_object =
                    json_object_array_get_idx(src, ii);
                /* This handles the `json_type_null` case */
                if jso1.is_null() {
                    jso_0 = 0 as *mut json_object
                } else if json_object_deep_copy_recursive(jso1, src,
                                                          0 as
                                                              *const std::os::raw::c_char,
                                                          ii, &mut jso_0,
                                                          shallow_copy) <
                              0 as std::os::raw::c_int {
                    json_object_put(jso_0);
                    return -(1 as std::os::raw::c_int)
                }
                if json_object_array_add(*dst, jso_0) < 0 as std::os::raw::c_int {
                    json_object_put(jso_0);
                    return -(1 as std::os::raw::c_int)
                }
                ii = ii.wrapping_add(1)
            }
        }
        _ => { }
    }
    if shallow_copy_rc != 2 as std::os::raw::c_int {
        return json_object_copy_serializer_data(src, *dst)
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_object_deep_copy(mut src: *mut json_object,
                                               mut dst: *mut *mut json_object,
                                               mut shallow_copy:
                                                   Option<json_c_shallow_copy_fn>)
 -> std::os::raw::c_int {
    let mut rc: std::os::raw::c_int = 0;
    /* Check if arguments are sane ; *dst must not point to a non-NULL object */
    if src.is_null() || dst.is_null() || !(*dst).is_null() {
        *__error() = 22 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    if shallow_copy.is_none() {
        shallow_copy =
            Some(json_c_shallow_copy_default as json_c_shallow_copy_fn)
    }
    rc =
        json_object_deep_copy_recursive(src, 0 as *mut json_object,
                                        0 as *const std::os::raw::c_char,
                                        -(1 as std::os::raw::c_int) as size_t, dst,
                                        shallow_copy);
    if rc < 0 as std::os::raw::c_int {
        json_object_put(*dst);
        *dst = 0 as *mut json_object
    }
    return rc;
}
