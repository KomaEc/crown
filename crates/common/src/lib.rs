#![feature(rustc_private)]
#![feature(step_trait)]
#![feature(trusted_step)]
#![feature(min_specialization)]
#![feature(associated_type_defaults)]
#![feature(split_array)]
#![feature(array_windows)]
#![feature(allocator_api)]

extern crate rustc_abi;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_type_ir;

pub mod captures;
pub mod compiler_interface;
pub mod data_structure;
pub mod discretization;
pub mod macros;
pub mod rewrite;
pub mod struct_dependency;
pub mod tracing_setup;

