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
use common::rewrite::{Rewrite, RewriteMode};
use rustc_middle::ty::TyCtxt;

pub const PREPROCESSES: &[for<'r> fn(TyCtxt<'r>, RewriteMode)] = &[phase_1, phase_2, phase_3];

fn phase_1(tcx: TyCtxt, mode: RewriteMode) {
    let mut rewriter = Vec::new();

    ensure_nullness(tcx, &mut rewriter);

    link_incomplete_types(tcx, &mut rewriter);

    rewriter.write(mode)
}

fn phase_2(tcx: TyCtxt, mode: RewriteMode) {
    let mut rewriter = Vec::new();

    canonicalize_structs(tcx, &mut rewriter);

    rewriter.write(mode)
}

fn phase_3(tcx: TyCtxt, mode: RewriteMode) {
    let mut rewriter = Vec::new();

    // TODO link_statics

    link_functions(tcx, &mut rewriter);

    rewriter.write(mode)
}
