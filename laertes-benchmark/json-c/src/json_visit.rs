#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, register_tool)]
extern "C" {
    pub type __sFILEX;
    pub type json_object;
    #[no_mangle]
    static mut __stderrp: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_get_type(obj: *const json_object) -> json_type;
    #[no_mangle]
    fn json_object_get_object(obj: *const json_object) -> *mut lh_table;
    #[no_mangle]
    fn json_object_array_length(obj: *const json_object) -> size_t;
    #[no_mangle]
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t)
     -> *mut json_object;
}
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type uintptr_t = std::os::raw::c_ulong;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_entry {
    pub k: *const libc::c_void,
    pub k_is_constant: std::os::raw::c_int,
    pub v: *const libc::c_void,
    pub next: *mut lh_entry,
    pub prev: *mut lh_entry,
}
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
pub type json_c_visit_userfunc
    =
    unsafe extern "C" fn(_: *mut json_object, _: std::os::raw::c_int,
                         _: *mut json_object, _: *const std::os::raw::c_char,
                         _: *mut size_t, _: *mut libc::c_void) -> std::os::raw::c_int;
#[no_mangle]
pub unsafe extern "C" fn json_c_visit(mut jso: *mut json_object,
                                      mut future_flags: std::os::raw::c_int,
                                      mut userfunc:
                                          Option<json_c_visit_userfunc>,
                                      mut userarg: *mut libc::c_void)
 -> std::os::raw::c_int {
    let mut ret: std::os::raw::c_int =
        _json_c_visit(jso, 0 as *mut json_object, 0 as *const std::os::raw::c_char,
                      0 as *mut size_t, userfunc, userarg);
    match ret {
        0 | 7547 | 767 | 7867 => { return 0 as std::os::raw::c_int }
        _ => { return -(1 as std::os::raw::c_int) }
    };
}
/*
 * Copyright (c) 2016 Eric Haszlakiewicz
 *
 * This is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 */
unsafe extern "C" fn _json_c_visit(mut jso: *mut json_object,
                                   mut parent_jso: *mut json_object,
                                   mut jso_key: *const std::os::raw::c_char,
                                   mut jso_index: *mut size_t,
                                   mut userfunc:
                                       Option<json_c_visit_userfunc>,
                                   mut userarg: *mut libc::c_void)
 -> std::os::raw::c_int {
    let mut userret: std::os::raw::c_int =
        userfunc.expect("non-null function pointer")(jso, 0 as std::os::raw::c_int,
                                                     parent_jso, jso_key,
                                                     jso_index, userarg);
    match userret {
        0 => { }
        7547 | 767 | 7867 | -1 => { return userret }
        _ => {
            fprintf(__stderrp,
                    b"ERROR: invalid return value from json_c_visit userfunc: %d\n\x00"
                        as *const u8 as *const std::os::raw::c_char, userret);
            return -(1 as std::os::raw::c_int)
        }
    }
    match json_object_get_type(jso) as std::os::raw::c_uint {
        0 | 1 | 2 | 3 | 6 => {
            // we already called userfunc above, move on to the next object
            return 0 as std::os::raw::c_int
        }
        4 => {
            let mut key: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
            let mut child: *mut json_object = 0 as *mut json_object;
            let mut entrykey: *mut lh_entry =
                (*json_object_get_object(jso)).head;
            let mut entry_nextkey: *mut lh_entry = 0 as *mut lh_entry;
            while !({
                        if !entrykey.is_null() {
                            key =
                                (*entrykey).k as uintptr_t as
                                    *mut libc::c_void as *mut std::os::raw::c_char;
                            child =
                                (*entrykey).v as uintptr_t as
                                    *mut libc::c_void as *mut json_object;
                            entry_nextkey = (*entrykey).next
                        }
                        entrykey
                    }).is_null() {
                userret =
                    _json_c_visit(child, jso, key, 0 as *mut size_t, userfunc,
                                  userarg);
                if userret == 767 as std::os::raw::c_int { break ; }
                if userret == 7867 as std::os::raw::c_int ||
                       userret == -(1 as std::os::raw::c_int) {
                    return userret
                }
                if userret != 0 as std::os::raw::c_int &&
                       userret != 7547 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"INTERNAL ERROR: _json_c_visit returned %d\n\x00"
                                as *const u8 as *const std::os::raw::c_char, userret);
                    return -(1 as std::os::raw::c_int)
                }
                entrykey = entry_nextkey
            }
        }
        5 => {
            let mut array_len: size_t = json_object_array_length(jso);
            let mut ii: size_t = 0;
            ii = 0 as std::os::raw::c_int as size_t;
            while ii < array_len {
                let mut child_0: *mut json_object =
                    json_object_array_get_idx(jso, ii);
                userret =
                    _json_c_visit(child_0, jso, 0 as *const std::os::raw::c_char,
                                  &mut ii, userfunc, userarg);
                if userret == 767 as std::os::raw::c_int { break ; }
                if userret == 7867 as std::os::raw::c_int ||
                       userret == -(1 as std::os::raw::c_int) {
                    return userret
                }
                if userret != 0 as std::os::raw::c_int &&
                       userret != 7547 as std::os::raw::c_int {
                    fprintf(__stderrp,
                            b"INTERNAL ERROR: _json_c_visit returned %d\n\x00"
                                as *const u8 as *const std::os::raw::c_char, userret);
                    return -(1 as std::os::raw::c_int)
                }
                ii = ii.wrapping_add(1)
            }
        }
        _ => {
            fprintf(__stderrp,
                    b"INTERNAL ERROR: _json_c_visit found object of unknown type: %d\n\x00"
                        as *const u8 as *const std::os::raw::c_char,
                    json_object_get_type(jso) as std::os::raw::c_uint);
            return -(1 as std::os::raw::c_int)
        }
    }
    // Call userfunc for the second type on container types, after all
	//  members of the container have been visited.
	// Non-container types will have already returned before this point.
    userret =
        userfunc.expect("non-null function pointer")(jso, 0x2 as std::os::raw::c_int,
                                                     parent_jso, jso_key,
                                                     jso_index, userarg);
    match userret {
        7547 | 767 | 0 => {
            // These are not really sensible during JSON_C_VISIT_SECOND, 
		// but map them to JSON_C_VISIT_CONTINUE anyway.
		// FALLTHROUGH
            return 0 as std::os::raw::c_int
        }
        7867 | -1 => { return userret }
        _ => {
            fprintf(__stderrp,
                    b"ERROR: invalid return value from json_c_visit userfunc: %d\n\x00"
                        as *const u8 as *const std::os::raw::c_char, userret);
            return -(1 as std::os::raw::c_int)
        }
    };
    // NOTREACHED
}
