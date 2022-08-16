//! Mapping a def_id (from a group of def_ids, say functions) to
//! a set of things. This set is represented by an interval of
//! indices.

use std::{marker::PhantomData, ops::Range};

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

pub trait InnerRep {
    // type Item<T>;
}

#[derive(Debug)]
pub struct Array;
#[derive(Debug)]
pub struct ArraySubPart;
#[derive(Debug)]
pub struct Singleton;

impl InnerRep for Array {
    // type Item<T> = T;
}

impl InnerRep for ArraySubPart {
    // type Item<T> = T;
}

impl InnerRep for Singleton {
    // type Item<T> = usize;
}

/// TODO: specialize for different inner rep
/// Discretisation of a set of things common to a group of def_ids.
///
/// # Example Usages
/// Discretise the set of all locals with pointer type of functions in a crate.
#[derive(Debug)]
pub struct ItemSet<I, Rep>
where
    Rep: InnerRep,
{
    belongers: FxHashMap<DefId, usize>,
    /// Sets of contents (represented by an interval of index `I`) of each belonger.
    items: Vec<I>,
    /// Indices of contents of each belonger. Pointers into `content_indices`
    item_indices_start: Vec<usize>,
    item_indices: Vec<usize>,
    _rep: PhantomData<*const Rep>,
}

impl<I, Rep> ItemSet<I, Rep>
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
    Rep: InnerRep,
{
    #[inline]
    pub fn new<'tcx, ItemHolder, F, G, S, P, It>(
        tcx: TyCtxt<'tcx>,
        dids: &[DefId],
        first_item: I,
        item_holder_iter: F,
        mut with_content: G,
        mut step: S,
        // to_step: P,
    ) -> Self
    where
        ItemHolder: 'tcx,
        F: Fn(TyCtxt<'tcx>, DefId) -> It,
        G: FnMut(I),
        S: FnMut(DefId) -> P,
        P: Fn(usize, ItemHolder) -> bool,
        It: Iterator<Item = ItemHolder>,
    {
        let belongers: FxHashMap<DefId, usize> = dids
            .iter()
            .enumerate()
            .map(|(idx, did)| (*did, idx))
            .collect();

        let mut items = Vec::with_capacity(belongers.len() + 1);
        items.push(first_item);
        let mut item_indices_start = Vec::with_capacity(belongers.len() + 1);
        item_indices_start.push(0);
        let mut item_indices = Vec::new();

        let mut next_item = first_item;
        let mut next_start = 0;

        for &did in dids {
            let to_step = step(did);

            let mut offset = 0;

            for (idx, holder) in item_holder_iter(tcx, did).enumerate() {
                item_indices.push(offset);
                if to_step(idx, holder) {
                    with_content(next_item);
                    next_item += 1;
                    offset += 1;
                }
                next_start += 1;
            }
            items.push(next_item);
            item_indices_start.push(next_start);
        }

        ItemSet {
            belongers,
            items,
            item_indices_start,
            item_indices,
            _rep: PhantomData,
        }
    }

    #[inline]
    pub fn belongers(&self) -> impl Iterator<Item = &DefId> {
        self.belongers.keys()
    }

    #[inline]
    pub fn has_entry(&self, belonger: DefId) -> bool {
        self.belongers.contains_key(&belonger)
    }

    #[inline]
    fn get_contents_inner(&self, inner_idx: usize) -> Range<I> {
        let start = self.items[inner_idx];
        let end = self.items[inner_idx + 1];
        Range { start, end }
    }

    #[inline]
    pub fn get_contents(&self, belonger: DefId) -> Range<I> {
        let inner_idx = self.belongers[&belonger];
        self.get_contents_inner(inner_idx)
    }

    #[inline]
    pub fn get_content(&self, belonger: DefId, idx: usize) -> I {
        println!("getting content {:?}:{idx}", belonger);
        let inner_idx = self.belongers[&belonger];
        let offset = self.item_indices[self.item_indices_start[inner_idx] + idx];
        let Range { start, end } = self.get_contents_inner(inner_idx);
        println!("range: {:?}~{:?}, offset: {offset}", start, end);
        assert!(start + (offset as u32) < end);
        start + (offset as u32)
    }
}
