use etrace::some_or;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::{BodyOwnerKind, HirId, LangItem, def_id::LocalDefId};
use rustc_middle::{
    mir::{
        AggregateKind, AssignOp, BasicBlock, BinOp, Body, CastKind, Local, Location, Operand,
        Place, Rvalue, Statement, StatementKind, SwitchTargets, Terminator, TerminatorKind, UnOp,
    },
    thir::{
        self, AdtExpr, BlockId, ExprId, ExprKind, LogicalOp, Pat, PatKind, StmtKind, Thir,
        visit::Visitor as TVisitor,
    },
    ty::{Ty, TyCtxt, TyKind},
};
use rustc_span::{BytePos, Span};
use smallvec::{SmallVec, smallvec};

/// `Thir` of THIR to `Body` of MIR
#[derive(Debug, Default)]
pub struct ThirToMir {
    /// THIR binding's `HirId` to the corresponding MIR `Local`
    pub binding_to_local: FxHashMap<HirId, Local>,

    /// Thir expression to relavant locations
    ///
    /// For `Binary`, `Unary`, `Cast`, `PointerCoercion`, `Borrow`, `RawBorrow`, `Repeat`, `Array`,
    /// `Tuple`, and `Adt`, the location of the single statement evaluating the expression. The lhs
    /// stores the result.
    ///
    /// For `Deref`, `Field`, `Index`, `VarRef`, and `UpvarRef`, the location of the single
    /// statement evaluating the expression. Such a statement may not exist if the expression is an
    /// lvalue or a part of a longer place expression.
    ///
    /// For `LogicalOp`, the location of the single statement storing true or false as the result
    /// *and* the locations of possibly multiple terminators performing the jump from other
    /// branches.
    ///
    /// For `Assign` and `AssignOp`, the location of the single statement performing the
    /// assignment.
    ///
    /// For `Return`, the locations of *possibly multiple* statements that assign the return value
    /// to _0. When no return value is specified, the statement is _0 = (). When the return value
    /// is an If, there are multiple statements, one for each branch.
    ///
    /// For `Break`, the locations of possibly *multiple* statements, just like for `Return`.
    /// Currently, only the case without a value is supported.
    pub expr_to_locs: FxHashMap<ExprId, Locations>,

    /// THIR `Block` to MIR basic blocks
    pub block_to_bbs: FxHashMap<BlockId, FxHashSet<BasicBlock>>,

    /// THIR `Loop` expression to MIR basic blocks
    pub loop_to_bbs: FxHashMap<ExprId, FxHashSet<BasicBlock>>,

    /// THIR `If` expression to MIR basic blocks
    pub if_to_bbs: FxHashMap<ExprId, IfBlocks>,
}

pub type Locations = SmallVec<[Location; 1]>;

/// MIR basic blocks related to THIR If expression
#[derive(Debug)]
pub struct IfBlocks {
    /// The block reached first when the condition is true.
    pub true_entry: BasicBlock,
    /// The block reached first when the condition is false.
    pub false_entry: BasicBlock,
    /// All the blocks for the true branch.
    pub true_blocks: FxHashSet<BasicBlock>,
    /// All the blocks for the false branch. Empty when there is no else branch.
    pub false_blocks: FxHashSet<BasicBlock>,
}

