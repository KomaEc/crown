use std::collections::HashSet;

use analysis::{
    api::AnalysisResults, call_graph::Func, fatness_analysis, mutability_analysis, null_analysis,
    ownership_analysis,
};
use either::Either;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{
        BasicBlock, Body, CastKind, Constant, ConstantKind, Local, Location, Operand, Place,
        PlaceRef, ProjectionElem, Rvalue, Statement, StatementKind, TerminatorKind,
    },
    ty::{Const, ScalarInt, TyCtxt, TyKind},
};
use rustc_span::{BytePos, Span};
use tracing::{debug, debug_span, info_span, trace, warn};

pub struct BodyRewriteCtxt<'tcx, 'a, 'b> {
    pub tcx: TyCtxt<'tcx>,
    pub rewriter: &'a mut rewriter::Rewriter,
    pub ownership: &'a ownership_analysis::InterSummary,
    pub mutability: &'a mutability_analysis::CrateResults<'tcx, 'b>,
    pub fatness: &'a fatness_analysis::CrateResults<'tcx, 'b>,
    pub null: &'a null_analysis::CrateResults<'tcx, 'b>,
    pub func: Func,
    pub def_id: LocalDefId,
    pub body: &'a Body<'tcx>,
}

impl BodyRewriteCtxt<'_, '_, '_> {
    fn rewrite(&mut self, span: Span, to: impl Into<String>, msg: &'static str) {
        let to = to.into();
        debug!(?span, ?to, "suggestion");
        self.rewriter
            .make_suggestion(self.tcx, span, msg.into(), to.into());
    }

    fn span_to_snippet(&self, span: Span) -> String {
        self.tcx.sess.source_map().span_to_snippet(span).unwrap()
    }
}

pub fn rewrite_body(cx: &mut BodyRewriteCtxt) {
    let fn_name = cx.tcx.def_path_str(cx.def_id.to_def_id());
    let _guard = info_span!("body", ?fn_name).entered();
    // some terminators also cause statement rewrites, so we track which ones have been rewritten
    // and don't try to rewrite them twice
    let mut rewritten_stmts = HashSet::new();
    for (bb_id, bb) in cx.body.basic_blocks().iter_enumerated() {
        let terminator = bb.terminator();
        if let TerminatorKind::Call { func: callee, .. } = &terminator.kind {
            let const_ty = callee.constant().unwrap().literal.ty();
            let TyKind::FnDef(callee_def_id, _) = const_ty.kind() else { panic!() };
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
                    for (idx, stmt) in bb.statements.iter().enumerate() {
                        if span.contains(stmt.source_info.span) {
                            trace!("free {span:?} contains {:?}", stmt.source_info.span);
                            rewritten_stmts.insert(Location {
                                block: bb_id,
                                statement_index: idx,
                            });
                        }
                    }
                }
                "std::ptr::mut_ptr::<impl *mut T>::offset"
                | "std::ptr::const_ptr::<impl *const T>::offset" => {
                    rewrite_offset(cx, bb_id, &mut rewritten_stmts);
                }
                "std::ptr::mut_ptr::<impl *mut T>::is_null"
                | "std::ptr::const_ptr::<impl *const T>::is_null" => {
                    let call_span = span.with_lo(BytePos(span.hi().0 - ".is_null()".len() as u32));
                    cx.rewrite(call_span, ".is_none()", "");
                }
                _ => rewrite_terminator(cx, bb_id),
            }
        }

        let stmts = bb.statements.iter().enumerate().map(|(idx, stmt)| {
            (
                Location {
                    block: bb_id,
                    statement_index: idx,
                },
                stmt,
            )
        });
        for (loc, stmt) in stmts {
            let _guard = debug_span!("stmt", ?loc).entered();
            if rewritten_stmts.contains(&loc) {
                continue;
            }
            if handle_split_assignment(cx, loc, &mut rewritten_stmts) {
                continue;
            }
            let StatementKind::Assign(box (l_place, rvalue)) = &stmt.kind else { continue };
            let source = cx.span_to_snippet(stmt.source_info.span);
            if source.contains(" = ") {
                trace!("reassignment");
                rewrite_reassignment(cx, loc, stmt);
            } else {
                // binding or temporary. rules for these are the same so far
                trace!("binding");
                let new_source = rewrite_rvalue(cx, loc, rvalue, loc, l_place.as_ref(), &source);
                if new_source != source {
                    cx.rewrite(stmt.source_info.span, new_source, "");
                }
            }
        }
    }
}

