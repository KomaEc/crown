#![feature(rustc_private)]
#![feature(let_else)]

mod linkage;
mod null_stmt;

extern crate rustc_ast;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_middle;
extern crate rustc_span;

use linkage::{canonicalize_structs, link_functions, link_incomplete_types};
use null_stmt::insert_null_statement;
use orc_common::rewrite::{Rewrite, RewriteMode};
use rustc_middle::ty::TyCtxt;


pub trait Preprocess {
    fn preprocess(tcx: TyCtxt, mode: RewriteMode);
}

pub enum Phase<const PHASE: usize> {}


impl Preprocess for Phase<0> {
    fn preprocess(tcx: TyCtxt, mode: RewriteMode) {
        let mut rewriter = Vec::new();

        insert_null_statement(tcx, &mut rewriter);

        link_incomplete_types(tcx, &mut rewriter);

        rewriter.write(mode)
    }
}


impl Preprocess for Phase<1> {
    fn preprocess(tcx: TyCtxt, mode: RewriteMode) {
        let mut rewriter = Vec::new();

        canonicalize_structs(tcx, &mut rewriter);

        // TODO link_statics

        link_functions(tcx, &mut rewriter);

        rewriter.write(mode)
    }
}
