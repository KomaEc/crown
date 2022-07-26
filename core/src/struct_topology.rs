use graph_ext::implementation::sparse_bit_vector::{self, SparseBitVectorGraph};
use rustc_data_structures::graph::iterate::post_order_from;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::ty::{TyCtxt, TyKind};
use rustc_type_ir::TyKind::Adt;

use crate::analysis::place_ext::place_abs::AggregateOffset;

pub struct StructTopology {
    /// Structs in post order of the dependency graph.
    /// Dependency graph encodes direct dependencies between user defined structs.
    /// For instance, in `struct S { f: T } struct T;`
    /// `S` is considered dependent on `T`;
    /// in `struct S { f: *mut T } struct T;`
    /// `S` is not considered dependent on `T`
    post_order: Vec<DefId>,
    /// struct -> field -> aggregate offset of this field
    aggregate_offset: FxHashMap<DefId, Vec<AggregateOffset>>,
}

impl StructTopology {
    pub fn new(tcx: TyCtxt, structs: Vec<DefId>) -> Self {
        // structs.sort();
        let structs = IndexVec::from_raw(structs);
        let graph: SparseBitVectorGraph<u32> = sparse_bit_vector::SparseBitVectorGraph::new(
            structs.len(),
            structs.iter_enumerated().flat_map(|(sid, &did)| {
                let adt_def = tcx.adt_def(did);
                assert!(adt_def.is_struct());
                let structs_ref = &structs;
                adt_def.all_fields().filter_map(move |field_def| {
                    let ty = tcx.type_of(field_def.did);
                    if let TyKind::Adt(sub_adt_def, _) = ty.kind() {
                        return structs_ref
                            .binary_search(&sub_adt_def.did())
                            .map(|sub_sid| (sid, sub_sid))
                            .ok();
                    }
                    None
                })
            }),
        );

        let mut visited = BitSet::new_empty(structs.len());
        let mut post_order = Vec::with_capacity(structs.len());
        for sid in structs.indices() {
            if !visited.contains(sid) {
                // struct dep graph must be acyclic
                debug_assert!(
                    rustc_data_structures::graph::iterate::TriColorDepthFirstSearch::new(&graph)
                        .run_from(
                            sid,
                            &mut rustc_data_structures::graph::iterate::CycleDetector
                        )
                        .is_none()
                );

                let mut nodes = post_order_from(&graph, sid)
                    .into_iter()
                    .filter_map(|sid| visited.insert(sid).then(|| structs[sid]))
                    .collect();
                post_order.append(&mut nodes);
            }
        }
        assert_eq!(post_order.len(), structs.len());

        let mut aggregate_offset = FxHashMap::default();

        for &did in &post_order {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).kind() else { unreachable!("impossible") };
            assert!(adt_def.is_struct());

            let mut offset = AggregateOffset::from_u32(0);

            let offsets = adt_def
                .all_fields()
                .map(|field_def| {
                    offset = offset
                        + match field_def.ty(tcx, subst_ref).kind() {
                            TyKind::RawPtr(..) | TyKind::Ref(..) => 1,
                            TyKind::Adt(sub_adt_def, _) if sub_adt_def.is_box() => 1,
                            // non-user defined structs are ignored
                            TyKind::Adt(..) if {
                                if cfg!(debug_assertions) {
                                    let is_local = field_def.did.is_local();
                                    assert_eq!(is_local, !post_order.contains(&field_def.did));
                                    is_local
                                } else {
                                    field_def.did.is_local()
                                }
                            } => 0,
                            TyKind::Adt(..) => {
                                let offsets: &Vec<AggregateOffset> = aggregate_offset
                                    .get(&field_def.did)
                                    .expect("sub-structs should have been initialised");
                                offsets.last().map(|offset| offset.as_usize()).unwrap_or(0)
                            }
                            _ => 0,
                        };
                    offset
                })
                .collect();

            aggregate_offset.insert(did, offsets);
        }

        StructTopology {
            post_order,
            aggregate_offset,
        }
    }

    #[inline]
    pub fn structs_in_post_order(&self) -> &[DefId] {
        &self.post_order[..]
    }

    // #[inline]
    // pub fn aggregate_offset(&self) -> &FxHashMap<DefId, Vec<AggregateOffset>> {
    //     &self.aggregate_offset
    // }

    #[inline]
    pub fn struct_offset(&self, did: &DefId) -> Option<AggregateOffset> {
        self.aggregate_offset
            .get(did)?
            // .expect("expect user defined top-level struct")
            .last()
            .map(|&offset| offset)
            .or(Some(0u32.into()))
    }

    #[inline]
    pub fn field_offsets(&self, did: &DefId) -> Option<&[AggregateOffset]> {
        self.aggregate_offset.get(did).map(|vec| &vec[..])
        //.expect("expect user defined top-level struct")[..]
    }
}

#[cfg(test)]
mod tests {}
