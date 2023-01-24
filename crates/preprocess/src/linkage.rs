use common::rewrite::{Rewrite, RewriteMode};
use petgraph::unionfind::UnionFind;
use rustc_hash::FxHashMap;
use rustc_hir::{
    def::{DefKind, Res},
    intravisit::{self, Visitor},
    Expr, ExprKind, ForeignItem, ForeignItemKind, ItemKind, Node, OwnerNode, Pat, PatKind, Path,
    QPath, Ty,
};
use rustc_middle::{hir::nested_filter::OnlyBodies, ty::TyCtxt};
use rustc_span::sym;

use crate::{owner_items, perform_rewrite};

pub fn link_functions(tcx: TyCtxt, mode: RewriteMode) {
    perform_rewrite(link_functions_internal, tcx, mode)
}

fn link_functions_internal(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    // (1) Find all `#[no_mangle]` or `#[export_name=...]` functions, and index them by symbol.
    let mut symbol_to_def = FxHashMap::default();
    for item in owner_items(tcx).filter(|item| matches!(item.kind, ItemKind::Fn(..))) {
        let hir_id = item.hir_id();
        let attrs = tcx.hir().attrs(hir_id);
        if attrs.iter().any(|attr| attr.has_name(sym::no_mangle)) {
            symbol_to_def.insert(item.ident.name, item.owner_id.def_id);
        } else if let Some(name) = attrs
            .iter()
            .find(|attr| attr.has_name(sym::export_name))
            .and_then(|attr| attr.value_str())
        {
            symbol_to_def.insert(name, item.owner_id.def_id);
        }
    }

    // (2) Find all extern fns, and index them by def_id.
    let mut extern_def_to_symbol = FxHashMap::default();
    for foreign_item in foreign_items(tcx)
        .filter(|foreign_item| matches!(foreign_item.kind, ForeignItemKind::Fn(..)))
    {
        extern_def_to_symbol.insert(foreign_item.owner_id.def_id, foreign_item.ident.name);
    }

    // (3) Adjust references to extern fns to refer to the `#[no_mangle]` definition instead.
    let mut vis = resolved_path_visitor(tcx, |path| {
        let Res::Def(DefKind::Fn, did) = path.res else { return };
        let Some(local_did) = did.as_local() else { return };
        let Some(symbol) = extern_def_to_symbol.get(&local_did) else { return };
        let Some(&did) = symbol_to_def.get(&symbol) else { return };

        let span = path.span;
        let replacement = "crate::".to_owned() + &tcx.def_path_str(did.to_def_id());
        rewriter.replace(tcx, span, replacement)
    });
    for item in owner_items(tcx) {
        vis.visit_item(item);
    }

    // (4) Remove unused externs
    for foreign_item in foreign_items(tcx)
        .filter(|foreign_item| matches!(foreign_item.kind, ForeignItemKind::Fn(..)))
    {
        // println!("try erasing {}", foreign_item.ident);
        let did = foreign_item.owner_id.def_id;
        // Drop any items that resolve to a symbol in another module.
        if let Some(symbol) = extern_def_to_symbol.get(&did) {
            if symbol_to_def.contains_key(&symbol) {
                let span = foreign_item.span;
                rewriter.erase(tcx, span);

                let hir_id = foreign_item.hir_id();
                let attrs = tcx.hir().attrs(hir_id);
                for attr in attrs {
                    let span = attr.span;
                    rewriter.erase(tcx, span)
                }
            }
        }
    }

    // todo!()
}

pub fn link_incomplete_types(tcx: TyCtxt, mode: RewriteMode) {
    perform_rewrite(link_incomplete_types_internal, tcx, mode)
}

