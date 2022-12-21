#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, register_tool)]
extern "C" {
    #[no_mangle]
    fn json_object_get_object(obj: *const json_object) -> *mut lh_table;
}
pub type __darwin_size_t = std::os::raw::c_ulong;
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
pub struct json_object_iterator {
    pub opaque_: *const libc::c_void,
}
/* *
*******************************************************************************
* @file json_object_iterator.c
*
* Copyright (c) 2009-2012 Hewlett-Packard Development Company, L.P.
*
* This library is free software; you can redistribute it and/or modify
* it under the terms of the MIT license. See COPYING for details.
*
*******************************************************************************
*/
/* *
 * How It Works
 *
 * For each JSON Object, json-c maintains a linked list of zero
 * or more lh_entry (link-hash entry) structures inside the
 * Object's link-hash table (lh_table).
 *
 * Each lh_entry structure on the JSON Object's linked list
 * represents a single name/value pair.  The "next" field of the
 * last lh_entry in the list is set to NULL, which terminates
 * the list.
 *
 * We represent a valid iterator that refers to an actual
 * name/value pair via a pointer to the pair's lh_entry
 * structure set as the iterator's opaque_ field.
 *
 * We follow json-c's current pair list representation by
 * representing a valid "end" iterator (one that refers past the
 * last pair) with a NULL value in the iterator's opaque_ field.
 *
 * A JSON Object without any pairs in it will have the "head"
 * field of its lh_table structure set to NULL.  For such an
 * object, json_object_iter_begin will return an iterator with
 * the opaque_ field set to NULL, which is equivalent to the
 * "end" iterator.
 *
 * When iterating, we simply update the iterator's opaque_ field
 * to point to the next lh_entry structure in the linked list.
 * opaque_ will become NULL once we iterate past the last pair
 * in the list, which makes the iterator equivalent to the "end"
 * iterator.
 */
// / Our current representation of the "end" iterator;
// /
// / @note May not always be NULL
static mut kObjectEndIterValue: *const libc::c_void =
    0 as *const libc::c_void;
/* *
 * ****************************************************************************
 */
#[no_mangle]
pub unsafe extern "C" fn json_object_iter_begin(mut obj: *mut json_object)
 -> json_object_iterator {
    let mut iter: json_object_iterator =
        json_object_iterator{opaque_: 0 as *const libc::c_void,};
    let mut pTable: *mut lh_table = 0 as *mut lh_table;
    // / @note json_object_get_object will return NULL if passed NULL
    // /       or a non-json_type_object instance
    pTable = json_object_get_object(obj);
    // / @note For a pair-less Object, head is NULL, which matches our
    // /       definition of the "end" iterator
    iter.opaque_ = (*pTable).head as *const libc::c_void;
    return iter;
}
/* *
 * ****************************************************************************
 */
#[no_mangle]
pub unsafe extern "C" fn json_object_iter_end(mut obj: *const json_object)
 -> json_object_iterator {
    let mut iter: json_object_iterator =
        json_object_iterator{opaque_: 0 as *const libc::c_void,};
    iter.opaque_ = kObjectEndIterValue;
    return iter;
}
/* *
 * ****************************************************************************
 */
#[no_mangle]
pub unsafe extern "C" fn json_object_iter_next(mut iter:
                                                   *mut json_object_iterator) {
    (*iter).opaque_ =
        (*((*iter).opaque_ as *const lh_entry)).next as *const libc::c_void;
}
/* *
 * ****************************************************************************
 */
#[no_mangle]
pub unsafe extern "C" fn json_object_iter_peek_name(mut iter:
                                                        *const json_object_iterator)
 -> *const std::os::raw::c_char {
    return (*((*iter).opaque_ as *const lh_entry)).k as *const std::os::raw::c_char;
}
/* *
 * ****************************************************************************
 */
#[no_mangle]
pub unsafe extern "C" fn json_object_iter_peek_value(mut iter:
                                                         *const json_object_iterator)
 -> *mut json_object {
    return (*((*iter).opaque_ as *const lh_entry)).v as uintptr_t as
               *mut libc::c_void as *mut json_object;
}
/* *
 * ****************************************************************************
 */
#[no_mangle]
pub unsafe extern "C" fn json_object_iter_equal(mut iter1:
                                                    *const json_object_iterator,
                                                mut iter2:
                                                    *const json_object_iterator)
 -> json_bool {
    return ((*iter1).opaque_ == (*iter2).opaque_) as std::os::raw::c_int;
}
/* *
 * ****************************************************************************
 */
#[no_mangle]
pub unsafe extern "C" fn json_object_iter_init_default()
 -> json_object_iterator {
    let mut iter: json_object_iterator =
        json_object_iterator{opaque_: 0 as *const libc::c_void,};
    /* *
     * @note Make this a negative, invalid value, such that
     *       accidental access to it would likely be trapped by the
     *       hardware as an invalid address.
     */
    iter.opaque_ = 0 as *const libc::c_void;
    return iter;
}
