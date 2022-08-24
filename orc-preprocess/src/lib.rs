#![feature(rustc_private)]
#![feature(let_else)]

extern crate rustc_hir;
extern crate rustc_middle;

use rustc_hir::{OwnerNode, ItemKind, ExprKind, StmtKind, Block};
use rustc_middle::ty::TyCtxt;

pub fn preprocess(tcx: TyCtxt) {
    insert_null_statement(tcx)
}

fn insert_null_statement_for_block(tcx: TyCtxt, block: &Block) {
    for stmt in block.stmts {
        let StmtKind::Expr(expr) = stmt.kind else { break };
        match expr.kind {
            ExprKind::Block(inner_block, _) => insert_null_statement_for_block(tcx, inner_block),
            ExprKind::If(cond, truth_branch, false_branch) => {
                println!("{:?}", cond);
            }
            _ => {}
        }
    }
}

fn insert_null_statement(tcx: TyCtxt) {
    let mut rewriter = orc_common::rewriter::Rewriter::default();

    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        let ItemKind::Fn(_, _, body_id) = item.kind else { continue };
        let hir_body = tcx.hir().body(body_id);
        let ExprKind::Block(block, _) = hir_body.value.kind else { continue };
        insert_null_statement_for_block(tcx, block)
    }
}
