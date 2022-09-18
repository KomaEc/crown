use orc_common::rewrite::Rewrite;
use petgraph::unionfind::UnionFind;
use rustc_hash::FxHashMap;
use rustc_hir::{
    def::{DefKind, Res},
    def_id::LocalDefId,
    intravisit::{self, Visitor},
    Expr, ExprKind, ForeignItem, ForeignItemKind, Item, ItemKind, OwnerNode, Pat, PatKind, Path,
    QPath, Ty,
};
use rustc_middle::{hir::nested_filter::OnlyBodies, ty::TyCtxt};
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
                                    let replacement =
                                        "crate::".to_owned() + &tcx.def_path_str(did.to_def_id());
                                    self.rewriter.replace(tcx, span, replacement)
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
        // println!("try erasing {}", foreign_item.ident);
        let did = foreign_item.def_id;
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

pub(crate) fn link_incomplete_types(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    let mut name_to_complete = FxHashMap::default();
    let mut incomplete_to_name = FxHashMap::default();

    // (1) Find complete type definitions, and index them by name.
    for item in items(tcx) {
        let complete = match item.kind {
            ItemKind::Struct(..) => true,
            ItemKind::Union(..) => true,
            ItemKind::Enum(..) => true,
            ItemKind::TyAlias(..) => true,
            _ => false,
        };

        if complete {
            let def_id = item.def_id.to_def_id();
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
            let def_id = foreign_item.def_id.to_def_id();
            incomplete_to_name.insert(def_id, foreign_item.ident.name);

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

    // (3) Replace references to incomplete types with references to same-named complete types.
    let mut vis = resolved_path_visitor(tcx, |path| {
        let Res::Def(_, did) = path.res else { return };

        if let Some(&name) = incomplete_to_name.get(&did) {
            if let Some(did) = name_to_complete.get(&name) {
                let span = path.span;
                let replacement = "crate::".to_owned() + &tcx.def_path_str(did[0]);
                rewriter.replace(tcx, span, replacement)
            }
        }
    });
    for item in items(tcx) {
        vis.visit_item(item)
    }
    for foreign_item in foreign_items(tcx) {
        vis.visit_foreign_item(foreign_item)
    }
}

pub(crate) fn canonicalize_structs(tcx: TyCtxt, rewriter: &mut impl Rewrite) {
    // (1) Compute equivalent classes by name
    let mut structs = Vec::new();
    let mut struct_idx = FxHashMap::default();
    for (idx, item) in items(tcx)
        .filter(|item| matches!(item.kind, ItemKind::Struct(..) | ItemKind::Union(..)))
        .enumerate()
    {
        let did = item.def_id.to_def_id();
        struct_idx.insert(did, idx);
        structs.push(did);
    }

    let mut equivalent_classes = UnionFind::new(structs.len());
    for (idx1, &s) in structs[..structs.len() - 1].iter().enumerate() {
        for idx2 in idx1 + 1..structs.len() {
            let t = structs[idx2];
            let s_symbol = tcx.item_name(s);
            let t_symbol = tcx.item_name(t);
            if s_symbol == t_symbol {
                equivalent_classes.union(idx1, idx2);
            }
        }
    }
    let equivalent_classes = equivalent_classes.into_labeling();

    // (3) Erase non-canonical structs
    let mut version = 0;
    for item in
        items(tcx).filter(|item| matches!(item.kind, ItemKind::Struct(..) | ItemKind::Union(..)))
    {
        // println!("try erasing {}", item.ident);
        let struct_idx = struct_idx[&item.def_id.to_def_id()];
        if equivalent_classes[struct_idx] == struct_idx {
            continue;
        }
        let span = item.span;
        // println!("erased: {:?}", span);
        // rewriter.erase(tcx, span);
        rewriter.replace(tcx, span, format!("struct OrcGeneratedXXX{version};"));
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
    for item in
        items(tcx).filter(|item| !matches!(item.kind, ItemKind::Struct(..) | ItemKind::Union(..)))
    {
        // skip automatically derived items
        let hir_id = item.hir_id();
        let attrs = tcx.hir().attrs(hir_id);
        if attrs
            .iter()
            .any(|attr| attr.has_name(sym::automatically_derived))
        {
            continue;
        }
        vis.visit_item(item)
    }
    for foreign_item in foreign_items(tcx) {
        vis.visit_foreign_item(foreign_item)
    }

    // let structs = items(tcx)
    //     .filter_map(|item| {
    //         matches!(item.kind, ItemKind::Struct(..)).then_some(item.def_id.to_def_id())
    //     })
    //     .collect::<Vec<_>>();

    // // (1) Build struct dependency
    // let struct_dependency = StructDependency::new(
    //     tcx,
    //     &structs[..],
    //     |field_ty, graph| {
    //         let mut ty = field_ty;
    //         loop {
    //             let index = ty.builtin_index();
    //             let deref = ty.builtin_deref(true).map(|ty_and_mut| ty_and_mut.ty);

    //             if let Some(inner_ty) = index.or(deref) {
    //                 ty = inner_ty
    //             } else {
    //                 break;
    //             }
    //         }
    //         if let TyKind::Adt(inner_adt_def, _) = ty.kind() {
    //             if graph.contains_node(inner_adt_def.did()) {
    //                 return Some(inner_adt_def.did());
    //             }
    //         }
    //         None
    //     },
    //     |field_def| field_def.ident(tcx),
    // );

    // // (2) Compute equivalent struct classes
    // let mut sccs = VecArray::with_data_capacity(structs.len());
    // TarjanScc::new().run(struct_dependency.graph(), |nodes| {
    //     println!("{:?}", nodes);
    //     sccs.push_array(nodes.iter().copied().map(|did| (did, tcx.item_name(did))))
    // });
    // let sccs = sccs.done();

    // let mut scc_idx = FxHashMap::default();
    // for (idx, scc) in sccs.iter().enumerate() {
    //     for &(did, _) in scc {
    //         scc_idx.insert(did, idx);
    //     }
    // }
    // let scc_idx = scc_idx;

    // let mut equivalent_classes = UnionFind::new(sccs.array_count());
    // for (idx1, scc1) in sccs.iter().enumerate() {
    //     for idx2 in idx1 + 1..sccs.array_count() {
    //         let scc2 = &sccs[idx2];
    //         if scc1.len() == scc2.len()
    //             && scc1
    //                 .iter()
    //                 .map(|(_, s)| s)
    //                 .zip(scc2.iter().map(|(_, s)| s))
    //                 .all(|(f, g)| *f == *g)
    //         {
    //             equivalent_classes.union(idx1, idx2);
    //         }
    //     }
    // }
    // let equivalent_classes = equivalent_classes.into_labeling();

    // // (3) Erase non-canonical structs
    // let mut version = 0;
    // for item in items(tcx).filter(|item| matches!(item.kind, ItemKind::Struct(..))) {
    //     // println!("try erasing {}", item.ident);
    //     let Some(&scc_idx) = scc_idx.get(&item.def_id.to_def_id()) else { unreachable!() };
    //     let rep_idx = equivalent_classes[scc_idx];
    //     if rep_idx == scc_idx {
    //         continue;
    //     }
    //     let span = item.span;
    //     // println!("erased: {:?}", span);
    //     // rewriter.erase(tcx, span);
    //     rewriter.replace(tcx, span, format!("struct OrcGeneratedXXX{version};"));
    //     version += 1;

    //     let hir_id = item.hir_id();
    //     let attrs = tcx.hir().attrs(hir_id);
    //     for attr in attrs {
    //         let span = attr.span;
    //         rewriter.erase(tcx, span)
    //     }
    // }

    // // (4) Rewrite references to canonical one
    // let mut vis = resolved_path_visitor(tcx, |path| {
    //     let Res::Def(DefKind::Struct, did) = path.res else { return };
    //     let Some(&scc_idx) = scc_idx.get(&did) else { return };
    //     let rep_idx = equivalent_classes[scc_idx];
    //     if rep_idx == scc_idx {
    //         return;
    //     }

    //     let name = tcx.item_name(did);
    //     let &rep_did = sccs[rep_idx]
    //         .iter()
    //         .find_map(|(did, symbol)| (*symbol == name).then_some(did))
    //         .unwrap();

    //     let span = path.span;
    //     let replacement = "crate::".to_owned() + &tcx.def_path_str(rep_did);
    //     rewriter.replace(tcx, span, replacement)
    // });
    // for item in items(tcx).filter(|item| !matches!(item.kind, ItemKind::Struct(..))) {
    //     // skip automatically derived items
    //     let hir_id = item.hir_id();
    //     let attrs = tcx.hir().attrs(hir_id);
    //     if attrs.iter().any(|attr| attr.has_name(sym::automatically_derived)) {
    //         continue
    //     }
    //     vis.visit_item(item)
    // }
    // for foreign_item in foreign_items(tcx) {
    //     vis.visit_foreign_item(foreign_item)
    // }

    // let names = sccs.repack(|(did, ident)| ident);
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
