use rustc_data_structures::steal::Steal;
use rustc_hash::FxHashMap;
use rustc_hir::{HirId, *};
use rustc_middle::{
    hir::nested_filter,
    thir::{self, ArmId, BlockId, ExprId, StmtId},
    ty::{
        self, TyCtxt, TypeckResults,
        adjustment::{Adjust, AutoBorrow},
    },
};

pub fn map_hir_to_thir<'tcx>(tcx: TyCtxt<'tcx>) -> HirToThir {
    let mut hir_to_thir = HirToThir::default();
    for def_id in tcx.hir_body_owners() {
        let body_id = tcx.hir_node_by_def_id(def_id).body_id().unwrap();
        let body = tcx.hir_body(body_id);
        let typeck_results = tcx.typeck(def_id);
        let (thir, texpr) = tcx.thir_body(def_id).unwrap();
        let mut mapper = HirToThirMapper {
            tcx,
            thir: &thir.borrow(),
            typeck_results,
            hir_to_thir,
        };
        mapper.map_expr_to_expr(body.value, texpr);
        hir_to_thir = mapper.hir_to_thir;
    }
    hir_to_thir
}

#[derive(Debug, Default)]
pub struct HirToThir {
    pub arms: FxHashMap<HirId, ArmId>,
    pub blocks: FxHashMap<HirId, BlockId>,
    pub exprs: FxHashMap<HirId, ExprId>,
    pub stmts: FxHashMap<HirId, StmtId>,
}

struct HirToThirMapper<'a, 'tcx> {
    tcx: TyCtxt<'tcx>,
    thir: &'a thir::Thir<'tcx>,
    typeck_results: &'tcx TypeckResults<'tcx>,
    hir_to_thir: HirToThir,
}

