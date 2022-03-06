#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![feature(ptr_internals)]
#![feature(new_uninit)]
#![feature(maybe_uninit_slice)]
#![feature(slice_ptr_get)]

extern crate libc;

pub mod src {
    pub mod buffer_slice;
    // pub mod test;
} // mod src
