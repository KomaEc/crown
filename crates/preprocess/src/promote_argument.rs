use common::rewrite::{RewriteMode, Rewrite, get_snippet};
use rustc_hir::{OwnerNode, ItemKind, intravisit::{Visitor, self}, Expr, ExprKind, QPath, HirId, def::Res, Node, PatKind, UnOp};
use rustc_middle::ty::{TyCtxt, TypeckResults};
use rustc_span::symbol::Ident;

use crate::perform_rewrite;


pub fn promote_argument(tcx: TyCtxt, mode: RewriteMode) {
    perform_rewrite(promote_argument_impl, tcx, mode)
}

fn promote_argument_impl(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        let ItemKind::Fn(_, _, body_id) = item.kind else { continue };
        let hir_body = tcx.hir().body(body_id);
        // println!("{}", rustc_hir_pretty::id_to_string(&tcx.hir(), item.hir_id()));
        // println!("body kind: {:?}", hir_body.value);

        let typeck = tcx.typeck(item.owner_id.def_id);

        let mut vis = Promote { tcx, typeck, promoted_identifier: 0, rewriter };
        vis.visit_body(hir_body);
    }
}

struct Promote<'me, 'hir, R> {
    tcx: TyCtxt<'hir>,
    typeck: &'hir TypeckResults<'hir>,
    promoted_identifier: usize,
    rewriter: &'me mut R,
}

impl<R: Rewrite> Promote<'_, '_, R> {
    fn promoted_store(&mut self) -> String {
        let res = format!("crown_promoted_local_{}", self.promoted_identifier);
        self.promoted_identifier += 1;
        res
    }
}

impl<'me, 'hir, R: Rewrite> Visitor<'hir> for Promote<'me, 'hir, R> {
    fn visit_expr(&mut self, expr: &'hir Expr<'hir>) {
        match expr.kind {
            ExprKind::Call(_, xs) => {
                let index = find_promotable(self.tcx, self.typeck, xs);
                if index != 0 {
                    // println!("{}", rustc_hir_pretty::id_to_string(&self.tcx.hir(), expr.hir_id));

                    let promoted_store = self.promoted_store();
                    let expr_to_be_promoted = get_snippet(self.tcx, xs[index].span).text.1;
                    self.rewriter.replace(self.tcx, expr.span.shrink_to_lo(), format!("{{let {} = {};", &promoted_store, expr_to_be_promoted));
                    self.rewriter.replace(self.tcx, xs[index].span, promoted_store);
                    self.rewriter.replace(self.tcx, expr.span.shrink_to_hi(), "}".to_owned());
                }
            }
            ExprKind::Assign(lhs, rhs, span) if matches!(lhs.kind, ExprKind::Index(..)) => {
                let ExprKind::Index(a, index) = lhs.kind else { unreachable!() };
                if let Some(x) = find_base_ident(self.tcx, a) {
                    if is_present(self.tcx, x, index) {
                        // println!("expression to be promoted: {}", rustc_hir_pretty::id_to_string(&self.tcx.hir(), index.hir_id));
                        // println!("{}", rustc_hir_pretty::id_to_string(&self.tcx.hir(), expr.hir_id));
                    }
                }
            }
            _ => intravisit::walk_expr(self, expr),
        }
    }
}

fn find_promotable<'hir>(tcx: TyCtxt<'hir>, typeck: &'hir TypeckResults<'hir>, args: &'hir [Expr<'hir>]) -> usize {
    for (index, [arg1, arg2]) in args.array_windows().enumerate() {
        if may_conflict(tcx, typeck, arg1, arg2) {
            // println!("expression to be promoted: {}", rustc_hir_pretty::id_to_string(&tcx.hir(), arg2.hir_id));
            return index + 1;
        }
    }
    0
}

fn may_conflict<'hir>(tcx: TyCtxt<'hir>, typeck: &'hir TypeckResults<'hir>, x: &'hir Expr<'hir>, y: &'hir Expr<'hir>) -> bool {
    if typeck.expr_ty(x).is_any_ptr() && potentially_borrowed(x) && typeck.expr_ty(y).is_primitive() {
        if let Some(x) = find_base_ident(tcx, x) {
            return is_present(tcx, x, y)
        }
    }
    false
}

fn find_base_ident<'hir>(tcx: TyCtxt<'hir>, expr: &Expr<'hir>) -> Option<Ident> {
    match &expr.kind {
        ExprKind::Path(path) => {
            // println!("getting name for {}", rustc_hir_pretty::id_to_string(&tcx.hir(), expr.hir_id));
            get_name(tcx, path)
        }
        ExprKind::Field(x, _) => find_base_ident(tcx, x),
        ExprKind::Unary(unop, x) if matches!(unop, UnOp::Deref) => find_base_ident(tcx, x),
        ExprKind::AddrOf(_, _, x) => find_base_ident(tcx, x),
        _ => None
    }
}

fn get_name<'hir>(tcx: TyCtxt<'hir>, path: &QPath<'hir>) -> Option<Ident> {
    if let QPath::Resolved(_, path) = *path {
        if let Res::Local(hir_id) = path.res {
            let node = tcx.hir().get(hir_id);
            // println!("getting node {}", rustc_hir_pretty::id_to_string(&tcx.hir(), hir_id));
            // println!("debug node: {:?}", node);
            let Node::Pat(pat) = node else { unreachable!() };
            if let PatKind::Binding(_, _, ident, _) = pat.kind {
                return Some(ident)
            }
        }
    }
    None
}

fn potentially_borrowed(expr: &Expr) -> bool {
    match &expr.kind {
        ExprKind::Path(..) => true,
        ExprKind::AddrOf(..) => true,
        _ => false
    }
}

fn is_present<'hir>(tcx: TyCtxt<'hir>, ident: Ident, expr: &'hir Expr<'hir>) -> bool {
    let mut vis = Exists {
        tcx,
        ident,
        answer: false,
    };
    vis.visit_expr(expr);
    vis.answer
}

struct Exists<'hir> {
    tcx: TyCtxt<'hir>,
    ident: Ident,
    answer: bool,
}

impl<'hir> Visitor<'hir> for Exists<'hir> {
    fn visit_qpath(&mut self, qpath: &'hir QPath<'hir>, _: HirId, _: rustc_span::Span) {
        self.answer |= matches!(get_name(self.tcx, qpath), Some(ident) if self.ident == ident);
    }
}

