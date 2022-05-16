pub mod rewrite_libc_call;
mod rewrite_library_call;

use crate::rewrite::rewrite_body::rewrite_libc_call::rewrite_libc_call;
use crate::rewrite::rewrite_body::rewrite_library_call::rewrite_library_call;
use analysis::{
    call_graph::Func, fat_thin_analysis, mutability_analysis, ownership_analysis,
    ssa::RichLocation, ty_ext::TyExt,
};
use either::Either;
use rewriter::Rewriter;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::LocalDefId;
use rustc_index::{
    bit_set::BitSet,
    vec::{Idx, IndexVec},
};
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, Body, Local, Location, Operand, Place, ProjectionElem, Rvalue,
        Statement, StatementKind, Terminator, TerminatorKind, VarDebugInfoContents,
    },
    ty::{TyCtxt, TyKind},
};
use rustc_span::{Span, Symbol};

use super::Ownership;

pub fn rewrite_fn_body(
    tcx: TyCtxt<'_>,
    rewriter: &mut Rewriter,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    func: Func,
    fn_did: LocalDefId,
) {
    let body = tcx.optimized_mir(fn_did);

    let mut user_vars = BitSet::new_empty(body.local_decls.len());
    let mut names = FxHashMap::default();

    for var_debug in &body.var_debug_info {
        match var_debug.value {
            VarDebugInfoContents::Place(place) => {
                let local = place
                    .as_local()
                    .expect("user variable should be mapped to a local");
                //if body.local_decls[local].ty.is_ptr_but_not_fn_ptr() {
                user_vars.insert(local);
                names.insert(local, var_debug.name);
                //}
            }
            VarDebugInfoContents::Const(constant) => {
                tracing::warn!("user constant {:?} is not processed", constant)
            }
        }
    }

    struct StatementRewriteVisitor<'me, 'tcx> {
        tcx: TyCtxt<'tcx>,
        rewriter: &'me mut Rewriter,
        body: &'me Body<'tcx>,
        func: Func,
        ownership_analysis: &'me ownership_analysis::InterSummary,
        mutability_analysis: &'me mutability_analysis::InterSummary,
        fatness_analysis: &'me fat_thin_analysis::CrateSummary,
        user_vars: &'me BitSet<Local>,
        names: &'me FxHashMap<Local, Symbol>,
        editted_locations: IndexVec<BasicBlock, BitSet<usize>>,
    }

    impl<'me, 'tcx> Visitor<'tcx> for StatementRewriteVisitor<'me, 'tcx> {
        // fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
            if let StatementKind::Assign(box (place, _)) = &statement.kind {
                if self.user_vars.contains(place.local)
                //&& place.ty(self.body, self.tcx).ty.is_ptr_but_not_fn_ptr()
                {
                    if let RewriteUseResult::LHSToBeRewritten { rhs_span } = rewrite_use(
                        self.tcx,
                        &mut *self.rewriter,
                        self.body,
                        self.func,
                        self.ownership_analysis,
                        self.mutability_analysis,
                        self.fatness_analysis,
                        self.user_vars,
                        self.names,
                        location,
                        &mut self.editted_locations,
                    ) {
                        // we need to further rewrite lhs

                        if rhs_span.contains(statement.source_info.span) {
                            // this occurs upon let binding
                            assert_eq!(rhs_span, statement.source_info.span);
                            assert!(place.as_local().is_some());
                        } else {
                            let replacement = mutating_ctxt_replacement(
                                self.tcx,
                                self.body,
                                place,
                                self.names[&place.local],
                                MutatingCtxt::Store,
                            );
                            self.rewriter.make_suggestion(
                                self.tcx,
                                statement.source_info.span.shrink_to_lo().until(rhs_span),
                                "rewrite lhs".to_owned(),
                                format!("{replacement} = "),
                            );
                        }
                    }
                }
            }
            self.super_statement(statement, location)
        }
    }

    let mut statement_rewrite_phase = StatementRewriteVisitor {
        tcx,
        rewriter,
        body,
        func,
        ownership_analysis,
        mutability_analysis,
        fatness_analysis,
        user_vars: &user_vars,
        names: &names,
        editted_locations: body
            .basic_blocks()
            .iter()
            .map(|bb| BitSet::new_empty(bb.statements.len() + 1))
            .collect(),
    };

    statement_rewrite_phase.visit_body(body);

    let mut editted_locations = statement_rewrite_phase.editted_locations;

    for (bb, bb_data) in body.basic_blocks().iter_enumerated() {
        let location = Location {
            block: bb,
            statement_index: bb_data.statements.len(),
        };
        if !editted_locations[location.block].contains(location.statement_index) {
            let _ = rewrite_terminator(
                tcx,
                rewriter,
                body,
                func,
                ownership_analysis,
                mutability_analysis,
                fatness_analysis,
                &user_vars,
                &names,
                bb_data.terminator(),
                location,
                &mut editted_locations,
            );
        }
    }

    for (bb, bb_data) in body.basic_blocks().iter_enumerated() {
        let mut location = Location {
            block: bb,
            statement_index: 0,
        };
        for statement in &bb_data.statements {
            if let StatementKind::Assign(box (lhs, rvalue)) = &statement.kind {
                if !editted_locations[location.block].contains(location.statement_index) {
                    assert!(!user_vars.contains(lhs.local));
                    // we rewrite all the resting temporaries
                    if let Rvalue::Use(rhs) = rvalue {
                        if let Some(rhs) = rhs.place() {
                            if !user_vars.contains(rhs.local) {
                                continue;
                            }

                            let replacement = nonmutating_ctxt_replacement(
                                tcx,
                                body,
                                &rhs,
                                names[&rhs.local],
                                false,
                                NonMutatingCtxt::Copy,
                            );

                            rewriter.make_suggestion(
                                tcx,
                                statement.source_info.span,
                                "rewrite uses".to_string(),
                                replacement,
                            );
                        }
                    }
                }
            }

            location.statement_index += 1;
        }
    }
}

