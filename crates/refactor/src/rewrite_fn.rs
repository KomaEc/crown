mod boundary;
mod libc_call;
mod library_call;

use analysis::{
    ssa::consume::RichLocation,
    use_def::{def_use_chain, DefUseChain},
};
use common::rewrite::Rewrite;
use either::Either::Left;
use rustc_hash::FxHashMap;
use rustc_hir::{def_id::DefId, ItemKind};
use rustc_middle::{
    mir::{
        Body, Constant, Local, LocalInfo, Location, NonDivergingIntrinsic, Operand, Place, Rvalue,
        Statement, StatementKind, Terminator, TerminatorKind, VarDebugInfoContents, RETURN_PLACE,
    },
    ty::{TyCtxt, TyKind, Ty},
};
use rustc_span::{Span, Symbol};
use rustc_type_ir::TyKind::FnDef;
use smallvec::SmallVec;

use crate::{rewrite_ty::rewrite_hir_ty, FnLocals, PointerKind, StructFields, RawMeta};

pub fn rewrite_fns(
    fns: &[DefId],
    fn_decision: &FnLocals,
    struct_decision: &StructFields,
    rewriter: &mut impl Rewrite,
    tcx: TyCtxt,
    type_only: bool,
) {
    for &did in fns {
        let local_data = fn_decision.local_data(&did);
        let body = tcx.optimized_mir(did);
        rewrite_fn_sig(body, local_data, rewriter, tcx);
        if !type_only {
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

    // show_def_use_chain(body, &def_use_chain);

    let rewrite_ctxt = FnRewriteCtxt {
        local_decision,
        fn_decision,
        struct_decision,
        body,
        def_use_chain: &def_use_chain,
        user_idents: &user_idents,
        tcx,
    };

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
    location: Location,
    def_use_chain: &DefUseChain,
    body: &Body<'tcx>,
    tcx: TyCtxt<'tcx>,
) -> Place<'tcx> {
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
        place = rplace.project_deeper(place.projection, tcx)
    }

    place
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
#[derive(Clone, Copy)]
enum ValueType<'me> {
    Ptr(&'me [PointerKind]),
    Struct(DefId),
    Irrelavent
}

impl<'me> ValueType<'me> {
    fn from_ptr_ctxt(ty: Ty, ctxt: &'me [PointerKind]) -> Self {
        if ctxt.is_empty() {
            if let TyKind::Adt(adt_def, _) = ty.kind() {
                if adt_def.is_struct() {
                    return ValueType::Struct(adt_def.did())
                }
            }
            ValueType::Irrelavent
        } else {
            ValueType::Ptr(ctxt)
        }
    }

    fn is_ptr(self) -> bool {
        matches!(self, Self::Ptr(..))
    }

    fn is_move_obj<'tcx>(&self, rewrite_ctxt: &FnRewriteCtxt<'tcx, 'me>) -> bool {
        match self {
            ValueType::Ptr(ptr_kinds) => {
                matches!(ptr_kinds.first(), Some(ptr_kind) if ptr_kind.is_move() || ptr_kind.is_raw_move())
            },
            ValueType::Struct(did) => {
                let fields_data = rewrite_ctxt.struct_decision.field_data(did);
                fields_data.iter().any(|field| {
                    field
                        .iter()
                        .any(|ptr_kind| ptr_kind.is_move() || ptr_kind.is_raw_move())
                })
            },
            ValueType::Irrelavent => false,
        }
    }

    fn is_copy_obj<'tcx>(&self, rewrite_ctxt: &FnRewriteCtxt<'tcx, 'me>) -> bool {
        match self {
            ValueType::Ptr(ptr_kinds) => {
                matches!(ptr_kinds.first(), Some(ptr_kind) if ptr_kind.is_copy())
            },
            ValueType::Struct(_) => !self.is_move_obj(rewrite_ctxt),
            ValueType::Irrelavent => true,
        }
    }

    // fn is_mut_borrow<'tcx>(&self, rewrite_ctxt: &FnRewriteCtxt<'tcx, 'me>) -> bool {
    //     !self.is_copy_obj(rewrite_ctxt) && !self.is_move_obj(rewrite_ctxt)
    // }

    fn is_raw_ptr(self) -> bool {
        matches!(self, ValueType::Ptr(ptr_kinds) if matches!(ptr_kinds.first(), Some(ptr_kind) if ptr_kind.is_raw()))
    }
}

