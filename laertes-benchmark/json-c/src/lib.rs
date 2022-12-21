#![feature(linkage)]
#![feature(rustc_private)]
#![feature(register_tool)]

#![feature(
    extern_types,
    libc,
    ptr_offset_from,
    used,
)]
#![feature(c_variadic)]
// #![feature(const_transmute)]
extern crate libc;

pub mod arraylist;
pub mod debug;
pub mod json_c_version;
pub mod json_object;
pub mod json_object_iterator;
pub mod json_pointer;
pub mod json_tokener;
pub mod json_util;
pub mod json_visit;
pub mod linkhash;
pub mod printbuf;
pub mod random_seed;
pub mod strerror_override;

mod c_funcs;
