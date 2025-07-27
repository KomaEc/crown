use std::cell::{Ref, RefCell};

use rustc_index::IndexVec;
use rustc_middle::ty::TyCtxt;
use rustc_type_ir::TyKind::Adt;

use crate::access_path::{
    KLimited, peel_pointers,
    sizeofable::{IndirectionGraph, SizeOfable},
    struct_lookup::{StructIndex, StructLookup},
};

/// Expanding access path starting from an expression of type
/// [`StructIndex`], after exactly n pointer chasing, the leaf
/// nodes `(num_pointers, Option<StructIndex>)` plus additionally
/// the position information (given `k_limit` = `n`)
pub struct NthIndirectionGraph {
    cache: RefCell<NthIndirectionGraphData>,
    indirection_graph: IndirectionGraph,
}

pub struct NthIndirectionGraphData {
    structs_indices: Vec<usize>,
    leaves: Vec<(usize, Option<StructIndex>, usize)>,
    num_structs_plus_one: usize,
}

impl NthIndirectionGraph {
    pub fn nth_indirections(
        &self,
        struct_index: KLimited<StructIndex>,
        size_of: &impl SizeOfable,
    ) -> Ref<'_, [(usize, Option<StructIndex>, usize)]> {
        while self.next_k_limit() <= struct_index.k_limit {
            self.induce(size_of);
        }

        unsafe { self.nth_indirections_unchecked(struct_index) }
    }

    fn next_k_limit(&self) -> usize {
        let cache = self.cache.borrow();
        assert_eq!(cache.structs_indices.len() % cache.num_structs_plus_one, 0);
        let k_limit = cache.structs_indices.len() / cache.num_structs_plus_one;
        k_limit
    }

    unsafe fn nth_indirections_unchecked(
        &self,
        struct_index: KLimited<StructIndex>,
    ) -> Ref<'_, [(usize, std::option::Option<StructIndex>, usize)]> {
        let cache = self.cache.borrow();

        Ref::map(cache, |cache| {
            &cache.leaves[cache.structs_indices
                [struct_index.data.as_usize() + cache.num_structs_plus_one * struct_index.k_limit]
                ..cache.structs_indices[struct_index.data.as_usize()
                    + cache.num_structs_plus_one * struct_index.k_limit
                    + 1]]
        })
    }

    pub fn new(struct_lookup: &StructLookup, tcx: TyCtxt) -> Self {
        let mut structs_indices = Vec::with_capacity(struct_lookup.num_structs() + 1);
        let mut leaves = vec![];

        structs_indices.push(leaves.len());
        for _ in struct_lookup.post_order() {
            structs_indices.push(leaves.len());
        }
        let mut graph = IndexVec::new();

        for (_, did) in struct_lookup.post_order() {
            let Adt(adt_def, subst_ref) = tcx.type_of(did).skip_binder().kind() else {
                unreachable!("impossible")
            };
            assert!(adt_def.is_struct());

            let mut fields = vec![];

            for field_def in adt_def.all_fields() {
                let ty = field_def.ty(tcx, subst_ref);
                let (num_pointers, ty) = peel_pointers(ty);

                let struct_index = ty.ty_adt_def().and_then(|adt_def| {
                    adt_def
                        .is_struct()
                        .then(|| struct_lookup.index(adt_def.did()))
                });
                fields.push((num_pointers, struct_index));
                leaves.push((num_pointers, struct_index, 0));
            }
            graph.push(fields);
        }

        let leaves_data = NthIndirectionGraphData {
            num_structs_plus_one: struct_lookup.num_structs() + 1,
            structs_indices,
            leaves,
        };

        NthIndirectionGraph {
            cache: RefCell::new(leaves_data),
            indirection_graph: graph,
        }
    }

    pub fn induce(&self, size_of: &impl SizeOfable) {
        let k_limit = self.next_k_limit();
        let leaves_len = self.cache.borrow().leaves.len();
        self.cache.borrow_mut().structs_indices.push(leaves_len);

        let mut buffer = vec![];
        for (struct_index, fields) in self.indirection_graph.iter_enumerated() {
            for (&(num_pointers, another_struct), start_offset) in fields
                .iter()
                .zip(size_of.start_offsets(KLimited::new(k_limit, struct_index)))
            {
                if num_pointers >= k_limit {
                    self.cache.borrow_mut().leaves.push((
                        num_pointers - k_limit,
                        another_struct,
                        start_offset + k_limit - 1,
                    ))
                } else if let Some(another_struct) = another_struct {
                    buffer.extend_from_slice(&unsafe {
                        self.nth_indirections_unchecked(KLimited::new(
                            k_limit - num_pointers,
                            another_struct,
                        ))
                    });
                    for (leaf_num_pointers, leaf_struct, position) in buffer.drain(0..) {
                        self.cache.borrow_mut().leaves.push((
                            leaf_num_pointers,
                            leaf_struct,
                            start_offset + num_pointers + position,
                        ))
                    }
                }
            }

            let leaves_len = self.cache.borrow().leaves.len();
            self.cache.borrow_mut().structs_indices.push(leaves_len);
        }
    }
}
