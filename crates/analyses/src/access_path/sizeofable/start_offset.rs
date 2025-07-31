use std::cell::{Ref, RefCell};

use rustc_index::IndexVec;
use utils::rustc::RustProgram;

use crate::access_path::{
    KLimited,
    sizeofable::SizeOfable,
    struct_lookup::{IndirectionGraph, IsIndirectionGraph, StructIndex, StructLookup},
};

pub struct StartOffset {
    cache: RefCell<StartOffsetData>,
    indirection_graph: IndirectionGraph,
}

pub struct StartOffsetData {
    /// struct_idx -> offset_start..offset_end
    structs_indices: IndexVec<StructIndex, usize>,
    /// compactly represents k_limit -> offsets
    offsets: Vec<usize>,
    offsets_size: usize,
}

impl SizeOfable for StartOffset {
    fn size_of(&self, struct_index: KLimited<StructIndex>) -> usize {
        while self.next_k_limit() <= struct_index.k_limit {
            self.induce();
        }

        unsafe { self.size_of_unchecked(struct_index) }
    }

    fn start_offset(&self, struct_index: KLimited<StructIndex>, field_idx: usize) -> usize {
        while self.next_k_limit() <= struct_index.k_limit {
            self.induce();
        }

        unsafe { self.start_offset_unchecked(struct_index, field_idx) }
    }

    fn start_offsets(
        &self,
        struct_index: KLimited<StructIndex>,
    ) -> impl IntoIterator<Item = usize> {
        while self.next_k_limit() <= struct_index.k_limit {
            self.induce();
        }

        unsafe { self.start_offsets_unchecked(struct_index) }
            .iter()
            .copied()
            .collect::<Vec<_>>()
    }
}

impl StartOffset {
    unsafe fn size_of_unchecked(&self, struct_index: KLimited<StructIndex>) -> usize {
        let cache = self.cache.borrow();
        let end = cache.structs_indices[struct_index.data + 1] - 1;
        cache.offsets[cache.offsets_size * struct_index.k_limit + end]
    }

    unsafe fn start_offset_unchecked(
        &self,
        struct_index: KLimited<StructIndex>,
        field_idx: usize,
    ) -> usize {
        let cache = self.cache.borrow();
        let start = cache.structs_indices[struct_index.data];
        let end = cache.structs_indices[struct_index.data + 1];
        cache.offsets[cache.offsets_size * struct_index.k_limit + start
            ..cache.offsets_size * struct_index.k_limit + end][field_idx]
    }

    unsafe fn start_offsets_unchecked<'a>(
        &'a self,
        struct_index: KLimited<StructIndex>,
    ) -> Ref<'a, [usize]> {
        let cache = self.cache.borrow();
        let start = cache.structs_indices[struct_index.data];
        let end = cache.structs_indices[struct_index.data + 1];
        Ref::map(cache, |cache| {
            &cache.offsets[cache.offsets_size * struct_index.k_limit + start
                ..cache.offsets_size * struct_index.k_limit + end]
        })
    }

    fn next_k_limit(&self) -> usize {
        let cache = self.cache.borrow();
        assert_eq!(cache.offsets.len() % cache.offsets_size, 0);
        let k_limit = cache.offsets.len() / cache.offsets_size;

        k_limit
    }

    pub fn new(program: &RustProgram, struct_lookup: &StructLookup) -> Self {
        let graph = IndirectionGraph::new_indirection_graph(struct_lookup, program.tcx);

        let mut structs_indices = IndexVec::with_capacity(struct_lookup.num_structs() + 1);
        let mut offsets = Vec::new();
        structs_indices.push(offsets.len());
        for (_, &def_id) in struct_lookup.post_order() {
            let ty = program.tcx.type_of(def_id).skip_binder();
            let adt_def = ty.ty_adt_def().unwrap();
            assert!(adt_def.is_struct());

            offsets.push(0);
            for _ in adt_def.all_fields() {
                offsets.push(0);
            }

            structs_indices.push(offsets.len());
        }
        let offsets_size = offsets.len();

        let start_offset_data = StartOffsetData {
            offsets_size,
            structs_indices,
            offsets,
        };

        StartOffset {
            cache: RefCell::new(start_offset_data),
            indirection_graph: graph,
        }
    }

    pub fn induce(&self) {
        let k_limit = self.next_k_limit();

        for fields in self.indirection_graph.iter() {
            let mut offset = 0;
            self.cache.borrow_mut().offsets.push(offset);

            for &(num_pointers, another_struct) in fields {
                if k_limit <= num_pointers {
                    offset += k_limit;
                } else if let Some(another_struct) = another_struct {
                    offset += num_pointers
                        + unsafe {
                            self.size_of_unchecked(KLimited::new(
                                k_limit - num_pointers,
                                another_struct,
                            ))
                        };
                } else {
                    offset += num_pointers;
                }

                self.cache.borrow_mut().offsets.push(offset);
            }
        }
    }
}
