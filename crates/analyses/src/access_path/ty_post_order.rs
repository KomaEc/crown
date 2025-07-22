use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_middle::ty::TyKind;
use rustc_type_ir::TyKind::Adt;
use utils::{
    petgraph::{algo::TarjanScc, prelude::DiGraphMap},
    rustc::RustProgram,
    rustc_hash::FxHashMap,
};

use crate::mir::CxDefId;

rustc_index::newtype_index! {
    #[orderable]
    #[debug_format = "{}"]
    pub struct StructIndex {
    }
}

pub struct TyPostOrder {
    /// structs in post order
    post_order: IndexVec<StructIndex, DefId>,
    rev_lookup: FxHashMap<DefId, StructIndex>,
}

impl TyPostOrder {
    pub fn new(program: &RustProgram) -> Self {
        let &RustProgram {
            tcx, ref structs, ..
        } = program;

        let mut graph: DiGraphMap<CxDefId, ()> =
            DiGraphMap::with_capacity(structs.len(), structs.len());
        structs.iter().for_each(|did| {
            graph.add_node(CxDefId::new(tcx, *did));
        });
        for did in structs.iter() {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).skip_binder().kind() else {
                unreachable!("impossible")
            };
            assert!(adt_def.is_struct());
            for field_def in adt_def.all_fields() {
                let mut ty = field_def.ty(tcx, subst_ref);
                while let TyKind::Array(inner_ty, _) = ty.kind() {
                    ty = *inner_ty;
                }
                if let TyKind::Adt(field_adt_def, _) = ty.kind() {
                    let field_did = field_adt_def.did();
                    if graph.contains_node(CxDefId::new(tcx, field_did)) {
                        graph.add_edge(
                            CxDefId::new(tcx, *did),
                            CxDefId::new(tcx, field_adt_def.did()),
                            (),
                        );
                    }
                }
            }
        }

        let mut post_order: IndexVec<StructIndex, DefId> = IndexVec::with_capacity(structs.len());
        TarjanScc::new().run(&graph, |nodes| {
            post_order.extend(nodes.iter().map(|cx_did| cx_did.did))
        });

        let rev_map: FxHashMap<DefId, StructIndex> = post_order
            .iter_enumerated()
            .map(|(idx, &did)| (did, idx))
            .collect();

        TyPostOrder {
            post_order,
            rev_lookup: rev_map,
        }
    }
}
