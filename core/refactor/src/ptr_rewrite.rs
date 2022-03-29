use analysis::{
    call_graph::Func,
    fat_thin_analysis::CrateSummary,
    ssa::{rename::SSAIdx, RichLocation},
};
use either::Either;
use rustc_ast::Mutability;
use rustc_hir::{def_id::LocalDefId, FnSig, ItemKind, Ty, TyKind, VariantData};
use rustc_middle::{
    mir::{
        BasicBlock, Body, Local, Location, PlaceRef, ProjectionElem, Rvalue, StatementKind,
        TerminatorKind,
    },
    ty::TyCtxt,
};
use rustc_target::abi::VariantIdx;

pub fn array_rewrite(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewrite::Rewriter,
    analysis_results: &CrateSummary,
    fn_defs: &[LocalDefId],
    struct_defs: &[LocalDefId],
) {
    for &def_id in fn_defs {
        let hir_body = tcx.hir().expect_item(def_id);
        match &hir_body.kind {
            ItemKind::Fn(sig, _generics, _body_id) => {
                array_rewrite_fn_sig(tcx, rewriter, &analysis_results, sig, def_id);
            }
            _ => panic!("wrong item kind"),
        }

        array_rewrite_body(tcx, rewriter, &analysis_results, def_id);
    }

    for &def_id in struct_defs {
        let hir_body = tcx.hir().expect_item(def_id);
        match &hir_body.kind {
            ItemKind::Struct(variant_data, _generics) => {
                array_rewrite_struct(tcx, rewriter, &analysis_results, variant_data, def_id);
            }
            _ => panic!("wrong item kind"),
        }
    }
}

fn rewrite_ty(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewrite::Rewriter,
    ty: &Ty,
    results: &[Option<bool>],
) {
    fn visit_nested_pointer(ty: &Ty, nested_level: usize, f: &mut impl FnMut(&Ty, usize)) {
        if let TyKind::Ptr(mut_ty) = &ty.kind {
            visit_nested_pointer(mut_ty.ty, nested_level + 1, f);
        }
        f(ty, nested_level);
    }

    let mut did_rewrite_any = false;
    let mut new_ty = String::new();
    visit_nested_pointer(ty, 0, &mut |ty, nested_level| {
        let mut_ty = match &ty.kind {
            TyKind::Ptr(mut_ty) => mut_ty,
            _ => {
                new_ty = tcx.sess.source_map().span_to_snippet(ty.span).unwrap();
                return;
            }
        };

        let ptr_prefix = if mut_ty.mutbl == Mutability::Mut {
            "*mut"
        } else {
            "*const"
        };
        if results[nested_level] == Some(true) {
            // this level of pointer is fat
            // this could be faster (format! is slow) but oh well
            new_ty = format!("{ptr_prefix} [{new_ty}]");
            did_rewrite_any = true;
        } else {
            new_ty = format!("{ptr_prefix} {new_ty}");
        }
    });

    if did_rewrite_any {
        rewriter.make_suggestion(tcx, ty.span, String::from("fatten pointers"), new_ty);
    }
}

fn array_rewrite_fn_sig(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewrite::Rewriter,
    analysis_results: &CrateSummary,
    sig: &FnSig,
    def_id: LocalDefId,
) {
    let func = analysis_results
        .call_graph
        .lookup_function(&def_id.to_def_id())
        .unwrap();
    let locals = &analysis_results.lambda_ctxt.locals[func];
    let args_iter = locals.iter().skip(1).take(sig.decl.inputs.len());
    let arg_results_iter = args_iter.map(|ls| {
        ls.get(SSAIdx::from_usize(0))
            .unwrap()
            .clone()
            .map(|l| analysis_results.lambda_ctxt.assumptions[l])
            .collect::<Vec<_>>()
    });
    for (arg_ty, results) in sig.decl.inputs.iter().zip(arg_results_iter) {
        rewrite_ty(tcx, rewriter, arg_ty, &results);
    }
}

fn array_rewrite_struct(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewrite::Rewriter,
    summary: &CrateSummary,
    variant_data: &VariantData,
    def_id: LocalDefId,
) {
    let results = summary.lambda_ctxt.field_defs[&def_id.to_def_id()][0usize.into()]
        .iter()
        .map(|ls| {
            ls.clone()
                .map(|l| summary.lambda_ctxt.assumptions[l])
                .collect::<Vec<_>>()
        });
    for (field, results) in variant_data.fields().iter().zip(results) {
        rewrite_ty(tcx, rewriter, &field.ty, &results);
    }
}

