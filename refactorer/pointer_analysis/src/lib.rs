#![feature(rustc_private)]
#![feature(min_specialization)]
#![feature(box_patterns)]

#[macro_use]
extern crate index;

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_errors;
extern crate rustc_span;
extern crate rustc_target;

pub mod andersen;
