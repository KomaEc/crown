#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(let_chains)]

pub mod complex_place_reporter;
pub mod place_tracer;
pub mod unused_ptr_decl;

extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_session;
extern crate rustc_span;
