#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn bsearch(__key: *const libc::c_void, __base: *const libc::c_void,
               __nel: size_t, __width: size_t,
               __compar:
                   Option<unsafe extern "C" fn(_: *const libc::c_void,
                                               _: *const libc::c_void)
                              -> std::os::raw::c_int>) -> *mut libc::c_void;
    #[no_mangle]
    fn qsort(__base: *mut libc::c_void, __nel: size_t, __width: size_t,
             __compar:
                 Option<unsafe extern "C" fn(_: *const libc::c_void,
                                             _: *const libc::c_void)
                            -> std::os::raw::c_int>);
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: std::os::raw::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut libc::c_void;
}
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type size_t = __darwin_size_t;
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
/*
 * $Id: arraylist.c,v 1.4 2006/01/26 02:16:28 mclark Exp $
 *
 * Copyright (c) 2004, 2005 Metaparadigm Pte. Ltd.
 * Michael Clark <michael@metaparadigm.com>
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 *
 */
/* STDC_HEADERS */
/* HAVE_STRINGS_H */
#[no_mangle]
pub unsafe extern "C" fn array_list_new(mut free_fn:
                                            Option<array_list_free_fn>)
 -> *mut array_list {
    let mut arr: *mut array_list = 0 as *mut array_list;
    arr =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<array_list>() as std::os::raw::c_ulong) as
            *mut array_list;
    if arr.is_null() { return 0 as *mut array_list }
    (*arr).size = 32 as std::os::raw::c_int as size_t;
    (*arr).length = 0 as std::os::raw::c_int as size_t;
    (*arr).free_fn = free_fn;
    (*arr).array =
        calloc(::std::mem::size_of::<*mut libc::c_void>() as std::os::raw::c_ulong,
               (*arr).size) as *mut *mut libc::c_void;
    if (*arr).array.is_null() {
        free(arr as *mut libc::c_void);
        return 0 as *mut array_list
    }
    return arr;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_free(mut arr: *mut array_list) {
    let mut i: size_t = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*arr).length {
        if !(*(*arr).array.offset(i as isize)).is_null() {
            (*arr).free_fn.expect("non-null function pointer")(*(*arr).array.offset(i
                                                                                        as
                                                                                        isize));
        }
        i = i.wrapping_add(1)
    }
    free((*arr).array as *mut libc::c_void);
    free(arr as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn array_list_get_idx(mut arr: *mut array_list,
                                            mut i: size_t)
 -> *mut libc::c_void {
    if i >= (*arr).length { return 0 as *mut libc::c_void }
    return *(*arr).array.offset(i as isize);
}
unsafe extern "C" fn array_list_expand_internal(mut arr: *mut array_list,
                                                mut max: size_t)
 -> std::os::raw::c_int {
    let mut t: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_size: size_t = 0;
    if max < (*arr).size { return 0 as std::os::raw::c_int }
    /* Avoid undefined behaviour on size_t overflow */
    if (*arr).size >=
           (0xffffffffffffffff as
                std::os::raw::c_ulong).wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong)
       {
        new_size = max
    } else {
        new_size = (*arr).size << 1 as std::os::raw::c_int;
        if new_size < max { new_size = max }
    }
    if new_size >
           (!(0 as std::os::raw::c_int as
                  size_t)).wrapping_div(::std::mem::size_of::<*mut libc::c_void>()
                                            as std::os::raw::c_ulong) {
        return -(1 as std::os::raw::c_int)
    }
    t =
        realloc((*arr).array as *mut libc::c_void,
                new_size.wrapping_mul(::std::mem::size_of::<*mut libc::c_void>()
                                          as std::os::raw::c_ulong));
    if t.is_null() { return -(1 as std::os::raw::c_int) }
    (*arr).array = t as *mut *mut libc::c_void;
    memset((*arr).array.offset((*arr).size as isize) as *mut libc::c_void,
           0 as std::os::raw::c_int,
           new_size.wrapping_sub((*arr).size).wrapping_mul(::std::mem::size_of::<*mut libc::c_void>()
                                                               as
                                                               std::os::raw::c_ulong));
    (*arr).size = new_size;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_put_idx(mut arr: *mut array_list,
                                            mut idx: size_t,
                                            mut data: *mut libc::c_void)
 -> std::os::raw::c_int {
    if idx >
           (0xffffffffffffffff as
                std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
       {
        return -(1 as std::os::raw::c_int)
    }
    if array_list_expand_internal(arr,
                                  idx.wrapping_add(1 as std::os::raw::c_int as
                                                       std::os::raw::c_ulong)) != 0 {
        return -(1 as std::os::raw::c_int)
    }
    if idx < (*arr).length && !(*(*arr).array.offset(idx as isize)).is_null()
       {
        (*arr).free_fn.expect("non-null function pointer")(*(*arr).array.offset(idx
                                                                                    as
                                                                                    isize));
    }
    let ref mut fresh0 = *(*arr).array.offset(idx as isize);
    *fresh0 = data;
    if (*arr).length <= idx {
        (*arr).length = idx.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_add(mut arr: *mut array_list,
                                        mut data: *mut libc::c_void)
 -> std::os::raw::c_int {
    return array_list_put_idx(arr, (*arr).length, data);
}
#[no_mangle]
pub unsafe extern "C" fn array_list_sort(mut arr: *mut array_list,
                                         mut sort_fn:
                                             Option<unsafe extern "C" fn(_:
                                                                             *const libc::c_void,
                                                                         _:
                                                                             *const libc::c_void)
                                                        -> std::os::raw::c_int>) {
    qsort((*arr).array as *mut libc::c_void, (*arr).length,
          ::std::mem::size_of::<*mut libc::c_void>() as std::os::raw::c_ulong,
          sort_fn);
}
#[no_mangle]
pub unsafe extern "C" fn array_list_bsearch(mut key: *mut *const libc::c_void,
                                            mut arr: *mut array_list,
                                            mut sort_fn:
                                                Option<unsafe extern "C" fn(_:
                                                                                *const libc::c_void,
                                                                            _:
                                                                                *const libc::c_void)
                                                           -> std::os::raw::c_int>)
 -> *mut libc::c_void {
    return bsearch(key as *const libc::c_void,
                   (*arr).array as *const libc::c_void, (*arr).length,
                   ::std::mem::size_of::<*mut libc::c_void>() as
                       std::os::raw::c_ulong, sort_fn);
}
#[no_mangle]
pub unsafe extern "C" fn array_list_length(mut arr: *mut array_list)
 -> size_t {
    return (*arr).length;
}
#[no_mangle]
pub unsafe extern "C" fn array_list_del_idx(mut arr: *mut array_list,
                                            mut idx: size_t,
                                            mut count: size_t)
 -> std::os::raw::c_int {
    let mut i: size_t = 0;
    let mut stop: size_t = 0;
    stop = idx.wrapping_add(count);
    if idx >= (*arr).length || stop > (*arr).length {
        return -(1 as std::os::raw::c_int)
    }
    i = idx;
    while i < stop {
        if !(*(*arr).array.offset(i as isize)).is_null() {
            (*arr).free_fn.expect("non-null function pointer")(*(*arr).array.offset(i
                                                                                        as
                                                                                        isize));
        }
        i = i.wrapping_add(1)
    }
    memmove((*arr).array.offset(idx as isize) as *mut libc::c_void,
            (*arr).array.offset(stop as isize) as *const libc::c_void,
            (*arr).length.wrapping_sub(stop).wrapping_mul(::std::mem::size_of::<*mut libc::c_void>()
                                                              as
                                                              std::os::raw::c_ulong));
    (*arr).length =
        ((*arr).length as std::os::raw::c_ulong).wrapping_sub(count) as size_t as
            size_t;
    return 0 as std::os::raw::c_int;
}
