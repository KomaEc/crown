use petgraph::prelude::DiGraphMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{FieldDef, Ty, TyCtxt};
use rustc_type_ir::TyKind::Adt;

pub struct StructDependency<Field> {
    graph: DiGraphMap<DefId, Field>,
}

impl<Field> StructDependency<Field> {
    pub fn new<F, G>(tcx: TyCtxt, structs: &[DefId], on_field: F, from_field: G) -> Self
    where
        F: Fn(Ty, &DiGraphMap<DefId, Field>) -> Option<DefId>,
        G: Fn(&FieldDef) -> Field,
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
                if let Some(inner_did) = on_field(ty, &graph) {
                    assert!(graph.contains_node(did));
                    graph.add_edge(did, inner_did, from_field(field_def));
                }
            }
        }
        StructDependency { graph }
    }

    #[inline]
    pub fn graph(&self) -> &DiGraphMap<DefId, Field> {
        &self.graph
    }
}
