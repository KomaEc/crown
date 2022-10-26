use std::ops::Range;

use analysis::use_def::def_use_chain;
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

pub fn rewrite_fns(fns: &[DefId], rewriter: &mut impl Rewrite, tcx: TyCtxt) {
    for &did in fns {
        let body = tcx.optimized_mir(did);
        rewrite_fn(body, rewriter, tcx);
    }
}

fn rewrite_fn<'tcx>(body: &Body<'tcx>, rewriter: &mut impl Rewrite, tcx: TyCtxt<'tcx>) {
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

    println!("@{:?}", body.source.def_id());
    let def_use_chain = def_use_chain(body, tcx);

    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        println!("{:?}:", bb);
        let mut statement_index = 0;
        for statement in bb_data.statements.iter() {
            println!("  {:?}", statement);

            let location = Location { block: bb, statement_index };
            let uses = def_use_chain.uses(location)
                .map(|local| (local, def_use_chain.def_loc(local, location)))
                .map(|(local, loc)| format!("{:?}@{:?}", local, loc))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  using: {uses}");


            statement_index += 1;
        }
        if let Some(terminator) = &bb_data.terminator {
            println!("  {:?}", terminator.kind);
        }
    }
}

pub enum RhsRewriteResult {
    LetBinding,
    Expr(Span),
    Malloc(Span),
}
