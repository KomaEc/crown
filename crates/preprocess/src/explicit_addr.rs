use rustc_hir::{
    BorrowKind, Expr, ExprKind, ItemKind, Mutability, OwnerNode,
    intravisit::{self, Visitor},
};
use rustc_middle::ty::TyCtxt;
use utils::rewrite::Rewrite;

pub fn explicit_addr(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    for maybe_owner in tcx.hir_crate(()).owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else {
            continue;
        };
        let OwnerNode::Item(item) = owner.node() else {
            continue;
        };
        let ItemKind::Fn { body, .. } = item.kind else {
            continue;
        };
        let hir_body = tcx.hir_body(body);
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
