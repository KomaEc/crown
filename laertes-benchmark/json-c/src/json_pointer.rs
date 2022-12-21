#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, extern_types, linkage,
           ptr_offset_from, register_tool)]
extern "C" {
    pub type json_object;
    #[no_mangle]
    fn json_object_put(obj: *mut json_object) -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_is_type(obj: *const json_object, type_0: json_type)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_object_add(obj: *mut json_object, key: *const std::os::raw::c_char,
                              val: *mut json_object) -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_object_get_ex(obj: *const json_object,
                                 key: *const std::os::raw::c_char,
                                 value: *mut *mut json_object) -> json_bool;
    #[no_mangle]
    fn json_object_array_length(obj: *const json_object) -> size_t;
    #[no_mangle]
    fn json_object_array_add(obj: *mut json_object, val: *mut json_object)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_array_put_idx(obj: *mut json_object, idx: size_t,
                                 val: *mut json_object) -> std::os::raw::c_int;
    #[no_mangle]
    fn json_object_array_get_idx(obj: *const json_object, idx: size_t)
     -> *mut json_object;
    #[no_mangle]
    fn __error() -> *mut std::os::raw::c_int;
    #[no_mangle]
    fn strdup(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strrchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: std::os::raw::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn strtol(_: *const std::os::raw::c_char, _: *mut *mut std::os::raw::c_char,
              _: std::os::raw::c_int) -> std::os::raw::c_long;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    static mut _DefaultRuneLocale: _RuneLocale;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __darwin_ct_rune_t = std::os::raw::c_int;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_wchar_t = std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type int32_t = std::os::raw::c_int;
pub type size_t = __darwin_size_t;
pub type json_bool = std::os::raw::c_int;
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isdigit(mut _c: std::os::raw::c_int) -> std::os::raw::c_int {
    return __isctype(_c, 0x400 as std::os::raw::c_long as std::os::raw::c_ulong);
}
/*
 * Copyright (c) 2016 Alexandru Ardelean.
 *
 * This is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 *
 */
/* *
 * JavaScript Object Notation (JSON) Pointer
 *   RFC 6901 - https://tools.ietf.org/html/rfc6901
 */
unsafe extern "C" fn string_replace_all_occurrences_with_char(mut s:
                                                                  *mut std::os::raw::c_char,
                                                              mut occur:
                                                                  *const std::os::raw::c_char,
                                                              mut repl_char:
                                                                  std::os::raw::c_char) {
    let mut slen: std::os::raw::c_int =
        strlen(s) as
            std::os::raw::c_int; /* length of the occurence, minus the char we're replacing */
    let mut skip: std::os::raw::c_int =
        strlen(occur).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int;
    let mut p: *mut std::os::raw::c_char = s;
    loop  {
        p = strstr(p, occur);
        if p.is_null() { break ; }
        *p = repl_char;
        p = p.offset(1);
        slen -= skip;
        memmove(p as *mut libc::c_void,
                p.offset(skip as isize) as *const libc::c_void,
                (slen as std::os::raw::c_long -
                     p.offset_from(s) as std::os::raw::c_long +
                     1 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_ulong);
        /* includes null char too */
    };
}
unsafe extern "C" fn is_valid_index(mut jo: *mut json_object,
                                    mut path: *const std::os::raw::c_char,
                                    mut idx: *mut int32_t) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut len: std::os::raw::c_int = strlen(path) as std::os::raw::c_int;
    /* this code-path optimizes a bit, for when we reference the 0-9 index range in a JSON array
	   and because leading zeros not allowed */
    if len == 1 as std::os::raw::c_int {
        if isdigit(*path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) !=
               0 {
            *idx =
                *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                    '0' as i32
        } else { *__error() = 22 as std::os::raw::c_int; return 0 as std::os::raw::c_int }
    } else {
        /* leading zeros not allowed per RFC */
        if *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '0' as i32 {
            *__error() = 22 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
        /* RFC states base-10 decimals */
        i = 0 as std::os::raw::c_int;
        while i < len {
            if isdigit(*path.offset(i as isize) as std::os::raw::c_int) == 0 {
                *__error() = 22 as std::os::raw::c_int;
                return 0 as std::os::raw::c_int
            }
            i += 1
        }
        *idx =
            strtol(path, 0 as *mut *mut std::os::raw::c_char, 10 as std::os::raw::c_int) as
                int32_t;
        if *idx < 0 as std::os::raw::c_int {
            *__error() = 22 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
    }
    len = json_object_array_length(jo) as std::os::raw::c_int;
    if *idx >= len { *__error() = 2 as std::os::raw::c_int; return 0 as std::os::raw::c_int }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn json_pointer_get_single_path(mut obj: *mut json_object,
                                                  mut path: *mut std::os::raw::c_char,
                                                  mut value:
                                                      *mut *mut json_object)
 -> std::os::raw::c_int {
    if json_object_is_type(obj, json_type_array) != 0 {
        let mut idx: int32_t = 0;
        if is_valid_index(obj, path, &mut idx) == 0 {
            return -(1 as std::os::raw::c_int)
        }
        obj = json_object_array_get_idx(obj, idx as size_t);
        if !obj.is_null() {
            if !value.is_null() { *value = obj }
            return 0 as std::os::raw::c_int
        }
        /* Entry not found */
        *__error() = 2 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    /* RFC states that we first must eval all ~1 then all ~0 */
    string_replace_all_occurrences_with_char(path,
                                             b"~1\x00" as *const u8 as
                                                 *const std::os::raw::c_char,
                                             '/' as i32 as std::os::raw::c_char);
    string_replace_all_occurrences_with_char(path,
                                             b"~0\x00" as *const u8 as
                                                 *const std::os::raw::c_char,
                                             '~' as i32 as std::os::raw::c_char);
    if json_object_object_get_ex(obj, path, value) == 0 {
        *__error() = 2 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    return 0 as std::os::raw::c_int;
}
unsafe extern "C" fn json_pointer_set_single_path(mut parent:
                                                      *mut json_object,
                                                  mut path:
                                                      *const std::os::raw::c_char,
                                                  mut value: *mut json_object)
 -> std::os::raw::c_int {
    if json_object_is_type(parent, json_type_array) != 0 {
        let mut idx: int32_t = 0;
        /* RFC (Chapter 4) states that '-' may be used to add new elements to an array */
        if *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               '-' as i32 &&
               *path.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   '\u{0}' as i32 {
            return json_object_array_add(parent, value)
        }
        if is_valid_index(parent, path, &mut idx) == 0 {
            return -(1 as std::os::raw::c_int)
        }
        return json_object_array_put_idx(parent, idx as size_t, value)
    }
    /* path replacements should have been done in json_pointer_get_single_path(),
	   and we should still be good here */
    if json_object_is_type(parent, json_type_object) != 0 {
        return json_object_object_add(parent, path, value)
    }
    /* Getting here means that we tried to "dereference" a primitive JSON type (like string, int, bool).
	   i.e. add a sub-object to it */
    *__error() = 2 as std::os::raw::c_int;
    return -(1 as std::os::raw::c_int);
}
unsafe extern "C" fn json_pointer_get_recursive(mut obj: *mut json_object,
                                                mut path: *mut std::os::raw::c_char,
                                                mut value:
                                                    *mut *mut json_object)
 -> std::os::raw::c_int {
    let mut endp: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut rc: std::os::raw::c_int = 0;
    /* All paths (on each recursion level must have a leading '/' */
    if *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '/' as i32 {
        *__error() = 22 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    path = path.offset(1);
    endp = strchr(path, '/' as i32);
    if !endp.is_null() { *endp = '\u{0}' as i32 as std::os::raw::c_char }
    /* If we err-ed here, return here */
    rc =
        json_pointer_get_single_path(obj, path,
                                     &mut obj); /* Put the slash back, so that the sanity check passes on next recursion level */
    if rc != 0 { return rc }
    if !endp.is_null() {
        *endp = '/' as i32 as std::os::raw::c_char;
        return json_pointer_get_recursive(obj, endp, value)
    }
    /* We should be at the end of the recursion here */
    if !value.is_null() { *value = obj }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_get(mut obj: *mut json_object,
                                          mut path: *const std::os::raw::c_char,
                                          mut res: *mut *mut json_object)
 -> std::os::raw::c_int {
    let mut path_copy: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut rc: std::os::raw::c_int = 0;
    if obj.is_null() || path.is_null() {
        *__error() = 22 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    if *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           '\u{0}' as i32 {
        if !res.is_null() { *res = obj }
        return 0 as std::os::raw::c_int
    }
    /* pass a working copy to the recursive call */
    path_copy = strdup(path);
    if path_copy.is_null() {
        *__error() = 12 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    rc = json_pointer_get_recursive(obj, path_copy, res);
    free(path_copy as *mut libc::c_void);
    return rc;
}
// In rust/src/json_pointer_f.c
#[no_mangle]
pub unsafe extern "C" fn json_pointer_set(mut obj: *mut *mut json_object,
                                          mut path: *const std::os::raw::c_char,
                                          mut value: *mut json_object)
 -> std::os::raw::c_int {
    let mut endp: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut path_copy: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut set: *mut json_object = 0 as *mut json_object;
    let mut rc: std::os::raw::c_int = 0;
    if obj.is_null() || path.is_null() {
        *__error() = 22 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    if *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           '\u{0}' as i32 {
        json_object_put(*obj);
        *obj = value;
        return 0 as std::os::raw::c_int
    }
    if *path.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '/' as i32 {
        *__error() = 22 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    /* If there's only 1 level to set, stop here */
    endp = strrchr(path, '/' as i32);
    if endp == path {
        path = path.offset(1);
        return json_pointer_set_single_path(*obj, path, value)
    }
    /* pass a working copy to the recursive call */
    path_copy = strdup(path);
    if path_copy.is_null() {
        *__error() = 12 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    *path_copy.offset(endp.offset_from(path) as std::os::raw::c_long as
                          isize) = '\u{0}' as i32 as std::os::raw::c_char;
    rc = json_pointer_get_recursive(*obj, path_copy, &mut set);
    free(path_copy as *mut libc::c_void);
    if rc != 0 { return rc }
    endp = endp.offset(1);
    return json_pointer_set_single_path(set, endp, value);
}
// In rust/src/json_pointer_f.c
