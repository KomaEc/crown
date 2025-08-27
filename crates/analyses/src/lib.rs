#![feature(rustc_private)]
#![feature(array_windows)]
#![feature(box_patterns)]
#![feature(min_specialization)]

extern crate rustc_abi;
// extern crate rustc_borrowck;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_span;
extern crate rustc_type_ir;

pub mod access_path;
pub mod alias;
// #[allow(unused)]
pub mod borrow;
mod encoding;
pub mod lattice;
pub mod liveness;
pub mod mir;
mod output_params;
pub(crate) mod reaching_definitions;
#[allow(unused)]
mod ssa;
pub mod type_qualifier;
pub mod use_def;
