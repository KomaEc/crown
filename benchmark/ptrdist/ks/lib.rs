#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_transmute)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]

extern crate libc;

pub mod src {
    pub mod KS_1;
    pub mod KS_2;
} // mod src
