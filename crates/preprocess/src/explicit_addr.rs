use common::rewrite::Rewrite;
use rustc_hir::{
    intravisit::{self, Visitor},
    BorrowKind, Expr, ExprKind, ItemKind, Mutability, OwnerNode,
};
use rustc_middle::ty::TyCtxt;

pub fn explicit_addr(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        let ItemKind::Fn(_, _, body_id) = item.kind else { continue };
        let hir_body = tcx.hir().body(body_id);
        ExplicitAddr { rewriter, tcx }.visit_expr(&hir_body.value);
    }
}

struct ExplicitAddr<'me, 'hir, R> {
    tcx: TyCtxt<'hir>,
    rewriter: &'me mut R,
}

impl<'me, 'hir, R> Visitor<'hir> for ExplicitAddr<'me, 'hir, R>
where
    R: Rewrite,
{
    fn visit_expr(&mut self, expr: &'hir Expr<'hir>) {
        if let ExprKind::AddrOf(BorrowKind::Ref, Mutability::Mut, inner_expr) = expr.kind {
            self.rewriter.replace(
                self.tcx,
                expr.span.until(inner_expr.span),
                "&raw mut ".to_owned(),
            );
            return intravisit::walk_expr(self, inner_expr);
        }

        intravisit::walk_expr(self, expr)
    }
}
