use rustc_ast::{
    ast::*,
    mut_visit::{self, MutVisitor},
};

use rustc_ast_pretty::pprust;

pub(crate) struct UnnecessaryRawMutRemover;

impl MutVisitor for UnnecessaryRawMutRemover {
    fn visit_expr(&mut self, ex: &mut Expr) -> Self::Result {
        mut_visit::walk_expr(self, ex);

        // * &raw mut x -> x
        if let ExprKind::Unary(UnOp::Deref, inner) = &ex.kind
            && let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, inner) = &inner.kind
        {
            *ex = *inner.clone();
        }

        // &raw mut &raw mut *x -> &raw mut x as *mut *mut _
        if let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, inner) = &ex.kind
            && let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, inner) = &inner.kind
            && let ExprKind::Unary(UnOp::Deref, inner) = &inner.kind
        {
            *ex = utils::expr!(
                "&raw mut ({}) as *mut *mut _",
                pprust::expr_to_string(&inner)
            );
        }

        // * f(&raw mut x) -> f(&mut x)
        if let ExprKind::Call(box _func_expr, args) = &mut ex.kind {
            for arg in args.iter_mut() {
                if let ExprKind::AddrOf(BorrowKind::Raw, Mutability::Mut, inner) = arg.clone().kind
                {
                    arg.kind = ExprKind::AddrOf(BorrowKind::Ref, Mutability::Mut, inner);
                }
            }
        }
    }
}
