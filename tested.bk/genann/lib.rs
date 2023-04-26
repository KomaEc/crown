#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![feature(strict_provenance)]
#![feature(core_intrinsics)]
#![feature(raw_ref_op)]


extern crate libc;
pub mod src {
pub mod example1;
pub mod example2;
pub mod example3;
pub mod example4;
pub mod genann;
pub mod test;
} // mod src
