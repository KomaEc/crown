#![feature(rustc_private)]
#![feature(let_else)]

extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_middle;
extern crate rustc_span;

use orc_common::rewriter::{RewriteMode, Rewriter};
use rustc_hir::{Block, Expr, ExprKind, ItemKind, OwnerNode, StmtKind, UnOp};
use rustc_middle::ty::TyCtxt;
use rustc_span::BytePos;

pub fn preprocess(tcx: TyCtxt, mode: RewriteMode) {
    insert_null_statement(tcx, mode)
}

fn insert_null_stmt_to_branch(
    tcx: TyCtxt,
    stmt_str: String,
    branch: &Expr,
    rewriter: &mut Rewriter,
) {
    let branch_span_lo = branch.span.lo();
    let empty_span_after_curly_brace = branch
        .span
        .with_lo(branch_span_lo + BytePos(1))
        .shrink_to_lo();
    rewriter.make_suggestion(tcx, empty_span_after_curly_brace, String::new(), stmt_str)
}

fn insert_null_statement_for_block(tcx: TyCtxt, block: &Block, rewriter: &mut Rewriter) {
    block
        .stmts
        .iter()
        .filter_map(|stmt| match stmt.kind {
            StmtKind::Expr(expr) | StmtKind::Semi(expr) => Some(expr),
            _ => None,
        })
        .chain(block.expr)
        .for_each(|expr| match expr.kind {
            ExprKind::Block(inner_block, _) => {
                insert_null_statement_for_block(tcx, inner_block, rewriter)
            }
            ExprKind::If(cond, truth_branch, false_branch) => {
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
                        let ExprKind::Path(..) = ptr.kind else { return };

                        let stmt_str = rustc_hir_pretty::id_to_string(&tcx.hir(), ptr.hir_id)
                            + " = 0 as *mut _;";

                        if sign {
                            insert_null_stmt_to_branch(tcx, stmt_str, truth_branch, rewriter)
                        } else {
                            if let Some(false_branch) = false_branch {
                                insert_null_stmt_to_branch(tcx, stmt_str, false_branch, rewriter)
                            } else {
                                let empty_span_after_curly_brace = truth_branch.span.shrink_to_hi();
                                rewriter.make_suggestion(
                                    tcx,
                                    empty_span_after_curly_brace,
                                    String::new(),
                                    "else { ".to_string() + &stmt_str + " }",
                                )
                            }
                        }

                        // println!("The span of truth branch: {:?}", truth_branch.span);
                    }
                    _ => {}
                }
            }
            _ => {}
        });
}

fn insert_null_statement(tcx: TyCtxt, mode: RewriteMode) {
    let mut rewriter = Rewriter::default();

    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        let ItemKind::Fn(_, _, body_id) = item.kind else { continue };
        let hir_body = tcx.hir().body(body_id);
        let ExprKind::Block(block, _) = hir_body.value.kind else { todo!("function body is assumed to be a block") };
        insert_null_statement_for_block(tcx, block, &mut rewriter)
    }

    rewriter.write(mode)
}