fn array_rewrite_body(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewrite::Rewriter,
    summary: &CrateSummary,
    def_id: LocalDefId,
) {
    let func = summary
        .call_graph
        .lookup_function(&def_id.to_def_id())
        .unwrap();
    let body = tcx.optimized_mir(def_id);
    for (bb_id, bb) in body.basic_blocks().iter_enumerated() {
        let terminator = bb.terminator();
        match &terminator.kind {
            TerminatorKind::Call { func: callee, .. } => {
                let const_ty = callee.constant().unwrap().literal.ty();
                let callee_def_id = match const_ty.kind() {
                    rustc_middle::ty::TyKind::FnDef(def_id, _substs) => def_id,
                    _ => panic!(),
                };
                let callee_name = tcx.def_path_str(*callee_def_id);
                match callee_name.as_str() {
                    x if x.ends_with("::realloc") => {
                        array_rewrite_realloc(tcx, rewriter, summary, func, body, bb_id);
                    }
                    x if x.ends_with("::calloc") => {
                        array_rewrite_calloc(tcx, rewriter, summary, func, body, bb_id);
                    }
                    "std::ptr::mut_ptr::<impl *mut T>::offset"
                    | "std::ptr::const_ptr::<impl *const T>::offset" => {
                        array_rewrite_offset(tcx, rewriter, summary, func, body, bb_id);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn array_rewrite_realloc(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewrite::Rewriter,
    summary: &CrateSummary,
    func: Func,
    body: &Body,
    bb_id: BasicBlock,
) {
    let bb = &body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = body.terminator_loc(bb_id);
    let (args, destination) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    if place_fatness(summary, func, destination.0.as_ref(), terminator_loc) != Some(true) {
        return;
    }

    let ptr = args[0].place().unwrap().as_local().unwrap();
    let size = args[1].place().unwrap().as_local().unwrap();
    let ptr_expr = uncast(tcx, summary, func, body, terminator_loc, ptr).unwrap();
    let size_expr = uncast(tcx, summary, func, body, terminator_loc, size).unwrap();

    let cast_statement = match body.stmt_at(destination.1.start_location()) {
        Either::Left(s) => s,
        Either::Right(_) => todo!("realloc dest isn't statement"),
    };
    // let x = realloc(_, _) as *mut Foo
    let cast_span = match &cast_statement.kind {
        StatementKind::Assign(box (_, Rvalue::Cast(_, op, _)))
            if op.place() == Some(destination.0) =>
        {
            cast_statement.source_info.span
        }
        _ => todo!("realloc not followed by cast"),
    };
    // realloc(_, _) as *mut Foo
    let realloc_span = cast_span.with_lo(terminator.source_info.span.lo());

    rewriter.make_suggestion(
        tcx,
        realloc_span,
        String::from("use vec instead of realloc"),
        format!(
            "{{ \
            let mut v: Vec<_> = Box::from_raw({}).into(); \
            v.resize({} as usize, 0); \
            Box::into_raw(v.into_boxed_slice()) \
        }}",
            ptr_expr, size_expr,
        ),
    );
}

fn array_rewrite_calloc(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewrite::Rewriter,
    summary: &CrateSummary,
    func: Func,
    body: &Body,
    bb_id: BasicBlock,
) {
    let bb = &body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = body.terminator_loc(bb_id);
    let (args, destination) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    if place_fatness(summary, func, destination.0.as_ref(), terminator_loc) != Some(true) {
        return;
    }

    let nmemb = args[0].place().unwrap().as_local().unwrap();
    let nmemb_expr = uncast(tcx, summary, func, body, terminator_loc, nmemb).unwrap();

    let cast_statement = match body.stmt_at(destination.1.start_location()) {
        Either::Left(s) => s,
        Either::Right(_) => todo!("calloc dest isn't statement"),
    };
    // let x = calloc(_) as *mut Foo
    let cast_span = match &cast_statement.kind {
        StatementKind::Assign(box (_, Rvalue::Cast(_, op, _)))
            if op.place() == Some(destination.0) =>
        {
            cast_statement.source_info.span
        }
        _ => todo!("calloc not followed by cast"),
    };
    // calloc(_, _) as *mut Foo
    let calloc_span = cast_span.with_lo(terminator.source_info.span.lo());

    rewriter.make_suggestion(
        tcx,
        calloc_span,
        String::from("use vec instead of calloc"),
        format!(
            "Box::into_raw(vec![Default::default(); {} as usize].into_boxed_slice())",
            nmemb_expr,
        ),
    );
}

fn array_rewrite_offset(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewrite::Rewriter,
    summary: &CrateSummary,
    func: Func,
    body: &Body,
    bb_id: BasicBlock,
) {
    let bb = &body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = body.terminator_loc(bb_id);
    let (args, destination) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    let ptr_place = args[0].place().unwrap().as_ref();
    if place_fatness(summary, func, ptr_place, terminator_loc) != Some(true) {
        return;
    }

    let offset = args[1].place().unwrap().as_local().unwrap();
    let offset_expr = uncast(tcx, summary, func, body, terminator_loc, offset).unwrap();

    let deref_stmt = match body.stmt_at(destination.1.start_location()) {
        Either::Left(s) => s,
        Either::Right(_) => return,
    };
}

fn place_fatness(
    summary: &CrateSummary,
    func: Func,
    place: PlaceRef,
    loc: Location,
) -> Option<bool> {
    let field_idx = place
        .projection
        .iter()
        .enumerate()
        .rev()
        .find(|(_i, elem)| matches!(elem, ProjectionElem::Field(_, _)));
    if let Some((idx, ProjectionElem::Field(field, ty))) = field_idx {
        assert!(place.projection[idx + 1..]
            .iter()
            .all(|elem| matches!(elem, ProjectionElem::Deref)));
        let n_derefs = place.projection.len() - 1 - idx;
        let struct_def_id = ty.ty_adt_def().unwrap().did;
        let mut lambda_range = summary.lambda_ctxt.field_defs[&struct_def_id]
            [VariantIdx::from_usize(0)][field.index()]
        .clone();
        let lambda = lambda_range.nth(n_derefs).unwrap();
        return summary.lambda_ctxt.assumptions[lambda];
    } else {
        assert!(place
            .projection
            .iter()
            .all(|elem| matches!(elem, ProjectionElem::Deref)));
        let n_derefs = place.projection.len();
        let ssa_idx = summary.ssa_name_source_map[func]
            .try_def(place.local, loc)
            .unwrap();
        let mut lambda_range = summary.lambda_ctxt.locals[func][place.local][ssa_idx].clone();
        let lambda = lambda_range.nth(n_derefs).unwrap();
        return summary.lambda_ctxt.assumptions[lambda];
    }
}

/// returns the source of an expression with all its outer `as _` removed
fn uncast(
    tcx: TyCtxt<'_>,
    summary: &CrateSummary,
    func: Func,
    body: &Body,
    loc: Location,
    local: Local,
) -> Option<String> {
    let source_map = &summary.ssa_name_source_map[func];
    let ssa_idx = source_map.try_use(local, loc).unwrap();
    let def_loc_rich = &summary.def_sites[func].defs[local][ssa_idx];
    let def_loc = match def_loc_rich {
        RichLocation::Mir(l) => *l,
        RichLocation::Entry => todo!(),
        _ => todo!(),
    };
    let def_stmt = match body.stmt_at(def_loc) {
        Either::Left(s) => s,
        Either::Right(terminator) => {
            return tcx
                .sess
                .source_map()
                .span_to_snippet(terminator.source_info.span)
                .ok()
        }
    };
    let _expr = tcx
        .sess
        .source_map()
        .span_to_snippet(def_stmt.source_info.span);
    match &def_stmt.kind {
        StatementKind::Assign(box (place, rvalue)) => {
            if let Rvalue::Cast(_kind, operand, _ty) = rvalue {
                return uncast(
                    tcx,
                    summary,
                    func,
                    body,
                    def_loc,
                    operand.place().unwrap().as_local().unwrap(),
                );
            }
            let span = def_stmt.source_info.span;
            return tcx.sess.source_map().span_to_snippet(span).ok();
        }
        _ => todo!(),
    }
}
