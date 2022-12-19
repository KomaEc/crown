#![feature(rustc_private)]

mod fold_let_ref_mut;
mod linkage;
mod signal_nullness;

extern crate rustc_ast;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_middle;
extern crate rustc_span;

use common::rewrite::{Rewrite, RewriteMode};
use linkage::{canonicalize_structs, link_functions, link_incomplete_types};
use rustc_hir::{Item, OwnerNode};
use rustc_middle::ty::TyCtxt;
use signal_nullness::signal_nullness;

pub const PREPROCESSES: &[for<'r> fn(TyCtxt<'r>, RewriteMode)] = &[phase_1, phase_2, phase_3, phase_4];

fn phase_1(tcx: TyCtxt, mode: RewriteMode) {
    let mut rewriter = Vec::new();

    signal_nullness(tcx, &mut rewriter);

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

fn phase_4(tcx: TyCtxt, mode: RewriteMode) {
    fold_let_ref_mut(tcx, mode)
}

pub use fold_let_ref_mut::fold_let_ref_mut;

fn owner_items(tcx: TyCtxt) -> impl Iterator<Item = &'_ Item<'_>> {
    tcx.hir()
        .krate()
        .owners
        .iter()
        .filter_map(|maybe_owner| maybe_owner.as_owner())
        .filter_map(|owner| {
            if let OwnerNode::Item(item) = owner.node() {
                Some(item)
            } else {
                None
            }
        })
}
