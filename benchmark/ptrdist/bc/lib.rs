#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(main)]
#![feature(ptr_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

extern crate libc;

pub mod src {
    pub mod bc;
    pub mod execute;
    pub mod global;
    pub mod load;
    pub mod main;
    pub mod number;
    pub mod scan;
    pub mod storage;
    pub mod util;
} // mod src
