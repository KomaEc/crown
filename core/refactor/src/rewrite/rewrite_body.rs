use analysis::{
    call_graph::Func, fat_thin_analysis, mutability_analysis, ownership_analysis,
    ssa::RichLocation, ty_ext::TyExt,
};
use either::Either;
use itertools::Itertools;
use rewriter::Rewriter;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::LocalDefId;
use rustc_index::{bit_set::BitSet, vec::Idx};
use rustc_middle::{
    mir::{
        visit::Visitor, Body, Local, Location, Operand, Place, PlaceRef, ProjectionElem, Rvalue,
        Statement, StatementKind, VarDebugInfoContents,
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
                log::warn!("user constant {:?} is not processed", constant)
            }
        }
    }

    struct BodyRewriteVisitor<'me, 'tcx> {
        tcx: TyCtxt<'tcx>,
        rewriter: &'me mut Rewriter,
        body: &'me Body<'tcx>,
        func: Func,
        ownership_analysis: &'me ownership_analysis::InterSummary,
        mutability_analysis: &'me mutability_analysis::InterSummary,
        fatness_analysis: &'me fat_thin_analysis::CrateSummary,
        user_vars: &'me BitSet<Local>,
        names: &'me FxHashMap<Local, Symbol>,
    }

    impl<'me, 'tcx> Visitor<'tcx> for BodyRewriteVisitor<'me, 'tcx> {
        // fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
            if let StatementKind::Assign(box (place, _)) = &statement.kind {
                if self.user_vars.contains(place.local)
                    && place.ty(self.body, self.tcx).ty.is_ptr_but_not_fn_ptr()
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
                    ) {
                        // we need to further rewrite lhs

                        assert!(!rhs_span.contains(statement.source_info.span));

                        let replacement = mutating_use_replacement(
                            self.tcx,
                            self.body,
                            place,
                            self.names[&place.local],
                            true,
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

    BodyRewriteVisitor {
        tcx,
        rewriter,
        body,
        func,
        ownership_analysis,
        mutability_analysis,
        fatness_analysis,
        user_vars: &user_vars,
        names: &names,
    }
    .visit_body(body);
}

/// rewrite the use of local `local` at location `location`
/// This is a recursive procedure that traces a chain of
/// intermediate statements until the RHS of the statement
/// is related to user_ptr
/// Precondition: lhs type = ptr
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
) -> Option<Span> {
    match body.stmt_at(location) {
        Either::Left(statement) => {
            match &statement.kind {
                StatementKind::Assign(box (lhs, rhs)) => {
                    let (ownership_ctxt, fatness_ctxt) = ptr_place_sig(
                        tcx,
                        body,
                        func,
                        ownership_analysis,
                        mutability_analysis,
                        fatness_analysis,
                        lhs,
                        location,
                    );

                    match rhs {
                        Rvalue::Use(rhs) => {
                            match rhs {
                                Operand::Copy(rhs) | Operand::Move(rhs) => {
                                    // end recursion right here
                                    if user_vars.contains(rhs.local) {
                                        // the below does not hold for simple statement like _0 = _1 !!!!!
                                        // assert!(!user_vars.contains(lhs.local), "{:?}", statement);
                                        // however, if it is indeed the case, we don't have to rewrite lhs!!!!!!
                                        // so it's fine!

                                        // perform the rewrite

                                        match ownership_ctxt {
                                            Ownership::Owning => {
                                                // std::mem::take this pointer

                                                let replacement = mutating_use_replacement(
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
                                                    format!("std::mem::take(&mut {replacement})"),
                                                );
                                            }
                                            Ownership::Transient { mutbl } if mutbl => {
                                                let replacement = mutating_use_replacement(
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
                                                    replacement,
                                                );
                                            }
                                            Ownership::Transient { .. } => {
                                                let is_base_clonable = mutability_analysis
                                                    .func_summaries[func]
                                                    .ssa_name_source_map
                                                    .try_use(rhs.local, location)
                                                    .map(|ssa_idx| {
                                                        let mu = mutability_analysis.func_summaries
                                                            [func]
                                                            .locals[rhs.local][ssa_idx];
                                                        !mutability_analysis
                                                            .approximate_mu_ctxt
                                                            .get()
                                                            .unwrap()[func][mu]
                                                    })
                                                    .unwrap_or(false);
                                                let replacement = nonmutating_use_replacement(
                                                    tcx,
                                                    body,
                                                    rhs,
                                                    names[&rhs.local],
                                                    is_base_clonable,
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
                                        );
                                    }
                                }
                                Operand::Constant(box constant) =>
                                // This is definitely wrong, but we assume that only null pointer can be pointer constant
                                {
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
                                }
                            }
                        }
                        Rvalue::Cast(_, Operand::Constant(box constant), _) => {
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
                        }

                        Rvalue::Cast(_, Operand::Copy(rhs) | Operand::Move(rhs), _) => None,
                        Rvalue::Ref(_, _, _) => todo!(),
                        Rvalue::AddressOf(_, _) => todo!(),
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
        Either::Right(terminator) => None,
    }
}

/// we don't rewrite fatness for now
/// this logic only works for places of the following form:
/// 1. (*..*s).f
/// 2. s.f
/// 3. p
/// TODO: update ownership context when going through fields!!!!!!!!
/// move context: 1. argument of std::mem::take; 2. LHS
fn mutating_use_replacement<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    place: &Place<'tcx>,
    var_name: Symbol,
    is_move_ctxt: bool,
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
    // if it is not under move context, reborrow the pointer
    if !is_move_ctxt {
        replacement += ".as_deref_mut()";
    }
    replacement
}

fn nonmutating_use_replacement<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    place: &Place<'tcx>,
    var_name: Symbol,
    is_base_clonable: bool,
) -> String {
    let mut replacement = var_name.to_string();
    let mut need_paren = false;

    if is_base_clonable {
        replacement += ".clone()";
    } else if body.local_decls[place.local].ty.is_ptr_but_not_fn_ptr() {
        replacement += ".as_deref()";
    }

    // logic of this loop: parent path is reborrowed upon creation
    for (base, proj) in place.iter_projections() {
        if need_paren {
            replacement = format!("({replacement})");
            need_paren = false;
        }
        let base_is_ptr = base.ty(body, tcx).ty.is_ptr_but_not_fn_ptr();
        if base_is_ptr {
            replacement += ".unwrap()"; // ".as_mut().map(|x| &**x)";
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
                replacement = replacement + "." + field_name + ".as_deref()";
            }
            _ => todo!(),
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
    let replacement = mutating_use_replacement(tcx, body, lhs, names[&lhs.local], true);

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
