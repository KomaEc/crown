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

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod common {
pub mod constants;
pub mod context;
pub mod dictionary;
pub mod platform;
pub mod transform;
} // mod common
pub mod dec {
pub mod bit_reader;
pub mod decode;
pub mod huffman;
pub mod state;
} // mod dec
pub mod enc {
pub mod backward_references;
pub mod backward_references_hq;
pub mod bit_cost;
pub mod block_splitter;
pub mod brotli_bit_stream;
pub mod cluster;
pub mod command;
pub mod compress_fragment;
pub mod compress_fragment_two_pass;
pub mod dictionary_hash;
pub mod encode;
pub mod encoder_dict;
pub mod entropy_encode;
pub mod fast_log;
pub mod histogram;
pub mod literal_cost;
pub mod memory;
pub mod metablock;
pub mod static_dict;
pub mod utf8_util;
} // mod enc
pub mod tools {
pub mod brotli;
} // mod tools
} // mod src
