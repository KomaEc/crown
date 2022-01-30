#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
// #![feature(const_transmute)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(main)]
#![feature(ptr_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub mod bitfields;

// pub mod conftest;
pub mod i386_asm;
pub mod libtcc;
pub mod tcc;
pub mod tccasm;
pub mod tccelf;
pub mod tccgen;
pub mod tccpp;
pub mod tccrun;
pub mod x86_64_gen;
pub mod x86_64_link;
