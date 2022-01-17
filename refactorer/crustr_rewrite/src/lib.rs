#![feature(rustc_private)]

// use rustfix::{Replacement, Snippet, Solution, Suggestion};

extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_passes;
extern crate rustc_span;

pub mod initial_rewrite;
