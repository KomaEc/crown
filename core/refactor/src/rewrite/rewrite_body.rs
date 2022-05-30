use analysis::{
    call_graph::Func,
    fat_thin_analysis,
    ssa::RichLocation, ownership_analysis, mutability_analysis,
};
use either::Either;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{
        BasicBlock, Body, Local, Location, PlaceRef, ProjectionElem, Rvalue, StatementKind,
        TerminatorKind, VarDebugInfoContents, VarDebugInfo, Operand, Statement, ConstantKind, CastKind, Constant, Place,
    },
    ty::{TyCtxt, Const, ScalarInt},
};
use rustc_target::abi::VariantIdx;

pub fn rewrite_body(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewriter::Rewriter,
    ownership: &ownership_analysis::InterSummary,
    mutability: &mutability_analysis::InterSummary,
    fatness: &fat_thin_analysis::CrateSummary,
    func: Func,
    def_id: LocalDefId,
) {
    let body = tcx.optimized_mir(def_id);
    for (bb_id, bb) in body.basic_blocks().iter_enumerated() {
        for (idx, stmt) in bb.statements.iter().enumerate() {
            let loc = Location {
                block: bb_id,
                statement_index: idx,
            };
            println!("{:?}", stmt.source_info.span);
            match &stmt.kind {
                StatementKind::Assign(box (place, rvalue)) => {
                    let var_debug_matches_place = |v: &VarDebugInfo| match v.value {
                        VarDebugInfoContents::Place(p) => p.as_local() == place.as_local(),
                        VarDebugInfoContents::Const(_) => false,
                    };
                    let source = tcx.sess.source_map().span_to_snippet(stmt.source_info.span).unwrap();

                    if source.contains(" = ") {
                        rewrite_reassignment(tcx, rewriter, ownership, mutability, fatness, func, body, loc, stmt);
                    } else if body.var_debug_info.iter().any(var_debug_matches_place) {
                        println!("binding");
                        // binding
                    } else {
                        println!("temporary");
                        // temporary
                    }
                }
                _ => {},
            }
        }

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
                    x if x.ends_with("::malloc") => {
                        rewrite_malloc(tcx, rewriter, ownership, mutability, fatness, func, body, bb_id);
                    }
                    x if x.ends_with("::realloc") => {
                        rewrite_realloc(tcx, rewriter, ownership, mutability, fatness, func, body, bb_id);
                    }
                    x if x.ends_with("::calloc") => {
                        rewrite_calloc(tcx, rewriter, ownership, mutability, fatness, func, body, bb_id);
                    }
                    "std::ptr::mut_ptr::<impl *mut T>::offset" | "std::ptr::const_ptr::<impl *const T>::offset" => {
                        rewrite_offset(tcx, rewriter, ownership, mutability, fatness, func, body, bb_id);
                    }
                    "std::ptr::mut_ptr::<impl *mut T>::is_null" | "std::ptr::const_ptr::<impl *const T>::is_null" => {
                        let span = terminator.source_info.span;
                        let source = tcx.sess.source_map().span_to_snippet(span).unwrap();
                        let replacement = format!("{}.is_none()", source.trim_end_matches(".is_null()"));
                        rewriter.make_suggestion(tcx, span, String::new(), replacement);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub fn rewrite_reassignment<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut rewriter::Rewriter,
    ownership: &ownership_analysis::InterSummary,
    mutability: &mutability_analysis::InterSummary,
    fatness: &fat_thin_analysis::CrateSummary,
    func: Func,
    body: &Body<'tcx>,
    loc: Location,
    stmt: &Statement<'tcx>,
) {
    // on lhs we have a bunch of projections, some of which need to be turned into other things -
    // *x into *(x.unwrap()) for example.
    // rewrite lhs: (*x).f -> x.unwrap().f
    // x is *mut S -> Option<Box<S>>
    // if f is not raw, then need to wrap rhs in Some
    // rewrite rhs: happens according to type of lhs
    // TODO: convert everything else to let-else too. it's just better
    println!("  {:?}", stmt.source_info.span);
    let StatementKind::Assign(box (l_place, rvalue)) = &stmt.kind else { panic!() };
    let stmt_source = tcx.sess.source_map().span_to_snippet(stmt.source_info.span).unwrap();
    // TODO: there can be a newline instead of a space here, which will cause it to panic, but we
    // probably need to have the whitespace, so we don't catch "=="
    let (lhs_source, rhs_source) = stmt_source.split_once(" = ").unwrap();

    let r_place = match rvalue {
        Rvalue::Use(Operand::Copy(r_place) | Operand::Move(r_place)) => r_place,
        // extremely cursed null pointer literal matcher
        Rvalue::Use(
            Operand::Constant(box Constant { literal: ConstantKind::Ty(Const { ty, val }), ..})
        ) | Rvalue::Cast(
            CastKind::Misc,
            Operand::Constant(box Constant { literal: ConstantKind::Ty(Const { val, .. }), ..}),
            ty
        ) if ty.is_unsafe_ptr() && val.try_to_scalar_int().map(ScalarInt::is_null) == Some(true) => {
            if place_ownership(tcx, ownership, func, body, l_place.clone(), loc).is_some() {
                rewriter.make_suggestion(
                    tcx,
                    stmt.source_info.span,
                    String::from("transform pointer assignment to use Option"),
                    format!("{lhs_source} = None"),
                );               
            }
            return;
        },
        x => unimplemented!("{x:?}"),
    };

    let mut source_stack = Vec::with_capacity(l_place.projection.len());
    source_stack.push(lhs_source);
    for (i, (_place, proj)) in l_place.iter_projections().rev().enumerate() {
        match proj {
            ProjectionElem::Deref => source_stack.push(source_stack[i]
                .trim_start_matches('(')
                .trim_start_matches('*')
                .trim_end_matches(')')),
            ProjectionElem::Field(_, _) => source_stack.push(source_stack[i]
                .trim_end_matches(char::is_alphanumeric)
                .trim_end_matches('.')),
            x => unimplemented!("{x:?}"),
        }
    }
    println!("      source stack: {source_stack:?}");
}

fn rewrite_malloc<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut rewriter::Rewriter,
    ownership: &ownership_analysis::InterSummary,
    _mutability: &mutability_analysis::InterSummary,
    _fatness: &fat_thin_analysis::CrateSummary,
    func: Func,
    body: &Body<'tcx>,
    bb_id: BasicBlock,
) {
    let bb = &body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = body.terminator_loc(bb_id);
    let (_args, (dest_place, dest_bb)) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    if place_ownership(tcx, ownership, func, body, dest_place, terminator_loc) != Some(true) {
        return;
    }

    let cast_statement = match body.stmt_at(dest_bb.start_location()) {
        Either::Left(s) => s,
        Either::Right(_) => todo!("malloc dest isn't statement"),
    };
    // let x = malloc(_) as *mut Foo
    let cast_span = match &cast_statement.kind {
        StatementKind::Assign(box (_, Rvalue::Cast(_, op, _)))
            if op.place() == Some(dest_place) =>
        {
            cast_statement.source_info.span
        }
        _ => todo!("malloc not followed by cast"),
    };
    // malloc(_) as *mut Foo
    let malloc_span = cast_span.with_lo(terminator.source_info.span.lo());

    rewriter.make_suggestion(
        tcx,
        malloc_span,
        String::from("use box instead of malloc"),
        String::from("Some(Box::new(Default::default()))"),
    );
}

fn rewrite_realloc(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewriter::Rewriter,
    _ownership: &ownership_analysis::InterSummary,
    _mutability: &mutability_analysis::InterSummary,
    fatness: &fat_thin_analysis::CrateSummary,
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
    if place_fatness(fatness, func, destination.0.as_ref(), terminator_loc) != Some(true) {
        return;
    }

    let ptr = args[0].place().unwrap().as_local().unwrap();
    let size = args[1].place().unwrap().as_local().unwrap();
    let ptr_expr = uncast(tcx, fatness, func, body, terminator_loc, ptr).unwrap();
    let size_expr = uncast(tcx, fatness, func, body, terminator_loc, size).unwrap();

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

fn rewrite_calloc(
    tcx: TyCtxt<'_>,
    rewriter: &mut rewriter::Rewriter,
    _ownership: &ownership_analysis::InterSummary,
    _mutability: &mutability_analysis::InterSummary,
    fatness: &fat_thin_analysis::CrateSummary,
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
    if place_fatness(fatness, func, destination.0.as_ref(), terminator_loc) != Some(true) {
        return;
    }

    let nmemb = args[0].place().unwrap().as_local().unwrap();
    let nmemb_expr = uncast(tcx, fatness, func, body, terminator_loc, nmemb).unwrap();

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

fn rewrite_offset(
    tcx: TyCtxt<'_>,
    _rewriter: &mut rewriter::Rewriter,
    _ownership: &ownership_analysis::InterSummary,
    _mutability: &mutability_analysis::InterSummary,
    fatness: &fat_thin_analysis::CrateSummary,
    func: Func,
    body: &Body,
    bb_id: BasicBlock,
) {
    let bb = &body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = body.terminator_loc(bb_id);
    let (args, (dest_place, dest_bb)) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    let dest_local = dest_place.as_local().unwrap();

    let ptr_place = args[0].place().unwrap().as_ref();
    if place_fatness(fatness, func, ptr_place, terminator_loc) != Some(true) {
        return;
    }

    let ptr = args[0].place().unwrap().as_local().unwrap();
    // ptr
    let ptr_expr = uncast(tcx, fatness, func, body, terminator_loc, ptr).unwrap();
    let offset = args[1].place().unwrap().as_local().unwrap();
    // ptr.offset(_)
    let offset_expr = uncast(tcx, fatness, func, body, terminator_loc, offset).unwrap();

    // write case:
    // *(ptr.offset(_)) = ...
    // read case:
    // unknown yet

    // we have a problem! buffer_good has an offset call at line 282 that *doesn't* terminate into
    // a using statement at all. there's a whole other basic block before the use. maybe look into
    // source map to find the use?
    let deref_stmt = match body.stmt_at(dest_bb.start_location()) {
        Either::Left(s) => s,
        Either::Right(_) => return,
    };
    
    match &deref_stmt.kind {
        StatementKind::Assign(box (place, rvalue)) => {
            if let PlaceRef { local, projection: [ProjectionElem::Deref] } = place.as_ref() {
                if local == dest_local {
                    // writing into the slice
                    todo!()
                }
            } else if let Rvalue::Use(operand) = rvalue {
                if operand.place() == Some(dest_place) {
                    // reading from the slice into a local
                    todo!()
                }
            }
        }
        _ => {},
    }
}

fn place_fatness(
    fatness: &fat_thin_analysis::CrateSummary,
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
        let struct_def_id = ty.ty_adt_def().unwrap().did.as_local().unwrap();
        let mut lambda_range = fatness.lambda_ctxt.field_defs[&struct_def_id]
            [VariantIdx::from_usize(0)][field.index()]
        .clone();
        let lambda = lambda_range.nth(n_derefs).unwrap();
        return fatness.lambda_ctxt.assumptions[lambda];
    } else {
        assert!(place
            .projection
            .iter()
            .all(|elem| matches!(elem, ProjectionElem::Deref)));
        let n_derefs = place.projection.len();
        let ssa_idx = fatness.ssa_name_source_map[func]
            .try_def(place.local, loc)
            .unwrap();
        let mut lambda_range = fatness.lambda_ctxt.locals[func][place.local][ssa_idx].clone();
        let lambda = lambda_range.nth(n_derefs).unwrap();
        return fatness.lambda_ctxt.assumptions[lambda];
    }
}

fn place_ownership<'tcx>(
    tcx: TyCtxt<'tcx>,
    ownership: &ownership_analysis::InterSummary,
    func: Func,
    body: &Body<'tcx>,
    place: Place<'tcx>,
    loc: Location,
) -> Option<bool> {
    assert!(place.projection.iter().all(|e| matches!(e, ProjectionElem::Field(_, _) | ProjectionElem::Deref)));
    let field_idx = place
        .iter_projections()
        .rev()
        .enumerate()
        .find(|(_i, (_place, elem))| matches!(elem, ProjectionElem::Field(_, _)));
    if let Some((idx, (place, ProjectionElem::Field(field, _ty)))) = field_idx {
        // TODO: is it?
        let n_derefs = idx;
        let struct_def_id = place.ty(body, tcx).ty.ty_adt_def().unwrap().did.as_local().unwrap();
        let mut rho_range = ownership.inter_ctxt.field_defs[&struct_def_id]
            [VariantIdx::from_usize(0)][field.index()]
        .clone();
        let rho = rho_range.nth(n_derefs).unwrap();
        return ownership.approximate_rho_ctxt.get().unwrap()[func][rho];
    } else {
        let n_derefs = place.projection.len();
        let ssa_idx = ownership
            .func_summaries[func]
            .ssa_name_source_map
            .try_def(place.local, loc)
            .unwrap();
        let mut rho_range = ownership.func_summaries[func].locals[place.local][ssa_idx].clone();
        let rho = rho_range.nth(n_derefs).unwrap();
        return ownership.approximate_rho_ctxt.get().unwrap()[func][rho];
    }
}

/// returns the source of an expression with all its outer `as _` removed
fn uncast(
    tcx: TyCtxt<'_>,
    fatness: &fat_thin_analysis::CrateSummary,
    func: Func,
    body: &Body,
    loc: Location,
    local: Local,
) -> Option<String> {
    let source_map = &fatness.ssa_name_source_map[func];
    let ssa_idx = source_map.try_use(local, loc).unwrap();
    let def_loc_rich = &fatness.def_sites[func].defs[local][ssa_idx];
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
        StatementKind::Assign(box (_place, rvalue)) => {
            if let Rvalue::Cast(_kind, operand, _ty) = rvalue {
                return uncast(
                    tcx,
                    fatness,
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