pub enum RewriteUseResult {
    Done,
    LHSToBeRewritten { rhs_span: Span },
    FromMalloc { malloc_span: Span },
}

/// rewrite the use of local `local` at location `location`
/// This is a recursive procedure that traces a chain of
/// intermediate statements until the RHS of the statement
/// is related to user_ptr
fn rewrite_use<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    body: &Body<'tcx>,
    func: Func,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    user_vars: &BitSet<Local>,
    names: &FxHashMap<Local, Symbol>,
    location: Location,
    editted_locations: &mut IndexVec<BasicBlock, BitSet<usize>>,
) -> RewriteUseResult {
    match body.stmt_at(location) {
        Either::Left(statement) => {
            assert!(
                editted_locations[location.block].insert(location.statement_index),
                "{:?}: {:?}",
                body.source.instance.def_id(),
                statement
            );

            match &statement.kind {
                StatementKind::Assign(box (lhs, rhs)) => {
                    let maybe_ptr_sig = lhs.ty(body, tcx).ty.is_ptr_but_not_fn_ptr().then(|| {
                        ptr_place_sig(
                            tcx,
                            body,
                            func,
                            ownership_analysis,
                            mutability_analysis,
                            fatness_analysis,
                            lhs,
                            location,
                        )
                    });

                    match rhs {
                        Rvalue::Use(rhs) => {
                            match rhs {
                                Operand::Copy(rhs) | Operand::Move(rhs) => {
                                    // end recursion right here
                                    if user_vars.contains(rhs.local) {
                                        // perform the rewrite

                                        match maybe_ptr_sig {
                                            None => {
                                                let is_base_ptr_clonable = body.local_decls
                                                    [rhs.local]
                                                    .ty
                                                    .is_ptr_but_not_fn_ptr()
                                                    && mutability_analysis.func_summaries[func]
                                                        .ssa_name_source_map
                                                        .try_use(rhs.local, location)
                                                        .map(|ssa_idx| {
                                                            let mu = mutability_analysis
                                                                .func_summaries[func]
                                                                .locals[rhs.local][ssa_idx];
                                                            !mutability_analysis
                                                                .approximate_mu_ctxt
                                                                .get()
                                                                .unwrap()[func][mu]
                                                        })
                                                        .unwrap();
                                                let replacement = nonmutating_ctxt_replacement(
                                                    tcx,
                                                    body,
                                                    rhs,
                                                    names[&rhs.local],
                                                    is_base_ptr_clonable,
                                                    NonMutatingCtxt::Copy,
                                                );

                                                rewriter.make_suggestion(
                                                    tcx,
                                                    statement.source_info.span,
                                                    "rewrite rhs".to_owned(),
                                                    replacement,
                                                );
                                            }
                                            Some((ownership_ctxt, fatness_ctxt)) => {
                                                match ownership_ctxt {
                                                    Ownership::Owning => {
                                                        // std::mem::take this pointer

                                                        let replacement = mutating_ctxt_replacement(
                                                            tcx,
                                                            body,
                                                            rhs,
                                                            names[&rhs.local],
                                                            MutatingCtxt::Move,
                                                        );

                                                        rewriter.make_suggestion(
                                                            tcx,
                                                            statement.source_info.span,
                                                            "rewrite rhs".to_owned(),
                                                            replacement,
                                                        );
                                                    }
                                                    Ownership::Transient { mutbl } if mutbl => {
                                                        let replacement = mutating_ctxt_replacement(
                                                            tcx,
                                                            body,
                                                            rhs,
                                                            names[&rhs.local],
                                                            MutatingCtxt::Borrow,
                                                        );

                                                        rewriter.make_suggestion(
                                                            tcx,
                                                            statement.source_info.span,
                                                            "rewrite rhs".to_owned(),
                                                            replacement,
                                                        );
                                                    }
                                                    Ownership::Transient { .. } => {
                                                        let is_base_ptr_clonable = body.local_decls
                                                            [rhs.local]
                                                            .ty
                                                            .is_ptr_but_not_fn_ptr()
                                                            && mutability_analysis.func_summaries
                                                                [func]
                                                                .ssa_name_source_map
                                                                .try_use(rhs.local, location)
                                                                .map(|ssa_idx| {
                                                                    let mu = mutability_analysis
                                                                        .func_summaries[func]
                                                                        .locals[rhs.local][ssa_idx];
                                                                    !mutability_analysis
                                                                        .approximate_mu_ctxt
                                                                        .get()
                                                                        .unwrap()[func][mu]
                                                                })
                                                                .unwrap();
                                                        let replacement =
                                                            nonmutating_ctxt_replacement(
                                                                tcx,
                                                                body,
                                                                rhs,
                                                                names[&rhs.local],
                                                                is_base_ptr_clonable,
                                                                NonMutatingCtxt::Borrow,
                                                            );

                                                        rewriter.make_suggestion(
                                                            tcx,
                                                            statement.source_info.span,
                                                            "rewrite rhs".to_owned(),
                                                            replacement,
                                                        );
                                                    }
                                                    Ownership::Raw => todo!(),
                                                }
                                            }
                                        }

                                        // for simple statement like _0 = _1 !!!!!
                                        // assert!(!user_vars.contains(lhs.local), "{:?}", statement) does not hold
                                        // however, if it is indeed the case, we don't have to rewrite lhs!!!!!!
                                        // so it's fine?
                                        if user_vars.contains(lhs.local) {
                                            return RewriteUseResult::Done;
                                        } else {
                                            return RewriteUseResult::LHSToBeRewritten {
                                                rhs_span: statement.source_info.span,
                                            };
                                        }
                                    } else {
                                        // This is the assumption we make that it is impossible
                                        // to have something like (*_1).2 = (*_3).4
                                        assert!(user_vars.contains(lhs.local));
                                        assert!(rhs.projection.is_empty());
                                        let rhs = rhs.local;
                                        // we consult the def-use chain of fatness analysis
                                        let source_map =
                                            &fatness_analysis.ssa_name_source_map[func];
                                        let fatness_ssa_idx =
                                            source_map.try_use(rhs, location).unwrap();
                                        let def_rich_location = &fatness_analysis.def_sites[func]
                                            .defs[rhs][fatness_ssa_idx];
                                        let def_location = match def_rich_location {
                                            &RichLocation::Mir(l) => l,
                                            &RichLocation::Phi(_) => todo!(),
                                            // we cannot end up in this branch, since
                                            // rhs is not user variable and must be initialised
                                            RichLocation::Entry => unreachable!(),
                                        };
                                        return rewrite_use(
                                            tcx,
                                            rewriter,
                                            body,
                                            func,
                                            ownership_analysis,
                                            mutability_analysis,
                                            fatness_analysis,
                                            user_vars,
                                            names,
                                            def_location,
                                            editted_locations,
                                        );
                                    }
                                }
                                Operand::Constant(box constant) =>
                                // This is definitely wrong, but we assume that only null pointer can be pointer constant
                                {
                                    if let Some((ownership_ctxt, fatness_ctxt)) = maybe_ptr_sig {
                                        rewrite_null_constant(
                                            tcx,
                                            rewriter,
                                            body,
                                            ownership_ctxt,
                                            fatness_ctxt,
                                            user_vars,
                                            names,
                                            lhs,
                                            statement.source_info.span,
                                        )
                                    } else {
                                        RewriteUseResult::Done
                                    }
                                }
                            }
                        }
                        Rvalue::Cast(_, Operand::Constant(box constant), _) => {
                            if let Some((ownership_ctxt, fatness_ctxt)) = maybe_ptr_sig {
                                rewrite_null_constant(
                                    tcx,
                                    rewriter,
                                    body,
                                    ownership_ctxt,
                                    fatness_ctxt,
                                    user_vars,
                                    names,
                                    lhs,
                                    statement.source_info.span,
                                )
                            } else {
                                RewriteUseResult::Done
                            }
                        }

                        Rvalue::Cast(_, Operand::Copy(rhs) | Operand::Move(rhs), _) => {
                            if maybe_ptr_sig.is_none() {
                                return RewriteUseResult::Done;
                            }
                            let (ownership_ctxt, fatness_ctxt) = maybe_ptr_sig.unwrap();

                            if user_vars.contains(rhs.local) {
                                todo!()
                            }

                            assert!(rhs.as_local().is_some());
                            if let Some(rhs) = rhs.as_local() {
                                // if it is under a safe pointer context
                                if !matches!(ownership_ctxt, Ownership::Raw) {
                                    let source_map = &fatness_analysis.ssa_name_source_map[func];
                                    let fatness_ssa_idx =
                                        source_map.try_use(rhs, location).unwrap();
                                    let def_rich_location = &fatness_analysis.def_sites[func].defs
                                        [rhs][fatness_ssa_idx];
                                    let def_location = match def_rich_location {
                                        &RichLocation::Mir(l) => l,
                                        &RichLocation::Phi(_) => todo!(),
                                        // we cannot end up in this branch, since
                                        // rhs is not user variable and must be initialised
                                        RichLocation::Entry => unreachable!(),
                                    };
                                    return match rewrite_use(
                                        tcx,
                                        rewriter,
                                        body,
                                        func,
                                        ownership_analysis,
                                        mutability_analysis,
                                        fatness_analysis,
                                        user_vars,
                                        names,
                                        def_location,
                                        editted_locations,
                                    ) {
                                        RewriteUseResult::Done => unreachable!(),
                                        RewriteUseResult::LHSToBeRewritten { rhs_span } => {
                                            rewriter.make_suggestion(
                                                tcx,
                                                rhs_span.shrink_to_hi().between(
                                                    statement.source_info.span.shrink_to_hi(),
                                                ),
                                                "remove cast".to_owned(),
                                                "".to_owned(),
                                            );
                                            RewriteUseResult::LHSToBeRewritten {
                                                rhs_span: statement.source_info.span,
                                            }
                                        }
                                        RewriteUseResult::FromMalloc { malloc_span } => {
                                            rewriter.make_suggestion(
                                                tcx,
                                                malloc_span.shrink_to_hi().between(
                                                    statement.source_info.span.shrink_to_hi(),
                                                ),
                                                "remove cast".to_owned(),
                                                "".to_owned(),
                                            );
                                            // TODO: this type needs complete re-computing
                                            let ty = lhs
                                                .ty(body, tcx)
                                                .projection_ty(tcx, ProjectionElem::Deref)
                                                .ty;
                                            let new_type_str = match ty.kind() {
                                                TyKind::Adt(adt_def, _) => tcx
                                                    .hir()
                                                    .expect_item(adt_def.did.expect_local())
                                                    .ident
                                                    .as_str(),
                                                _ if ty.is_primitive() => {
                                                    todo!()
                                                }
                                                _ => todo!(),
                                            };
                                            rewriter.make_suggestion(
                                                tcx,
                                                malloc_span,
                                                "removing malloc".to_string(),
                                                format!(
                                                    "Some(Box::new(<{new_type_str} as Default>::default()))",
                                                ),
                                            );
                                            RewriteUseResult::LHSToBeRewritten {
                                                rhs_span: statement.source_info.span,
                                            }
                                        }
                                    };
                                    /*
                                    return rewrite_use(
                                        tcx,
                                        rewriter,
                                        body,
                                        func,
                                        ownership_analysis,
                                        mutability_analysis,
                                        fatness_analysis,
                                        user_vars,
                                        names,
                                        def_location,
                                        editted_locations,
                                    )
                                    .map(|inner_span| {
                                        rewriter.make_suggestion(
                                            tcx,
                                            inner_span
                                                .shrink_to_hi()
                                                .between(statement.source_info.span.shrink_to_hi()),
                                            "remove cast".to_owned(),
                                            "".to_owned(),
                                        );
                                        statement.source_info.span
                                    });
                                    */
                                } else {
                                    todo!()
                                }
                            } else {
                                unreachable!()
                            }
                        }
                        _ => RewriteUseResult::Done,
                    }
                }
                _ => todo!(),
            }
        }
        Either::Right(terminator) => rewrite_terminator(
            tcx,
            rewriter,
            body,
            func,
            ownership_analysis,
            mutability_analysis,
            fatness_analysis,
            user_vars,
            names,
            terminator,
            location,
            editted_locations,
        ),
    }
}

