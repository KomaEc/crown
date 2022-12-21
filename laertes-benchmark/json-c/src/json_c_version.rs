#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
/*
 * Copyright (c) 2012 Eric Haszlakiewicz
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 */
#[no_mangle]
pub unsafe extern "C" fn json_c_version() -> *const std::os::raw::c_char {
    return b"0.13.1\x00" as *const u8 as *const std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn json_c_version_num() -> std::os::raw::c_int {
    return (0 as std::os::raw::c_int) << 16 as std::os::raw::c_int |
               (13 as std::os::raw::c_int) << 8 as std::os::raw::c_int | 0o1 as std::os::raw::c_int;
}
