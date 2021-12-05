#![feature(rustc_private)]
#![feature(min_specialization)]
#![feature(box_patterns)]

#[macro_use]
extern crate index;

extern crate rustc_middle;
extern crate rustc_serialize;

pub mod andersen;
