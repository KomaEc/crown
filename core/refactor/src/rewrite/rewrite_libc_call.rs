use analysis::{fat_thin_analysis, call_graph::Func, ssa::RichLocation};
use either::Either;
use rewriter::Rewriter;
use rustc_middle::{mir::{Terminator, TerminatorKind, Body, Local, Location, StatementKind, Rvalue}, ty::TyCtxt};

pub fn rewrite_libc_call<'tcx>(
    tcx: TyCtxt<'tcx>, 
    rewriter: &mut Rewriter, 
    body: &Body, 
    caller: Func, 
    fatness_analysis: &fat_thin_analysis::CrateSummary, 
    terminator: &Terminator<'tcx>, 
    location: Location,
) {
    let callee = match &terminator.kind {
        TerminatorKind::Call { func, .. } => func,
        _ => panic!(),
    };
    let const_ty = callee.constant().unwrap().literal.ty();
    let callee_def_id = match const_ty.kind() {
        rustc_middle::ty::TyKind::FnDef(def_id, _substs) => def_id,
        _ => panic!(),
    };
    let name = tcx.def_path_debug_str(*callee_def_id);
    match name {
        x if x.ends_with("::printf") => rewrite_printf(
            tcx, rewriter, fatness_analysis, caller, body, location, terminator
        ),
        _ => tracing::debug!("not handling foreign function {name}"),
    }
}

pub fn rewrite_printf<'tcx>(
    tcx: TyCtxt<'tcx>,
    rewriter: &mut Rewriter,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    func: Func,
    body: &Body,
    loc: Location,
    terminator: &Terminator<'tcx>,
) {
    let args = match &terminator.kind {
        TerminatorKind::Call { args, .. } => args,
        _ => panic!(),
    };
    let format_string_literal = uncast(
        tcx,
        fatness_analysis,
        func,
        body,
        loc,
        args[0].place().unwrap().as_local().unwrap()
    ).unwrap();
    let format_string = format_string_literal
        .trim_start_matches(r#"b""#)
        .trim_end_matches('"');
    let format_string = format_string.replace("%s", "{:?}");
    let format_string = format_string.replace("%d", "{:?}");
    if format_string.contains('%') {
        panic!("unsupported c formatter in {}", format_string);
    }
    
    let mut args_string = String::new();
    for (i, arg) in args.iter().enumerate() {
        args_string.push_str(&source(
            tcx,
            fatness_analysis,
            func,
            body,
            loc,
            arg.place().unwrap().as_local().unwrap(),
        ).unwrap());
        args_string.push_str(", ");
    }
    
    rewriter.make_suggestion(
        tcx,
        terminator.source_info.span,
        String::from("use print!() instead of printf"),
        format!(r#"print!({format_string}, {args_string})"#),
    );
}

/// returns the source of an expression with all its outer `as` removed
fn uncast(
    tcx: TyCtxt<'_>,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    func: Func,
    body: &Body,
    loc: Location,
    local: Local,
) -> Option<String> {
    let source_map = &fatness_analysis.ssa_name_source_map[func];
    let ssa_idx = source_map.try_use(local, loc).unwrap();
    let def_loc_rich = &fatness_analysis.def_sites[func].defs[local][ssa_idx];
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
                    fatness_analysis,
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

fn source(
    tcx: TyCtxt<'_>,
    fatness_analysis: &fat_thin_analysis::CrateSummary,
    func: Func,
    body: &Body,
    loc: Location,
    local: Local,
) -> Option<String> {
    let source_map = &fatness_analysis.ssa_name_source_map[func];
    let ssa_idx = source_map.try_use(local, loc).unwrap();
    let def_loc_rich = &fatness_analysis.def_sites[func].defs[local][ssa_idx];
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
            let span = def_stmt.source_info.span;
            return tcx.sess.source_map().span_to_snippet(span).ok();
        }
        _ => todo!(),
    }
}