fn link_incomplete_types_internal(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    let mut name_to_complete = FxHashMap::default();
    let mut incomplete_to_name = FxHashMap::default();

    // (1) Find complete type definitions, and index them by name.
    for item in owner_items(tcx) {
        let complete = match item.kind {
            ItemKind::Struct(..) => true,
            ItemKind::Union(..) => true,
            ItemKind::Enum(..) => true,
            ItemKind::TyAlias(..) => true,
            _ => false,
        };

        if complete {
            let def_id = item.owner_id.def_id.to_def_id();
            name_to_complete
                .entry(item.ident.name)
                .or_insert_with(Vec::new)
                .push(def_id);
        }
    }

    // (2) Find and remove incomplete type definitions (extern types), and index them by name.
    for foreign_item in foreign_items(tcx) {
        let incomplete = match foreign_item.kind {
            ForeignItemKind::Type => true,
            _ => false,
        };

        if incomplete {
            let def_id = foreign_item.owner_id.def_id.to_def_id();
            let name = foreign_item.ident.name;
            incomplete_to_name.insert(def_id, name);
        }
    }

    // (3) Erase incomplete types that have complete definitions; give those that do not a
    // representative.
    for (&incomplete, &name) in incomplete_to_name.iter() {
        // incomplete might be an extern type
        match name_to_complete.entry(name) {
            std::collections::hash_map::Entry::Occupied(_) => {
                let Node::ForeignItem(foreign_item) = tcx.hir().get_by_def_id(incomplete.expect_local()) else { unreachable!() };
                let span = foreign_item.span;
                rewriter.erase(tcx, span);

                let hir_id = foreign_item.hir_id();
                let attrs = tcx.hir().attrs(hir_id);
                for attr in attrs {
                    let span = attr.span;
                    rewriter.erase(tcx, span)
                }
            }
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(vec![incomplete]);
            }
        }
    }

    // (4) Replace references to incomplete types with references to same-named complete types.
    let mut vis = resolved_path_visitor(tcx, |path| {
        let Res::Def(_, maybe_incomplete) = path.res else { return };

        if let Some(&name) = incomplete_to_name.get(&maybe_incomplete) {
            if let Some(complete) = name_to_complete.get(&name) {
                if maybe_incomplete == complete[0] {
                    return;
                };
                let span = path.span;
                let replacement = "crate::".to_owned() + &tcx.def_path_str(complete[0]);
                rewriter.replace(tcx, span, replacement)
            }
        }
    });
    for item in owner_items(tcx) {
        vis.visit_item(item)
    }
    for foreign_item in foreign_items(tcx) {
        vis.visit_foreign_item(foreign_item)
    }
}

pub fn canonicalize_structs(tcx: TyCtxt, mode: RewriteMode) {
    perform_rewrite(canonicalize_structs_internal, tcx, mode)
}

