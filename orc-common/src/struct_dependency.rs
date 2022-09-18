use petgraph::prelude::DiGraphMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_type_ir::TyKind::Adt;

pub struct StructDependency {
    graph: DiGraphMap<DefId, ()>,
}

impl StructDependency {
    pub fn new<F>(tcx: TyCtxt, structs: &[DefId], callback: F) -> Self
    where
        F: Fn(Ty, &DiGraphMap<DefId, ()>) -> Option<DefId>,
    {
        let mut graph = DiGraphMap::with_capacity(structs.len(), structs.len());
        structs.iter().for_each(|did| {
            graph.add_node(*did);
        });
        for &did in structs.iter() {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());
            for field_def in adt_def.all_fields() {
                let ty = field_def.ty(tcx, subst_ref);
                if let Some(inner_did) = callback(ty, &graph) {
                    assert!(graph.contains_node(did));
                    graph.add_edge(did, inner_did, ());
                }
            }
        }
        StructDependency { graph }
    }

    #[inline]
    pub fn graph(&self) -> &DiGraphMap<DefId, ()> {
        &self.graph
    }
}
