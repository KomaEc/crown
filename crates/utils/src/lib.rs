#![feature(allocator_api)]
#![feature(array_windows)]
#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(if_let_guard)]

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_type_ir;
extern crate thin_vec;

pub use anyhow;
pub use either;
pub use itertools;
pub use petgraph;
pub use rustc_hash;
pub use similar;
pub use smallvec;
pub use tracing;

pub mod ast_util;
pub mod dsa;
pub mod ir_util;
pub mod libtree;
pub mod rewrite;
pub mod rustc;
#[cfg(feature = "test-utils")]
pub mod test;
