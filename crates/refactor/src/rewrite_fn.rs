mod boundary;
mod libc_call;
mod library_call;

use std::str::FromStr;

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
        Body, Constant, Local, LocalInfo, Location, NonDivergingIntrinsic, Operand, Place, Rvalue,
        Statement, StatementKind, Terminator, TerminatorKind, VarDebugInfoContents, RETURN_PLACE,
    },
    ty::{Ty, TyCtxt, TyKind},
};
use rustc_span::{Span, Symbol};
use rustc_type_ir::TyKind::FnDef;
use smallvec::SmallVec;
use syn::__private::ToTokens;

use crate::{
    rewrite_ty::rewrite_hir_ty, FnLocals, PointerKind, RawMeta, RefactorOptions, StructFields,
};

pub fn rewrite_fns(
    fns: &[DefId],
    fn_decision: &FnLocals,
    struct_decision: &StructFields,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
    options: &RefactorOptions,
) {
    let RefactorOptions {
        type_only,
        type_reconstruction,
        ..
    } = *options;
    for &did in fns {
        let local_data = fn_decision.local_data(&did);
        let body = tcx.optimized_mir(did);
        rewrite_fn_sig(body, local_data, rewriter, tcx, type_reconstruction);
        if !type_only && !tcx.fn_sig(did).c_variadic() {
            rewrite_fn(
                body,
                fn_decision.local_data(&did),
                fn_decision,
                struct_decision,
                rewriter,
                tcx,
            );
        }
    }
}

fn rewrite_fn_sig<'tcx>(
    body: &Body<'tcx>,
    decision: &[SmallVec<[PointerKind; 3]>],
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt<'tcx>,
    type_reconstruction: bool,
) {
    let fn_item = tcx.hir().expect_item(body.source.def_id().expect_local());
    let ItemKind::Fn(fn_sig, _, body_id) = &fn_item.kind else { unreachable!() };

    if let rustc_hir::FnRetTy::Return(ret_ty) = fn_sig.decl.output {
        rewrite_hir_ty(ret_ty, &decision[0], rewriter, tcx, type_reconstruction);
    }

    for (ty, decision) in itertools::izip!(fn_sig.decl.inputs, &decision[1..]) {
        rewrite_hir_ty(ty, decision, rewriter, tcx, type_reconstruction);
    }

    let fn_body = tcx.hir().body(*body_id);
    for param in fn_body.params {
        let rustc_hir::PatKind::Binding(annot, _, _, _) = param.pat.kind else { unreachable!() };
        if matches!(annot.1, rustc_hir::Mutability::Not) {
            rewriter.replace(tcx, param.span.shrink_to_lo(), "mut ".to_owned())
        }
    }
}

fn rewrite_fn<'tcx>(
    body: &Body<'tcx>,
    local_decision: &[SmallVec<[PointerKind; 3]>],
    fn_decision: &FnLocals,
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

    // analysis::use_def::show_def_use_chain(body, &def_use_chain);

    let rewrite_ctxt = FnRewriteCtxt {
        local_decision,
        fn_decision,
        struct_decision,
        body,
        def_use_chain: &def_use_chain,
        user_idents: &user_idents,
        tcx,
    };

    println!("Rewriting {}", tcx.def_path_str(body.source.def_id()));

    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        let mut statement_index = 0;
        for statement in bb_data.statements.iter() {
            let location = Location {
                block: bb,
                statement_index,
            };

            rewrite_ctxt.rewrite_statement(statement, location, rewriter);

            statement_index += 1;
        }

        if let Some(terminator) = &bb_data.terminator {
            let location = Location {
                block: bb,
                statement_index,
            };

            rewrite_ctxt.rewrite_terminator(terminator, location, rewriter);
        }
    }
}

fn accum_deref_copies<'tcx>(
    mut place: Place<'tcx>,
    mut location: Location,
    def_use_chain: &DefUseChain,
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> (Place<'tcx>, Location) {
    let mut local = place.local;

    while matches!(
        body.local_decls[local].local_info,
        Some(box LocalInfo::DerefTemp)
    ) {
        let def_loc = def_use_chain.def_loc(local, location);
        let RichLocation::Mir(def_loc) = def_loc else { panic!() };
        let Left(stmt) = body.stmt_at(def_loc) else { panic!() };
        let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
        let Rvalue::CopyForDeref(rplace) = rvalue else { panic!() };
        local = rplace.local;
        place = rplace.project_deeper(place.projection, tcx);
        location = def_loc;
    }

    (place, location)
}

pub struct FnRewriteCtxt<'tcx, 'me> {
    local_decision: &'me [SmallVec<[PointerKind; 3]>],
    fn_decision: &'me FnLocals,
    struct_decision: &'me StructFields,
    body: &'me Body<'tcx>,
    def_use_chain: &'me DefUseChain,
    user_idents: &'me FxHashMap<Local, Symbol>,
    tcx: TyCtxt<'tcx>,
}

/// Information of a lvalue/rvalue coming from a place
#[derive(Clone, Copy, Debug)]
enum PlaceCtxt<'me> {
    Ptr(&'me [PointerKind]),
    Struct(DefId),
    Irrelavent,
}