pub fn map_thir_to_mir(def_id: LocalDefId, verbose: bool, tcx: TyCtxt<'_>) -> ThirToMir {
    let is_const_or_static = matches!(
        tcx.hir_body_owner_kind(def_id),
        BodyOwnerKind::Const { .. } | BodyOwnerKind::Static(_)
    );

    let (thir, texpr) = tcx.thir_body(def_id).unwrap();
    let thir = thir.borrow();
    let texpr = &thir[texpr];

    let mut visitor = BindingVisitor {
        tcx,
        thir: &thir,
        bindings: FxHashMap::default(),
    };
    for param in &thir.params {
        if let Some(pat) = &param.pat {
            visitor.visit_pat(pat);
        }
    }
    visitor.visit_expr(texpr);

    let body = tcx.mir_drops_elaborated_and_const_checked(def_id);
    let body = body.borrow();

    let mut var_to_locals: FxHashMap<_, Vec<_>> = FxHashMap::default();
    for (local, local_decl) in body.local_decls.iter_enumerated() {
        let span = local_decl.source_info.span;
        if let Some((hir_id, ty)) = visitor.bindings.get(&span)
            && *ty == local_decl.ty
        {
            var_to_locals.entry(*hir_id).or_default().push(local);
        }
    }

    let mut binding_to_local = FxHashMap::default();
    for (hir_id, _) in visitor.bindings.values() {
        let locals = var_to_locals.get(hir_id);
        if let Some(locals) = locals
            && let [local, ..] = &locals[..]
        {
            binding_to_local.insert(*hir_id, *local);
        } else if verbose {
            println!(
                "{hir_id:?} {} {}",
                tcx.def_path_str(hir_id.owner.def_id),
                tcx.hir_name(*hir_id)
            );
        }
    }

    let mut stmt_span_to_locs: FxHashMap<LoHi, Locations> = FxHashMap::default();
    let mut term_span_to_locs: FxHashMap<LoHi, Locations> = FxHashMap::default();

    for bb in body.basic_blocks.reverse_postorder() {
        let bbd = &body.basic_blocks[*bb];
        for (i, stmt) in bbd.statements.iter().enumerate() {
            if !matches!(stmt.kind, StatementKind::Assign { .. }) {
                continue;
            }
            let location = Location {
                block: *bb,
                statement_index: i,
            };
            stmt_span_to_locs
                .entry(stmt.source_info.span.into())
                .or_default()
                .push(location);
        }
        let term = bbd.terminator();
        let location = Location {
            block: *bb,
            statement_index: bbd.statements.len(),
        };
        term_span_to_locs
            .entry(term.source_info.span.into())
            .or_default()
            .push(location);
    }

    let mut ctx = Ctx {
        tcx,
        thir: &thir,
        body: &body,
        verbose,

        stmt_span_to_locs,
        term_span_to_locs,

        rhs_to_assigns: FxHashMap::default(),
        nested_logical_exprs: FxHashSet::default(),
        promoted_ptrs: FxHashMap::default(),

        thir_to_mir: ThirToMir::default(),
    };
    ctx.thir_to_mir.binding_to_local = binding_to_local;

    for (expr_id, expr) in thir.exprs.iter_enumerated() {
        match expr.kind {
            ExprKind::Borrow { arg, .. } => {
                let arg = unwrap_expr(arg, &thir);
                match &thir[arg].kind {
                    // check for &[]
                    ExprKind::Array { fields } if is_const_or_static || fields.is_empty() => {
                        ctx.promoted_ptrs.insert(arg, expr_id);
                    }
                    // check for &Some(f)
                    ExprKind::Adt(box AdtExpr {
                        adt_def,
                        fields: box [field],
                        ..
                    }) if tcx.is_lang_item(adt_def.did(), LangItem::Option) => {
                        let coercion = unwrap_expr(field.expr, &thir);
                        let coercion_expr = &thir[coercion];
                        if let ExprKind::PointerCoercion { .. } = coercion_expr.kind
                            && coercion_expr.ty.is_fn_ptr()
                        {
                            ctx.promoted_ptrs.insert(arg, expr_id);
                            ctx.promoted_ptrs.insert(coercion, expr_id);
                        }
                    }
                    _ => {}
                }
            }
            ExprKind::Assign { lhs, rhs } => {
                if let TyKind::Adt(adt_def, _) = thir[rhs].ty.kind()
                    && super::def_id_to_symbol(adt_def.did(), tcx)
                        .unwrap()
                        .as_str()
                        == "VaListImpl"
                {
                } else if let Some(assign) =
                    ctx.find_assign_location(thir[lhs].ty, expr.span.into(), false)
                {
                    ctx.thir_to_mir
                        .expr_to_locs
                        .insert(expr_id, smallvec![assign.loc]);
                    ctx.rhs_to_assigns.insert(unwrap_expr(rhs, &thir), assign);
                } else {
                    ctx.print_debug("Assign", expr.span.into());
                }
            }
            ExprKind::If {
                cond,
                then,
                else_opt,
                ..
            } => {
                let conds = cond_dest(cond, &thir, &mut ctx.nested_logical_exprs);
                let mut true_targets = vec![];
                let mut false_targets = vec![];
                for cond in &conds {
                    if let Some(targets) = ctx.find_switch_int(cond.expr_id) {
                        let when_true = targets.otherwise();
                        let when_false = targets.target_for_value(0);
                        match cond.when_true {
                            None => {}
                            Some(true) => true_targets.push(when_true),
                            Some(false) => false_targets.push(when_true),
                        }
                        match cond.when_false {
                            None => {}
                            Some(true) => true_targets.push(when_false),
                            Some(false) => false_targets.push(when_false),
                        }
                    } else {
                        ctx.print_debug("If", thir[cond.expr_id].span.into());
                    }
                }
                let find_target = |targets: &[BasicBlock]| match targets {
                    [] => panic!(),
                    [bb] => *bb,
                    _ => {
                        let mut targets: Vec<_> = targets
                            .iter()
                            .map(|bb| {
                                let term = body.basic_blocks[*bb].terminator();
                                if let TerminatorKind::Goto { target } = term.kind {
                                    vec![*bb, target]
                                } else {
                                    vec![*bb]
                                }
                            })
                            .collect();
                        let mut target = targets.pop().unwrap();
                        for target1 in targets {
                            target.retain(|bb| target1.contains(bb));
                        }
                        target[0]
                    }
                };
                let true_target = find_target(&true_targets);
                let false_target = find_target(&false_targets);
                assert_ne!(true_target, false_target);
                let if_blocks = IfBlocks {
                    true_entry: true_target,
                    false_entry: false_target,
                    true_blocks: FxHashSet::default(),
                    false_blocks: FxHashSet::default(),
                };
                ctx.thir_to_mir.if_to_bbs.insert(expr_id, if_blocks);

                if let Some(continue_id) = unique_continue_of_block(then, &thir) {
                    let bb = &body.basic_blocks[true_target];
                    let term = bb.terminator();
                    assert!(matches!(term.kind, TerminatorKind::Goto { .. }));
                    let location = Location {
                        block: true_target,
                        statement_index: bb.statements.len(),
                    };
                    ctx.thir_to_mir
                        .expr_to_locs
                        .insert(continue_id, smallvec![location]);
                }
                if let Some(els) = else_opt
                    && let Some(continue_id) = unique_continue_of_block(els, &thir)
                {
                    let bb = &body.basic_blocks[false_target];
                    let term = bb.terminator();
                    assert!(matches!(term.kind, TerminatorKind::Goto { .. }));
                    let location = Location {
                        block: false_target,
                        statement_index: bb.statements.len(),
                    };
                    ctx.thir_to_mir
                        .expr_to_locs
                        .insert(continue_id, smallvec![location]);
                }
            }
            ExprKind::LogicalOp { lhs, rhs, .. } => {
                cond_dest(lhs, &thir, &mut ctx.nested_logical_exprs);
                cond_dest(rhs, &thir, &mut ctx.nested_logical_exprs);
            }
            _ => {}
        }
    }

    for (expr_id, expr) in thir.exprs.iter_enumerated() {
        match expr.kind {
            ExprKind::Deref { .. }
            | ExprKind::Field { .. }
            | ExprKind::Index { .. }
            | ExprKind::VarRef { .. }
            | ExprKind::UpvarRef { .. } => {
                let _ = ctx.handle_rvalue_opt(expr_id, |rvalue| matches!(rvalue, Rvalue::Use(_)));
            }
            ExprKind::Literal { .. } | ExprKind::NamedConst { .. } => {
                let _ = ctx.handle_rvalue_opt(expr_id, |rvalue| {
                    matches!(rvalue, Rvalue::Use(Operand::Constant(_)))
                });
            }
            ExprKind::Block { .. } => {
                let _ = ctx.handle_rvalue_opt(expr_id, |_| true);
            }

            ExprKind::Cast { .. } => {
                ctx.handle_rvalue("Cast", expr_id, |rvalue| matches!(rvalue, Rvalue::Cast(..)));
            }
            ExprKind::PointerCoercion { .. } => {
                if !ctx.promoted_ptrs.contains_key(&expr_id) {
                    ctx.handle_rvalue("PointerCoercion", expr_id, |rvalue| {
                        matches!(
                            rvalue,
                            Rvalue::Cast(CastKind::PointerCoercion(..) | CastKind::PtrToPtr, _, _)
                        )
                    });
                }
            }
            ExprKind::Borrow { .. } => {
                ctx.handle_rvalue("Borrow", expr_id, |rvalue| {
                    matches!(rvalue, Rvalue::Ref(..))
                });
            }
            ExprKind::RawBorrow { .. } => {
                ctx.handle_rvalue("RawBorrow", expr_id, |rvalue| {
                    matches!(rvalue, Rvalue::RawPtr(..))
                });
            }
            ExprKind::Repeat { count, .. } => {
                ctx.handle_rvalue("Repeat", expr_id, |rvalue| {
                    if Some(0) == count.try_to_target_usize(tcx) {
                        matches!(rvalue, Rvalue::Aggregate(box AggregateKind::Array(..), _))
                    } else {
                        matches!(rvalue, Rvalue::Repeat(..))
                    }
                });
            }
            ExprKind::Array { .. } => {
                if !ctx.promoted_ptrs.contains_key(&expr_id) {
                    ctx.handle_rvalue("Array", expr_id, |rvalue| {
                        matches!(rvalue, Rvalue::Aggregate(box AggregateKind::Array(..), _))
                    });
                }
            }
            ExprKind::Tuple { .. } => {
                ctx.handle_rvalue("Tuple", expr_id, |rvalue| {
                    matches!(rvalue, Rvalue::Aggregate(box AggregateKind::Tuple, _))
                });
            }
            ExprKind::Adt(_) => {
                if !ctx.promoted_ptrs.contains_key(&expr_id) {
                    ctx.handle_rvalue("Adt", expr_id, |rvalue| {
                        matches!(rvalue, Rvalue::Aggregate(box AggregateKind::Adt(..), _))
                    });
                }
            }
            ExprKind::Binary { op, .. } => {
                ctx.handle_rvalue("Binary", expr_id, |rvalue| {
                    if is_const_or_static
                        && matches!(op, BinOp::Add | BinOp::Sub | BinOp::Mul)
                        && expr.ty.is_integral()
                    {
                        matches!(rvalue, Rvalue::Use(Operand::Move(_)))
                    } else if let Rvalue::BinaryOp(mop, _) = rvalue {
                        *mop == op
                    } else {
                        false
                    }
                });
            }
            ExprKind::Unary { op, .. } => match op {
                UnOp::Neg => {
                    ctx.handle_rvalue("Unary Neg", expr_id, |rvalue| {
                        matches!(rvalue, Rvalue::UnaryOp(UnOp::Neg, _))
                    });
                }
                UnOp::Not => {
                    if !ctx.nested_logical_exprs.contains(&expr_id) {
                        ctx.handle_rvalue("Unary Not", expr_id, |rvalue| {
                            matches!(rvalue, Rvalue::UnaryOp(UnOp::Not, _))
                        });
                    }
                }
                UnOp::PtrMetadata => panic!(),
            },

            ExprKind::LogicalOp { .. } => {
                if !ctx.nested_logical_exprs.contains(&expr_id) {
                    if let Some(stmt) = ctx.find_stmt_location(expr_id, true, |_, rhs| {
                        matches!(rhs, Rvalue::Use(Operand::Constant(_)))
                    }) && let Some(terms) = ctx.find_term_locations(expr_id, |term| {
                        matches!(term, TerminatorKind::Goto { .. })
                    }) && let (term, terms) = terms
                        .into_iter()
                        .partition::<Locations, _>(|l| l.block == stmt.block)
                        && let [term] = &term[..]
                        && stmt.statement_index + 1 == term.statement_index
                        && !terms.is_empty()
                    {
                        let mut locs = SmallVec::with_capacity(1 + terms.len());
                        locs.push(stmt);
                        locs.extend(terms);
                        ctx.thir_to_mir.expr_to_locs.insert(expr_id, locs);
                    } else {
                        ctx.print_debug("LogicalOp", expr.span.into());
                    }
                }
            }

            ExprKind::Assign { .. } => {}
            ExprKind::AssignOp { op, lhs, .. } => {
                if let Some(assign) = ctx.find_assign_location(
                    thir[lhs].ty,
                    expr.span.into(),
                    matches!(op, AssignOp::DivAssign | AssignOp::RemAssign),
                ) {
                    ctx.thir_to_mir
                        .expr_to_locs
                        .insert(expr_id, smallvec![assign.loc]);
                } else {
                    ctx.print_debug("AssignOp", expr.span.into());
                }
            }

            ExprKind::Call { .. } => {
                if let Some(loc) = ctx
                    .find_call_location(expr_id)
                    .or_else(|| ctx.find_transmute_location(expr_id))
                {
                    ctx.thir_to_mir.expr_to_locs.insert(expr_id, smallvec![loc]);
                } else {
                    ctx.print_debug("Call", expr.span.into());
                }
            }

            ExprKind::Break { value, .. } => {
                // TODO: handle break with value
                assert!(value.is_none());
                if let Some(loc) = ctx.find_assign_ty_location(expr_id, |ty| ty.is_unit()) {
                    ctx.thir_to_mir.expr_to_locs.insert(expr_id, smallvec![loc]);
                } else {
                    ctx.print_debug("Break", expr.span.into());
                }
            }
            ExprKind::Continue { .. } => {}
            ExprKind::Return { value } => {
                let locs = if let Some(value) = value {
                    let mut values = smallvec![];
                    find_return_values(value, &thir, &mut values);
                    let mut locs = smallvec![];
                    for v in &values {
                        let loc = if matches!(thir[*v].kind, ExprKind::Call { .. }) {
                            ctx.find_call_location(*v)
                                .or_else(|| ctx.find_transmute_location(*v))
                        } else {
                            ctx.find_rvalue_location(*v, |_| true)
                        };
                        locs.push(some_or!(loc, break));
                    }
                    if locs.len() == values.len() {
                        Some(locs)
                    } else {
                        None
                    }
                } else {
                    ctx.find_assign_ty_location(expr_id, |ty| ty.is_unit())
                        .map(|loc| smallvec![loc])
                };
                if let Some(locs) = locs {
                    ctx.thir_to_mir.expr_to_locs.insert(expr_id, locs);
                } else {
                    ctx.print_debug("Return", expr.span.into());
                }
            }

            ExprKind::Loop { .. } => {}
            ExprKind::If { .. } => {}

            ExprKind::Let { .. } => {}   // TODO
            ExprKind::Match { .. } => {} // TODO

            ExprKind::ConstBlock { .. } => {}
            ExprKind::ValueTypeAscription { .. } => {}
            ExprKind::NonHirLiteral { .. } => {}
            ExprKind::ZstLiteral { .. } => {}
            ExprKind::ConstParam { .. } => {}
            ExprKind::StaticRef { .. } => {}
            ExprKind::ThreadLocalRef(_) => {}
            ExprKind::InlineAsm(_) => {}
            ExprKind::Use { .. } => {}
            ExprKind::NeverToAny { .. } => {}
            ExprKind::Scope { .. } => {}

            ExprKind::Box { .. } => panic!(),
            ExprKind::ByUse { .. } => panic!(),
            ExprKind::Become { .. } => panic!(),
            ExprKind::PlaceTypeAscription { .. } => panic!(),
            ExprKind::PlaceUnwrapUnsafeBinder { .. } => panic!(),
            ExprKind::ValueUnwrapUnsafeBinder { .. } => panic!(),
            ExprKind::WrapUnsafeBinder { .. } => panic!(),
            ExprKind::Closure(_) => panic!(),
            ExprKind::OffsetOf { .. } => panic!(),
            ExprKind::Yield { .. } => panic!(),
        }
    }

    for (expr_id, expr) in thir.exprs.iter_enumerated() {
        match expr.kind {
            ExprKind::Adt(..) | ExprKind::PointerCoercion { .. } | ExprKind::Array { .. } => {
                if let Some(arg) = ctx.promoted_ptrs.get(&expr_id) {
                    if let Some(locs) = ctx.thir_to_mir.expr_to_locs.get(arg) {
                        ctx.thir_to_mir.expr_to_locs.insert(expr_id, locs.clone());
                    } else {
                        ctx.print_debug("Promoted", expr.span.into());
                    }
                }
            }
            ExprKind::Block { block } => {
                let mut visitor = ExprVisitor {
                    tcx,
                    thir: &thir,
                    exprs: FxHashSet::default(),
                };
                visitor.visit_expr(expr);

                let mut bbs = ctx.collect_basic_blocks(visitor.exprs.into_iter());
                if let Some(locs) = ctx.thir_to_mir.expr_to_locs.get(&expr_id) {
                    bbs.extend(locs.iter().map(|loc| loc.block));
                }
                if !bbs.is_empty() {
                    ctx.thir_to_mir.block_to_bbs.insert(block, bbs);
                } else if let block = &thir.blocks[block]
                    && block.expr.is_none()
                    && let [stmt] = &block.stmts[..]
                    && let stmt = &thir[*stmt]
                    && let StmtKind::Expr {
                        expr: stmt_expr, ..
                    } = stmt.kind
                    && let stmt_expr = unwrap_expr(stmt_expr, &thir)
                    && let stmt_expr = &thir[stmt_expr]
                    && matches!(stmt_expr.kind, ExprKind::Continue { .. })
                {
                    // we give up when the block is `{ continue; }`
                } else {
                    ctx.print_debug("Block", expr.span.into());
                }
            }
            ExprKind::Loop { body } => {
                let body = unwrap_expr(body, &thir);
                let ExprKind::Block { block } = thir[body].kind else {
                    panic!()
                };
                if let Some(bbs) = ctx.thir_to_mir.block_to_bbs.get(&block) {
                    ctx.thir_to_mir.loop_to_bbs.insert(expr_id, bbs.clone());
                } else {
                    ctx.print_debug("Loop", expr.span.into());
                }
            }
            ExprKind::If { then, else_opt, .. } => {
                let then = unwrap_expr(then, &thir);
                let bbs = if let ExprKind::Block { block } = thir[then].kind
                    && let Some(bbs) = ctx.thir_to_mir.block_to_bbs.get(&block)
                {
                    bbs.clone()
                } else {
                    let mut visitor = ExprVisitor {
                        tcx,
                        thir: &thir,
                        exprs: FxHashSet::default(),
                    };
                    visitor.visit_expr(&thir[then]);
                    ctx.collect_basic_blocks(visitor.exprs.into_iter())
                };
                if !bbs.is_empty() {
                    let if_blocks = ctx.thir_to_mir.if_to_bbs.get_mut(&expr_id).unwrap();
                    if_blocks.true_blocks = bbs.clone();
                    if_blocks.true_blocks.insert(if_blocks.true_entry);
                } else {
                    ctx.print_debug("If then", thir[then].span.into());
                }

                if let Some(els) = else_opt {
                    let els = unwrap_expr(els, &thir);
                    let bbs = if let ExprKind::Block { block } = thir[els].kind
                        && let Some(bbs) = ctx.thir_to_mir.block_to_bbs.get(&block)
                    {
                        bbs.clone()
                    } else {
                        let mut visitor = ExprVisitor {
                            tcx,
                            thir: &thir,
                            exprs: FxHashSet::default(),
                        };
                        visitor.visit_expr(&thir[els]);
                        ctx.collect_basic_blocks(visitor.exprs.into_iter())
                    };
                    if !bbs.is_empty() {
                        let if_blocks = ctx.thir_to_mir.if_to_bbs.get_mut(&expr_id).unwrap();
                        if_blocks.false_blocks = bbs;
                        if_blocks.false_blocks.insert(if_blocks.false_entry);
                        debug_assert_eq!(
                            if_blocks
                                .true_blocks
                                .intersection(&if_blocks.false_blocks)
                                .count(),
                            0
                        );
                    } else {
                        ctx.print_debug("If else", thir[els].span.into());
                    }
                }
            }
            ExprKind::Match { .. } => {} // TODO
            _ => {}
        }
    }
    ctx.thir_to_mir
}

