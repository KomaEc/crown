use rustc_hir::{
    intravisit::{self, NestedVisitorMap, Visitor},
    FieldDef, ItemKind, OwnerNode,
};
use rustc_interface::interface::Compiler;
use rustc_middle::{hir::map::Map, ty::TyCtxt};

use crate::CompilerRunnable;

pub struct StructRewriteVisitor<'tcx> {
    pub tcx: TyCtxt<'tcx>,
}

impl<'tcx> Visitor<'tcx> for StructRewriteVisitor<'tcx> {
    type Map = Map<'tcx>;

    fn nested_visit_map(&mut self) -> NestedVisitorMap<Self::Map> {
        return NestedVisitorMap::OnlyBodies(self.tcx.hir());
    }

    fn visit_field_def(&mut self, field_def: &'tcx FieldDef<'tcx>) {
        intravisit::walk_field_def(self, field_def)
    }
}

pub struct CollectStructInfo;

impl CompilerRunnable for CollectStructInfo {
    type Output = ();

    fn run(compiler: &Compiler) -> Self::Output {
        compiler.enter(|queries| {
            // let ast_krate = queries.parse().unwrap().take();
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                // Every compilation contains a single crate.
                let hir_krate = tcx.hir().krate();
                let mut structs = Vec::new();
                for (did, owner) in hir_krate.owners.iter_enumerated() {
                    if let Some(owner_info) = owner {
                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let ItemKind::Struct(variant_data, _generics) = &item.kind {
                                let def_id = item.def_id;
                                assert_eq!(def_id, did);

                                println!("{:?}", variant_data);

                                structs.push(did)
                            }
                        }
                    }
                }
            })
        })
    }
}