impl<'tcx, 'me> FnRewriteCtxt<'tcx, 'me> {
    fn acquire_place_info(&self, place: &Place<'tcx>) -> ValueType<'me> {
        let FnRewriteCtxt {
            local_decision,
            struct_decision,
            body,
            def_use_chain: _,
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
                    ptr_kinds = &struct_decision.field_data(&adt_def.did())[f.index()][..];
                    ptr_kinds_index = 0;
                    ty = field_ty;
                }
                rustc_middle::mir::ProjectionElem::Index(_) => todo!(),
                _ => unreachable!(),
            }
        }

        let ptr = &ptr_kinds[ptr_kinds_index..];

        ValueType::from_ptr_ctxt(ty, ptr)
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
                // rewrite point: non-temporary place
                // this includes 1. place of which base local is a user defined variable
                // 2. place of which base local is a deref tmp, and the rvalue is not another deref tmp
                if user_idents.contains_key(&place.local)
                    || matches!(body.local_decls[place.local].local_info, Some(box LocalInfo::DerefTemp) if !matches!(rvalue, Rvalue::CopyForDeref(..)))
                {
                    let place = accum_deref_copies(*place, location, def_use_chain, body, tcx);
                    let span = statement.source_info.span;
                    let source_text = tcx.sess.source_map().span_to_snippet(span).unwrap();

                    // println!("rewrite {:?} @ {:?}, {:?}", place, location, span);

                    if let Some(assign_pos) = source_text
                        .find("+=")
                        .or(source_text.find("-="))
                        .or(source_text.find("="))
                    {
                        // lhs needs to be rewritten

                        assert!(assign_pos > 0);

                        let lhs_span =
                            span.with_hi(span.lo() + rustc_span::BytePos(assign_pos as u32));

                        self.rewrite_place_store(place, lhs_span, rewriter);
                    } // otherwise let-binding

                    let ctxt = self.acquire_place_info(&place);

                    self.rewrite_rvalue_at(rvalue, location, span, ctxt, rewriter);
                }
            }
            StatementKind::Intrinsic(box NonDivergingIntrinsic::Assume(_)) => {
                // rewrite point: assume
                rewriter.replace(tcx, statement.source_info.span, "()".to_owned())
            }
            _ => todo!(),
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
            body,
            def_use_chain,
            user_idents,
            ..
        } = *self;

        match &terminator.kind {
            TerminatorKind::SwitchInt { discr, .. } => {
                // rewrite point: if expr
                let place = discr.place().unwrap();
                self.rewrite_place_load_at(
                    place,
                    location,
                    terminator.source_info.span,
                    ValueType::Irrelavent,
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
                    let def_loc = def_use_chain.def_loc(RETURN_PLACE, location);
                    let return_ctxt = ValueType::from_ptr_ctxt(self.body.return_ty(), &self.local_decision[0]);
                    match def_loc {
                        RichLocation::Entry => {}
                        RichLocation::Phi(block) => {
                            for def_loc in def_use_chain.phi_def_locs(RETURN_PLACE, block) {
                                let RichLocation::Mir(def_loc) = def_loc else { todo!() };
                                let Left(stmt) = body.stmt_at(def_loc) else { return };
                                let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
                                self.rewrite_rvalue_at(
                                    rvalue,
                                    def_loc,
                                    stmt.source_info.span,
                                    return_ctxt,
                                    rewriter,
                                );
                            }
                        }
                        RichLocation::Mir(def_loc) => {
                            let Left(stmt) = body.stmt_at(def_loc) else { return };
                            let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
                            self.rewrite_rvalue_at(
                                rvalue,
                                def_loc,
                                stmt.source_info.span,
                                return_ctxt,
                                rewriter,
                            );
                        }
                    }
                }
            }
            TerminatorKind::Goto { .. } => {}
            TerminatorKind::Assert { .. } => {}
            _ => todo!(),
        }
    }

    fn rewrite_place_store(&self, place: Place<'tcx>, span: Span, rewriter: &mut impl Rewrite) {
        let FnRewriteCtxt {
            local_decision,
            struct_decision,
            body,
            def_use_chain: _,
            user_idents,
            tcx,
            ..
        } = *self;

        let mut replacement = user_idents
            .get(&place.local)
            .map(|symbol| symbol.to_string())
            .unwrap_or_else(|| {
                assert!(place.as_local().is_none());
                unimplemented!("rewrite immediate value, could be static, func call return")
            });

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

        // &ptr_kinds[ptr_kinds_index..]
    }

    fn rewrite_place_load_at(
        &self,
        place: Place<'tcx>,
        location: Location,
        span: Span,
        required: ValueType,
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

        let mut replacement = if let Some(replacement) = user_idents
            .get(&place.local)
            .map(|symbol| symbol.to_string())
        {
            replacement
        } else if place.as_local().is_none() {
            unimplemented!("rewrite immediate value, could be static, func call return")
        } else {
            assert!(place.as_local().is_some());
            let def_loc = def_use_chain.def_loc(place.local, location);

            match def_loc {
                RichLocation::Entry => todo!(),
                RichLocation::Phi(block) => {
                    // FIXME correctness? recursive?
                    for def_loc in def_use_chain.phi_def_locs(place.local, block) {
                        let RichLocation::Mir(def_loc) = def_loc else { todo!() };
                        let Left(stmt) = body.stmt_at(def_loc) else { return };
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
                    let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
                    self.rewrite_rvalue_at(
                        rvalue,
                        def_loc,
                        stmt.source_info.span,
                        required,
                        rewriter,
                    );
                    return;
                }
            }
        };

        let produced = self.acquire_place_info(&place);

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

                    if base_ptr_kind.is_raw() {
                        replacement = format!("*{replacement}");
                    } else {
                        let usage = if required.is_copy_obj(self) || produced.is_copy_obj(self) {
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

                    ptr_kinds = struct_decision.field_data(&adt_def.did())[f.index()]
                        .iter()
                        .copied();
                }
                rustc_middle::mir::ProjectionElem::Index(_) => todo!(),
                _ => unreachable!(),
            }
        }

        // incomplete usage rewrites
        if required.is_ptr() {
            let ptr_kind = ptr_kinds.next().unwrap();

            if ptr_kind.is_safe() {
                if need_paren {
                    replacement = format!("({replacement})");
                }

                if required.is_move_obj(self) {
                    if place.is_indirect() {
                        replacement += ".take()";
                    }
                } else if required.is_copy_obj(self) {
                    if ptr_kind.is_const() {
                        replacement += ".clone()";
                    } else {
                        replacement += ".as_deref()";
                    }
                } else {
                    replacement += ".as_deref_mut()"
                };
                if required.is_raw_ptr() {
                    replacement = format!("core::mem::transmute({replacement})")
                }
            } else if !required.is_raw_ptr(){
                if required.is_move_obj(self) {
                    replacement = format!("Some(Box::from_raw({replacement}))")
                } else if required.is_copy_obj(self) {
                    if need_paren {
                        replacement = format!("({replacement})");
                    }
                    replacement = format!("{replacement}.as_ref()");
                } else {
                    if need_paren {
                        replacement = format!("({replacement})");
                    }
                    replacement = format!("{replacement}.as_mut()");
                }
            }
        }

        rewriter.replace(tcx, span, replacement);
    }

    fn rewrite_operand_at(
        &self,
        operand: &Operand<'tcx>,
        location: Location,
        span: Span,
        required: ValueType,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            tcx,
            ..
        } = *self;

        match operand {
            Operand::Copy(place) | Operand::Move(place) => {
                let place = accum_deref_copies(*place, location, def_use_chain, body, tcx);
                self.rewrite_place_load_at(place, location, span, required, rewriter)
            }
            Operand::Constant(constant) => {
                self.try_rewrite_null_constant(constant, span, required, rewriter);
            }
        }
    }

    fn rewrite_rvalue_at(
        &self,
        rvalue: &Rvalue<'tcx>,
        location: Location,
        span: Span,
        required: ValueType,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt { tcx, .. } = *self;

        match rvalue {
            Rvalue::Use(operand) => {
                self.rewrite_operand_at(operand, location, span, required, rewriter)
            }
            Rvalue::AddressOf(_, _) => todo!(),
            Rvalue::BinaryOp(_, box (operand1, operand2))
            | Rvalue::CheckedBinaryOp(_, box (operand1, operand2)) => {
                self.rewrite_operand_at(operand1, location, span, ValueType::Irrelavent, rewriter);
                self.rewrite_operand_at(operand2, location, span, ValueType::Irrelavent, rewriter);
            }
            Rvalue::UnaryOp(_, operand) => {
                self.rewrite_operand_at(operand, location, span, ValueType::Irrelavent, rewriter)
            }
            Rvalue::CopyForDeref(_) => unreachable!(),
            Rvalue::Cast(_, operand, ty) => {
                match self.try_rewrite_malloc_from_dest(
                    operand,
                    location,
                    required.is_move_obj(self),
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
                        // if let Some(constant) = operand.constant() {
                        //     self.try_rewrite_null_constant(constant, span, load_ctxt, rewriter);
                        // }
                        self.rewrite_operand_at(operand, location, span, required, rewriter);
                    }
                }
            }
            _ => todo!("{:?} is not supported", rvalue),
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
        }
    }

    fn rewrite_call_default(
        &self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        _destination: Place<'tcx>,
        _fn_span: Span,
        location: Location,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt {
            body,
            def_use_chain,
            tcx,
            ..
        } = *self;

        println!("rewrite call {} by default", self.tcx.def_path_str(callee));

        let fn_sig = tcx.fn_sig(callee).skip_binder();
        for (arg, ty) in itertools::izip!(args, fn_sig.inputs()) {
            let ctxt = if ty.is_unsafe_ptr() {
                ValueType::from_ptr_ctxt(*ty, if ty.is_mutable_ptr() {
                    &[PointerKind::Raw(RawMeta::Mut)]
                } else {
                    &[PointerKind::Raw(RawMeta::Const)]
                })
            } else {
                ValueType::Irrelavent
            };
            if let Some(place) = arg.place() {
                let Some(local) = place.as_local() else { panic!() };
                let def_loc = def_use_chain.def_loc(local, location);
                let RichLocation::Mir(def_loc) = def_loc else { panic!() };

                let Left(stmt) = body.stmt_at(def_loc) else {
                    // terminator is rewritten
                    return
                };
                let StatementKind::Assign(box (_, rvalue)) = &stmt.kind else { panic!() };
                self.rewrite_rvalue_at(rvalue, def_loc, stmt.source_info.span, ctxt, rewriter);
            }
        }
    }

    fn try_rewrite_null_constant(
        &self,
        constant: &Constant<'tcx>,
        stmt_span: Span,
        required: ValueType,
        rewriter: &mut impl Rewrite,
    ) {
        let FnRewriteCtxt { tcx, .. } = *self;

        if let Some(scalar) = constant.literal.try_to_scalar_int() {
            if scalar.is_null() {
                if let ValueType::Ptr(ptr_kinds) = required {
                    let ptr_kind = ptr_kinds[0];
                    let span = constant.span.until(stmt_span.shrink_to_hi());
                    if ptr_kind.is_safe() {
                        rewriter.replace(tcx, span, "None".to_owned());
                    }
                }
            }
        }
    }
}
