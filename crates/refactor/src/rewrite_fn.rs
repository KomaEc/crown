mod location_map;

use analysis::{
    ssa::consume::RichLocation,
    use_def::{def_use_chain, DefUseChain},
};
use common::rewrite::Rewrite;
use either::Either::{Left, Right};
use rustc_hash::FxHashMap;
use rustc_hir::{def_id::DefId, ItemKind};
use rustc_middle::{
    mir::{
        Body, Local, LocalInfo, Location, NonDivergingIntrinsic, Place, Rvalue, StatementKind,
        TerminatorKind, VarDebugInfoContents,
    },
    ty::TyCtxt,
};
use rustc_span::{Span, Symbol};
use smallvec::SmallVec;

use self::location_map::LocationMap;
use crate::{rewrite_ty::rewrite_hir_ty, FnLocals, PointerKind, StructFields};

pub fn rewrite_fns(
    fns: &[DefId],
    fn_decision: &FnLocals,
    struct_decision: &StructFields,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
) {
    for &did in fns {
        let local_data = fn_decision.local_data(&did);
        let body = tcx.optimized_mir(did);
        rewrite_fn_sig(body, local_data, rewriter, tcx);
        rewrite_fn(
            body,
            fn_decision.local_data(&did),
            struct_decision,
            rewriter,
            tcx,
        );
    }
}

fn rewrite_fn_sig<'tcx>(
    body: &Body<'tcx>,
    decision: &[SmallVec<[PointerKind; 3]>],
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt<'tcx>,
) {
    let fn_item = tcx.hir().expect_item(body.source.def_id().expect_local());
    let ItemKind::Fn(fn_sig, _, _) = &fn_item.kind else { unreachable!() };

    if let rustc_hir::FnRetTy::Return(ret_ty) = fn_sig.decl.output {
        rewrite_hir_ty(ret_ty, &decision[0], rewriter, tcx);
    }

    for (ty, decision) in itertools::izip!(fn_sig.decl.inputs, &decision[1..]) {
        rewrite_hir_ty(ty, decision, rewriter, tcx);
    }
}

fn rewrite_fn<'tcx>(
    body: &Body<'tcx>,
    local_decision: &[SmallVec<[PointerKind; 3]>],
    struct_decision: &StructFields,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt<'tcx>,
) {
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
                    if user_idents.contains_key(&place.local) {
                        let span = statement.source_info.span;
                        let source_text = tcx.sess.source_map().span_to_snippet(span).unwrap();

                        if let Some(assign_pos) = source_text.chars().position(|c| c == '=') {
                            // lhs needs to be rewritten

                            assert!(assign_pos > 0);

                            if !place.projection.is_empty() {
                                let lhs_span = span
                                    .with_hi(span.lo() + rustc_span::BytePos(assign_pos as u32));

                                rewrite_place_at(
                                    *place,
                                    location,
                                    lhs_span,
                                    local_decision,
                                    struct_decision,
                                    body,
                                    &def_use_chain,
                                    &user_idents,
                                    rewriter,
                                    tcx,
                                );
                            }
                        } // otherwise let-binding
                    } else if matches!(body.local_decls[place.local].local_info, Some(box LocalInfo::DerefTemp) if !matches!(rvalue, Rvalue::CopyForDeref(..)))
                    {
                    }
                }
                StatementKind::Intrinsic(box NonDivergingIntrinsic::Assume(_)) => {
                    // rewrite point: assume
                    let without_semi_span = statement.source_info.span;
                    let span =
                        without_semi_span.with_hi(without_semi_span.hi() + rustc_span::BytePos(1));
                    rewriter.erase(tcx, span);
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
                TerminatorKind::Return => {
                    // rewrite point: return
                }
                TerminatorKind::Goto { .. } => {}
                TerminatorKind::Assert { .. } => {}
                _ => todo!(),
            }
        }
    }
}

pub enum Ctxt<'a> {
    Store(Span),
    Load(LoadCtxt, &'a [PointerKind]),
}

pub enum LoadCtxt {
    Move,
    Borrow,
    BorrowMut,
}

fn rewrite_place_at<'tcx, 'd>(
    place: Place<'tcx>,
    location: Location,
    span: Span,
    // ctxt: Ctxt<'_>,
    local_decision: &'d [SmallVec<[PointerKind; 3]>],
    struct_decision: &'d StructFields,
    body: &Body<'tcx>,
    def_use: &DefUseChain,
    user_idents: &FxHashMap<Local, Symbol>,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt<'tcx>,
) -> Option<impl Iterator<Item = PointerKind> + 'd> {
    let Some(mut replacement) = user_idents.get(&place.local).map(|symbol| symbol.to_string()) else {
        assert!(place.as_local().is_some());
        let def_loc = def_use.def_loc(place.local, location);
        let RichLocation::Mir(loc) = def_loc else {
            todo!()
        };
        // TODO
        return None;
    };

    let mut ptr_kinds = local_decision[place.local.as_usize()].iter().copied();
    let mut ty = body.local_decls[place.local].ty;
    let mut need_paren = false;

    // Invariant: [`replacement`] is a nullable pointer or a struct
    for proj in place.projection {
        if need_paren {
            replacement = format!("({replacement})");
            need_paren = false;
        }

        // perform projection
        match proj {
            rustc_middle::mir::ProjectionElem::Deref => {
                let base_ptr_kind = ptr_kinds.next().unwrap();

                // match base_ptr_kind {
                //     PointerKind::Raw => replacement = format!("*{replacement}"),
                //     PointerKind::Shr => replacement = format!("*{replacement}.clone().unwrap()"),
                //     _ => replacement = format!("*{replacement}.as_deref_mut().unwrap()"),
                // }

                if base_ptr_kind.is_raw() {
                    replacement = format!("*{replacement}");
                } else {
                    replacement = format!("*{replacement}.as_deref_mut().unwrap()");
                }

                need_paren = true;
                ty = ty.builtin_deref(true).unwrap().ty;
            }
            rustc_middle::mir::ProjectionElem::Field(f, field_ty) => {
                assert!(ptr_kinds.next().is_none());

                let adt_def = ty.ty_adt_def().unwrap();
                let field_name = &adt_def.variants()[0usize.into()].fields[f.index()]
                    .name
                    .as_str();
                replacement = replacement + "." + field_name;
                ty = field_ty;

                ptr_kinds = struct_decision.field_data(&adt_def.did())[f.index()]
                    .iter()
                    .copied();
            }
            rustc_middle::mir::ProjectionElem::Index(_) => todo!(),
            _ => unreachable!(),
        }
    }

    rewriter.replace(tcx, span, replacement);

    return Some(ptr_kinds);
}

fn rewrite_rvalue_at<'tcx>(
    rvalue: Rvalue<'tcx>,
    location: Location,
    span: Span,
    // ctxt: Ctxt<'_>,
    local_decision: &[SmallVec<[PointerKind; 3]>],
    struct_decision: &StructFields,
    body: &Body<'tcx>,
    def_use: &DefUseChain,
    user_idents: &FxHashMap<Local, Symbol>,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt<'tcx>,
) {
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
