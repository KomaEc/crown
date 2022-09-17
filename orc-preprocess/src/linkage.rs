use orc_common::rewrite::Rewrite;
use rustc_hash::FxHashMap;
use rustc_hir::{
    def::{DefKind, Res},
    def_id::LocalDefId,
    intravisit::{self, Visitor},
    Expr, ExprKind, ForeignItem, ForeignItemKind, Item, ItemKind, OwnerNode, QPath,
};
use rustc_middle::ty::TyCtxt;
use rustc_span::{sym, Symbol};

pub(crate) fn link_functions(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    // (1) Find all `#[no_mangle]` or `#[export_name=...]` functions, and index them by symbol.
    let mut symbol_to_def = FxHashMap::default();
    for item in items(tcx).filter(|item| matches!(item.kind, ItemKind::Fn(..))) {
        let hir_id = item.hir_id();
        let attrs = tcx.hir().attrs(hir_id);
        if attrs.iter().any(|attr| attr.has_name(sym::no_mangle)) {
            symbol_to_def.insert(item.ident.name, item.def_id);
        } else if let Some(name) = attrs
            .iter()
            .find(|attr| attr.has_name(sym::export_name))
            .and_then(|attr| attr.value_str())
        {
            symbol_to_def.insert(name, item.def_id);
        }
    }

    // (2) Find all extern fns, and index them by def_id.
    let mut extern_def_to_symbol = FxHashMap::default();
    for foreign_item in foreign_items(tcx)
        .filter(|foreign_item| matches!(foreign_item.kind, ForeignItemKind::Fn(..)))
    {
        extern_def_to_symbol.insert(foreign_item.def_id, foreign_item.ident.name);
    }

    // (3) Adjust references to extern fns to refer to the `#[no_mangle]` definition instead.
    struct Vis<'me, 'tcx, R: Rewrite> {
        tcx: TyCtxt<'tcx>,
        rewriter: &'me mut R,
        symbol_to_def: &'me mut FxHashMap<Symbol, LocalDefId>,
        extern_def_to_symbol: &'me mut FxHashMap<LocalDefId, Symbol>,
    }
    let mut vis = Vis {
        tcx,
        rewriter,
        symbol_to_def: &mut symbol_to_def,
        extern_def_to_symbol: &mut extern_def_to_symbol,
    };
    impl<'me, 'hir, R: Rewrite> Visitor<'hir> for Vis<'me, 'hir, R> {
        fn visit_expr(&mut self, expr: &'hir Expr<'hir>) {
            let tcx = self.tcx;

            if let ExprKind::Path(path) = &expr.kind {
                if let QPath::Resolved(_, path) = path {
                    if let Res::Def(DefKind::Fn, did) = path.res {
                        if let Some(did) = did.as_local() {
                            if let Some(symbol) = self.extern_def_to_symbol.get(&did) {
                                if let Some(&did) = self.symbol_to_def.get(&symbol) {
                                    let span = path.span;
                                    let replacement = tcx.def_path_str(did.to_def_id());
                                    self.rewriter.make_suggestion(
                                        tcx,
                                        span,
                                        "".to_owned(),
                                        replacement,
                                    )
                                }
                            }
                        }
                    }
                }
            }

            intravisit::walk_expr(self, expr)
        }
    }
    for item in items(tcx) {
        let ItemKind::Fn(_, _, body_id) = item.kind else { continue };
        let body = tcx.hir().body(body_id);
        intravisit::walk_body(&mut vis, body)
    }

    // (4) Remove unused externs
    for foreign_item in foreign_items(tcx)
        .filter(|foreign_item| matches!(foreign_item.kind, ForeignItemKind::Fn(..))) 
    {
        let did = foreign_item.def_id;
        // Drop any items that resolve to a symbol in another module.
        if let Some(symbol) = extern_def_to_symbol.get(&did) {
            if symbol_to_def.contains_key(&symbol) {
                let span = foreign_item.vis_span;
                rewriter.make_suggestion(tcx, span, "".to_owned(), "".to_owned())
            }
        }
    }

    // todo!()
}

pub(crate) fn link_incomplete_types(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    // todo!()
}

pub(crate) fn canonicalize_structs(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    // todo!()
}

fn items(tcx: TyCtxt) -> impl Iterator<Item = &'_ Item<'_>> {
    tcx.hir()
        .krate()
        .owners
        .iter()
        .filter_map(|maybe_owner| maybe_owner.as_owner())
        .filter_map(|owner| {
            if let OwnerNode::Item(item) = owner.node() {
                Some(item)
            } else {
                None
            }
        })
}

fn foreign_items(tcx: TyCtxt) -> impl Iterator<Item = &'_ ForeignItem<'_>> {
    tcx.hir()
        .krate()
        .owners
        .iter()
        .filter_map(|maybe_owner| maybe_owner.as_owner())
        .filter_map(|owner| {
            if let OwnerNode::ForeignItem(foreign_item) = owner.node() {
                Some(foreign_item)
            } else {
                None
            }
        })
}