// See `rustc_mir_build/src/thir/cx` for the lowering logic.
impl<'tcx> HirToThirMapper<'_, 'tcx> {
    fn map_expr_to_expr(&mut self, expr: &'tcx Expr<'tcx>, mut texpr_id: ExprId) {
        let mut texpr = &self.thir.exprs[texpr_id];

        if let thir::ExprKind::Scope { value, .. } = &texpr.kind {
            texpr_id = *value;
            texpr = &self.thir.exprs[texpr_id];
        }

        for adjustment in self.typeck_results.expr_adjustments(expr).iter().rev() {
            match adjustment.kind {
                Adjust::NeverToAny => {
                    if !adjustment.target.is_never() {
                        let thir::ExprKind::NeverToAny { source } = &texpr.kind else {
                            panic!("Not NeverToAny:\n{texpr:?}")
                        };
                        texpr_id = *source;
                    }
                }
                Adjust::Deref(None) => {
                    let thir::ExprKind::Deref { arg } = &texpr.kind else {
                        panic!("Not Deref:\n{texpr:?}")
                    };
                    texpr_id = *arg;
                }
                Adjust::Deref(Some(_)) => {
                    let thir::ExprKind::Deref { arg } = &texpr.kind else {
                        panic!("Not Deref:\n{texpr:?}")
                    };
                    let arg = &self.thir.exprs[*arg];
                    let thir::ExprKind::Call {
                        args: box [arg], ..
                    } = &arg.kind
                    else {
                        panic!("Not Call:\n{arg:?}")
                    };
                    let arg = &self.thir.exprs[*arg];
                    let thir::ExprKind::Borrow { arg, .. } = &arg.kind else {
                        panic!("Not Borrow:\n{arg:?}")
                    };
                    texpr_id = *arg;
                }
                Adjust::Borrow(AutoBorrow::Ref(_)) => {
                    let thir::ExprKind::Borrow { arg, .. } = &texpr.kind else {
                        panic!("Not Borrow:\n{texpr:?}")
                    };
                    texpr_id = *arg;
                }
                Adjust::Borrow(AutoBorrow::RawPtr(_)) => {
                    let thir::ExprKind::RawBorrow { arg, .. } = &texpr.kind else {
                        panic!("Not RawBorrow:\n{texpr:?}")
                    };
                    texpr_id = *arg;
                }
                Adjust::Pointer(_) => {
                    let thir::ExprKind::PointerCoercion { source, .. } = &texpr.kind else {
                        panic!("Not PointerCoercion:\n{texpr:?}")
                    };
                    texpr_id = *source;
                }
                Adjust::ReborrowPin(_) => todo!(),
            }
            texpr = &self.thir.exprs[texpr_id];
        }

        self.hir_to_thir.exprs.insert(expr.hir_id, texpr_id);

        match expr.kind {
            ExprKind::ConstBlock(_) => {
                let thir::ExprKind::ConstBlock { .. } = &texpr.kind else {
                    panic!("Not ConstBlock:\n{texpr:?}")
                };
            }
            ExprKind::Array(exprs) => {
                let thir::ExprKind::Array { fields: texprs } = &texpr.kind else {
                    panic!("Not Array:\n{texpr:?}")
                };
                assert_eq!(exprs.len(), texprs.len());
                for (expr, texpr) in exprs.iter().zip(texprs) {
                    self.map_expr_to_expr(expr, *texpr);
                }
            }
            ExprKind::Call(fun, args) => {
                if self.typeck_results.is_method_call(expr) {
                    let thir::ExprKind::Call {
                        args: box [tfun, targs],
                        ..
                    } = &texpr.kind
                    else {
                        panic!("Not Call:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(fun, *tfun);
                    let targs = &self.thir.exprs[*targs];
                    let thir::ExprKind::Tuple { fields: targs } = &targs.kind else {
                        panic!("Not Tuple:\n{targs:?}")
                    };
                    assert_eq!(args.len(), targs.len());
                    for (arg, targ) in args.iter().zip(targs) {
                        self.map_expr_to_expr(arg, *targ);
                    }
                } else if let ty::FnDef(def_id, _) = self.typeck_results.expr_ty(fun).kind()
                    && let Some(intrinsic) = self.tcx.intrinsic(def_id)
                    && intrinsic.name == rustc_span::sym::box_new
                {
                    assert_eq!(args.len(), 1);
                    let thir::ExprKind::Box { value: texpr } = &texpr.kind else {
                        panic!("Not Box:\n{texpr:?}")
                    };
                    self.hir_to_thir.exprs.insert(fun.hir_id, texpr_id);
                    self.map_expr_to_expr(&args[0], *texpr);
                } else {
                    match &texpr.kind {
                        thir::ExprKind::Call {
                            fun: tfun,
                            args: targs,
                            ..
                        } => {
                            self.map_expr_to_expr(fun, *tfun);
                            assert_eq!(args.len(), targs.len());
                            for (arg, targ) in args.iter().zip(targs) {
                                self.map_expr_to_expr(arg, *targ);
                            }
                        }
                        thir::ExprKind::Adt(box thir::AdtExpr { fields, .. }) => {
                            self.hir_to_thir.exprs.insert(fun.hir_id, texpr_id);
                            assert_eq!(args.len(), fields.len());
                            for (arg, field) in args.iter().zip(fields) {
                                self.map_expr_to_expr(arg, field.expr);
                            }
                        }
                        _ => panic!("Not Call/Adt:\n{texpr:?}"),
                    }
                }
            }
            ExprKind::MethodCall(_, receiver, args, _) => {
                let thir::ExprKind::Call {
                    args: box [treceiver, targs @ ..],
                    ..
                } = &texpr.kind
                else {
                    panic!("Not Call:\n{texpr:?}")
                };
                self.map_expr_to_expr(receiver, *treceiver);
                assert_eq!(args.len(), targs.len());
                for (arg, targ) in args.iter().zip(targs) {
                    self.map_expr_to_expr(arg, *targ);
                }
            }
            ExprKind::Use(expr, _) => {
                let thir::ExprKind::ByUse { expr: texpr, .. } = &texpr.kind else {
                    panic!("Not ByUse:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::Tup(exprs) => {
                let thir::ExprKind::Tuple { fields: texprs } = &texpr.kind else {
                    panic!("Not Tuple:\n{texpr:?}")
                };
                assert_eq!(exprs.len(), texprs.len());
                for (expr, texpr) in exprs.iter().zip(texprs) {
                    self.map_expr_to_expr(expr, *texpr);
                }
            }
            ExprKind::Binary(_, lhs, rhs) => {
                if self.typeck_results.is_method_call(expr) {
                    let thir::ExprKind::Call {
                        args: box [tlhs, trhs],
                        ..
                    } = &texpr.kind
                    else {
                        panic!("Not Call:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(lhs, *tlhs);
                    self.map_expr_to_expr(rhs, *trhs);
                } else {
                    let (thir::ExprKind::Binary {
                        lhs: tlhs,
                        rhs: trhs,
                        ..
                    }
                    | thir::ExprKind::LogicalOp {
                        lhs: tlhs,
                        rhs: trhs,
                        ..
                    }) = &texpr.kind
                    else {
                        panic!("Not Binary/LogicalOp:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(lhs, *tlhs);
                    self.map_expr_to_expr(rhs, *trhs);
                }
            }
            ExprKind::Unary(UnOp::Deref, arg) => {
                if self.typeck_results.is_method_call(expr) {
                    let thir::ExprKind::Deref { arg: texpr } = &texpr.kind else {
                        panic!("Not Deref:\n{texpr:?}")
                    };
                    let texpr = &self.thir.exprs[*texpr];
                    let thir::ExprKind::Call {
                        args: box [targ], ..
                    } = &texpr.kind
                    else {
                        panic!("Not Call:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(arg, *targ);
                } else {
                    let thir::ExprKind::Deref { arg: targ } = &texpr.kind else {
                        panic!("Not Deref:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(arg, *targ);
                }
            }
            ExprKind::Unary(UnOp::Not, arg) => {
                if self.typeck_results.is_method_call(expr) {
                    let thir::ExprKind::Call {
                        args: box [targ], ..
                    } = &texpr.kind
                    else {
                        panic!("Not Call:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(arg, *targ);
                } else {
                    let thir::ExprKind::Unary { arg: targ, .. } = &texpr.kind else {
                        panic!("Not Unary:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(arg, *targ);
                }
            }
            ExprKind::Unary(UnOp::Neg, arg) => {
                if self.typeck_results.is_method_call(expr) {
                    let thir::ExprKind::Call {
                        args: box [targ], ..
                    } = &texpr.kind
                    else {
                        panic!("Not Call:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(arg, *targ);
                } else if let ExprKind::Lit(_) = arg.kind {
                    assert!(matches!(texpr.kind, thir::ExprKind::Literal { .. }));
                    self.hir_to_thir.exprs.insert(arg.hir_id, texpr_id);
                } else {
                    let thir::ExprKind::Unary { arg: targ, .. } = &texpr.kind else {
                        panic!("Not Unary:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(arg, *targ);
                }
            }
            ExprKind::Lit(_) => {
                let thir::ExprKind::Literal { .. } = &texpr.kind else {
                    panic!("Not Literal:\n{texpr:?}")
                };
            }
            ExprKind::Cast(expr, _) => {
                let texpr = if let thir::ExprKind::ValueTypeAscription { source, .. } = &texpr.kind
                {
                    &self.thir.exprs[*source]
                } else {
                    texpr
                };
                match &texpr.kind {
                    thir::ExprKind::Use { source: texpr }
                    | thir::ExprKind::PointerCoercion { source: texpr, .. } => {
                        self.map_expr_to_expr(expr, *texpr);
                    }
                    thir::ExprKind::Cast { source: texpr } => {
                        if matches!(expr.kind, ExprKind::Path(_)) {
                            self.hir_to_thir.exprs.insert(expr.hir_id, texpr_id);
                        } else {
                            self.map_expr_to_expr(expr, *texpr);
                        }
                    }
                    _ => panic!("Not Use/PointerCoercion/Cast:\n{texpr:?}"),
                }
            }
            ExprKind::Type(expr, _) => {
                let (thir::ExprKind::PlaceTypeAscription { source: texpr, .. }
                | thir::ExprKind::ValueTypeAscription { source: texpr, .. }) = &texpr.kind
                else {
                    panic!("Not PlaceTypeAscription/ValueTypeAscription:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::DropTemps(expr) => {
                let thir::ExprKind::Use { source: texpr, .. } = &texpr.kind else {
                    panic!("Not Use:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::Let(LetExpr { init, .. }) => {
                let thir::ExprKind::Let { expr: tinit, .. } = &texpr.kind else {
                    panic!("Not Let:\n{texpr:?}")
                };
                self.map_expr_to_expr(init, *tinit);
            }
            ExprKind::If(c, t, f) => {
                let thir::ExprKind::If {
                    cond: tc,
                    then: tt,
                    else_opt: tf,
                    ..
                } = &texpr.kind
                else {
                    panic!("Not If:\n{texpr:?}")
                };
                self.map_expr_to_expr(c, *tc);
                self.map_expr_to_expr(t, *tt);
                assert_eq!(f.is_some(), tf.is_some());
                if let (Some(f), Some(tf)) = (f, tf) {
                    self.map_expr_to_expr(f, *tf);
                }
            }
            ExprKind::Loop(block, _, _, _) => {
                let thir::ExprKind::Loop { body: tblock, .. } = &texpr.kind else {
                    panic!("Not Loop:\n{texpr:?}")
                };
                let tblock = &self.thir.exprs[*tblock];
                let thir::ExprKind::Block { block: tblock, .. } = &tblock.kind else {
                    panic!("Not Block:\n{tblock:?}")
                };
                self.map_block_to_block(block, *tblock);
            }
            ExprKind::Match(expr, arms, _) => {
                let thir::ExprKind::Match {
                    scrutinee: texpr,
                    arms: tarms,
                    ..
                } = &texpr.kind
                else {
                    panic!("Not Match:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
                assert_eq!(arms.len(), tarms.len());
                for (arm, tarm) in arms.iter().zip(tarms) {
                    self.map_arm_to_arm(arm, *tarm);
                }
            }
            ExprKind::Closure(Closure { def_id, .. }) => {
                let thir::ExprKind::Closure(box thir::ClosureExpr { closure_id, .. }) = &texpr.kind
                else {
                    panic!("Not Closure:\n{texpr:?}")
                };
                assert_eq!(def_id, closure_id);
            }
            ExprKind::Block(block, _) => {
                let thir::ExprKind::Block { block: tblock } = &texpr.kind else {
                    panic!("Not Block:\n{texpr:?}")
                };
                self.map_block_to_block(block, *tblock);
            }
            ExprKind::Assign(lhs, rhs, _) => {
                let thir::ExprKind::Assign {
                    lhs: tlhs,
                    rhs: trhs,
                } = &texpr.kind
                else {
                    panic!("Not Assign:\n{texpr:?}")
                };
                self.map_expr_to_expr(lhs, *tlhs);
                self.map_expr_to_expr(rhs, *trhs);
            }
            ExprKind::AssignOp(_, lhs, rhs) => {
                if self.typeck_results.is_method_call(expr) {
                    let thir::ExprKind::Call {
                        args: box [tlhs, trhs],
                        ..
                    } = &texpr.kind
                    else {
                        panic!("Not Call:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(lhs, *tlhs);
                    self.map_expr_to_expr(rhs, *trhs);
                } else {
                    let thir::ExprKind::AssignOp {
                        lhs: tlhs,
                        rhs: trhs,
                        ..
                    } = &texpr.kind
                    else {
                        panic!("Not AssignOp:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(lhs, *tlhs);
                    self.map_expr_to_expr(rhs, *trhs);
                }
            }
            ExprKind::Field(expr, _) => {
                let thir::ExprKind::Field { lhs: texpr, .. } = &texpr.kind else {
                    panic!("Not Field:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::Index(lhs, index, _) => {
                if self.typeck_results.is_method_call(expr) {
                    let thir::ExprKind::Deref { arg } = &texpr.kind else {
                        panic!("Not Deref:\n{texpr:?}")
                    };
                    let arg = &self.thir.exprs[*arg];
                    let thir::ExprKind::Call {
                        args: box [tlhs, tindex],
                        ..
                    } = &arg.kind
                    else {
                        panic!("Not Call:\n{arg:?}")
                    };
                    self.map_expr_to_expr(lhs, *tlhs);
                    self.map_expr_to_expr(index, *tindex);
                } else {
                    let thir::ExprKind::Index {
                        lhs: tlhs,
                        index: tindex,
                    } = &texpr.kind
                    else {
                        panic!("Not Index:\n{texpr:?}")
                    };
                    self.map_expr_to_expr(lhs, *tlhs);
                    self.map_expr_to_expr(index, *tindex);
                }
            }
            ExprKind::Path(_) => {}
            ExprKind::AddrOf(_, _, expr) => {
                let (thir::ExprKind::Borrow { arg: texpr, .. }
                | thir::ExprKind::RawBorrow { arg: texpr, .. }) = &texpr.kind
                else {
                    panic!("Not Borrow/RawBorrow:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::Break(_, expr) => {
                let thir::ExprKind::Break { value: texpr, .. } = &texpr.kind else {
                    panic!("Not Break:\n{texpr:?}")
                };
                assert_eq!(expr.is_some(), texpr.is_some());
                if let (Some(expr), Some(texpr)) = (expr, texpr) {
                    self.map_expr_to_expr(expr, *texpr);
                }
            }
            ExprKind::Continue(_) => {
                let thir::ExprKind::Continue { .. } = &texpr.kind else {
                    panic!("Not Continue:\n{texpr:?}")
                };
            }
            ExprKind::Ret(expr) => {
                let thir::ExprKind::Return { value: texpr } = &texpr.kind else {
                    panic!("Not Return:\n{texpr:?}")
                };
                assert_eq!(expr.is_some(), texpr.is_some());
                if let (Some(expr), Some(texpr)) = (expr, texpr) {
                    self.map_expr_to_expr(expr, *texpr);
                }
            }
            ExprKind::Become(expr) => {
                let thir::ExprKind::Become { value: texpr } = &texpr.kind else {
                    panic!("Not Become:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::InlineAsm(InlineAsm { operands, .. }) => {
                let thir::ExprKind::InlineAsm(box thir::InlineAsmExpr {
                    operands: toperands,
                    ..
                }) = &texpr.kind
                else {
                    panic!("Not InlineAsm:\n{texpr:?}")
                };
                assert_eq!(operands.len(), toperands.len());
                for ((operand, _), toperand) in operands.iter().zip(toperands) {
                    self.map_inline_asm_operand_to_inline_asm_operand(operand, toperand);
                }
            }
            ExprKind::OffsetOf(_, _) => {
                let thir::ExprKind::OffsetOf { .. } = &texpr.kind else {
                    panic!("Not OffsetOf:\n{texpr:?}")
                };
            }
            ExprKind::Struct(_, fields, base) => {
                let thir::ExprKind::Adt(box thir::AdtExpr {
                    fields: tfields,
                    base: tbase,
                    ..
                }) = &texpr.kind
                else {
                    panic!("Not Adt:\n{texpr:?}")
                };
                assert_eq!(fields.len(), tfields.len());
                for (field, tfield) in fields.iter().zip(tfields) {
                    self.map_expr_to_expr(field.expr, tfield.expr);
                }
                if let StructTailExpr::Base(base) = base {
                    let thir::AdtExprBase::Base(info) = tbase else {
                        panic!("Not Base:\n{tbase:?}")
                    };
                    self.map_expr_to_expr(base, info.base);
                }
            }
            ExprKind::Repeat(expr, _) => {
                let thir::ExprKind::Repeat { value: texpr, .. } = &texpr.kind else {
                    panic!("Not Repeat:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::Yield(expr, _) => {
                let thir::ExprKind::Yield { value: texpr, .. } = &texpr.kind else {
                    panic!("Not Yield:\n{texpr:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::UnsafeBinderCast(_, expr, _) => {
                let (thir::ExprKind::PlaceUnwrapUnsafeBinder { source: texpr }
                | thir::ExprKind::ValueUnwrapUnsafeBinder { source: texpr }
                | thir::ExprKind::WrapUnsafeBinder { source: texpr }) = &texpr.kind
                else {
                    panic!(
                        "Not PlaceUnwrapUnsafeBinder/ValueUnwrapUnsafeBinder/WrapUnsafeBinder:\n{texpr:?}"
                    )
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            ExprKind::Err(_) => panic!(),
        }
    }

    fn map_inline_asm_operand_to_inline_asm_operand(
        &mut self,
        operand: &'tcx InlineAsmOperand<'tcx>,
        toperand: &thir::InlineAsmOperand<'tcx>,
    ) {
        match operand {
            InlineAsmOperand::In { expr, .. } => {
                let thir::InlineAsmOperand::In { expr: texpr, .. } = toperand else {
                    panic!("Not In:\n{toperand:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            InlineAsmOperand::Out { expr, .. } => {
                let thir::InlineAsmOperand::Out { expr: texpr, .. } = toperand else {
                    panic!("Not Out:\n{toperand:?}")
                };
                assert_eq!(expr.is_some(), texpr.is_some());
                if let (Some(expr), Some(texpr)) = (expr, texpr) {
                    self.map_expr_to_expr(expr, *texpr);
                }
            }
            InlineAsmOperand::InOut { expr, .. } => {
                let thir::InlineAsmOperand::InOut { expr: texpr, .. } = toperand else {
                    panic!("Not InOut:\n{toperand:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            InlineAsmOperand::SplitInOut {
                in_expr, out_expr, ..
            } => {
                let thir::InlineAsmOperand::SplitInOut {
                    in_expr: tin_expr,
                    out_expr: tout_expr,
                    ..
                } = toperand
                else {
                    panic!("Not SplitInOut:\n{toperand:?}")
                };
                self.map_expr_to_expr(in_expr, *tin_expr);
                assert_eq!(out_expr.is_some(), tout_expr.is_some());
                if let (Some(out_expr), Some(tout_expr)) = (out_expr, tout_expr) {
                    self.map_expr_to_expr(out_expr, *tout_expr);
                }
            }
            InlineAsmOperand::Const { .. } => {
                let thir::InlineAsmOperand::Const { .. } = toperand else {
                    panic!("Not Const:\n{toperand:?}")
                };
            }
            InlineAsmOperand::SymFn { expr } => {
                let thir::InlineAsmOperand::SymFn { value: texpr } = toperand else {
                    panic!("Not SymFn:\n{toperand:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            InlineAsmOperand::SymStatic { def_id, .. } => {
                let thir::InlineAsmOperand::SymStatic { def_id: tdef_id } = toperand else {
                    panic!("Not SymStatic:\n{toperand:?}")
                };
                assert_eq!(def_id, tdef_id);
            }
            InlineAsmOperand::Label { block } => {
                let thir::InlineAsmOperand::Label { block: tblock } = toperand else {
                    panic!("Not Label:\n{toperand:?}")
                };
                self.map_block_to_block(block, *tblock);
            }
        }
    }

    fn map_block_to_block(&mut self, block: &'tcx Block<'tcx>, tblock: BlockId) {
        self.hir_to_thir.blocks.insert(block.hir_id, tblock);
        let tblock = &self.thir.blocks[tblock];
        let mut i = 0;
        for stmt in block.stmts {
            if matches!(stmt.kind, StmtKind::Item(_)) {
                continue;
            }
            let tstmt = tblock.stmts[i];
            self.map_stmt_to_stmt(stmt, tstmt);
            i += 1;
        }
        assert_eq!(block.expr.is_some(), tblock.expr.is_some());
        if let (Some(expr), Some(texpr)) = (block.expr, tblock.expr) {
            self.map_expr_to_expr(expr, texpr);
        }
    }

    fn map_stmt_to_stmt(&mut self, stmt: &'tcx Stmt<'tcx>, tstmt: StmtId) {
        self.hir_to_thir.stmts.insert(stmt.hir_id, tstmt);
        let tstmt = &self.thir.stmts[tstmt];
        match stmt.kind {
            StmtKind::Expr(expr) | StmtKind::Semi(expr) => {
                let thir::StmtKind::Expr { expr: texpr, .. } = &tstmt.kind else {
                    panic!("Not Expr:\n{tstmt:?}")
                };
                self.map_expr_to_expr(expr, *texpr);
            }
            StmtKind::Let(LetStmt { init, els, .. }) => {
                let thir::StmtKind::Let {
                    initializer: tinit,
                    else_block: tels,
                    ..
                } = &tstmt.kind
                else {
                    panic!("Not Let:\n{tstmt:?}")
                };
                assert_eq!(init.is_some(), tinit.is_some());
                if let (Some(init), Some(tinit)) = (init, tinit) {
                    self.map_expr_to_expr(init, *tinit);
                }
                assert_eq!(els.is_some(), tels.is_some());
                if let (Some(els), Some(tels)) = (els, tels) {
                    self.map_block_to_block(els, *tels);
                }
            }
            StmtKind::Item(_) => panic!(),
        }
    }

    fn map_arm_to_arm(&mut self, arm: &'tcx Arm<'tcx>, tarm: ArmId) {
        self.hir_to_thir.arms.insert(arm.hir_id, tarm);
        let tarm = &self.thir.arms[tarm];
        assert_eq!(arm.guard.is_some(), tarm.guard.is_some());
        if let (Some(guard), Some(tguard)) = (arm.guard, tarm.guard) {
            self.map_expr_to_expr(guard, tguard);
        }
        self.map_expr_to_expr(arm.body, tarm.body);
    }
}

impl HirToThir {
    #[inline]
    pub fn get_thir(hir_id: HirId, tcx: TyCtxt<'_>) -> &Steal<thir::Thir<'_>> {
        tcx.thir_body(tcx.hir_enclosing_body_owner(hir_id))
            .unwrap()
            .0
    }

    #[inline]
    pub fn get_arm<'a, 'tcx>(
        &self,
        hir_id: HirId,
        thir: &'a thir::Thir<'tcx>,
    ) -> Option<&'a thir::Arm<'tcx>> {
        self.arms.get(&hir_id).and_then(|&id| thir.arms.get(id))
    }

    #[inline]
    pub fn get_block<'a>(
        &self,
        hir_id: HirId,
        thir: &'a thir::Thir<'_>,
    ) -> Option<&'a thir::Block> {
        self.blocks.get(&hir_id).and_then(|&id| thir.blocks.get(id))
    }

    #[inline]
    pub fn get_expr<'a, 'tcx>(
        &self,
        hir_id: HirId,
        thir: &'a thir::Thir<'tcx>,
    ) -> Option<&'a thir::Expr<'tcx>> {
        self.exprs.get(&hir_id).and_then(|&id| thir.exprs.get(id))
    }

    #[inline]
    pub fn get_stmt<'a, 'tcx>(
        &self,
        hir_id: HirId,
        thir: &'a thir::Thir<'tcx>,
    ) -> Option<&'a thir::Stmt<'tcx>> {
        self.stmts.get(&hir_id).and_then(|&id| thir.stmts.get(id))
    }
}

pub struct HirToThirChecker<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub hir_to_thir: HirToThir,
}

impl<'tcx> intravisit::Visitor<'tcx> for HirToThirChecker<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_arm(&mut self, arm: &'tcx Arm<'tcx>) {
        let thir = HirToThir::get_thir(arm.hir_id, self.tcx);
        self.hir_to_thir
            .get_arm(arm.hir_id, &thir.borrow())
            .unwrap_or_else(|| panic!("{arm:?}"));
        intravisit::walk_arm(self, arm);
    }

    fn visit_block(&mut self, block: &'tcx Block<'tcx>) {
        let thir = HirToThir::get_thir(block.hir_id, self.tcx);
        self.hir_to_thir
            .get_block(block.hir_id, &thir.borrow())
            .unwrap_or_else(|| panic!("{block:?}"));
        intravisit::walk_block(self, block);
    }

    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        let thir = HirToThir::get_thir(expr.hir_id, self.tcx);
        self.hir_to_thir
            .get_expr(expr.hir_id, &thir.borrow())
            .unwrap_or_else(|| panic!("{expr:?}"));
        intravisit::walk_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) {
        if !matches!(stmt.kind, StmtKind::Item(_)) {
            // Skip item stmts, as they are not in THIR.
            let thir = HirToThir::get_thir(stmt.hir_id, self.tcx);
            self.hir_to_thir
                .get_stmt(stmt.hir_id, &thir.borrow())
                .unwrap_or_else(|| panic!("{stmt:?}"));
        }
        intravisit::walk_stmt(self, stmt);
    }
}
