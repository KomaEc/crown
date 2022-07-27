#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(let_else)]

extern crate rustc_hir;
extern crate rustc_hash;
extern crate rustc_index;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;

pub mod fatness;
pub mod mutability;
pub mod null;
mod usage;
