pub(crate) mod nth_indirection;

use std::cell::Ref;

use rustc_middle::ty::{Ty, TyCtxt};
use utils::either::Either::{Left, Right};

use crate::access_path::{
    KLimited, ctxt::AccessPathsCx, peel_pointers, sizeofable::SizeOfable,
    struct_lookup::StructIndex,
};

impl<SizeOf: SizeOfable> AccessPathsCx<SizeOf> {
    fn size_of_aux(&self, decomposed: KLimited<(usize, Option<StructIndex>)>) -> usize {
        let KLimited { k_limit, data } = decomposed;
        let (num_pointers, struct_index) = data;

        if k_limit <= num_pointers {
            k_limit
        } else if let Some(struct_index) = struct_index {
            num_pointers
                + self
                    .size_of
                    .size_of(KLimited::new(k_limit - num_pointers, struct_index))
        } else {
            num_pointers
        }
    }

    fn nth_indirections(
        &self,
        struct_index: KLimited<StructIndex>,
    ) -> Ref<'_, [(usize, Option<StructIndex>, usize)]> {
        self.nth_indirection
            .nth_indirections(struct_index, &self.size_of)
    }

    pub fn lift(&self, ty: KLimited<Ty>, delta: usize, tcx: TyCtxt) -> impl Iterator<Item = usize> {
        let target_k_limit = ty.k_limit + delta;
        let (num_pointers, inner_ty) = peel_pointers(ty.data);

        if num_pointers >= ty.k_limit {
            return Left(0..ty.k_limit);
        }

        let Some(struct_index) = inner_ty
            .ty_adt_def()
            .and_then(|adt_def| self.struct_lookup.try_index(adt_def.did()))
        else {
            return Left(0..std::cmp::min(num_pointers, target_k_limit));
        };

        let leaves = self.nth_indirections(KLimited::new(ty.k_limit - num_pointers, struct_index));

        let leaves = leaves
            .iter()
            .map(move |&(leaf_num_pointers, leaf_struct, position)| {
                (
                    position + num_pointers,
                    self.size_of_aux(KLimited::new(delta, (leaf_num_pointers, leaf_struct))),
                )
            })
            .collect::<Vec<_>>();

        let mut leaves = leaves.into_iter().peekable();

        Right(
            (0..self.size_of(ty, tcx))
                .enumerate()
                .scan(0, move |state, (index, offset)| {
                    let position = offset + *state;
                    while let Some(&(position, size)) = leaves.peek()
                        && position == index
                    {
                        let _ = leaves.next();
                        *state += size;
                    }
                    Some(position)
                }),
        )
    }
}
