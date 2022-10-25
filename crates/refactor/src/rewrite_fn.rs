use std::ops::Range;

use common::rewrite::Rewrite;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{
        BasicBlock, Body, Local, Location, Rvalue, StatementKind, TerminatorKind,
        VarDebugInfoContents,
    },
    ty::TyCtxt,
};
use rustc_span::{Span, Symbol};
use smallvec::{smallvec, SmallVec};



pub fn rewrite_fn(body: &Body, rewriter: &mut impl Rewrite, tcx: TyCtxt) {
    let user_idents = body
        .var_debug_info
        .iter()
        .filter_map(|debug_info| match debug_info.value {
            VarDebugInfoContents::Place(place) => {
                let local = place
                    .as_local()
                    .expect("user variable should be mapped to a local");
                Some((local, debug_info.name))
            }
            _ => None,
        })
        .collect::<FxHashMap<_, _>>();
}

pub enum RhsRewriteResult {
    LetBinding,
    Expr(Span),
    Malloc(Span),
}