impl<'me> PlaceCtxt<'me> {
    fn from_ptr_ctxt(ty: Ty, ctxt: &'me [PointerKind]) -> Self {
        if ctxt.is_empty() {
            if let TyKind::Adt(adt_def, _) = ty.kind() {
                if adt_def.is_struct() {
                    return PlaceCtxt::Struct(adt_def.did());
                }
            }
            PlaceCtxt::Irrelavent
        } else {
            PlaceCtxt::Ptr(ctxt)
        }
    }

    fn expect_ptr(self) -> &'me [PointerKind] {
        match self {
            PlaceCtxt::Ptr(ptr_kinds) => ptr_kinds,
            _ => unreachable!(),
        }
    }

    fn is_ptr(self) -> bool {
        matches!(self, Self::Ptr(..))
    }

    fn is_move_obj<'tcx>(&self, rewrite_ctxt: &FnRewriteCtxt<'tcx, 'me>) -> bool {
        match self {
            PlaceCtxt::Ptr(ptr_kinds) => {
                matches!(ptr_kinds.first(), Some(ptr_kind) if ptr_kind.is_move() || ptr_kind.is_raw_move())
            }
            PlaceCtxt::Struct(did) => rewrite_ctxt
                .struct_decision
                .is_owning(rewrite_ctxt.tcx, did),
            PlaceCtxt::Irrelavent => false,
        }
    }

    fn is_rustc_move_obj<'tcx>(&self, rewrite_ctxt: &FnRewriteCtxt<'tcx, 'me>) -> bool {
        match self {
            PlaceCtxt::Ptr(ptr_kinds) => {
                matches!(ptr_kinds.first(), Some(ptr_kind) if ptr_kind.is_move())
            }
            PlaceCtxt::Struct(did) => {
                let fields_data = rewrite_ctxt.struct_decision.field_data(did);
                fields_data.iter().any(|field| {
                    field
                        .iter()
                        .any(|ptr_kind| ptr_kind.is_move() || ptr_kind.is_raw_move())
                })
            }
            PlaceCtxt::Irrelavent => false,
        }
    }

    fn is_copy_obj<'tcx>(&self, rewrite_ctxt: &FnRewriteCtxt<'tcx, 'me>) -> bool {
        match self {
            PlaceCtxt::Ptr(ptr_kinds) => {
                matches!(ptr_kinds.first(), Some(ptr_kind) if ptr_kind.is_copy())
            }
            PlaceCtxt::Struct(_) => !self.is_move_obj(rewrite_ctxt),
            PlaceCtxt::Irrelavent => true,
        }
    }

    fn is_rustc_copy_obj<'tcx>(&self, rewrite_ctxt: &FnRewriteCtxt<'tcx, 'me>) -> bool {
        match self {
            PlaceCtxt::Ptr(ptr_kinds) => {
                matches!(ptr_kinds.first(), Some(ptr_kind) if ptr_kind.is_raw() || ptr_kind.is_const())
            }
            PlaceCtxt::Struct(_) => !self.is_rustc_move_obj(rewrite_ctxt),
            PlaceCtxt::Irrelavent => true,
        }
    }

    fn is_raw_ptr(self) -> bool {
        matches!(self, PlaceCtxt::Ptr(ptr_kinds) if matches!(ptr_kinds.first(), Some(ptr_kind) if ptr_kind.is_raw()))
    }

    fn is_irrelavent(self) -> bool {
        matches!(self, Self::Irrelavent)
    }
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
enum PlaceLoadMode {
    ByValue = 0,
    ByRef = 1,
    ByAddr = 2,
}

