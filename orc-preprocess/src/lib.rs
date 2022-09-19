#![feature(rustc_private)]
#![feature(let_else)]

mod ensure_null;
mod linkage;

extern crate rustc_ast;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_middle;
extern crate rustc_span;

use ensure_null::ensure_nullness;
use linkage::{canonicalize_structs, link_functions, link_incomplete_types};
use orc_common::rewrite::{Rewrite, RewriteMode};
use rustc_middle::ty::TyCtxt;

pub trait Preprocess {
    fn preprocess(tcx: TyCtxt, mode: RewriteMode);
}

pub enum Phase<const PHASE: usize> {}

impl Preprocess for Phase<0> {
    fn preprocess(tcx: TyCtxt, mode: RewriteMode) {
        let mut rewriter = Vec::new();

        ensure_nullness(tcx, &mut rewriter);

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
