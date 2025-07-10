#![feature(allocator_api)]
#![feature(array_windows)]
#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_type_ir;

pub use anyhow;
pub use itertools;
pub use petgraph;
pub use rustc_hash;
pub use similar;
pub use smallvec;
pub use tracing;

pub mod dsa;
pub mod libtree;
pub mod rustc;
