#![feature(rustc_private)]
#![feature(let_else)]

extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_middle;
extern crate rustc_span;

use orc_common::rewriter::{RewriteMode, Rewriter};
use rustc_hir::{
    intravisit::{self, Visitor},
    Expr, ExprKind, ItemKind, OwnerNode, UnOp,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::BytePos;

pub fn preprocess(tcx: TyCtxt, mode: RewriteMode) {
    insert_null_statement(tcx, mode)
}

struct NullStmtInsertor<'me, 'hir> {
    tcx: TyCtxt<'hir>,
    rewriter: &'me mut Rewriter,
}
impl<'me, 'hir> Visitor<'hir> for NullStmtInsertor<'me, 'hir> {
    fn visit_expr(&mut self, expr: &'hir Expr<'hir>) {
        if let ExprKind::If(cond, truth_branch, false_branch) = expr.kind {
            let ExprKind::DropTemps(mut cond) = cond.kind else {
                panic!("for some reasons, if conditions are all droptemps.");
            };
            // peel off negations
            let mut sign = true;
            while let ExprKind::Unary(UnOp::Not, expr) = cond.kind {
                sign = !sign;
                cond = expr;
            }

            match cond.kind {
                ExprKind::MethodCall(path, args, _) if path.ident.as_str() == "is_null" => {
                    let ptr = args.first().unwrap();

                    // currently we only rewrite variables not complex expressions
                    if let ExprKind::Path(..) = ptr.kind {
                        let stmt_str = rustc_hir_pretty::id_to_string(&self.tcx.hir(), ptr.hir_id)
                            + " = 0 as *mut _;";

                        if sign {
                            self.insert_to_branch(stmt_str, truth_branch);
                        } else {
                            if let Some(false_branch) = false_branch {
                                self.insert_to_branch(stmt_str, false_branch);
                            } else {
                                let empty_span_after_curly_brace = truth_branch.span.shrink_to_hi();
                                self.rewriter.make_suggestion(
                                    self.tcx,
                                    empty_span_after_curly_brace,
                                    String::new(),
                                    "else { ".to_string() + &stmt_str + " }",
                                )
                            }
                        }

                        return;
                    }
                }
                _ => {}
            }
        }

        intravisit::walk_expr(self, expr)
    }
}

impl<'me, 'hir> NullStmtInsertor<'me, 'hir> {
    fn insert_to_branch(&mut self, stmt_str: String, branch: &Expr) {
        let branch_span_lo = branch.span.lo();
        let empty_span_after_curly_brace = branch
            .span
            .with_lo(branch_span_lo + BytePos(1))
            .shrink_to_lo();
        self.rewriter.make_suggestion(
            self.tcx,
            empty_span_after_curly_brace,
            String::new(),
            stmt_str,
        )
    }
}

fn insert_null_statement(tcx: TyCtxt, mode: RewriteMode) {
    let mut rewriter = Rewriter::default();

    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        let ItemKind::Fn(_, _, body_id) = item.kind else { continue };
        let hir_body = tcx.hir().body(body_id);
        NullStmtInsertor {
            rewriter: &mut rewriter,
            tcx,
        }
        .visit_expr(&hir_body.value);
    }

    rewriter.write(mode)
}