fn rewrite_terminator<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    body: &Body<'tcx>,
    caller: Func,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    user_vars: &BitSet<Local>,
    names: &FxHashMap<Local, Symbol>,
    terminator: &Terminator<'tcx>,
    location: Location,
    editted_locations: &mut IndexVec<BasicBlock, BitSet<usize>>,
) -> RewriteUseResult {
    assert!(
        editted_locations[location.block].insert(location.statement_index),
        "{:?}: {:?}",
        body.source.instance.def_id(),
        terminator.kind
    );

    match &terminator.kind {
        TerminatorKind::Return => {
            return RewriteUseResult::Done;
        }
        TerminatorKind::Call {
            func: callee,
            args,
            destination,
            cleanup,
            from_hir_call,
            fn_span,
        } => {
            let ty = callee
                .constant()
                .expect("closures or function pointers are not supported!")
                .ty();
            if let &rustc_middle::ty::TyKind::FnDef(callee_did, _generic_args) = ty.kind() {
                match callee_did.as_local() {
                    Some(did) => {
                        if matches!(
                            tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::ForeignItem(_))
                        ) {
                            // self.model_libc_call(callee_did, args, destination, location);
                            // return;
                            return rewrite_libc_call(
                                tcx,
                                rewriter,
                                body,
                                caller,
                                ownership_analysis,
                                mutability_analysis,
                                fatness_analysis,
                                user_vars,
                                names,
                                callee_did,
                                args,
                                *destination,
                                *fn_span,
                                location,
                                editted_locations,
                            );
                            //.then(|| {
                            // panic!("malloc span {:?}", terminator.source_info.span);
                            //    Some(terminator.source_info.span)
                            //})
                            //.unwrap_or_else(|| None);
                        } else if matches!(
                            tcx.hir().find_by_def_id(did),
                            Some(rustc_hir::Node::Item(_))
                        ) {
                            /*
                            TODO: check raw context?
                            let (ownership_sigs, mutability_sigs) = (
                                &ownership_analysis.func_sigs[ownership_analysis
                                    .call_graph
                                    .lookup_function(&callee_did)
                                    .unwrap()]
                                    .sig,
                                &mutability_analysis.func_sigs[mutability_analysis
                                    .call_graph
                                    .lookup_function(&callee_did)
                                    .unwrap()]
                                    .sig,
                            );
                            */

                            for arg in args {
                                if let Some(arg) = arg.place() {
                                    let arg = arg
                                        .as_local()
                                        .expect("arguments are assumed to be temporaries");
                                    assert!(
                                        !user_vars.contains(arg),
                                        "arguments are assumed to be temporaries"
                                    );
                                    let source_map = &fatness_analysis.ssa_name_source_map[caller];
                                    let fatness_ssa_idx =
                                        source_map.try_use(arg, location).unwrap();
                                    let def_rich_location = &fatness_analysis.def_sites[caller]
                                        .defs[arg][fatness_ssa_idx];
                                    let def_location = match def_rich_location {
                                        &RichLocation::Mir(l) => l,
                                        &RichLocation::Phi(_) => todo!(),
                                        // we cannot end up in this branch, since
                                        // rhs is not user variable and must be initialised
                                        RichLocation::Entry => unreachable!(),
                                    };

                                    let _ = rewrite_use(
                                        tcx,
                                        rewriter,
                                        body,
                                        caller,
                                        ownership_analysis,
                                        mutability_analysis,
                                        fatness_analysis,
                                        user_vars,
                                        names,
                                        def_location,
                                        editted_locations,
                                    );
                                } else {
                                    unreachable!("arguments are not allowed to be constants")
                                }
                            }

                            if let Some((dest, _)) = *destination {
                                // if destination place is a temporary
                                if !user_vars.contains(dest.local) {
                                    return RewriteUseResult::LHSToBeRewritten {
                                        rhs_span: terminator.source_info.span,
                                    };
                                } else {
                                    // assert!(false, "destination place is assumed to be a temporary in {:?}", terminator.kind)
                                    assert!(dest.projection.is_empty(), "destination place is assumed to be a local, and we need not rewrite")
                                }
                            }
                            // self.model_boundary(callee_did, args, destination, location);
                            // return;
                            return RewriteUseResult::Done; //todo!()
                        }

                        unreachable!()
                    }
                    None => {
                        // self.model_library_call(callee_did, args, destination, location);
                        // return;
                        rewrite_library_call(
                            tcx,
                            rewriter,
                            body,
                            caller,
                            ownership_analysis,
                            mutability_analysis,
                            fatness_analysis,
                            user_vars,
                            names,
                            callee_did,
                            args,
                            *destination,
                            *fn_span,
                            location,
                            editted_locations,
                        );
                        RewriteUseResult::Done
                    }
                }
            } else {
                unreachable!()
            }
        }
        _ => RewriteUseResult::Done,
    }
}

