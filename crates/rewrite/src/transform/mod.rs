use analyses::type_qualifier::foster::mutability;
use itertools::izip;
use rustc_ast::{
    HasNodeId, ItemKind,
    ast::*,
    mut_visit::{self, MutVisitor},
    ptr::P,
};
use rustc_ast_pretty::pprust;
use rustc_hash::FxHashMap;
use rustc_hir::{
    Expr as HirExpr, ExprKind as HirExprKind, HirId, Node as HirNode, PatKind as HirPatKind, QPath,
    StmtKind as HirStmtKind, def::Res, def_id::DefId,
};
use rustc_middle::ty::{Ty as MirTy, TyKind as MirTyKind};
use rustc_span::symbol::Ident;
use smallvec::SmallVec;
use utils::{ir_util::map_thir_to_mir, rustc::RustProgram};

use crate::{
    Analysis,
    collect::collect_diffs,
    decision::{PtrKind, PtrKindDiff, SigDecisions},
};
use thin_vec::thin_vec;
use utils::ir_util::IrMappings;

pub mod post;

pub(crate) struct TransformVisitor<'tcx, 'a> {
    rust_program: &'a RustProgram<'tcx>,
    sig_decs: SigDecisions,
    ptr_diffs: FxHashMap<HirId, PtrKindDiff>,
    ir_mappings: IrMappings<'a>,
    pub updated: bool,
}

