//! Extra type information pre-computed to accelerate
//! access path construction, matching, etc.

use rustc_hir::def_id::DefId;
use rustc_middle::ty::Ty;
use utils::rustc::RustProgram;

use crate::access_path::{
    KLimited, peel_pointers,
    sizeofable::{SizeOfable, size_of::SizeOf, start_offset::StartOffset},
    struct_lookup::StructLookup,
};

pub struct AccessPathsCx<SizeOf = StartOffset> {
    pub(crate) struct_lookup: StructLookup,
    pub(crate) size_of: SizeOf,
}

type InefficientAccessPathsCx = AccessPathsCx<SizeOf>;

impl InefficientAccessPathsCx {
    pub fn new_inefficient(program: &RustProgram) -> Self {
        let struct_lookup = StructLookup::new(program);
        let size_of = SizeOf::new(&struct_lookup, program.tcx);

        Self {
            struct_lookup,
            size_of,
        }
    }
}

impl AccessPathsCx {
    pub fn new(program: &RustProgram) -> Self {
        let struct_lookup = StructLookup::new(program);
        let start_offset = StartOffset::new(program, &struct_lookup);

        Self {
            struct_lookup,
            size_of: start_offset,
        }
    }
}

impl<SizeOf: SizeOfable> AccessPathsCx<SizeOf> {
    pub fn size_of(&self, ty: KLimited<Ty>) -> usize {
        let (num_pointers, inner_ty) = peel_pointers(ty.data);

        if ty.k_limit <= num_pointers {
            return ty.k_limit;
        }

        let Some(struct_index) = inner_ty
            .ty_adt_def()
            .and_then(|adt_def| self.struct_lookup.try_index(adt_def.did()))
        else {
            return num_pointers;
        };

        num_pointers
            + self.size_of.size_of(KLimited {
                k_limit: ty.k_limit - num_pointers,
                data: struct_index,
            })
    }

    pub fn start_offset(&self, st: KLimited<DefId>, field_idx: usize) -> usize {
        let struct_index = st.map(|did| self.struct_lookup.index(did));
        self.size_of.start_offset(struct_index, field_idx)
    }

    pub fn start_offsets(&self, st: KLimited<DefId>) -> impl Iterator<Item = usize> {
        let struct_index = st.map(|did| self.struct_lookup.index(did));
        self.size_of.start_offsets(struct_index).into_iter()
    }
}