#[derive(Clone, Copy)]
pub enum MutatingCtxt {
    Store,
    Move,
    Borrow,
}

#[derive(Clone, Copy)]
pub enum NonMutatingCtxt {
    Copy,
    Borrow,
}

/// we don't rewrite fatness for now
/// this logic only works for places of the following form:
/// 1. (*..*s).f
/// 2. s.f
/// 3. p
/// TODO: update ownership context when going through fields!!!!!!!!
fn mutating_ctxt_replacement<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    place: &Place<'tcx>,
    var_name: Symbol,
    mutating_ctxt: MutatingCtxt,
) -> String {
    let mut replacement = var_name.to_string();
    let mut need_paren = false;
    // logic of this loop: when there is projection, immediately reborrow parent path
    for (base, proj) in place.iter_projections() {
        if need_paren {
            replacement = format!("({replacement})");
            need_paren = false;
        }
        let base_is_ptr = base.ty(body, tcx).ty.is_ptr_but_not_fn_ptr();
        if base_is_ptr {
            replacement += ".as_deref_mut().unwrap()"; // ".as_mut().map(|mut x| &mut **x)";
        }
        match proj {
            ProjectionElem::Deref => {
                assert!(base_is_ptr);
                replacement = format!("*{replacement}");
                need_paren = true;
            }
            ProjectionElem::Field(f, _) => {
                let place_ty = base.ty(body, tcx);
                let ty = place_ty.ty;
                let variant_idx = place_ty
                    .variant_index
                    .unwrap_or(rustc_target::abi::VariantIdx::new(0));
                let adt_def = ty.ty_adt_def().unwrap();
                let field_def = &adt_def.variants[variant_idx].fields[f.index()];
                let field_name = field_def.name.as_str();
                replacement = replacement + "." + field_name;
            }
            _ => todo!(),
        }
    }

    match mutating_ctxt {
        MutatingCtxt::Store => {}
        MutatingCtxt::Move => {
            if place.is_indirect() {
                if need_paren {
                    replacement = format!("({replacement})");
                }
                replacement += ".take()"; // format!("std::mem::take(&mut {replacement})");
            }
        }
        MutatingCtxt::Borrow => {
            if need_paren {
                replacement = format!("({replacement})");
            }
            replacement += ".as_deref_mut()"
        }
    }
    replacement
}

