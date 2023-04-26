#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![feature(strict_provenance)]
#![feature(core_intrinsics)]
#![feature(raw_ref_op)]


extern crate libc;
pub mod src {
pub mod src {
pub mod bounds;
pub mod node;
pub mod point;
pub mod quadtree;
} // mod src
pub mod test;
} // mod src
