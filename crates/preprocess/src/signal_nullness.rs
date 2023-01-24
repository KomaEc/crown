use common::rewrite::{Rewrite, RewriteMode};
use rustc_hir::{
    intravisit::{self, Visitor},
    Expr, ExprKind, ItemKind, LoopSource, OwnerNode, UnOp,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::BytePos;

use crate::perform_rewrite;

pub fn signal_nullness(tcx: TyCtxt, mode: RewriteMode) {
    perform_rewrite(signal_nullness_internal, tcx, mode)
}

fn signal_nullness_internal(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    // let mut rewriter = Vec::new(); //Rewriter::default();

    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        let ItemKind::Fn(_, _, body_id) = item.kind else { continue };
        let hir_body = tcx.hir().body(body_id);
        // println!("{}", rustc_hir_pretty::id_to_string(&tcx.hir(), item.hir_id()));
        // println!("body kind: {:?}", hir_body.value);
        SignalNullness {
            rewriter,
            tcx,
            in_while_loop: false,
        }
        .visit_expr(&hir_body.value);
    }
}

struct SignalNullness<'me, 'hir, R> {
    tcx: TyCtxt<'hir>,
    rewriter: &'me mut R,
    in_while_loop: bool,
}
impl<'me, 'hir, R> Visitor<'hir> for SignalNullness<'me, 'hir, R>
where
    R: Rewrite,
{
    fn visit_expr(&mut self, expr: &'hir Expr<'hir>) {
        match expr.kind {
            ExprKind::If(cond, truth_branch, false_branch) => {
                let is_while_loop_cond = self.in_while_loop;
                self.in_while_loop = false;
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
                    ExprKind::MethodCall(path, receiver, _, _)
                        if path.ident.as_str() == "is_null" =>
                    {
                        // let ptr = args.first().unwrap();
                        let ptr = receiver;

                        // currently we only rewrite variables not complex expressions
                        if
                        /* let ExprKind::Path(..) = ptr.kind */
                        true {
                            // rewrite is ensured, explicitly recurse into two branches
                            intravisit::walk_expr(self, truth_branch);
                            false_branch
                                .map(|false_branch| intravisit::walk_expr(self, false_branch));

                            let ptr_name =
                                rustc_hir_pretty::id_to_string(&self.tcx.hir(), ptr.hir_id);
                            let stmt_str =
                                // format!("std::intrinsics::assume({ptr_name} as usize == 0);");
                                format!("std::intrinsics::assume(({ptr_name}).addr() == 0);");

                            if sign {
                                self.insert_to_branch(stmt_str, truth_branch);
                            } else if !is_while_loop_cond {
                                // normal if { } else { }
                                if let Some(false_branch) = false_branch {
                                    // give up if { } else if { } ..
                                    if !matches!(false_branch.kind, ExprKind::If(..)) {
                                        self.insert_to_branch(stmt_str, false_branch);
                                    }
                                } else {
                                    let empty_span_after_curly_brace =
                                        truth_branch.span.shrink_to_hi();
                                    self.rewriter.replace(
                                        self.tcx,
                                        empty_span_after_curly_brace,
                                        "else { ".to_string() + &stmt_str + " }",
                                    )
                                }
                            } else {
                                // while !p.is_null() {}
                                // while loop always has false branch, to hold { break; }
                                // its span for some reason is the whole loop expression
                                let span = false_branch.unwrap().span.shrink_to_hi();
                                self.rewriter.replace(self.tcx, span, stmt_str);
                            }

                            return;
                        }
                    }
                    _ => {}
                }
            }
            ExprKind::Loop(_, _, LoopSource::While, _) => {
                self.in_while_loop = true;
            }
            _ => {}
        }

        intravisit::walk_expr(self, expr)
    }
}

impl<'me, 'hir, R> SignalNullness<'me, 'hir, R>
where
    R: Rewrite,
{
    fn insert_to_branch(&mut self, stmt_str: String, branch: &Expr) {
        let branch_span_lo = branch.span.lo();
        let empty_span_after_curly_brace = branch
            .span
            .with_lo(branch_span_lo + BytePos(1))
            .shrink_to_lo();
        self.rewriter
            .replace(self.tcx, empty_span_after_curly_brace, stmt_str)
    }
}