fn rewrite_terminator<'tcx>(cx: &mut BodyRewriteCtxt<'tcx, '_, '_>, bb_id: BasicBlock) {
    let bb = &cx.body.basic_blocks()[bb_id];
    let terminator = bb.terminator();
    let terminator_loc = cx.body.terminator_loc(bb_id);
    let TerminatorKind::Call {
        func: callee, destination: Some((dest_place, _dest_bb)), ..
    } = &terminator.kind else { panic!() };
    let const_ty = callee.constant().unwrap().literal.ty();
    let TyKind::FnDef(callee_def_id, _) = const_ty.kind() else { panic!() };
    let Some(callee_def_id) = callee_def_id.as_local() else { return };

    let is_raw_ptr = cx
        .tcx
        .fn_sig(callee_def_id)
        .no_bound_vars()
        .unwrap()
        .output()
        .is_unsafe_ptr();
    if !is_raw_ptr {
        return;
    }

    let l_null = cx
        .null
        .place_result(cx.tcx, cx.def_id, terminator_loc, dest_place.as_ref());
    let r_null = cx.null.sig_result(callee_def_id, Local::from_usize(0), 0);

    let span = terminator.source_info.span;
    let source = cx.span_to_snippet(span);

    match (l_null, r_null) {
        (Some(true), Some(false)) => {
            cx.rewrite(span.shrink_to_lo(), "Some(", "");
            cx.rewrite(span.shrink_to_hi(), ")", "");
        }
        (Some(false), Some(true)) => cx.rewrite(span.shrink_to_hi(), ".unwrap()", ""),
        _ => {}
    }
}

fn handle_split_assignment<'tcx>(
    cx: &mut BodyRewriteCtxt<'tcx, '_, '_>,
    loc: Location,
    rewritten_stmts: &mut HashSet<Location>,
) -> bool {
    let stmt = cx.body.stmt_at(loc).left().unwrap();
    let StatementKind::Assign(box (l_place, rvalue)) = &stmt.kind else { return false };
    let next_loc = loc.successor_within_block();
    let Some(next_stmt) = cx.body.stmt_at(next_loc).left() else { return false };
    let StatementKind::Assign(box (next_l_place, next_rvalue)) = &next_stmt.kind else {
        return false
    };
    let (Rvalue::Use(next_op) | Rvalue::Cast(_, next_op, _)) = next_rvalue else { return false };
    let Some(next_r_place) = next_op.place() else { return false };
    let span = next_stmt.source_info.span;
    if !(next_r_place.as_local().is_some()
        && next_r_place.as_local() == l_place.as_local()
        && span.contains(stmt.source_info.span))
    {
        return false;
    }

    let source = cx.span_to_snippet(span);
    if !source.contains(" = ") {
        return false;
    }
    let (l_source, r_source) = source.split_once(" = ").unwrap();
    let l_span = span.with_hi(BytePos(span.lo().0 + l_source.len() as u32));
    let r_span = span.with_lo(BytePos(span.hi().0 - r_source.len() as u32));
    let l_new = rewrite_place_expr(cx, next_loc, next_l_place.clone(), false, l_source);
    // important detaiil - using stmt1 rvalue in rewriting stmt2
    let r_new = rewrite_rvalue(cx, loc, rvalue, next_loc, next_l_place.as_ref(), r_source);
    if l_new != l_source {
        cx.rewrite(l_span, l_new, "");
    }
    if r_new != r_source {
        cx.rewrite(r_span, r_new, "");
    }
    rewritten_stmts.insert(next_loc);
    return true;
}

pub fn rewrite_reassignment<'tcx>(
    cx: &mut BodyRewriteCtxt<'tcx, '_, '_>,
    loc: Location,
    stmt: &Statement<'tcx>,
) {
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
    let rhs_new = rewrite_rvalue(cx, loc, rvalue, loc, l_place.as_ref(), rhs_source);
    if lhs_new != lhs_source {
        cx.rewrite(lhs_span, lhs_new, "");
    }
    if rhs_new != rhs_source {
        cx.rewrite(rhs_span, rhs_new, "");
    }
}

