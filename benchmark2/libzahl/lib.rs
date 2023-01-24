#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]
#![feature(strict_provenance)]
#![feature(core_intrinsics)]
#![feature(raw_ref_op)]


extern crate libc;
pub mod src {
pub mod allocator;
pub mod zabs;
pub mod zadd;
pub mod zand;
pub mod zbits;
pub mod zbset;
pub mod zbtest;
pub mod zcmp;
pub mod zcmpi;
pub mod zcmpmag;
pub mod zcmpu;
pub mod zdiv;
pub mod zdivmod;
pub mod zerror;
pub mod zfree;
pub mod zgcd;
pub mod zload;
pub mod zlsb;
pub mod zlsh;
pub mod zmod;
pub mod zmodmul;
pub mod zmodpow;
pub mod zmodpowu;
pub mod zmodsqr;
pub mod zmul;
pub mod zneg;
pub mod znot;
pub mod zor;
pub mod zperror;
pub mod zpow;
pub mod zpowu;
pub mod zptest;
pub mod zrand;
pub mod zrsh;
pub mod zsave;
pub mod zset;
pub mod zseti;
pub mod zsets;
pub mod zsetu;
pub mod zsetup;
pub mod zsplit;
pub mod zsqr;
pub mod zstr;
pub mod zstr_length;
pub mod zsub;
pub mod zswap;
pub mod ztrunc;
pub mod zunsetup;
pub mod zxor;
} // mod src