fn canonicalize_structs_internal(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    // (1) Compute equivalent classes by name
    let mut structs = Vec::new();
    let mut struct_idx = FxHashMap::default();
    for (idx, item) in owner_items(tcx)
        .filter(|item| matches!(item.kind, ItemKind::Struct(..) | ItemKind::Union(..)))
        .enumerate()
    {
        let did = item.owner_id.def_id.to_def_id();
        struct_idx.insert(did, idx);
        structs.push(did);
    }

    if structs.is_empty() {
        return;
    }

    let mut equivalent_classes = UnionFind::new(structs.len());
    for (idx1, &s) in structs[..structs.len() - 1].iter().enumerate() {
        for idx2 in idx1 + 1..structs.len() {
            let t = structs[idx2];
            let s_symbol = tcx.item_name(s);
            let t_symbol = tcx.item_name(t);
            let s = tcx.adt_def(s);
            let t = tcx.adt_def(t);
            let maybe_equiv = s_symbol == t_symbol
                || s.is_union()
                    && t.is_union()
                    && s_symbol.as_str().starts_with("C2RustUnnamed")
                    && t_symbol.as_str().starts_with("C2RustUnnamed");
            if maybe_equiv {
                if s.all_fields().count() == t.all_fields().count()
                    && s.all_fields()
                        .zip(t.all_fields())
                        .all(|(f, g)| f.name == g.name)
                {
                    equivalent_classes.union(idx1, idx2);
                }
            }
        }
    }
    let equivalent_classes = equivalent_classes.into_labeling();

    // (3) Erase non-canonical structs
    let mut version = 0;
    for item in owner_items(tcx)
        .filter(|item| matches!(item.kind, ItemKind::Struct(..) | ItemKind::Union(..)))
    {
        // println!("try erasing {}", item.ident);
        let struct_idx = struct_idx[&item.owner_id.def_id.to_def_id()];
        if equivalent_classes[struct_idx] == struct_idx {
            continue;
        }
        let span = item.span;
        // println!("erased: {:?}", span);
        // rewriter.erase(tcx, span);
        rewriter.replace(
            tcx,
            span,
            format!("struct ErasedByPreprocessor{version} {{ dummy: () }}"),
        );
        version += 1;

        let hir_id = item.hir_id();
        let attrs = tcx.hir().attrs(hir_id);
        for attr in attrs {
            let span = attr.span;
            rewriter.erase(tcx, span)
        }
    }

    // (4) Rewrite references to canonical one
    let mut vis = resolved_path_visitor(tcx, |path| {
        let Res::Def(_, did) = path.res else { return };
        let Some(&struct_idx) = struct_idx.get(&did) else { return };
        let rep_idx = equivalent_classes[struct_idx];
        if rep_idx == struct_idx {
            return;
        }

        let rep_did = structs[rep_idx];

        let span = path.span;
        let replacement = "crate::".to_owned() + &tcx.def_path_str(rep_did);
        rewriter.replace(tcx, span, replacement)
    });
    for item in owner_items(tcx) {
        // skip automatically derived items
        let hir_id = item.hir_id();
        let attrs = tcx.hir().attrs(hir_id);
        if attrs
            .iter()
            .any(|attr| attr.has_name(sym::automatically_derived))
        {
            continue;
        }

        // skip erased structs
        if matches!(item.kind, ItemKind::Struct(..) | ItemKind::Union(..)) {
            let struct_idx = struct_idx[&item.owner_id.def_id.to_def_id()];
            if equivalent_classes[struct_idx] != struct_idx {
                continue;
            }
        }
        vis.visit_item(item)
    }
    for foreign_item in foreign_items(tcx) {
        vis.visit_foreign_item(foreign_item)
    }
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

// fn visit_resolved_paths(tcx: TyCtxt, callback: impl FnMut(&Path)) {

//     let mut vis = Vis { tcx, callback };
//     for item in items(tcx) {
//         vis.visit_item(item)
//     }
// }

fn resolved_path_visitor<'tcx, F>(tcx: TyCtxt, callback: F) -> ResolvedPathVisitor<'_, F>
where
    F: FnMut(&Path),
{
    ResolvedPathVisitor { tcx, callback }
}

struct ResolvedPathVisitor<'tcx, F> {
    tcx: TyCtxt<'tcx>,
    callback: F,
}
impl<'hir, F> Visitor<'hir> for ResolvedPathVisitor<'hir, F>
where
    F: FnMut(&Path),
{
    type NestedFilter = OnlyBodies;

    fn nested_visit_map(&mut self) -> Self::Map {
        self.tcx.hir()
    }

    fn visit_expr(&mut self, expr: &'hir Expr<'hir>) {
        match &expr.kind {
            ExprKind::Path(path) | &ExprKind::Struct(path, _, _) => {
                if let QPath::Resolved(_, path) = path {
                    (self.callback)(path)
                }
            }
            _ => {}
        }

        // if let ExprKind::Path(path) = &expr.kind {
        //     if let QPath::Resolved(_, path) = path {
        //         (self.callback)(path)
        //     }
        // }

        intravisit::walk_expr(self, expr)
    }

    fn visit_ty(&mut self, ty: &'hir Ty<'hir>) {
        if let rustc_hir::TyKind::Path(path) = &ty.kind {
            if let QPath::Resolved(_, path) = path {
                (self.callback)(path)
            }
        }

        intravisit::walk_ty(self, ty)
    }

    fn visit_pat(&mut self, p: &'hir Pat<'hir>) {
        if let PatKind::Path(path) = &p.kind {
            if let QPath::Resolved(_, path) = path {
                (self.callback)(path)
            }
        }

        intravisit::walk_pat(self, p)
    }
}
