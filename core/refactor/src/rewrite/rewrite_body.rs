use std::collections::HashSet;

use analysis::{
    api::AnalysisResults, call_graph::Func, fat_thin_analysis, mutability_analysis, null_analysis,
    ownership_analysis, ssa::RichLocation,
};
use either::Either;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{
        BasicBlock, Body, CastKind, Constant, ConstantKind, Local, Location, Operand, Place,
        PlaceRef, ProjectionElem, Rvalue, Statement, StatementKind, TerminatorKind,
    },
    ty::{Const, ScalarInt, TyCtxt},
};
use rustc_span::{BytePos, Span};

pub struct BodyRewriteCtxt<'tcx, 'a, 'b> {
    pub tcx: TyCtxt<'tcx>,
    pub rewriter: &'a mut rewriter::Rewriter,
    pub ownership: &'a ownership_analysis::InterSummary,
    pub mutability: &'a mutability_analysis::InterSummary,
    pub fatness: &'a fat_thin_analysis::CrateSummary,
    pub null: &'a null_analysis::CrateResults<'tcx, 'b>,
    pub func: Func,
    pub def_id: LocalDefId,
    pub body: &'a Body<'tcx>,
}

impl BodyRewriteCtxt<'_, '_, '_> {
    fn rewrite(&mut self, span: Span, to: impl Into<String>, msg: &'static str) {
        self.rewriter
            .make_suggestion(self.tcx, span, msg.into(), to.into());
    }

    fn span_to_snippet(&self, span: Span) -> String {
        self.tcx.sess.source_map().span_to_snippet(span).unwrap()
    }
}

