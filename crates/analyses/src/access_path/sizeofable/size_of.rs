use std::cell::RefCell;

use rustc_index::IndexVec;
use rustc_middle::ty::TyCtxt;
use rustc_type_ir::TyKind::Adt;

use crate::access_path::{
    KLimited, peel_pointers,
    sizeofable::{IndirectionGraph, SizeOfable},
    struct_lookup::{StructIndex, StructLookup},
};

/// Compute the size of a composite type (currently only structs
/// are considered). Here the size does *not* mean the physical
/// memory size but the number of postfix pointers.
pub struct SizeOf {
    /// k_limit -> [`StructIndex`] -> size
    cache: RefCell<Vec<IndexVec<StructIndex, usize>>>,
    indirection_graph: IndirectionGraph,
}

impl SizeOfable for SizeOf {
    fn size_of(&self, struct_index: KLimited<StructIndex>) -> usize {
        while self.cache.borrow().len() <= struct_index.k_limit {
            self.induce();
        }

        self.cache.borrow()[struct_index.k_limit][struct_index.data]
    }

    fn start_offset(&self, struct_index: KLimited<StructIndex>, field_idx: usize) -> usize {
        self.indirection_graph[struct_index.data]
            .iter()
            .take(field_idx)
            .fold(0, |acc_offset, &(num_pointers, another_struct)| {
                let offset = if struct_index.k_limit <= num_pointers {
                    struct_index.k_limit
                } else if let Some(another_struct) = another_struct {
                    num_pointers
                        + self.size_of(KLimited::new(
                            struct_index.k_limit - num_pointers,
                            another_struct,
                        ))
                } else {
                    num_pointers
                };

                acc_offset + offset
            })
    }

    fn start_offsets(
        &self,
        struct_index: KLimited<StructIndex>,
    ) -> impl IntoIterator<Item = usize> {
        std::iter::once(0).chain(self.indirection_graph[struct_index.data].iter().scan(
            0,
            move |acc_offset, &(num_pointers, another_struct)| {
                let offset = if struct_index.k_limit <= num_pointers {
                    struct_index.k_limit
                } else if let Some(another_struct) = another_struct {
                    num_pointers
                        + self.size_of(KLimited::new(
                            struct_index.k_limit - num_pointers,
                            another_struct,
                        ))
                } else {
                    num_pointers
                };

                *acc_offset += offset;
                Some(*acc_offset)
            },
        ))
    }
}

impl SizeOf {
    pub fn new(struct_lookup: &StructLookup, tcx: TyCtxt) -> Self {
        let zero = IndexVec::from_elem_n(0, struct_lookup.num_structs());

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
                fields.push((
                    num_pointers,
                    ty.ty_adt_def().and_then(|adt_def| {
                        adt_def
                            .is_struct()
                            .then(|| struct_lookup.index(adt_def.did()))
                    }),
                ));
            }

            graph.push(fields);
        }

        Self {
            cache: RefCell::new(vec![zero]),
            indirection_graph: graph,
        }
    }

    pub fn induce(&self) {
        let k_limit = self.cache.borrow().len();
        let mut n = IndexVec::new();

        for fields in self.indirection_graph.iter() {
            let mut size = 0;

            for &(num_pointers, another_struct) in fields {
                if k_limit <= num_pointers {
                    size += k_limit;
                } else if let Some(another_struct) = another_struct {
                    if num_pointers == 0 {
                        size += num_pointers + n[another_struct]
                    } else {
                        size += num_pointers
                            + self.cache.borrow()[k_limit - num_pointers][another_struct]
                    }
                } else {
                    size += num_pointers;
                }
            }

            n.push(size);
        }

        self.cache.borrow_mut().push(n);
    }
}
