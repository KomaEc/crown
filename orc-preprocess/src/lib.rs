#![feature(rustc_private)]
#![feature(let_else)]

mod null_stmt;
mod linkage;

extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_middle;
extern crate rustc_span;

use linkage::{link_functions, link_incomplete_types, canonicalize_structs};
use null_stmt::insert_null_statement;
use orc_common::rewrite::{RewriteMode, Rewrite};
use rustc_middle::ty::TyCtxt;

pub fn preprocess(tcx: TyCtxt, mode: RewriteMode) {
    let mut rewriter = Vec::new();
    // desugar_while_loop(tcx, mode)
    insert_null_statement(tcx, &mut rewriter);

    link_functions(tcx, &mut rewriter);

    link_incomplete_types(tcx, &mut rewriter);

    canonicalize_structs(tcx, &mut rewriter);
    
    rewriter.write(mode)
}