impl MutVisitor for TransformVisitor<'_, '_> {
    fn visit_item(&mut self, item: &mut Item) {
        let node_id = item.node_id();
        let ItemKind::Fn(box fn_item) = &mut item.kind else {
            return mut_visit::walk_item(self, item);
        };
        let hir_item = self
            .ir_mappings
            .ast_to_hir
            .get_item(node_id, self.rust_program.tcx)
            .unwrap_or_else(|| panic!("Failed to find HIR item for Item {:?}", item.span));
        let local_def_id = hir_item.owner_id.def_id;
        let def_id = local_def_id.to_def_id();
        let mir_body = self
            .rust_program
            .tcx
            .mir_drops_elaborated_and_const_checked(local_def_id)
            .borrow();

        // Rewrite function signature
        let local_input_decls = mir_body
            .args_iter()
            .map(|local| mir_body.local_decls[local].clone())
            .collect::<Vec<_>>();

        let input_len = self.sig_input_len(def_id); // exclude variadic arguments
        let sig_dec = self.sig_decs.expect(&def_id);

        // Currently intra-procedural borrow inference: skip return type, only consider parameters
        for (idx, (local_decl, param)) in
            izip!(local_input_decls, fn_item.sig.decl.inputs.iter_mut())
                .take(input_len)
                .enumerate()
        {
            let ty_res = mir_ty_to_ty(&local_decl.ty); // resolved type (no type alias)
            self.rewrite_ty(&mut param.ty, ty_res, &sig_dec.input_decs[idx]);
            if let PatKind::Ident(binding_mode, ..) = &mut param.pat.kind {
                *binding_mode = BindingMode::MUT;
            }
        }
        mut_visit::walk_item(self, item);
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Self::Result {
        let _expr = expr.clone();
        // visit children first; we can assume that the function arguments have been rewritten
        mut_visit::walk_expr(self, expr);
        let hir_expr_opt = self.get_hir_expr(expr);
        match &mut expr.kind {
            ExprKind::Assign(box lhs, box rhs, _) => {
                // assignment to dereferenced pointer
                if let HirExprKind::Assign(hir_lhs, _, _) = hir_expr_opt.unwrap().kind
                    && let HirExprKind::Unary(UnOp::Deref, hir_lhs_deref) = hir_lhs.kind
                    && let HirExprKind::Path(qpath) = &hir_lhs_deref.kind  // TODO: support multiple deref
                    && let QPath::Resolved(_, path) = qpath
                    && let Res::Local(local_id) = path.res
                    && let Some(ptr_diff) = self.ptr_diffs.get(&local_id)
                {
                    match ptr_diff {
                        PtrKindDiff {
                            before: PtrKind::Raw(_),
                            after: PtrKind::OptMutRef,
                        } => {
                            let ExprKind::Unary(UnOp::Deref, lhs_deref) = &mut lhs.kind else {
                                unreachable!("Expected deref expression on LHS: {:?}", expr.span);
                            };
                            **lhs_deref = self.append_as_deref_mut_raw((**lhs_deref).clone());
                        }
                        PtrKindDiff {
                            before: PtrKind::Raw(_),
                            after: PtrKind::Ref(_),
                        } => {
                            // *lhs = rhs;
                            // Nothing to rewrite
                        }
                        _ => (),
                    }
                }
                // direct assignment
                if let HirExprKind::Assign(lhs, _rhs, _) = hir_expr_opt.unwrap().kind
                    && let HirExprKind::Path(qpath) = &lhs.kind
                    && let QPath::Resolved(_, path) = qpath
                    && let Res::Local(local_id) = path.res
                    && let Some(ptr_diff) = self.ptr_diffs.get(&local_id)
                {
                    match ptr_diff {
                        PtrKindDiff {
                            before: PtrKind::Raw(_),
                            after: PtrKind::OptMutRef,
                        } => {
                            unreachable!(
                                "Output parameters cannot be introduced in direct assignments: {:?}",
                                expr.span
                            );
                        }
                        PtrKindDiff {
                            before: PtrKind::Raw(mutability),
                            after: PtrKind::Ref(_mutability),
                        } => {
                            // lhs = &(mut) *rhs;
                            assert!(mutability == _mutability);
                            *rhs = prepend_ref_deref(rhs.clone(), *mutability);
                        }
                        _ => (),
                    }
                }
            }
            ExprKind::Path(..) => {
                // usage site
                let hir_expr = hir_expr_opt.unwrap();
                if let HirExprKind::Path(qpath) = &hir_expr.kind
                    && let QPath::Resolved(_, path) = qpath
                    && let Res::Local(local_id) = path.res
                    && let Some(ptr_diff) = self.ptr_diffs.get(&local_id).cloned()
                {
                    let parent_node = self.expect_parent_node(hir_expr.hir_id);
                    if let HirNode::Expr(parent_expr) = parent_node {
                        match parent_expr.kind {
                            // HirExprKind::Call(..) => return, // handled in ExprKind::Call below
                            HirExprKind::Assign(lhs, _, _) => {
                                if lhs.hir_id == hir_expr.hir_id {
                                    // assignment to this variable, handled in ExprKind::Assign above
                                    return;
                                }
                            }
                            _ => {
                                let grandparent_node = self.expect_parent_node(parent_expr.hir_id);
                                if let HirNode::Expr(grandparent_expr) = grandparent_node
                                    && let HirExprKind::Assign(lhs, _, _) = grandparent_expr.kind
                                    && let HirExprKind::Unary(UnOp::Deref, lhs_deref) = &lhs.kind
                                {
                                    if lhs_deref.hir_id == hir_expr.hir_id {
                                        // assignment to dereferenced pointer, handled in ExprKind::Assign above
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    match ptr_diff {
                        PtrKindDiff {
                            before: PtrKind::Raw(_mutability),
                            after: PtrKind::OptMutRef,
                        } => {
                            *expr = self.append_as_deref_mut_raw(expr.clone());
                        }
                        PtrKindDiff {
                            before: PtrKind::Raw(mutability),
                            after: PtrKind::Ref(_mutability),
                        } => {
                            // expr -> &raw (mut) *expr
                            assert!(mutability == _mutability);
                            // TODO: if parent expr is unary deref, skip casting
                            *expr = self.cast_to_mut_raw(expr.clone(), mutability);
                        }
                        PtrKindDiff {
                            before: PtrKind::Ref(_),
                            after: PtrKind::Raw(_),
                        } => {
                            unreachable!("Raw pointers adapting to references: {:?}", expr.span);
                        }
                        _ => (),
                    }
                }
            }
            ExprKind::Call(box _func_expr, args) => {
                // function call
                let hir_expr = hir_expr_opt.unwrap();
                if let HirExprKind::Call(hir_func_expr, hir_args) = hir_expr.kind
                    && let HirExprKind::Path(func_qpath) = &hir_func_expr.kind
                    && let QPath::Resolved(_, func_path) = func_qpath
                    && let Res::Def(_, func_did) = func_path.res
                    && let Some(sig_dec) = self.sig_decs.get(&func_did)
                {
                    let input_len = self.sig_input_len(func_did); // exclude variadic arguments
                    for (i, (arg, _hir_arg)) in izip!(args.iter_mut(), hir_args.iter())
                        .take(input_len)
                        .enumerate()
                    {
                        // Note: the arguments have been visited and rewritten to *mut T
                        // Hir arguments stays the same, so may not match the AST arguments
                        match &sig_dec.input_decs.get(i).unwrap_or_else(|| {
                                panic!(
                                    "Function call argument index out of bounds: {} in {:?}, function: {:?}",
                                    i, expr.span, self.rust_program.tcx.def_path_str(func_did)
                                )
                            }) {
                                Some(PtrKind::OptMutRef) => {
                                    if let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, box inner) = &arg.kind {
                                        // Some(&mut arg)
                                        **arg = wrap_in_some_mut_ref((*inner).clone());
                                    } else {
                                        // arg.as_mut()
                                        **arg = append_as_mut(*arg.clone());
                                    }
                                }
                                Some(PtrKind::Ref(mutability)) => {
                                    if let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, box inner) = &arg.kind &&
                                    let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, _) = &inner.kind {
                                        // &raw mut &raw mut _ -> &mut (&raw mut _)
                                        **arg = prepend_ref((*inner).clone(), *mutability);
                                    } else {
                                        if *mutability {
                                            **arg = append_as_mut_unwrap(*arg.clone());
                                        } else {
                                            **arg = append_as_ref_unwrap(*arg.clone());
                                        }
                                    }
                                    // if let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, box inner) = &arg.kind {
                                    //     // arg is &raw mut arg_inner
                                    //     // &(mut) arg_inner
                                    //     **arg = prepend_ref((*inner).clone(), *mutability);
                                    // } else {
                                    //     // &(mut) *arg
                                    //     // **arg = prepend_ref_deref(*arg.clone(), *mutability);

                                    //     // arg.as_mut().unwrap()
                                    //     if *mutability {
                                    //         **arg = append_as_mut_unwrap(*arg.clone());
                                    //     } else {
                                    //         **arg = append_as_ref_unwrap(*arg.clone());
                                    //     }
                                    // }
                                }
                                _ => (),
                            }
                    }
                }
            }
            _ => (),
        }
    }

    fn flat_map_stmt(&mut self, mut stmt: Stmt) -> SmallVec<[Stmt; 1]> {
        let hir_stmt_opt = self.get_hir_stmt(&stmt);
        match &mut stmt.kind {
            StmtKind::Let(box local) => {
                if let HirStmtKind::Let(hir_let) = hir_stmt_opt.unwrap().kind
                    && let HirPatKind::Binding(_, binding_hir_id, _, _) = hir_let.pat.kind
                    && let Some(ptr_diff) = self.ptr_diffs.get(&binding_hir_id)
                {
                    match ptr_diff {
                        PtrKindDiff {
                            before: PtrKind::Raw(_),
                            after: PtrKind::OptMutRef,
                        } => {
                            unreachable!(
                                "Output parameters cannot be introduced in let statements: {:?}",
                                stmt.span
                            );
                        }
                        PtrKindDiff {
                            before: PtrKind::Raw(mutability),
                            after: PtrKind::Ref(_mutability),
                        } => {
                            assert!(mutability == _mutability);
                            let mutability = *mutability;
                            if let Some(ty) = &mut local.ty {
                                let hir_ty = hir_let.ty.unwrap();
                                let typeck = self.rust_program.tcx.typeck(hir_ty.hir_id.owner);
                                let hir_ty_res = typeck.node_type(hir_ty.hir_id);
                                let ty_res = mir_ty_to_ty(&hir_ty_res);
                                self.rewrite_ty(ty, ty_res, &Some(PtrKind::Ref(mutability)));
                            }
                            match &mut local.kind {
                                LocalKind::Init(box rhs) | LocalKind::InitElse(box rhs, _) => {
                                    *rhs = prepend_ref_deref(rhs.clone(), mutability);
                                }
                                LocalKind::Decl => {
                                    // No initializer, do nothing
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        mut_visit::walk_flat_map_stmt(self, stmt)
    }
}

impl<'tcx, 'a> TransformVisitor<'tcx, 'a> {
    pub fn new(
        rust_program: &'a RustProgram<'tcx>,
        analysis: &Analysis,
        ir_mappings: IrMappings<'a>,
    ) -> TransformVisitor<'tcx, 'a> {
        let sig_decs = SigDecisions::new(rust_program, analysis);
        let ptr_diffs = collect_diffs(rust_program, analysis);
        TransformVisitor {
            rust_program,
            sig_decs,
            ptr_diffs,
            ir_mappings,
            updated: false,
        }
    }

    fn expect_parent_node(&self, hir_id: HirId) -> HirNode<'tcx> {
        let (_, parent_node) = self
            .rust_program
            .tcx
            .hir_parent_iter(hir_id)
            .next()
            .unwrap();
        parent_node
    }

    fn is_function_arg(&self, hir_id: HirId) -> bool {
        if let Some(outer_expr) = self.get_outer_expr(hir_id)
            && let HirNode::Expr(parent_expr) = self.expect_parent_node(outer_expr.hir_id)
            && let HirExprKind::Call(_func, args) = parent_expr.kind
            && let Some(_) = args.iter().position(|arg| arg.hir_id == outer_expr.hir_id)
        {
            true
        } else {
            false
        }
    }

    // Get the outermost expression that contains casting and dereferencing
    fn get_outer_expr(&self, hir_id: HirId) -> Option<&HirExpr<'tcx>> {
        let mut out_expr = None;
        for (_, parent_node) in self.rust_program.tcx.hir_parent_iter(hir_id) {
            match parent_node {
                HirNode::Expr(parent_expr) => match parent_expr.kind {
                    HirExprKind::Cast(..)
                    | HirExprKind::AddrOf(..)
                    | HirExprKind::Unary(UnOp::Deref, _) => out_expr = Some(parent_expr),
                    _ => break,
                },
                _ => break,
            }
        }
        out_expr
    }

    fn sig_input_len(&self, def_id: DefId) -> usize {
        self.rust_program
            .tcx
            .fn_sig(def_id)
            .skip_binder()
            .inputs()
            .skip_binder()
            .len()
    }

    fn get_hir_expr(&self, expr: &Expr) -> Option<&rustc_hir::Expr<'tcx>> {
        self.ir_mappings
            .ast_to_hir
            .get_expr(expr.node_id(), self.rust_program.tcx)
    }

    fn get_hir_stmt(&self, stmt: &Stmt) -> Option<&rustc_hir::Stmt<'tcx>> {
        self.ir_mappings
            .ast_to_hir
            .get_stmt(stmt.node_id(), self.rust_program.tcx)
    }

    fn expr_ty(&self, expr: &Expr) -> Ty {
        let hir_expr = self
            .get_hir_expr(expr)
            .unwrap_or_else(|| panic!("Failed to find HIR expr for Expr {:?}", expr.span));
        let typeck = self.rust_program.tcx.typeck(hir_expr.hir_id.owner);
        let mir_ty = typeck.expr_ty(hir_expr);
        let ty_res = mir_ty_to_ty(&mir_ty);
        ty_res
    }

    // Get the inner type if the expr is a pointer
    fn expr_ptr_ty(&self, expr: &Expr) -> Ty {
        let ty = self.expr_ty(expr);
        match &ty.kind {
            TyKind::Ptr(mut_ty) => (*mut_ty.ty).clone(),
            _ => panic!("Expected pointer type for type {:#?}", ty),
        }
    }

    // fn map_raw_unwrap(&self, orig: Expr) -> Expr {
    //     let ptr_ty = self.expr_ptr_ty(&orig);
    //     utils::expr!(
    //         "{}.map(|r| r as *mut _).unwrap_or(std::ptr::null_mut::<{}>())",
    //         pprust::expr_to_string(&orig),
    //         pprust::ty_to_string(&ptr_ty)
    //     )
    // }

    // expr -> expr.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()
    fn append_as_deref_mut_raw(&self, orig: Expr) -> Expr {
        let ptr_ty = self.expr_ptr_ty(&orig);
        utils::expr!(
            "({}).as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut::<{}>())",
            pprust::expr_to_string(&orig),
            pprust::ty_to_string(&ptr_ty)
        )
    }

    // expr -> &raw (mut) *expr
    fn cast_to_mut_raw(&self, orig: Expr, mutability: bool) -> Expr {
        let deref = Expr {
            id: rustc_ast::DUMMY_NODE_ID,
            kind: ExprKind::Unary(UnOp::Deref, P(orig.clone())),
            span: rustc_span::DUMMY_SP,
            attrs: thin_vec![],
            tokens: None,
        };
        Expr {
            id: rustc_ast::DUMMY_NODE_ID,
            kind: ExprKind::AddrOf(
                BorrowKind::Raw,
                if mutability {
                    Mutability::Mut
                } else {
                    Mutability::Not
                },
                P(deref),
            ),
            span: rustc_span::DUMMY_SP,
            attrs: thin_vec![],
            tokens: None,
        }
    }

    fn rewrite_ty(&mut self, ty: &mut Ty, ty_res: Ty, dec: &Option<PtrKind>) {
        match dec {
            Some(PtrKind::OptMutRef) => {
                let ptr_mut_ty = expect_ptr(ty, ty_res);
                // Create Option<&mut T>
                assert!(ptr_mut_ty.mutbl == rustc_ast::Mutability::Mut);

                let inner_ref = P(Ty {
                    id: rustc_ast::DUMMY_NODE_ID,
                    kind: TyKind::Ref(None, ptr_mut_ty),
                    span: rustc_span::DUMMY_SP,
                    tokens: None,
                });

                // Create the Option path
                let option_path = Path {
                    span: rustc_span::DUMMY_SP,
                    segments: vec![PathSegment {
                        ident: Ident::from_str("Option"),
                        id: rustc_ast::DUMMY_NODE_ID,
                        args: Some(P(GenericArgs::AngleBracketed(AngleBracketedArgs {
                            span: rustc_span::DUMMY_SP,
                            args: [AngleBracketedArg::Arg(GenericArg::Type(inner_ref))].into(),
                        }))),
                    }]
                    .into(),
                    tokens: None,
                };

                ty.kind = TyKind::Path(None, option_path);
                self.updated = true;
            }
            Some(PtrKind::Ref(mutability)) => {
                let mut ptr_mut_ty = expect_ptr(ty, ty_res);
                ptr_mut_ty.mutbl = if *mutability {
                    rustc_ast::Mutability::Mut
                } else {
                    rustc_ast::Mutability::Not
                };
                ty.kind = TyKind::Ref(None, ptr_mut_ty);
                self.updated = true;
            }
            _ => {}
        }
    }
}

fn expect_ptr(ty: &mut Ty, ty_res: Ty) -> MutTy {
    match std::mem::replace(&mut ty.kind, TyKind::Infer) {
        TyKind::Ptr(ptr) => ptr,
        _ => match ty_res.kind {
            TyKind::Ptr(ptr) => ptr,
            _ => panic!("Expected pointer type for type {:#?}", ty),
        },
    }
}

fn mir_ty_to_ty(mir_ty: &MirTy) -> Ty {
    let ty_str = mir_ty
        .to_string()
        .replace("src::", "crate::src::")
        .replace("bin::", "crate::bin::");
    utils::ty!("{}", ty_str)
}

// expr -> expr.as_mut()
fn append_as_mut(orig: Expr) -> Expr {
    Expr {
        id: rustc_ast::DUMMY_NODE_ID,
        kind: ExprKind::MethodCall(Box::new(MethodCall {
            seg: PathSegment {
                ident: Ident::from_str("as_mut"),
                id: rustc_ast::DUMMY_NODE_ID,
                args: None,
            },
            receiver: P(orig),
            args: thin_vec![],
            span: rustc_span::DUMMY_SP,
        })),
        span: rustc_span::DUMMY_SP,
        attrs: thin_vec![],
        tokens: None,
    }
}

fn append_as_mut_unwrap(orig: Expr) -> Expr {
    utils::expr!("({}).as_mut().unwrap()", pprust::expr_to_string(&orig))
}

fn append_as_ref_unwrap(orig: Expr) -> Expr {
    utils::expr!("({}).as_ref().unwrap()", pprust::expr_to_string(&orig))
}

// fn append_opt_as_raw(orig: Expr) -> Expr {
//     utils::expr!("{}.as_deref().unwrap()", pprust::expr_to_string(&orig))
// }

fn prepend_ref(orig: Expr, mutability: bool) -> Expr {
    Expr {
        id: rustc_ast::DUMMY_NODE_ID,
        kind: ExprKind::AddrOf(
            BorrowKind::Ref,
            if mutability {
                Mutability::Mut
            } else {
                Mutability::Not
            },
            P(orig),
        ),
        span: rustc_span::DUMMY_SP,
        attrs: thin_vec![],
        tokens: None,
    }
}

// expr -> &(mut) *expr
fn prepend_ref_deref(orig: Expr, mutability: bool) -> Expr {
    // *expr
    let deref_expr = Expr {
        id: rustc_ast::DUMMY_NODE_ID,
        kind: ExprKind::Unary(UnOp::Deref, P(orig)),
        span: rustc_span::DUMMY_SP,
        attrs: thin_vec![],
        tokens: None,
    };
    // &mut *expr
    prepend_ref(deref_expr, mutability)
}

// expr -> Some(&mut expr)
fn wrap_in_some_mut_ref(orig: Expr) -> Expr {
    utils::expr!("Some(&mut {})", pprust::expr_to_string(&orig))
}

// expr -> Some(expr)
fn wrap_in_some(orig: Expr) -> Expr {
    let some_path = Path {
        span: rustc_span::DUMMY_SP,
        segments: vec![PathSegment {
            ident: Ident::from_str("Some"),
            id: rustc_ast::DUMMY_NODE_ID,
            args: None,
        }]
        .into(),
        tokens: None,
    };
    Expr {
        id: rustc_ast::DUMMY_NODE_ID,
        kind: ExprKind::Call(
            P(Expr {
                id: rustc_ast::DUMMY_NODE_ID,
                kind: ExprKind::Path(None, some_path),
                span: rustc_span::DUMMY_SP,
                attrs: thin_vec![],
                tokens: None,
            }),
            thin_vec![P(orig)],
        ),
        span: rustc_span::DUMMY_SP,
        attrs: thin_vec![],
        tokens: None,
    }
}

fn reborrow_mut_opt(orig: Expr) -> Expr {
    utils::expr!(
        "({}).as_mut().map(|x| &mut **x)",
        pprust::expr_to_string(&orig)
    )
}
