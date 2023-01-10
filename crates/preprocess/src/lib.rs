#![feature(rustc_private)]

mod char_array_transmute;
mod explicit_addr;
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
use explicit_addr::explicit_addr;
use linkage::{canonicalize_structs, link_functions, link_incomplete_types};
use rustc_hir::{Item, OwnerNode};
use rustc_middle::ty::TyCtxt;

use crate::signal_nullness::signal_nullness;

pub const PREPROCESSES: &[for<'r> fn(TyCtxt<'r>, RewriteMode)] = &[
    char_array_transmute,
    fold_let_ref_mut,
    signal_nullness,
    link_incomplete_types,
    canonicalize_structs,
    link_functions,
];

pub use char_array_transmute::char_array_transmute;
pub use fold_let_ref_mut::fold_let_ref_mut;

pub fn use_explicit_addr(tcx: TyCtxt, mode: RewriteMode) {
    let mut rewriter = Vec::new();
    explicit_addr(tcx, &mut rewriter);
    rewriter.write(mode)
}

fn perform_rewrite<F>(mut f: F, tcx: TyCtxt, mode: RewriteMode)
where
    F: FnMut(TyCtxt<'_>, &mut Vec<rustfix::Suggestion>),
{
    let mut rewriter = Vec::new();
    f(tcx, &mut rewriter);
    rewriter.write(mode)
}

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