pub fn rewrite_rvalue<'tcx>(
    cx: &mut BodyRewriteCtxt<'tcx, '_, '_>,
    loc: Location,
    rvalue: &Rvalue<'tcx>,
    l_loc: Location,
    l_place: PlaceRef<'tcx>,
    source: &str,
) -> String {
    match rvalue {
        // rvalue is just a place
        Rvalue::Use(Operand::Copy(r_place) | Operand::Move(r_place)) => {
            let mut new = rewrite_place_expr(cx, loc, r_place.clone(), true, &source);

            let l_ownership = cx.ownership.place_result(cx.tcx, cx.def_id, l_loc, l_place);
            let r_ownership = cx
                .ownership
                .place_result(cx.tcx, cx.def_id, loc, r_place.as_ref());
            let l_mutability = cx
                .mutability
                .place_result(cx.tcx, cx.def_id, l_loc, l_place);
            let l_null = cx.null.place_result(cx.tcx, cx.def_id, l_loc, l_place);
            let r_null = cx
                .null
                .place_result(cx.tcx, cx.def_id, loc, r_place.as_ref());
            if r_null == Some(true) && r_ownership == Some(true) {
                if l_ownership == Some(true) {
                    new.push_str(".take()");
                } else if l_ownership == Some(false) {
                    if l_mutability == Some(true) {
                        new.push_str(".as_deref_mut()");
                    } else {
                        new.push_str(".as_deref()");
                    }
                }
            }

            match (l_null, r_null) {
                (Some(true), Some(false)) => {
                    new.insert_str(0, "Some(");
                    new.push(')');
                }
                (Some(false), Some(true)) => new.push_str(".unwrap()"),
                _ => {}
            }
            new
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
            warn!(?rvalue, "not rewriting unsupported rvalue");
            return source.to_owned();
        }
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
                let ownership = cx
                    .ownership
                    .place_result(cx.tcx, cx.def_id, loc, base_place);
                let mutability = cx
                    .mutability
                    .place_result(cx.tcx, cx.def_id, loc, base_place);
                let nullable = cx
                    .null
                    .place_result(cx.tcx, cx.def_id, loc, base_place)
                    .unwrap();
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
    let TerminatorKind::Call {
        destination: Some((dest_place, dest_bb)), ..
    } = terminator.kind else { panic!() };
    if cx
        .ownership
        .place_result(cx.tcx, cx.def_id, terminator_loc, dest_place.as_ref())
        != Some(true)
    {
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
    if cx
        .null
        .place_result(cx.tcx, cx.def_id, terminator_loc, dest_place.as_ref())
        == Some(true)
    {
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
    let TerminatorKind::Call {
        args, destination: Some((dest_place, dest_bb)), ..
    } = &terminator.kind else { panic!() };
    if cx
        .fatness
        .place_result(cx.tcx, cx.def_id, terminator_loc, dest_place.as_ref())
        != Some(true)
    {
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
            if op.place() == Some(*dest_place) =>
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
    let TerminatorKind::Call {
        args, destination: Some((dest_place, dest_bb)), ..
    } = &terminator.kind else { panic!() };
    if cx
        .fatness
        .place_result(cx.tcx, cx.def_id, terminator_loc, dest_place.as_ref())
        != Some(true)
    {
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
            if op.place() == Some(*dest_place) =>
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
    let TerminatorKind::Call {
        args, destination: Some((dest_place, dest_bb)), ..
    } = &terminator.kind else { panic!() };
    let dest_local = dest_place.as_local().unwrap();

    let ptr_place = args[0].place().unwrap().as_ref();
    if cx
        .fatness
        .place_result(cx.tcx, cx.def_id, terminator_loc, ptr_place)
        != Some(true)
    {
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
                if operand.place() == Some(*dest_place) {
                    // reading from the slice into a local
                    todo!()
                }
            }
        }
        _ => {}
    }
}

/// returns the source of an expression with all its outer `as _` removed
fn uncast(_cx: &mut BodyRewriteCtxt, _loc: Location, _local: Local) -> Option<String> {
    todo!("figure out how to get defs now that ssa fatness and mutability are gone");
}