fn nonmutating_ctxt_replacement<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    place: &Place<'tcx>,
    var_name: Symbol,
    mut is_base_ptr_clonable: bool,
    nonmutating_ctxt: NonMutatingCtxt,
) -> String {
    let mut replacement = var_name.to_string();
    let mut need_paren = false;
    // logic of this loop: when there is projection, immediately reborrow parent path
    for (base, proj) in place.iter_projections() {
        if need_paren {
            replacement = format!("({replacement})");
            need_paren = false;
        }
        if is_base_ptr_clonable {
            replacement += ".clone().unwrap()";
            is_base_ptr_clonable = false;
        } else if base.ty(body, tcx).ty.is_ptr_but_not_fn_ptr() {
            replacement += ".as_deref().unwrap()"; // ".as_mut().map(|mut x| &mut **x)";
        }
        match proj {
            ProjectionElem::Deref => {
                replacement = format!("*{replacement}");
                need_paren = true;
            }
            ProjectionElem::Field(f, _) => {
                let place_ty = base.ty(body, tcx);
                let ty = place_ty.ty;
                let variant_idx = place_ty
                    .variant_index
                    .unwrap_or(rustc_target::abi::VariantIdx::new(0));
                let adt_def = ty.ty_adt_def().unwrap();
                let field_def = &adt_def.variants[variant_idx].fields[f.index()];
                let field_name = field_def.name.as_str();
                replacement = replacement + "." + field_name;
            }
            _ => todo!(),
        }
    }

    match nonmutating_ctxt {
        NonMutatingCtxt::Copy => {}
        NonMutatingCtxt::Borrow => {
            if need_paren {
                replacement = format!("({replacement})");
            }
            if is_base_ptr_clonable {
                replacement += ".clone()";
            } else {
                replacement += ".as_deref()";
            }
        }
    }
    replacement
}