impl<'tcx, 'me> FnRewriteCtxt<'tcx, 'me> {
    fn acquire_place_info(&self, place: &Place<'tcx>) -> PlaceCtxt<'me> {
        let FnRewriteCtxt {
            local_decision,
            struct_decision,
            body,
            ..
        } = *self;

        let mut ptr_kinds = &local_decision[place.local.as_usize()][..];
        let mut ptr_kinds_index = 0;
        let mut ty = body.local_decls[place.local].ty;
        for proj in place.projection {
            match proj {
                rustc_middle::mir::ProjectionElem::Deref => {
                    ptr_kinds_index += 1;
                    ty = ty.builtin_deref(true).unwrap().ty;
                }
                rustc_middle::mir::ProjectionElem::Field(f, field_ty) => {
                    assert_eq!(ptr_kinds_index, ptr_kinds.len());
                    let adt_def = ty.ty_adt_def().unwrap();
                    if adt_def.is_union() {
                        return PlaceCtxt::Irrelavent;
                    }
                    ptr_kinds = &struct_decision.field_data(&adt_def.did())[f.index()][..];
                    ptr_kinds_index = 0;
                    ty = field_ty;
                }
                rustc_middle::mir::ProjectionElem::Index(_) => ty = ty.builtin_index().unwrap(),
                _ => unreachable!(),
            }
        }

        let ptr = &ptr_kinds[ptr_kinds_index..];

        PlaceCtxt::from_ptr_ctxt(ty, ptr)
    }

    /// [`expr_span`] is already rewritten
    fn adapt_usage(
        &self,
        expr_span: Span,
        ty: Ty,
        is_indirect: bool,
        produced: PlaceCtxt,
        required: PlaceCtxt,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt { tcx, .. } = *self;

        if required.is_rustc_move_obj(self) {
            if produced.is_rustc_move_obj(self) {
                // move to move
                if is_indirect {
                    rewriter.replace(tcx, expr_span.shrink_to_hi(), ".take()".to_owned())
                }
            } else {
                assert!(required.is_ptr());
                assert!(produced.is_raw_ptr());
                rewriter.replace(
                    tcx,
                    expr_span.shrink_to_lo(),
                    "Some(Box::from_raw(".to_owned(),
                );
                rewriter.replace(tcx, expr_span.shrink_to_hi(), "))".to_owned())
            }
        } else if required.is_rustc_copy_obj(self) {
            if required.is_move_obj(self) {
                // raw move
                assert!(required.is_raw_ptr());

                // we can not have the assertion below because of lacking precision
                // assert!(produced.is_move_obj(self));
                if produced.is_rustc_move_obj(self) {
                    assert!(produced.is_ptr());
                    // Box to raw (move)
                    rewriter.replace(
                        tcx,
                        expr_span.shrink_to_hi(),
                        ".map(|b| Box::into_raw(b)).unwrap_or(std::ptr::null_mut())".to_owned(),
                    )
                } else {
                    assert!(produced.is_raw_ptr());
                    // nothing to be done here
                }
            } else if required.is_raw_ptr() {
                // raw mut or raw const
                assert!(produced.is_ptr());
                let pointee_ty = ty.builtin_deref(true).unwrap().ty;
                let mut pointee_ty_str = format!("{pointee_ty}");
                if pointee_ty_str.starts_with("src::") || pointee_ty_str.starts_with("bin::") {
                    pointee_ty_str = "crate::".to_owned() + &pointee_ty_str
                }
                let pointee_ty_str = pointee_ty_str;

                if !produced.is_raw_ptr() {
                    // &mut to *const,*mut or & to *const

                    let (usage, target_ty) = if required.expect_ptr()[0].is_raw_const() {
                        (
                            if produced.expect_ptr()[0].is_const() {
                                "clone"
                            } else {
                                "as_deref"
                            },
                            "const",
                        )
                    } else {
                        ("as_deref_mut", "mut")
                    };

                    rewriter.replace(
                        tcx,
                        expr_span.shrink_to_hi(),
                        format!(
                            ".{usage}().map(|r| r as *{target_ty} _).unwrap_or(std::ptr::null{}())",
                            (target_ty == "mut").then_some("_mut").unwrap_or("")
                        ),
                    )
                } else if required.expect_ptr()[0].is_raw_const()
                    && produced.expect_ptr()[0].is_raw_mut()
                {
                    // raw mut to raw const
                    rewriter.replace(
                        tcx,
                        expr_span.shrink_to_hi(),
                        format!(" as *const {pointee_ty_str}"),
                    )
                }
            } else if required.is_ptr() {
                assert!(produced.is_ptr());
                // &ref
                if produced.is_raw_ptr() {
                    rewriter.replace(tcx, expr_span.shrink_to_hi(), ".as_ref()".to_owned())
                } else if !produced.is_rustc_copy_obj(self) {
                    rewriter.replace(tcx, expr_span.shrink_to_hi(), ".as_deref()".to_owned())
                } else {
                    rewriter.replace(tcx, expr_span.shrink_to_hi(), ".clone()".to_owned())
                }
            } else if required.is_irrelavent() {
                if produced.is_ptr() && !produced.is_raw_ptr() {
                    // irrelavent context, cast expr into raw pointer
                    // this happens when comparing addr
                    rewriter.replace(
                        tcx,
                        expr_span.shrink_to_hi(),
                        ".as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null())".to_owned(),
                    )
                }
            }

            // else copyable structs, or primitive, directly copy, nothing to do here
        } else {
            assert!(required.is_ptr());
            assert!(produced.is_ptr());
            if produced.is_raw_ptr() {
                rewriter.replace(tcx, expr_span.shrink_to_hi(), ".as_mut()".to_owned())
            } else {
                rewriter.replace(tcx, expr_span.shrink_to_hi(), ".as_deref_mut()".to_owned())
            }
        }
    }

    fn rewrite_statement(
        &self,
        statement: &Statement<'tcx>,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            user_idents,
            tcx,
            ..
        } = *self;

        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                // TODO constant immediate_lvalue
                let is_func_call_dest = place.is_indirect() && {
                    match def_use_chain.def_loc(place.local, location) {
                        RichLocation::Entry => false,
                        RichLocation::Phi(_) => false,
                        RichLocation::Mir(def_loc) => {
                            matches!(body.stmt_at(def_loc), Right(..))
                        }
                    }
                };

                let is_static_ref = body.local_decls[place.local].is_ref_to_static();

                // rewrite point: non-temporary place
                // this includes 1. place of which base local is a user defined variable
                // 2. place of which base local is a deref tmp, and the rvalue is not another deref tmp
                if user_idents.contains_key(&place.local)
                    || matches!(body.local_decls[place.local].local_info, Some(box LocalInfo::DerefTemp) if !matches!(rvalue, Rvalue::CopyForDeref(..)))
                    || is_func_call_dest
                    || is_static_ref
                {
                    let span = statement.source_info.span;
                    let source_text = tcx.sess.source_map().span_to_snippet(span).unwrap();

                    let source_token_stream =
                        proc_macro2::TokenStream::from_str(&source_text).unwrap();
                    let parsed_expr =
                        syn::parse2::<syn::Expr>(source_token_stream).expect(&source_text);

                    let mut assign_op_pos = None;

                    match parsed_expr {
                        syn::Expr::Assign(assign) => {
                            let assign_op_str = format!("{}", assign.eq_token.to_token_stream());
                            assign_op_pos = source_text.find(&assign_op_str);
                            assert!(assign_op_pos.is_some());
                        }
                        _ => {}
                    }

                    if let Some(assign_pos) = assign_op_pos {
                        // lhs needs to be rewritten for assignment statement

                        assert!(assign_pos > 0);

                        let lhs_span =
                            span.with_hi(span.lo() + rustc_span::BytePos(assign_pos as u32));

                        self.rewrite_place_store(*place, location, lhs_span, rewriter);
                    }

                    let ctxt = self.acquire_place_info(&place);

                    self.rewrite_rvalue_at(rvalue, location, span, ctxt, rewriter);
                }
            }
            StatementKind::Intrinsic(box NonDivergingIntrinsic::Assume(_)) => {
                // rewrite point: assume
                rewriter.replace(tcx, statement.source_info.span, "()".to_owned())
            }
            _ => tracing::error!("{:?} is ignored", statement),
        }
    }

    fn rewrite_terminator(
        &self,
        terminator: &Terminator<'tcx>,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            fn_decision,
            def_use_chain,
            user_idents,
            tcx,
            ..
        } = *self;

        match &terminator.kind {
            TerminatorKind::SwitchInt { discr, .. } => {
                // rewrite point: if expr, match expr
                let mut span = terminator.source_info.span;
                let source_text = common::rewrite::get_snippet(tcx, span).text.1;
                let source_token_stream = proc_macro2::TokenStream::from_str(&source_text).unwrap();
                let parsed_expr = syn::parse2::<syn::Expr>(source_token_stream);
                // if starts with match, the parsed thing cannot be an expr
                if parsed_expr.is_err() && source_text.starts_with("match") {
                    let match_span = span.with_hi(span.lo() + rustc_span::BytePos(5));
                    rewriter.replace(tcx, match_span, "match ".to_owned());
                    span = span.with_lo(span.lo() + rustc_span::BytePos(5));
                }
                let place = discr.place().unwrap();
                self.rewrite_place_load_at::<{ PlaceLoadMode::ByValue as u8 }>(
                    place,
                    location,
                    span,
                    PlaceCtxt::Irrelavent,
                    rewriter,
                );
            }
            TerminatorKind::Call {
                func,
                args,
                destination,
                fn_span,
                ..
            } => {
                // rewrite point: call
                self.rewrite_call(
                    func,
                    args,
                    *destination,
                    *fn_span,
                    location,
                    fn_decision,
                    rewriter,
                );
            }
            TerminatorKind::Return => {
                // rewrite point: return
                if !user_idents.contains_key(&RETURN_PLACE) {
                    let return_ctxt = PlaceCtxt::from_ptr_ctxt(
                        self.body.return_ty(),
                        &self.local_decision[0],
                    );
                    if !matches!(
                        def_use_chain.def_loc(RETURN_PLACE, location),
                        RichLocation::Entry
                    ) {
                        self.rewrite_temporary(RETURN_PLACE, location, return_ctxt, rewriter);
                    }
                }
            }
            TerminatorKind::Goto { .. } => {}
            TerminatorKind::Assert { .. } => {}
            TerminatorKind::Drop { .. } => {
                // only happens when playing with va list
            }
            _ => todo!(
                "terminator kind {:?} @ {:?}",
                terminator.kind,
                terminator.source_info.span
            ),
        }
    }

    fn rewrite_place_store(
        &self,
        place: Place<'tcx>,
        location: Location,
        span: Span,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            local_decision,
            struct_decision,
            body,
            def_use_chain,
            user_idents,
            tcx,
            ..
        } = *self;

        let (resolved_place, _) = accum_deref_copies(place, location, def_use_chain, body, tcx);
        let place = resolved_place;

        let is_static_ref = body.local_decls[place.local].is_ref_to_static();

        let mut replacement = if let Some(replacement) = user_idents
            .get(&place.local)
            .map(|symbol| symbol.to_string())
        {
            replacement
        } else if is_static_ref {
            let &LocalInfo::StaticRef { def_id, is_thread_local } = body.local_decls[place.local].local_info.as_deref().unwrap() else { unreachable!() };
            assert!(!is_thread_local, "thread local is not supported");
            let static_name = tcx.def_path_str(def_id);
            if static_name.starts_with(&tcx.def_path_str(body.source.def_id())) {
                static_name.split("::").last().unwrap().to_owned()
            } else {
                format!("crate::{}", static_name)
            }
        } else {
            tracing::error!("rewrite immediate value, could be static, func call return");
            return;
        };

        let mut index_spans: SmallVec<[Span; 1]> = smallvec::smallvec![];

        let mut ptr_kinds = local_decision[place.local.as_usize()].iter().copied();
        let mut ty = body.local_decls[place.local].ty;
        let mut need_paren = false;

        let mut projection = place.projection.iter();
        if is_static_ref {
            let _ = projection.next();
            let _ = ptr_kinds.next();
            ty = ty.builtin_deref(true).unwrap().ty;
        }

        // Invariant: [`replacement`] is a nullable pointer or a struct
        for proj in projection {
            if need_paren {
                replacement = format!("({replacement})");
                need_paren = false;
            }

            // perform projection
            match proj {
                rustc_middle::mir::ProjectionElem::Deref => {
                    let base_ptr_kind = ptr_kinds.next().unwrap();

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

                    if adt_def.is_union() {
                        // FIXME
                        ptr_kinds = (&[]).iter().copied();
                        continue;
                    }

                    ptr_kinds = struct_decision.field_data(&adt_def.did())[f.index()]
                        .iter()
                        .copied();
                }
                rustc_middle::mir::ProjectionElem::Index(index) => {
                    replacement = replacement + "[" + INDEX_SEPARATOR + "]";
                    self.rewrite_index_at(index, location, rewriter);
                    let index_span = self.get_temporary_def_span(index, location);
                    index_spans.push(index_span);
                    ty = ty.builtin_index().unwrap();
                }
                _ => unreachable!(),
            }
        }

        rewrite_place(tcx, span, replacement, &index_spans, rewriter)
    }

    /// rewrite place load at a location
    /// note that place base may be a deref copy temporary
    fn rewrite_place_load_at<const PLACE_LOAD_MODE: u8>(
        &self,
        place: Place<'tcx>,
        location: Location,
        span: Span,
        required: PlaceCtxt,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            local_decision,
            struct_decision,
            body,
            user_idents,
            tcx,
            def_use_chain,
            ..
        } = *self;

        // TODO resolve place only when local is deref copy
        let (resolved_place, resolved_location) =
            accum_deref_copies(place, location, def_use_chain, body, tcx);

        let is_static_ref = body.local_decls[place.local].is_ref_to_static();

        let mut replacement = if let Some(replacement) = user_idents
            .get(&resolved_place.local)
            .map(|symbol| symbol.to_string())
        {
            replacement
        } else if is_static_ref {
            let &LocalInfo::StaticRef { def_id, is_thread_local } = body.local_decls[place.local].local_info.as_deref().unwrap() else { unreachable!() };
            assert!(!is_thread_local, "thread local is not supported");
            let static_name = tcx.def_path_str(def_id);
            if static_name.starts_with(&tcx.def_path_str(body.source.def_id())) {
                static_name.split("::").last().unwrap().to_owned()
            } else {
                format!("crate::{}", static_name)
            }
        } else if resolved_place.as_local().is_none() {
            // FIXME usage!
            self.rewrite_temporary(
                resolved_place.local,
                resolved_location,
                PlaceCtxt::Irrelavent,
                rewriter,
            );
            if resolved_location == location && place.is_indirect() && place.projection.len() == 1 {
                assert_eq!(place, resolved_place);
                let def_span = self.get_temporary_def_span(place.local, location);
                if PLACE_LOAD_MODE == PlaceLoadMode::ByAddr as u8 {
                    let required_ptr_kind = required.expect_ptr()[0];
                    if required_ptr_kind.is_mut() {
                        rewriter.replace(tcx, span.until(def_span), "Some(&mut *".to_owned());
                        rewriter.replace(tcx, def_span.shrink_to_hi(), ")".to_owned());
                    } else {
                        rewriter.replace(
                            tcx,
                            span.until(def_span),
                            "core::ptr::addr_of_mut!(*".to_owned(),
                        );
                        rewriter.replace(tcx, def_span.shrink_to_hi(), ")".to_owned());
                    }
                }
            }
            return;
        } else {
            // deref copies must be dereferenced!
            assert_eq!(resolved_location, location);
            assert_eq!(place, resolved_place);
            self.rewrite_temporary(resolved_place.local, location, required, rewriter);
            return;
        };

        let place = resolved_place;

        let mut index_spans: SmallVec<[Span; 1]> = smallvec::smallvec![];

        let produced = self.acquire_place_info(&place);

        let mut ptr_kinds = local_decision[place.local.as_usize()].iter().copied();
        let mut ty = body.local_decls[place.local].ty;
        let mut need_paren = false;

        let mut projection = place.projection.iter();
        if is_static_ref {
            let _ = projection.next();
            let _ = ptr_kinds.next();
            ty = ty.builtin_deref(true).unwrap().ty;
        }

        // Invariant: [`replacement`] is a nullable pointer or a struct
        for proj in projection {
            if need_paren {
                replacement = format!("({replacement})");
                need_paren = false;
            }

            // perform projection
            match proj {
                rustc_middle::mir::ProjectionElem::Deref => {
                    let base_ptr_kind = ptr_kinds.next().unwrap();

                    if base_ptr_kind.is_raw() {
                        replacement = format!("*{replacement}");
                    } else {
                        let usage = if PLACE_LOAD_MODE == PlaceLoadMode::ByValue as u8
                            && (required.is_copy_obj(self) && !produced.is_rustc_move_obj(self)
                                || produced.is_copy_obj(self))
                        {
                            if base_ptr_kind.is_const() {
                                "clone"
                            } else {
                                "as_deref"
                            }
                        } else {
                            "as_deref_mut"
                        };
                        replacement = format!("*{replacement}.{usage}().unwrap()");
                    }

                    need_paren = true;
                    ty = ty.builtin_deref(true).unwrap().ty;
                }
                rustc_middle::mir::ProjectionElem::Field(f, field_ty) => {
                    assert!(ptr_kinds.next().is_none());

                    let Some(adt_def) = ty.ty_adt_def() else {
                        // this happens in checked add. no rewrite for this case
                        return;
                    };
                    let field_name = &adt_def.variants()[0usize.into()].fields[f.index()]
                        .name
                        .as_str();
                    replacement = replacement + "." + field_name;
                    ty = field_ty;

                    if adt_def.is_union() {
                        // FIXME
                        ptr_kinds = (&[]).iter().copied();
                        continue;
                    }

                    ptr_kinds = struct_decision.field_data(&adt_def.did())[f.index()]
                        .iter()
                        .copied();
                }
                rustc_middle::mir::ProjectionElem::Index(index) => {
                    replacement = replacement + "[" + INDEX_SEPARATOR + "]";
                    self.rewrite_index_at(index, location, rewriter);
                    let index_span = self.get_temporary_def_span(index, location);
                    index_spans.push(index_span);
                    ty = ty.builtin_index().unwrap();
                }
                _ => unreachable!(),
            }
        }

        if need_paren {
            replacement = format!("({replacement})");
        }

        if PLACE_LOAD_MODE == PlaceLoadMode::ByRef as u8 {
            let source_text = common::rewrite::get_snippet(tcx, span).text.1;
            if source_text.contains("as_mut_ptr()") {
                replacement += ".as_mut_ptr()";
            } else if source_text.contains("as_va_list") {
                replacement += ".as_va_list()";
            } else {
                panic!("unexpected &mut @ {:?}", span)
            }
            rewrite_place(tcx, span, replacement, &index_spans, rewriter);
            return;
        } else if PLACE_LOAD_MODE == PlaceLoadMode::ByAddr as u8 {
            let required_ptr_kind = required.expect_ptr()[0];
            if required_ptr_kind.is_mut() {
                replacement = format!("Some(&mut {replacement})");
            } else if required_ptr_kind.is_const() {
                replacement = format!("Some(& {replacement})");
            } else if required_ptr_kind.is_raw_const() {
                replacement = format!("core::ptr::addr_of!({replacement})");
            } else {
                replacement = format!("core::ptr::addr_of_mut!({replacement})");
            }

            rewrite_place(tcx, span, replacement, &index_spans, rewriter);
            return;
        }

        assert_eq!(PLACE_LOAD_MODE, PlaceLoadMode::ByValue as u8);

        rewrite_place(tcx, span, replacement, &index_spans, rewriter);

        self.adapt_usage(span, ty, place.is_indirect(), produced, required, rewriter)
    }

    fn get_temporary_def_span(&self, local: Local, location: Location) -> Span {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            user_idents,
            ..
        } = *self;
        assert!(!user_idents.contains_key(&local));
        let def_loc = def_use_chain.def_loc(local, location);

        let RichLocation::Mir(def_loc) = def_loc else { unreachable!() };
        match body.stmt_at(def_loc) {
            Left(stmt) => stmt.source_info.span,
            Right(term) => term.source_info.span,
        }
    }

    fn rewrite_temporary(
        &self,
        local: Local,
        location: Location,
        required: PlaceCtxt,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            user_idents,
            ..
        } = *self;
        assert!(!user_idents.contains_key(&local));
        let def_loc = def_use_chain.def_loc(local, location);

        match def_loc {
            RichLocation::Entry => todo!(),
            RichLocation::Phi(block) => {
                // FIXME correctness? recursive?
                for def_loc in def_use_chain.phi_def_locs(local, block) {
                    let RichLocation::Mir(def_loc) = def_loc else { todo!() };
                    let Left(stmt) = body.stmt_at(def_loc) else { continue };
                    let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
                    self.rewrite_rvalue_at(
                        rvalue,
                        def_loc,
                        stmt.source_info.span,
                        required,
                        rewriter,
                    );
                }
                return;
            }
            RichLocation::Mir(def_loc) => {
                let Left(stmt) = body.stmt_at(def_loc) else { return };
                let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else {
                    if let StatementKind::Deinit(..) = &stmt.kind {
                        // happens only when S { f: T { g: .. } }
                        return;
                    } else {
                        unreachable!()
                    }
                };
                self.rewrite_rvalue_at(rvalue, def_loc, stmt.source_info.span, required, rewriter);
                return;
            }
        }
    }

    fn rewrite_index_at(&self, index: Local, location: Location, rewriter: &mut impl Rewrite) {
        self.rewrite_temporary(index, location, PlaceCtxt::Irrelavent, rewriter)
    }

    fn rewrite_operand_at(
        &self,
        operand: &Operand<'tcx>,
        location: Location,
        span: Span,
        required: PlaceCtxt,
        rewriter: &mut impl Rewrite,
    ) {
        match operand {
            Operand::Copy(place) | Operand::Move(place) => self
                .rewrite_place_load_at::<{ PlaceLoadMode::ByValue as u8 }>(
                    *place, location, span, required, rewriter,
                ),
            Operand::Constant(constant) => {
                self.rewrite_constant(constant, span, required, rewriter);
            }
        }
    }

    fn rewrite_rvalue_at(
        &self,
        rvalue: &Rvalue<'tcx>,
        location: Location,
        span: Span,
        required: PlaceCtxt,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt { tcx, .. } = *self;

        match rvalue {
            Rvalue::Use(operand) => {
                self.rewrite_operand_at(operand, location, span, required, rewriter)
            }
            Rvalue::BinaryOp(_, box (operand1, operand2))
            | Rvalue::CheckedBinaryOp(_, box (operand1, operand2)) => {
                let source_text = tcx.sess.source_map().span_to_snippet(span).unwrap();

                let source_token_stream = proc_macro2::TokenStream::from_str(&source_text).unwrap();
                let parsed_expr =
                    syn::parse2::<syn::Expr>(source_token_stream).expect(&source_text);

                if let syn::Expr::AssignOp(assign) = parsed_expr {
                    // special case

                    let assign_op_str = format!("{}", assign.op.to_token_stream());
                    let assign_op_pos = source_text.find(&assign_op_str).unwrap();
                    let operand1_span =
                        span.with_hi(span.lo() + rustc_span::BytePos(assign_op_pos as u32));
                    let operand2_span = span.with_lo(
                        span.lo()
                            + rustc_span::BytePos(assign_op_pos as u32)
                            + rustc_span::BytePos(assign_op_str.len() as u32),
                    );
                    self.rewrite_place_store(
                        operand1.place().unwrap(),
                        location,
                        operand1_span,
                        rewriter,
                    );
                    self.rewrite_operand_at(
                        operand2,
                        location,
                        operand2_span,
                        PlaceCtxt::Irrelavent,
                        rewriter,
                    );
                    return;
                }

                self.rewrite_operand_at(
                    operand1,
                    location,
                    span,
                    PlaceCtxt::Irrelavent,
                    rewriter,
                );
                self.rewrite_operand_at(
                    operand2,
                    location,
                    span,
                    PlaceCtxt::Irrelavent,
                    rewriter,
                );
            }
            Rvalue::UnaryOp(_, operand) => self.rewrite_operand_at(
                operand,
                location,
                span,
                PlaceCtxt::Irrelavent,
                rewriter,
            ),
            Rvalue::CopyForDeref(_) => unreachable!("{:?}", span),
            Rvalue::Cast(_, operand, ty) => {
                match self.try_rewrite_alloc_from_dest(
                    operand,
                    location,
                    required.is_rustc_move_obj(self),
                    rewriter,
                ) {
                    Ok(result) => {
                        if let Some(malloc_span) = result {
                            let replacement = {
                                let ty = ty.builtin_deref(true).unwrap().ty;
                                format!("Some(Box::new(<crate::{ty} as Default>::default()))")
                            };
                            rewriter.replace(tcx, malloc_span, replacement);
                            rewriter.erase(tcx, malloc_span.between(span.shrink_to_hi()));
                        }
                    }
                    Err(()) => {
                        self.rewrite_operand_at(operand, location, span, required, rewriter);
                    }
                }
            }
            Rvalue::AddressOf(rustc_mutability, place) => {
                if matches!(rustc_mutability, rustc_ast::Mutability::Mut) {
                    self.rewrite_place_load_at::<{ PlaceLoadMode::ByAddr as u8 }>(
                        *place, location, span, required, rewriter,
                    );
                } else {
                    tracing::warn!("const addr is ignored")
                }
            }
            Rvalue::Ref(_, borrow_kind, place) => {
                let rustc_mutability = borrow_kind.to_mutbl_lossy();
                if matches!(rustc_mutability, rustc_ast::Mutability::Mut) {
                    self.rewrite_place_load_at::<{ PlaceLoadMode::ByRef as u8 }>(
                        *place, location, span, required, rewriter,
                    );
                } else {
                    tracing::warn!("const reference is ignored")
                }
            }
            _ => tracing::error!("{:?} is not supported", rvalue), // _ => todo!("{:?} is not supported", rvalue),
        }
    }

    fn rewrite_call(
        &self,
        func: &Operand<'tcx>,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        fn_span: Span,
        location: Location,
        fn_decision: &FnLocals,
        rewriter: &mut impl Rewrite,
    ) {
        if let Some(func) = func.constant() {
            let ty = func.ty();
            let &FnDef(callee, _) = ty.kind() else { unreachable!() };
            if let Some(local_did) = callee.as_local() {
                match self.tcx.hir().find_by_def_id(local_did).unwrap() {
                    // this crate
                    rustc_hir::Node::Item(_) => {
                        self.rewrite_boundary(
                            callee,
                            args,
                            destination,
                            fn_span,
                            location,
                            fn_decision,
                            rewriter,
                        );
                    }
                    // extern
                    rustc_hir::Node::ForeignItem(foreign_item) => {
                        self.rewrite_libc_call(
                            foreign_item,
                            args,
                            destination,
                            fn_span,
                            location,
                            rewriter,
                        );
                    }
                    // in libxml2.rust/src/xmlschemastypes.rs/{} impl_xmlSchemaValDate/set_mon
                    rustc_hir::Node::ImplItem(_) => { /* TODO */ }
                    _ => unreachable!(),
                }
            } else {
                // library
                self.rewrite_library_call(callee, args, destination, fn_span, location, rewriter);
            }
        } else {
            // closure or fn ptr
            /* TODO */
            for arg in args {
                let Some(arg) = arg.place() else { continue };
                let arg = arg.as_local().unwrap();
                let ty = self.body.local_decls[arg].ty;
                let ctxt = if ty.is_unsafe_ptr() {
                    PlaceCtxt::from_ptr_ctxt(
                        ty,
                        if ty.is_mutable_ptr() {
                            &[PointerKind::Raw(RawMeta::Mut)]
                        } else {
                            &[PointerKind::Raw(RawMeta::Const)]
                        },
                    )
                } else if ty.is_region_ptr() {
                    PlaceCtxt::from_ptr_ctxt(
                        ty,
                        if ty.is_mutable_ptr() {
                            &[PointerKind::Mut]
                        } else {
                            &[PointerKind::Const]
                        },
                    )
                } else {
                    PlaceCtxt::Irrelavent
                };
                self.rewrite_temporary(arg, location, ctxt, rewriter);
            }
        }
    }

    fn rewrite_call_default(
        &self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        _destination: Place<'tcx>,
        fn_span: Span,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt { body, .. } = *self;

        println!(
            "rewrite call {} @ {:?} by default",
            self.tcx.def_path_str(callee),
            fn_span
        );

        for arg in args {
            if let Some(place) = arg.place() {
                let Some(local) = place.as_local() else { panic!() };

                let ty = body.local_decls[local].ty;

                let ctxt = if ty.is_unsafe_ptr() {
                    PlaceCtxt::from_ptr_ctxt(
                        ty,
                        if ty.is_mutable_ptr() {
                            &[PointerKind::Raw(RawMeta::Mut)]
                        } else {
                            &[PointerKind::Raw(RawMeta::Const)]
                        },
                    )
                } else if ty.is_region_ptr() {
                    PlaceCtxt::from_ptr_ctxt(
                        ty,
                        if ty.is_mutable_ptr() {
                            &[PointerKind::Mut]
                        } else {
                            &[PointerKind::Const]
                        },
                    )
                } else {
                    PlaceCtxt::Irrelavent
                };

                self.rewrite_temporary(local, location, ctxt, rewriter);
            }
        }
    }

    fn rewrite_constant(
        &self,
        constant: &Constant<'tcx>,
        stmt_span: Span,
        required: PlaceCtxt,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt { tcx, .. } = *self;

        if let Some(scalar) = constant.literal.try_to_scalar() {
            match scalar {
                rustc_const_eval::interpret::Scalar::Int(scalar) => {
                    if scalar.is_null() {
                        if let PlaceCtxt::Ptr(ptr_kinds) = required {
                            let ptr_kind = ptr_kinds[0];
                            let span = constant.span.until(stmt_span.shrink_to_hi());
                            if ptr_kind.is_safe() {
                                rewriter.replace(tcx, span, "None".to_owned());
                            }
                        }
                    }
                }
                rustc_const_eval::interpret::Scalar::Ptr(_, _) => {}
            }
        }
    }
}

fn rewrite_place(
    tcx: TyCtxt,
    span: Span,
    replacement: String,
    index_spans: &[Span],
    rewriter: &mut impl Rewrite,
) {
    if replacement.find(INDEX_SEPARATOR).is_none() {
        rewriter.replace(tcx, span, replacement)
    } else {
        let mut rest = span;
        let replacements = replacement.split(INDEX_SEPARATOR);
        let mut index_spans = index_spans.iter().copied();
        for replacement in replacements {
            match index_spans.next() {
                Some(index_span) => {
                    let part = rest.until(index_span);
                    rewriter.replace(tcx, part, replacement.to_owned());
                    rest = index_span.between(rest.shrink_to_hi());
                }
                None => {
                    rewriter.replace(tcx, rest, replacement.to_owned());
                }
            }
        }
    }
}

/// Although Chinese is a popular language, I believe the following word does not appear too
/// frequently in legacy C code
const INDEX_SEPARATOR: &str = "索引";