pub fn rewrite_body(cx: &mut BodyRewriteCtxt) {
    // some terminators also cause statement rewrites, so we track which ones have been rewritten
    // and don't try to rewrite them twice
    let mut rewritten_stmts = HashSet::new();
    for (bb_id, bb) in cx.body.basic_blocks().iter_enumerated() {
        let terminator = bb.terminator();
        match &terminator.kind {
            TerminatorKind::Call { func: callee, .. } => {
                let const_ty = callee.constant().unwrap().literal.ty();
                let callee_def_id = match const_ty.kind() {
                    rustc_middle::ty::TyKind::FnDef(def_id, _substs) => def_id,
                    _ => panic!(),
                };
                let callee_name = cx.tcx.def_path_str(*callee_def_id);
                let span = terminator.source_info.span;
                match callee_name.as_str() {
                    x if x.ends_with("::malloc") => {
                        rewrite_malloc(cx, bb_id, &mut rewritten_stmts);
                    }
                    x if x.ends_with("::realloc") => {
                        rewrite_realloc(cx, bb_id, &mut rewritten_stmts);
                    }
                    x if x.ends_with("::calloc") => {
                        rewrite_calloc(cx, bb_id, &mut rewritten_stmts);
                    }
                    x if x.ends_with("::free") => {
                        cx.rewrite(span, "", "delete explicit free() call");
                    }
                    "std::ptr::mut_ptr::<impl *mut T>::offset"
                    | "std::ptr::const_ptr::<impl *const T>::offset" => {
                        rewrite_offset(cx, bb_id, &mut rewritten_stmts);
                    }
                    "std::ptr::mut_ptr::<impl *mut T>::is_null"
                    | "std::ptr::const_ptr::<impl *const T>::is_null" => {
                        let call_span =
                            span.with_lo(BytePos(span.hi().0 - ".is_null()".len() as u32));
                        cx.rewrite(call_span, ".is_none()", "");
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        let stmts = bb.statements.iter().enumerate()
            .map(|(idx, stmt)| (Location {
                block: bb_id,
                statement_index: idx,
            }, stmt))
            .filter(|(loc, _)| !rewritten_stmts.contains(loc));
        for (loc, stmt) in stmts {
            match &stmt.kind {
                StatementKind::Assign(box (l_place, rvalue)) => {
                    let source = cx.span_to_snippet(stmt.source_info.span);
                    if source.contains(" = ") {
                        rewrite_reassignment(cx, loc, stmt);
                    } else {
                        // binding or temporary. rules for these are the same so far
                        let new_source = rewrite_rvalue(cx, loc, rvalue, &source);
                        if new_source != source {
                            cx.rewrite(stmt.source_info.span, new_source, "");
                        }
                    }

                    if let Rvalue::Use(Operand::Copy(r_place) | Operand::Move(r_place)) = rvalue {
                        let lhs_ownership = place_result(cx, cx.ownership, loc, l_place.as_ref());
                        let rhs_ownership = place_result(cx, cx.ownership, loc, r_place.as_ref());
                        let lhs_mutability = place_result(cx, cx.mutability, loc, l_place.as_ref());
                        if rhs_ownership == Some(true) && lhs_ownership == Some(true) {
                            cx.rewrite(stmt.source_info.span.shrink_to_hi(), ".take()", "");
                        } else if rhs_ownership == Some(true) && lhs_ownership == Some(false) {
                            let call = if lhs_mutability == Some(true) {
                                String::from(".as_deref_mut()")
                            } else {
                                String::from(".as_deref()")
                            };
                            cx.rewrite(stmt.source_info.span.shrink_to_hi(), call, "");
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn rewrite_rvalue<'tcx>(
    cx: &mut BodyRewriteCtxt<'tcx, '_, '_>,
    loc: Location,
    rvalue: &Rvalue<'tcx>,
    source: &str,
) -> String {
    match rvalue {
        // rvalue is just a place
        Rvalue::Use(Operand::Copy(r_place) | Operand::Move(r_place)) => {
            if r_place.projection.len() == 0 {
                return source.to_owned();
            }
            rewrite_place_expr(cx, loc, r_place.clone(), true, source)
        }
        // extremely cursed matcher - rvalue is null pointer literal
        Rvalue::Use(Operand::Constant(box Constant {
            literal: ConstantKind::Ty(Const { ty, val }),
            ..
        }))
        | Rvalue::Cast(
            CastKind::Misc,
            Operand::Constant(box Constant {
                literal: ConstantKind::Ty(Const { val, .. }),
                ..
            }),
            ty,
        ) if ty.is_unsafe_ptr()
            && val.try_to_scalar_int().map(ScalarInt::is_null) == Some(true) =>
        {
            return String::from("None");
        }
        rvalue => {
            tracing::warn!(?rvalue, "not rewriting unsupported rvalue");
            return source.to_owned();
        }
    }
}

pub fn rewrite_reassignment<'tcx>(cx: &mut BodyRewriteCtxt<'tcx, '_, '_>, loc: Location, stmt: &Statement<'tcx>) {
    // TODO: convert everything else to let-else too. it's just better
    let StatementKind::Assign(box (l_place, rvalue)) = &stmt.kind else { panic!() };
    let span = stmt.source_info.span;
    let stmt_source = cx.span_to_snippet(span);
    // TODO: there can be a newline instead of a space here, which will cause it to panic, but we
    // probably need to have the whitespace, so we don't catch "=="
    let (lhs_source, rhs_source) = stmt_source.split_once(" = ").unwrap();

    let lhs_span = span.with_hi(BytePos(span.lo().0 + lhs_source.len() as u32));
    let rhs_span = span.with_lo(BytePos(span.hi().0 - rhs_source.len() as u32));

    let lhs_new = rewrite_place_expr(cx, loc, l_place.clone(), false, lhs_source);
    let rhs_new = rewrite_rvalue(cx, loc, rvalue, rhs_source);
    if lhs_new != lhs_source {
        cx.rewrite(lhs_span, lhs_new, "");
    }
    if rhs_new != rhs_source {
        cx.rewrite(rhs_span, rhs_new, "");
    }
}

pub fn rewrite_place_expr<'tcx>(
    cx: &mut BodyRewriteCtxt<'tcx, '_, '_>,
    loc: Location,
    place: Place<'tcx>,
    is_rvalue: bool,
    source: &str,
) -> String {
    let mut source_stack = Vec::with_capacity(place.projection.len());
    source_stack.push(source);
    for (i, (_place, proj)) in place.iter_projections().rev().enumerate() {
        match proj {
            ProjectionElem::Deref => source_stack.push(
                source_stack[i]
                    .trim_start_matches('(')
                    .trim_start_matches('*')
                    .trim_end_matches(')'),
            ),
            ProjectionElem::Field(_, _) => source_stack.push(
                source_stack[i]
                    .trim_end_matches(char::is_alphanumeric)
                    .trim_end_matches('.'),
            ),
            x => unimplemented!("{x:?}"),
        }
    }

    let mut new_source = (*source_stack.last().unwrap()).to_owned();
    for (i, (base_place, proj)) in place.iter_projections().enumerate() {
        let i = place.projection.len() - i - 1;
        match proj {
            ProjectionElem::Deref => {
                let ownership = place_result(cx, cx.ownership, loc, base_place);
                let mutability = place_result(cx, cx.mutability, loc, base_place);
                let nullable = place_result(cx, cx.null, loc, base_place).unwrap();
                if ownership == Some(true) && nullable {
                    new_source = format!("(*{new_source}.as_mut().unwrap())");
                } else if ownership == Some(false) && nullable {
                    let maybe_as = match (is_rvalue, mutability) {
                        (false, _) => "",
                        (true, Some(true)) => ".as_mut()",
                        (true, Some(false)) => ".as_ref()",
                        (true, None) => panic!(),
                    };
                    new_source = format!("(*{new_source}{maybe_as}.unwrap())");
                } else {
                    new_source = format!("(*{new_source})");
                }
            }
            ProjectionElem::Field(_, _) => {
                let field_name = source_stack[i].rsplit_once('.').unwrap().1;
                new_source = format!("{new_source}.{field_name}");
            }
            x => unimplemented!("{x:?}"),
        }
    }

    new_source
}

fn rewrite_malloc<'tcx>(
    cx: &mut BodyRewriteCtxt,
    bb_id: BasicBlock,
    rewritten_stmts: &mut HashSet<Location>,
) {
    let bb = &cx.body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = cx.body.terminator_loc(bb_id);
    let (_args, (dest_place, dest_bb)) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    if place_result(cx, cx.ownership, terminator_loc, dest_place.as_ref()) != Some(true) {
        return;
    }

    let cast_loc = dest_bb.start_location();
    let cast_statement = match cx.body.stmt_at(cast_loc) {
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
    let malloc_source = cx.span_to_snippet(malloc_span);
    let ty = malloc_source.rsplit_once(' ').unwrap().1;

    let mut new_source = format!("Box::<{ty}>::new(unsafe {{ std::mem::zeroed() }})");
    if place_result(cx, cx.null, terminator_loc, dest_place.as_ref()) == Some(true) {
        new_source.insert_str(0, "Some(");
        new_source.push(')');
    }

    rewritten_stmts.insert(cast_loc);
    cx.rewrite(malloc_span, new_source, "use box instead of malloc");
}

fn rewrite_realloc(
    cx: &mut BodyRewriteCtxt,
    bb_id: BasicBlock,
    rewritten_stmts: &mut HashSet<Location>,
) {
    let bb = &cx.body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = cx.body.terminator_loc(bb_id);
    let (args, (dest_place, dest_bb)) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    if place_result(cx, cx.fatness, terminator_loc, dest_place.as_ref()) != Some(true) {
        return;
    }

    let ptr = args[0].place().unwrap().as_local().unwrap();
    let size = args[1].place().unwrap().as_local().unwrap();
    let ptr_expr = uncast(cx, terminator_loc, ptr).unwrap();
    let size_expr = uncast(cx, terminator_loc, size).unwrap();

    let cast_loc = dest_bb.start_location();
    let cast_statement = match cx.body.stmt_at(cast_loc) {
        Either::Left(s) => s,
        Either::Right(_) => todo!("realloc dest isn't statement"),
    };
    // let x = realloc(_, _) as *mut Foo
    let cast_span = match &cast_statement.kind {
        StatementKind::Assign(box (_, Rvalue::Cast(_, op, _)))
            if op.place() == Some(dest_place) =>
        {
            cast_statement.source_info.span
        }
        _ => todo!("realloc not followed by cast"),
    };
    // realloc(_, _) as *mut Foo
    let realloc_span = cast_span.with_lo(terminator.source_info.span.lo());

    rewritten_stmts.insert(cast_loc);
    cx.rewrite(
        realloc_span,
        format!(
            "{{ \
            let mut v: Vec<_> = Box::from_raw({}).into(); \
            v.resize({} as usize, 0); \
            Box::into_raw(v.into_boxed_slice()) \
        }}",
            ptr_expr, size_expr,
        ),
        "use vec instead of realloc",
    );
}

fn rewrite_calloc(
    cx: &mut BodyRewriteCtxt,
    bb_id: BasicBlock,
    rewritten_stmts: &mut HashSet<Location>,
) {
    let bb = &cx.body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = cx.body.terminator_loc(bb_id);
    let (args, (dest_place, dest_bb)) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    if place_result(cx, cx.fatness, terminator_loc, dest_place.as_ref()) != Some(true) {
        return;
    }

    let nmemb = args[0].place().unwrap().as_local().unwrap();
    let nmemb_expr = uncast(cx, terminator_loc, nmemb).unwrap();

    let cast_loc = dest_bb.start_location();
    let cast_statement = match cx.body.stmt_at(cast_loc) {
        Either::Left(s) => s,
        Either::Right(_) => todo!("calloc dest isn't statement"),
    };
    // let x = calloc(_) as *mut Foo
    let cast_span = match &cast_statement.kind {
        StatementKind::Assign(box (_, Rvalue::Cast(_, op, _)))
            if op.place() == Some(dest_place) =>
        {
            cast_statement.source_info.span
        }
        _ => todo!("calloc not followed by cast"),
    };
    // calloc(_, _) as *mut Foo
    let calloc_span = cast_span.with_lo(terminator.source_info.span.lo());

    rewritten_stmts.insert(cast_loc);
    cx.rewrite(
        calloc_span,
        format!(
            "Box::into_raw(vec![Default::default(); {} as usize].into_boxed_slice())",
            nmemb_expr,
        ),
        "use vec instead of calloc",
    );
}

fn rewrite_offset(
    cx: &mut BodyRewriteCtxt,
    bb_id: BasicBlock,
    rewritten_stmts: &mut HashSet<Location>,
) {
    let bb = &cx.body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = cx.body.terminator_loc(bb_id);
    let (args, (dest_place, dest_bb)) = match &terminator.kind {
        TerminatorKind::Call {
            args, destination, ..
        } => (args, destination.unwrap()),
        _ => panic!(),
    };
    let dest_local = dest_place.as_local().unwrap();

    let ptr_place = args[0].place().unwrap().as_ref();
    if place_result(cx, cx.fatness, terminator_loc, ptr_place) != Some(true) {
        return;
    }

    let ptr = args[0].place().unwrap().as_local().unwrap();
    // ptr
    let ptr_expr = uncast(cx, terminator_loc, ptr).unwrap();
    let offset = args[1].place().unwrap().as_local().unwrap();
    // ptr.offset(_)
    let offset_expr = uncast(cx, terminator_loc, offset).unwrap();

    // write case:
    // *(ptr.offset(_)) = ...
    // read case:
    // unknown yet

    // we have a problem! buffer_good has an offset call at line 282 that *doesn't* terminate into
    // a using statement at all. there's a whole other basic block before the use. maybe look into
    // source map to find the use?
    let deref_stmt = match cx.body.stmt_at(dest_bb.start_location()) {
        Either::Left(s) => s,
        Either::Right(_) => return,
    };

    match &deref_stmt.kind {
        StatementKind::Assign(box (place, rvalue)) => {
            if let PlaceRef {
                local,
                projection: [ProjectionElem::Deref],
            } = place.as_ref()
            {
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
        _ => {}
    }
}

fn place_result<'tcx, A: AnalysisResults>(
    cx: &BodyRewriteCtxt<'tcx, '_, '_>,
    analysis: &A,
    loc: Location,
    place: PlaceRef<'tcx>,
) -> Option<bool> {
    if !place.ty(cx.body, cx.tcx).ty.is_unsafe_ptr() {
        // no ownership info for non-pointer type
        return None;
    }
    assert!(place
        .projection
        .iter()
        .all(|e| matches!(e, ProjectionElem::Field(_, _) | ProjectionElem::Deref)));
    let field_idx = place
        .projection
        .iter()
        .rev()
        .enumerate()
        .find(|(_i, elem)| matches!(elem, ProjectionElem::Field(_, _)));
    if let Some((idx, ProjectionElem::Field(field, _ty))) = field_idx {
        // place of the adt
        let place = PlaceRef {
            local: place.local,
            projection: &place.projection[..place.projection.len() - 1],
        };
        // TODO: is it?
        let n_derefs = idx;
        let struct_def_id = place
            .ty(cx.body, cx.tcx)
            .ty
            .ty_adt_def()
            .unwrap()
            .did
            .as_local()
            .unwrap();
        return analysis.field_result(struct_def_id, *field, n_derefs);
    } else {
        let n_derefs = place.projection.len();
        return analysis.local_result_at(cx.def_id, place.local, loc, n_derefs);
    }
}

/// returns the source of an expression with all its outer `as _` removed
fn uncast(cx: &mut BodyRewriteCtxt, loc: Location, local: Local) -> Option<String> {
    let source_map = &cx.fatness.ssa_name_source_map[cx.func];
    let ssa_idx = source_map.try_use(local, loc).unwrap();
    let def_loc_rich = &cx.fatness.def_sites[cx.func].defs[local][ssa_idx];
    let def_loc = match def_loc_rich {
        RichLocation::Mir(l) => *l,
        RichLocation::Entry => todo!(),
        _ => todo!(),
    };
    let def_stmt = match cx.body.stmt_at(def_loc) {
        Either::Left(s) => s,
        Either::Right(terminator) => {
            return Some(cx.span_to_snippet(terminator.source_info.span));
        }
    };
    match &def_stmt.kind {
        StatementKind::Assign(box (_place, rvalue)) => {
            if let Rvalue::Cast(_kind, operand, _ty) = rvalue {
                return uncast(cx, def_loc, operand.place().unwrap().as_local().unwrap());
            }
            let span = def_stmt.source_info.span;
            return Some(cx.span_to_snippet(span));
        }
        _ => todo!(),
    }
}