fn unwrap_expr(mut expr_id: ExprId, thir: &Thir<'_>) -> ExprId {
    loop {
        let expr = &thir[expr_id];
        match expr.kind {
            ExprKind::Scope { value, .. }
            | ExprKind::Use { source: value }
            | ExprKind::NeverToAny { source: value } => {
                expr_id = value;
            }
            _ => {
                return expr_id;
            }
        }
    }
}

fn unique_continue_of_block(expr_id: ExprId, thir: &Thir<'_>) -> Option<ExprId> {
    let expr_id = unwrap_expr(expr_id, thir);
    let ExprKind::Block { block } = thir[expr_id].kind else {
        return None;
    };
    let block = &thir.blocks[block];
    if block.expr.is_some() {
        return None;
    }
    let [stmt] = &block.stmts[..] else {
        return None;
    };
    let stmt = &thir[*stmt];
    let StmtKind::Expr { expr: expr_id, .. } = stmt.kind else {
        return None;
    };
    let expr_id = unwrap_expr(expr_id, thir);
    let expr = &thir[expr_id];
    let ExprKind::Continue { .. } = expr.kind else {
        return None;
    };
    Some(expr_id)
}

fn find_return_values(expr: ExprId, thir: &Thir<'_>, values: &mut SmallVec<[ExprId; 1]>) {
    let expr = unwrap_expr(expr, thir);
    match thir[expr].kind {
        ExprKind::If { then, else_opt, .. } => {
            find_return_values(then, thir, values);
            find_return_values(else_opt.unwrap(), thir, values);
        }
        ExprKind::Block { block } => {
            let block = &thir.blocks[block];
            // `block.expr` can be `None` when a statement in the block diverges.
            if let Some(expr) = block.expr {
                find_return_values(expr, thir, values);
            }
        }
        _ => {
            values.push(expr);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct CondWithDest {
    expr_id: ExprId,
    when_true: Option<bool>,
    when_false: Option<bool>,
}

fn cond_dest(
    expr_id: ExprId,
    thir: &Thir<'_>,
    logical_exprs: &mut FxHashSet<ExprId>,
) -> Vec<CondWithDest> {
    let expr_id = unwrap_expr(expr_id, thir);
    match thir[expr_id].kind {
        ExprKind::LogicalOp { op, lhs, rhs } => {
            logical_exprs.insert(expr_id);
            let mut l = cond_dest(lhs, thir, logical_exprs);
            let r = cond_dest(rhs, thir, logical_exprs);
            match op {
                LogicalOp::And => {
                    for c in &mut l {
                        c.when_true = c.when_true.filter(|b| !*b);
                        c.when_false = c.when_false.filter(|b| !*b);
                    }
                }
                LogicalOp::Or => {
                    for c in &mut l {
                        c.when_true = c.when_true.filter(|b| *b);
                        c.when_false = c.when_false.filter(|b| *b);
                    }
                }
            }
            l.extend(r);
            l
        }
        ExprKind::Unary { op: UnOp::Not, arg } => {
            logical_exprs.insert(expr_id);
            let mut v = cond_dest(arg, thir, logical_exprs);
            for c in &mut v {
                c.when_true = c.when_true.map(|b| !b);
                c.when_false = c.when_false.map(|b| !b);
            }
            v
        }
        _ => {
            vec![CondWithDest {
                expr_id,
                when_true: Some(true),
                when_false: Some(false),
            }]
        }
    }
}

/// MIR assignment statement
struct Assign<'a, 'tcx> {
    ty: Ty<'tcx>,
    rvalue: &'a Rvalue<'tcx>,
    loc: Location,
    span: LoHi,
}

struct Ctx<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    thir: &'a Thir<'tcx>,
    body: &'a Body<'tcx>,
    verbose: bool,
    stmt_span_to_locs: FxHashMap<LoHi, Locations>,
    term_span_to_locs: FxHashMap<LoHi, Locations>,

    /// Rhs expression to the corresponding MIR assignment statement.
    rhs_to_assigns: FxHashMap<ExprId, Assign<'a, 'tcx>>,

    /// Logical expressions (And, Or, Neg) used in the condition of an If or as an lhs or an rhs of
    /// another And or Or. The result of such an expression is not stored to a Local.
    nested_logical_exprs: FxHashSet<ExprId>,

    /// Expressions lowered to promoted pointers.
    ///
    /// Where `f` is a function, for `&Some(f)`, there is a statement for `&Some(f)` but not for
    /// `Some(f)` and `f`. Therefore, we keep the mappings from `Some(f)` to `&Some(f)` and from
    /// `f` to `&Some(f)`.
    ///
    /// For `&[]`, there is a statement for `&[]` but not for `[]`. Therefore, we keep the mapping
    /// from `[]` to `&[]`.
    promoted_ptrs: FxHashMap<ExprId, ExprId>,

    thir_to_mir: ThirToMir,
}

impl<'a, 'tcx> Ctx<'a, 'tcx> {
    /// When `Some` is returned, it is guaranteed to be non-empty.
    #[inline]
    fn find_locations<P: Fn(Location) -> bool>(
        &self,
        expr_id: ExprId,
        span_to_locs: &FxHashMap<LoHi, Locations>,
        pred: P,
    ) -> Option<Locations> {
        let expr = &self.thir[expr_id];
        let locs = span_to_locs.get(&expr.span.into())?;
        let locs: Locations = locs.iter().copied().filter(|loc| pred(*loc)).collect();
        if locs.is_empty() { None } else { Some(locs) }
    }

    #[inline]
    fn find_stmt_locations<P: Fn(&Place<'tcx>, &Rvalue<'tcx>) -> bool>(
        &self,
        expr_id: ExprId,
        pred: P,
    ) -> Option<Locations> {
        self.find_locations(expr_id, &self.stmt_span_to_locs, |loc| {
            let stmt = self.get_statement(loc);
            let StatementKind::Assign(box (lhs, rhs)) = &stmt.kind else {
                panic!()
            };
            pred(lhs, rhs)
        })
    }

    #[inline]
    fn find_stmt_location<P: Fn(&Place<'tcx>, &Rvalue<'tcx>) -> bool>(
        &self,
        expr_id: ExprId,
        unique: bool,
        pred: P,
    ) -> Option<Location> {
        let locs = self.find_stmt_locations(expr_id, pred)?;
        if unique && locs.len() != 1 {
            None
        } else {
            Some(locs[0])
        }
    }

    #[inline]
    fn find_term_locations<P: Fn(&TerminatorKind<'tcx>) -> bool>(
        &self,
        expr_id: ExprId,
        pred: P,
    ) -> Option<Locations> {
        self.find_locations(expr_id, &self.term_span_to_locs, |loc| {
            let term = self.get_terminator(loc);
            pred(&term.kind)
        })
    }

    #[inline]
    fn find_term_location<P: Fn(&TerminatorKind<'tcx>) -> bool>(
        &self,
        expr_id: ExprId,
        unique: bool,
        pred: P,
    ) -> Option<Location> {
        let locs = self.find_term_locations(expr_id, pred)?;
        if unique && locs.len() != 1 {
            None
        } else {
            Some(locs[0])
        }
    }

    fn find_switch_int(&self, expr_id: ExprId) -> Option<&'a SwitchTargets> {
        let loc = self.find_term_location(expr_id, true, |k| {
            matches!(k, TerminatorKind::SwitchInt { .. })
        })?;
        let term = self.get_terminator(loc);
        let TerminatorKind::SwitchInt { targets, .. } = &term.kind else {
            panic!()
        };
        Some(targets)
    }

    fn find_assign_ty_location<P: Fn(Ty<'tcx>) -> bool>(
        &self,
        expr_id: ExprId,
        pred: P,
    ) -> Option<Location> {
        self.find_stmt_location(expr_id, false, |lhs, _| {
            pred(lhs.ty(&self.body.local_decls, self.tcx).ty)
        })
    }

    fn find_call_location(&self, expr_id: ExprId) -> Option<Location> {
        self.find_term_location(expr_id, true, |k| {
            let TerminatorKind::Call { destination, .. } = k else {
                return false;
            };
            destination.ty(&self.body.local_decls, self.tcx).ty == self.thir[expr_id].ty
        })
    }

    fn find_transmute_location(&self, expr_id: ExprId) -> Option<Location> {
        self.find_stmt_location(expr_id, true, |lhs, rhs| {
            matches!(rhs, Rvalue::Cast(CastKind::Transmute, _, _))
                && lhs.ty(&self.body.local_decls, self.tcx).ty == self.thir[expr_id].ty
        })
    }

    fn find_rvalue_location<P: Fn(&Rvalue<'tcx>) -> bool>(
        &self,
        expr_id: ExprId,
        pred: P,
    ) -> Option<Location> {
        self.find_stmt_location(expr_id, false, |lhs, rhs| {
            lhs.ty(&self.body.local_decls, self.tcx).ty == self.thir[expr_id].ty && pred(rhs)
        })
    }

    fn handle_rvalue_opt<P: Fn(&Rvalue<'tcx>) -> bool>(
        &mut self,
        expr_id: ExprId,
        pred: P,
    ) -> Result<(), LoHi> {
        let expr = &self.thir[expr_id];
        if let Some(loc) = self.find_rvalue_location(expr_id, &pred) {
            self.thir_to_mir
                .expr_to_locs
                .insert(expr_id, smallvec![loc]);
            Ok(())
        } else if let Some(assign) = self.rhs_to_assigns.get(&expr_id) {
            if expr.ty == assign.ty && pred(assign.rvalue) {
                self.thir_to_mir
                    .expr_to_locs
                    .insert(expr_id, smallvec![assign.loc]);
                Ok(())
            } else {
                Err(assign.span)
            }
        } else {
            Err(expr.span.into())
        }
    }

    fn handle_rvalue<P: Fn(&Rvalue<'tcx>) -> bool>(&mut self, msg: &str, expr_id: ExprId, pred: P) {
        if let Err(span) = self.handle_rvalue_opt(expr_id, pred) {
            self.print_debug(msg, span);
        }
    }

    fn find_assign_location(
        &self,
        ty: Ty<'tcx>,
        span: LoHi,
        allow_checks: bool,
    ) -> Option<Assign<'a, 'tcx>> {
        let locs = self.stmt_span_to_locs.get(&span)?;
        for (i, loc) in locs.iter().enumerate() {
            let stmt = self.get_statement(*loc);
            let StatementKind::Assign(box (lhs, rhs)) = &stmt.kind else {
                panic!()
            };
            if i < locs.len() - 1 {
                if !allow_checks && !matches!(rhs, Rvalue::CopyForDeref(_)) {
                    break;
                }
            } else {
                let lhs_ty = lhs.ty(&self.body.local_decls, self.tcx).ty;
                if lhs_ty == ty {
                    let assign = Assign {
                        ty,
                        rvalue: rhs,
                        loc: *loc,
                        span,
                    };
                    return Some(assign);
                }
            }
        }
        None
    }

    #[inline]
    fn get_statement(&self, loc: Location) -> &'a Statement<'tcx> {
        &self.body.basic_blocks[loc.block].statements[loc.statement_index]
    }

    #[inline]
    fn get_terminator(&self, loc: Location) -> &'a Terminator<'tcx> {
        let bbd = &self.body.basic_blocks[loc.block];
        assert_eq!(loc.statement_index, bbd.statements.len());
        bbd.terminator()
    }

    #[inline]
    fn collect_basic_blocks(&self, exprs: impl Iterator<Item = ExprId>) -> FxHashSet<BasicBlock> {
        let mut bbs = FxHashSet::default();
        for expr in exprs {
            let locs = some_or!(self.thir_to_mir.expr_to_locs.get(&expr), continue);
            for loc in locs {
                bbs.insert(loc.block);
            }
        }
        bbs
    }

    fn write_debug<W: std::io::Write>(&self, mut w: W, msg: &str, span: LoHi) {
        writeln!(w, "{msg} {span:?}").unwrap();
        if let Some(locs) = self.stmt_span_to_locs.get(&span) {
            for loc in locs {
                let stmt = self.get_statement(*loc);
                writeln!(w, "  {loc:?}: {stmt:?}").unwrap();
            }
        }
        if let Some(locs) = self.term_span_to_locs.get(&span) {
            for loc in locs {
                let term = self.get_terminator(*loc);
                writeln!(w, "  {loc:?}: {:?}", term.kind).unwrap();
            }
        }
    }

    #[inline]
    fn print_debug(&self, msg: &str, span: LoHi) {
        if self.verbose {
            self.write_debug(std::io::stdout(), msg, span);
        }
    }

    #[allow(unused)]
    #[inline]
    fn mk_debug_str(&self, msg: &str, span: LoHi) -> String {
        let mut v = vec![];
        self.write_debug(&mut v, msg, span);
        String::from_utf8(v).unwrap()
    }
}

struct BindingVisitor<'a, 'tcx> {
    #[allow(unused)]
    tcx: TyCtxt<'tcx>,
    thir: &'a Thir<'tcx>,
    bindings: FxHashMap<Span, (HirId, Ty<'tcx>)>,
}

impl<'a, 'tcx> TVisitor<'a, 'tcx> for BindingVisitor<'a, 'tcx> {
    fn thir(&self) -> &'a Thir<'tcx> {
        self.thir
    }

    fn visit_pat(&mut self, pat: &'a Pat<'tcx>) {
        if let PatKind::Binding { var, ty, .. } = pat.kind {
            let old = self.bindings.insert(pat.span, (var.0, ty));
            if let Some((old, _)) = old {
                let old = self.tcx.hir_name(old);
                let var = self.tcx.hir_name(var.0);
                println!("{old} {var}");
            }
        }
        thir::visit::walk_pat(self, pat);
    }
}

struct ExprVisitor<'a, 'tcx> {
    #[allow(unused)]
    tcx: TyCtxt<'tcx>,
    thir: &'a Thir<'tcx>,
    exprs: FxHashSet<ExprId>,
}

impl<'a, 'tcx> TVisitor<'a, 'tcx> for ExprVisitor<'a, 'tcx> {
    fn thir(&self) -> &'a Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, expr: &'a thir::Expr<'tcx>) {
        if let ExprKind::Scope { value, .. }
        | ExprKind::Use { source: value }
        | ExprKind::NeverToAny { source: value } = expr.kind
        {
            self.exprs.insert(value);
        }
        thir::visit::walk_expr(self, expr);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LoHi {
    lo: BytePos,
    hi: BytePos,
}

impl From<Span> for LoHi {
    #[inline]
    fn from(span: Span) -> Self {
        Self {
            lo: span.lo(),
            hi: span.hi(),
        }
    }
}

impl From<LoHi> for Span {
    #[inline]
    fn from(lohi: LoHi) -> Self {
        Self::with_root_ctxt(lohi.lo, lohi.hi)
    }
}

impl std::fmt::Debug for LoHi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span: Span = (*self).into();
        span.fmt(f)
    }
}
