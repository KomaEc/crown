mod location_map;

use analysis::use_def::{def_use_chain, DefUseChain};
use common::rewrite::Rewrite;
use either::Either::{Left, Right};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{
        Body, Local, LocalInfo, Location, NonDivergingIntrinsic, Rvalue, StatementKind,
        TerminatorKind, VarDebugInfoContents,
    },
    ty::TyCtxt,
};
use rustc_span::{Span, Symbol};

use self::location_map::LocationMap;

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

    let def_use_chain = def_use_chain(body, tcx);

    show_def_use_chain(body, &def_use_chain);

    let mut rewrite_cache: RewriteCache = LocationMap::new(body);

    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        let mut statement_index = 0;
        for statement in bb_data.statements.iter() {
            let location = Location {
                block: bb,
                statement_index,
            };

            match &statement.kind {
                StatementKind::Assign(box (place, rvalue)) => {
                    // rewrite point: non-temporary place
                    // this includes 1. place of which base local is a user defined variable
                    // 2. place of which base local is a deref tmp, and the rvalue is not another deref tmp
                    if user_idents.contains_key(&place.local)
                        || matches!(body.local_decls[place.local].local_info, Some(box LocalInfo::DerefTemp) if !matches!(rvalue, Rvalue::CopyForDeref(..)))
                    {
                    }
                }
                StatementKind::Intrinsic(box NonDivergingIntrinsic::Assume(operand)) => {
                    // rewrite point: assume
                }
                _ => todo!(),
            }

            statement_index += 1;
        }

        if let Some(terminator) = &bb_data.terminator {
            let location = Location {
                block: bb,
                statement_index,
            };

            match &terminator.kind {
                TerminatorKind::SwitchInt { discr, .. } => {
                    // rewrite point: if expr
                }
                TerminatorKind::Call {
                    func,
                    args,
                    destination,
                    fn_span,
                    ..
                } => {
                    // rewrite point: call
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ExprRewriteResult {
    Expr(Span),
}

pub type RewriteCache = LocationMap<Option<ExprRewriteResult>>;

fn rewrite_use<'tcx>(
    body: &Body<'tcx>,
    location: Location,
    user_idents: &FxHashMap<Local, Symbol>,
    def_use_chain: &DefUseChain,
    rewrite_cache: &RewriteCache,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt<'tcx>,
) -> ExprRewriteResult {

    match body.stmt_at(location) {
        Left(statement) => {
            match &statement.kind {
                StatementKind::Assign(box (place, rvalue)) => {
                    match rvalue {
                        Rvalue::Use(_) => todo!(),
                        Rvalue::Repeat(_, _) => todo!(),
                        Rvalue::Ref(_, _, _) => todo!(),
                        Rvalue::ThreadLocalRef(_) => todo!(),
                        Rvalue::AddressOf(_, _) => todo!(),
                        Rvalue::Len(_) => todo!(),
                        Rvalue::Cast(_, _, _) => todo!(),
                        Rvalue::BinaryOp(_, _) => todo!(),
                        Rvalue::CheckedBinaryOp(_, _) => todo!(),
                        Rvalue::NullaryOp(_, _) => todo!(),
                        Rvalue::UnaryOp(_, _) => todo!(),
                        Rvalue::Discriminant(_) => todo!(),
                        Rvalue::Aggregate(_, _) => todo!(),
                        Rvalue::ShallowInitBox(_, _) => todo!(),
                        Rvalue::CopyForDeref(_) => todo!(),
                    }
                }
                StatementKind::Intrinsic(box NonDivergingIntrinsic::Assume(operand)) => {
                }
                _ => todo!(),
            }
        },
        Right(terminator) => todo!(),
    }

    todo!()
}

fn show_def_use_chain(body: &Body, def_use_chain: &DefUseChain) {
    println!("@{:?}", body.source.def_id());
    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        println!("{:?}:", bb);
        let mut statement_index = 0;
        for statement in bb_data.statements.iter() {
            println!("  {:?}", statement);

            let location = Location {
                block: bb,
                statement_index,
            };
            let uses = def_use_chain
                .uses(location)
                .map(|local| (local, def_use_chain.def_loc(local, location)))
                .map(|(local, loc)| format!("{:?}@{:?}", local, loc))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  using: {uses}");

            statement_index += 1;
        }
        if let Some(terminator) = &bb_data.terminator {
            println!("  {:?}", terminator.kind);
            let location = Location {
                block: bb,
                statement_index,
            };
            let uses = def_use_chain
                .uses(location)
                .map(|local| (local, def_use_chain.def_loc(local, location)))
                .map(|(local, loc)| format!("{:?}@{:?}", local, loc))
                .collect::<Vec<_>>()
                .join(", ");
            println!("  using: {uses}");
        }
    }
}