#[inline]
fn rewrite_null_constant<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    body: &Body<'tcx>,
    ownership_ctxt: Ownership,
    fatness_ctxt: bool,
    user_vars: &BitSet<Local>,
    names: &FxHashMap<Local, Symbol>,
    lhs: &Place<'tcx>,
    span: Span,
) -> RewriteUseResult {
    let replacement =
        mutating_ctxt_replacement(tcx, body, lhs, names[&lhs.local], MutatingCtxt::Store);

    if !matches!(ownership_ctxt, Ownership::Raw) {
        if user_vars.contains(lhs.local) {
            rewriter.make_suggestion(
                tcx,
                span,
                "remove null pointer".to_owned(),
                format!("{replacement} = None"),
            );
        } else {
            rewriter.make_suggestion(tcx, span, "remove null pointer".to_owned(), format!("None"));
            return RewriteUseResult::LHSToBeRewritten { rhs_span: span };
        }
    }

    RewriteUseResult::Done
}

fn ptr_place_sig<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    func: Func,
    ownership_analysis: &ownership_analysis::InterSummary,
    mutability_analysis: &mutability_analysis::InterSummary,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    place: &Place<'tcx>,
    location: Location,
) -> (Ownership, bool) {
    let mut n_derefs = 0;
    for (base, proj) in place.iter_projections().rev() {
        match proj {
            rustc_middle::mir::ProjectionElem::Field(field, _) => {
                let place_ty = base.ty(body, tcx);
                let ty = place_ty.ty;
                let variant_idx = place_ty
                    .variant_index
                    .unwrap_or(rustc_target::abi::VariantIdx::new(0));
                let adt_def = ty.ty_adt_def().unwrap();

                let rho = ownership_analysis.inter_ctxt.field_defs[&adt_def
                    .did
                    .as_local()
                    .expect("struct definitions should be in scope!!!")][variant_idx]
                    [field.index()]
                .start
                    + n_derefs;

                let lambda = fatness_analysis.lambda_ctxt.field_defs[&adt_def
                    .did
                    .as_local()
                    .expect("struct definitions should be in scope!!!")][variant_idx]
                    [field.index()]
                .start
                    + n_derefs;

                let ownership = if ownership_analysis.approximate_rho_ctxt.get().unwrap()[func][rho]
                    == Some(true)
                {
                    Ownership::Owning
                } else {
                    // Ownership::Transient { mutbl: true }
                    Ownership::Raw
                };

                let fatness = fatness_analysis.lambda_ctxt.assumptions[lambda].unwrap_or(false);

                return (ownership, fatness);
            }
            rustc_middle::mir::ProjectionElem::Deref => n_derefs += 1,
            _ => unimplemented!("projections other than deref and field are not supported!"),
        }
    }

    let (rho_ssa_idx, lambda_ssa_idx, mu_ssa_idx) = if place.projection.is_empty() {
        (
            ownership_analysis.func_summaries[func]
                .ssa_name_source_map
                .try_def(place.local, location)
                .unwrap(),
            fatness_analysis.ssa_name_source_map[func]
                .try_def(place.local, location)
                .unwrap(),
            mutability_analysis.func_summaries[func]
                .ssa_name_source_map
                .try_def(place.local, location)
                .unwrap(),
        )
    } else {
        (
            ownership_analysis.func_summaries[func]
                .ssa_name_source_map
                .try_use(place.local, location)
                .unwrap(),
            fatness_analysis.ssa_name_source_map[func]
                .try_use(place.local, location)
                .unwrap(),
            mutability_analysis.func_summaries[func]
                .ssa_name_source_map
                .try_use(place.local, location)
                .unwrap(),
        )
    };

    assert_eq!(lambda_ssa_idx, mu_ssa_idx);

    let rho =
        ownership_analysis.func_summaries[func].locals[place.local][rho_ssa_idx].start + n_derefs;
    let lambda =
        fatness_analysis.lambda_ctxt.locals[func][place.local][lambda_ssa_idx].start + n_derefs;
    // if n_derefs > 0, `mu` is meaning less
    let mu = mutability_analysis.func_summaries[func].locals[place.local][mu_ssa_idx];

    let ownership =
        if ownership_analysis.approximate_rho_ctxt.get().unwrap()[func][rho] == Some(true) {
            Ownership::Owning
        } else {
            let mutbl =
                n_derefs > 0 || mutability_analysis.approximate_mu_ctxt.get().unwrap()[func][mu];
            Ownership::Transient { mutbl }
        };

    let fatness = fatness_analysis.lambda_ctxt.assumptions[lambda].unwrap_or(false);

    (ownership, fatness)
}
