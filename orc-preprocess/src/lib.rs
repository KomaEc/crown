#![feature(rustc_private)]
#![feature(let_else)]

extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_middle;
extern crate rustc_span;

use orc_common::rewriter::{RewriteMode, Rewriter};
use rustc_hir::{
    intravisit::{self, Visitor},
    Expr, ExprKind, ItemKind, LoopSource, OwnerNode, UnOp,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::BytePos;

pub fn preprocess(tcx: TyCtxt, mode: RewriteMode) {
    // desugar_while_loop(tcx, mode)
    insert_null_statement(tcx, mode)
}

// struct WhileLoopDesugarer<'me, 'hir> {
//     tcx: TyCtxt<'hir>,
//     rewriter: &'me mut Rewriter,
// }

// impl<'me, 'hir> Visitor<'hir> for WhileLoopDesugarer<'me, 'hir> {
//     fn visit_expr(&mut self, expr: &'hir Expr<'hir>) {
//         let ExprKind::Loop(_, _, loop_src, span) = expr.kind else {
//             return intravisit::walk_expr(self, expr)
//         };
//         match loop_src {
//             LoopSource::Loop => return intravisit::walk_expr(self, expr),
//             LoopSource::While => {},
//             LoopSource::ForLoop => panic!("src code is assumed to not contain for loop"),
//         }
//         self.rewriter.make_suggestion(
//             self.tcx,
//             span,
//             "".to_string(),
//             rustc_hir_pretty::id_to_string(&self.tcx.hir(), expr.hir_id)
//         )
//     }
// }

// fn desugar_while_loop(tcx: TyCtxt, mode: RewriteMode) {
//     let mut rewriter = Rewriter::default();

//     for maybe_owner in tcx.hir().krate().owners.iter() {
//         let Some(owner) = maybe_owner.as_owner() else { continue };
//         let OwnerNode::Item(item) = owner.node() else { continue };
//         let ItemKind::Fn(_, _, body_id) = item.kind else { continue };
//         let hir_body = tcx.hir().body(body_id);
//         WhileLoopDesugarer {
//             rewriter: &mut rewriter,
//             tcx,
//         }
//         .visit_expr(&hir_body.value);
//     }

//     rewriter.write(mode)
// }

struct NullStmtInsertor<'me, 'hir> {
    tcx: TyCtxt<'hir>,
    rewriter: &'me mut Rewriter,
    skip: bool,
}
impl<'me, 'hir> Visitor<'hir> for NullStmtInsertor<'me, 'hir> {
    fn visit_expr(&mut self, expr: &'hir Expr<'hir>) {
        match expr.kind {
            ExprKind::If(cond, truth_branch, false_branch) if !self.skip => {
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
                            // rewrite is ensured, explicitly recurse into two branches
                            intravisit::walk_expr(self, truth_branch);
                            false_branch
                                .map(|false_branch| intravisit::walk_expr(self, false_branch));

                            let stmt_str =
                                rustc_hir_pretty::id_to_string(&self.tcx.hir(), ptr.hir_id)
                                    + " = 0 as *mut _;";

                            if sign {
                                self.insert_to_branch(stmt_str, truth_branch);
                            } else {
                                if let Some(false_branch) = false_branch {
                                    self.insert_to_branch(stmt_str, false_branch);
                                } else {
                                    let empty_span_after_curly_brace =
                                        truth_branch.span.shrink_to_hi();
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
            ExprKind::If(..) if self.skip => {
                self.skip = false;
            }
            ExprKind::Loop(_, _, LoopSource::While, _) => {
                self.skip = true;
            }
            _ => {}
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
        // println!("{}", rustc_hir_pretty::id_to_string(&tcx.hir(), item.hir_id()));
        // println!("body kind: {:?}", hir_body.value);
        NullStmtInsertor {
            rewriter: &mut rewriter,
            tcx,
            skip: false,
        }
        .visit_expr(&hir_body.value);
    }

    rewriter.write(mode)
}
