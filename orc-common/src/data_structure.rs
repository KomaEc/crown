//! Mapping a def_id (from a group of def_ids, say functions) to
//! a set of things. This set is represented by an interval of
//! indices.

pub mod assoc;
pub mod vec_array;

use std::ops::Range;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;

#[derive(Debug)]
pub struct DidItemsIndexing<I> {
    fx_hash_map: FxHashMap<DefId, usize>,
    indices: Vec<I>,
}

impl<I> DidItemsIndexing<I>
where
    I: std::ops::AddAssign<u32>
        + std::ops::Add<u32, Output = I>
        + Clone
        + Copy
        + std::fmt::Debug
        + PartialOrd
        + Ord
        + PartialEq
        + Eq,
{
    #[inline]
    pub fn new<'tcx>(
        dids: &[DefId],
        first_item: I,
        item_num: impl Fn(&DefId) -> usize,
        mut with_content: impl FnMut(I),
    ) -> Self
    {
        let fx_hash_map: FxHashMap<DefId, usize> = dids
            .iter()
            .enumerate()
            .map(|(idx, did)| (*did, idx))
            .collect();

        let mut items = Vec::with_capacity(fx_hash_map.len() + 1);
        items.push(first_item);

        let mut next_item = first_item;

        for did in dids {
            for _ in 0..item_num(did) {
                with_content(next_item);
                next_item += 1;
            }
            items.push(next_item);
        }

        DidItemsIndexing { fx_hash_map, indices: items }
    }

    #[inline]
    pub fn dids(&self) -> impl Iterator<Item = &DefId> {
        self.fx_hash_map.keys()
    }

    #[inline]
    pub fn has_entry(&self, belonger: DefId) -> bool {
        self.fx_hash_map.contains_key(&belonger)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&DefId, Range<I>)> {
        self.fx_hash_map
            .iter()
            .map(|(did, &idx)| (did, self.get_indices_inner(idx)))
    }

    #[inline]
    fn get_indices_inner(&self, did_idx: usize) -> Range<I> {
        let start = self.indices[did_idx];
        let end = self.indices[did_idx + 1];
        Range { start, end }
    }

    #[inline]
    pub fn get_indices(&self, did: DefId) -> Range<I> {
        let did_idx = self.fx_hash_map[&did];
        self.get_indices_inner(did_idx)
    }

    #[inline]
    pub fn get_index(&self, did: DefId, idx: usize) -> I {
        let did_idx = self.fx_hash_map[&did];
        let start = self.indices[did_idx];
        start + idx as u32
    }
}
