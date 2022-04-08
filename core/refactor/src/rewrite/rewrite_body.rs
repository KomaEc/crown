mod rewrite_library_call;

use crate::rewrite::rewrite_body::rewrite_library_call::rewrite_library_call;
use analysis::{
    call_graph::Func, fat_thin_analysis, mutability_analysis, ownership_analysis,
    ssa::RichLocation, ty_ext::TyExt, LocationMap,
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
    ty::TyCtxt,
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
                    if let Some(rhs_span) = rewrite_use(
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

                        assert!(!rhs_span.contains(statement.source_info.span));

                        let replacement = mutating_ctxt_replacement(
                            self.tcx,
                            self.body,
                            place,
                            self.names[&place.local],
                            false,
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

    /* 
    for (bb, bb_data) in body.basic_blocks().iter_enumerated() {
        let mut location = Location { block: bb, statement_index: 0 };
        for statement in &bb_data.statements {
            if let StatementKind::Assign(box (lhs, rvalue)) = &statement.kind {
                // we check that rvalue contains user variable
                if match rvalue {
                    Rvalue::Use(rhs) => {
                        if let Some(rhs) = rhs.place() {
                            !user_vars.contains(rhs.local)
                        } else {
                            true
                        }
                    },
                    Rvalue::BinaryOp(binop, box (op1, op2)) => {
                        (if let Some(rhs) = op1.place() {
                            !user_vars.contains(rhs.local)
                        } else {
                            true
                        })
                        &&
                        if let Some(rhs) = op2.place() {
                            !user_vars.contains(rhs.local)
                        } else {
                            true
                        }
                    },
                    Rvalue::CheckedBinaryOp(cbop, box (op1, op2)) => {
                        (if let Some(rhs) = op1.place() {
                            !user_vars.contains(rhs.local)
                        } else {
                            true
                        })
                        &&
                        if let Some(rhs) = op2.place() {
                            !user_vars.contains(rhs.local)
                        } else {
                            true
                        }
                    },
                    _ => true
                } {
                    continue
                }
            }
            else { continue }
            if !editted_locations[location.block].contains(location.statement_index) {
                let _ = rewrite_use(
                    tcx,
                    rewriter,
                    body,
                    func,
                    ownership_analysis,
                    mutability_analysis,
                    fatness_analysis,
                    &user_vars,
                    &names,
                    location,
                    &mut editted_locations,
                );
            }
            location.statement_index += 1;
        }
    }
    */
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
) -> Option<Span> {
    match body.stmt_at(location) {
        Either::Left(statement) => {
            assert!(editted_locations[location.block].insert(location.statement_index));

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
                                                    false,
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
                                                            false,
                                                        );

                                                        rewriter.make_suggestion(
                                                            tcx,
                                                            statement.source_info.span,
                                                            "rewrite rhs".to_owned(),
                                                            format!("std::mem::take(&mut {replacement})"),
                                                        );
                                                    }
                                                    Ownership::Transient { mutbl } if mutbl => {
                                                        let replacement = mutating_ctxt_replacement(
                                                            tcx,
                                                            body,
                                                            rhs,
                                                            names[&rhs.local],
                                                            true,
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
                                                                true,
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
                                            return None;
                                        } else {
                                            return Some(statement.source_info.span);
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
                                        None
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
                                None
                            }
                        }

                        Rvalue::Cast(_, Operand::Copy(rhs) | Operand::Move(rhs), _) => {
                            if maybe_ptr_sig.is_none() {
                                return None;
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
                                } else {
                                    todo!()
                                }
                            } else {
                                unreachable!()
                            }
                        }
                        _ => None
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
) -> Option<Span> {
    assert!(editted_locations[location.block].insert(location.statement_index));

    match &terminator.kind {
        TerminatorKind::Return => {
            return None;
            todo!()
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
                            super::rewrite_libc_call::rewrite_libc_call(
                                tcx,
                                rewriter,
                                body,
                                caller,
                                fatness_analysis,
                                terminator,
                                location,
                            );
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
                                    return Some(terminator.source_info.span);
                                } else {
                                    // assert!(false, "destination place is assumed to be a temporary in {:?}", terminator.kind)
                                    assert!(dest.projection.is_empty(), "destination place is assumed to be a local, and we need not rewrite")
                                }
                            }
                            // self.model_boundary(callee_did, args, destination, location);
                            // return;
                            return None; //todo!()
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
                        None
                    }
                }
            } else {
                unreachable!()
            }
        }
        _ => None,
    }
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
    reborrow: bool,
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
    if reborrow {
        replacement += ".as_deref_mut()";
    }
    replacement
}

fn nonmutating_ctxt_replacement<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    place: &Place<'tcx>,
    var_name: Symbol,
    mut is_base_ptr_clonable: bool,
    reborrow: bool,
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
    if reborrow {
        if is_base_ptr_clonable {
            replacement += ".clone()";
        } else {
            replacement += ".as_deref()";
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
) -> Option<Span> {
    let replacement = mutating_ctxt_replacement(tcx, body, lhs, names[&lhs.local], false);

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
            return Some(span);
        }
    }

    None
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
