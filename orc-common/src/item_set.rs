//! Mapping a def_id (from a group of def_ids, say functions) to
//! a set of things. This set is represented by an interval of
//! indices.

use std::{marker::PhantomData, ops::Range};

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

pub trait InnerRep<I> {
    // type Item<T>;
    type ItemsRep = Vec<I>;
}

#[derive(Debug)]
pub struct Array;
#[derive(Debug)]
pub struct ArraySubPart;

impl<I> InnerRep<I> for ArraySubPart {}

impl<I> InnerRep<I> for Array {}

// pub trait Mode {

//     fn reset_with<I>(old: &mut I, new: I);
// }
// pub struct NoReset;
// pub struct Reset;

// impl Mode for NoReset {
//     fn reset_with<I>(offset: &mut I, new: I) {}
// }
// impl Mode for Reset {
//     fn reset_with(offset: &mut usize) {
//         todo!()
//     }
// }

/// TODO: specialize for different inner rep
/// Discretisation of a set of things common to a group of def_ids.
///
/// # Example Usages
/// Discretise the set of all locals with pointer type of functions in a crate.
#[derive(Debug)]
pub struct ItemSet<I> {
    belongers: FxHashMap<DefId, usize>,
    /// Sets of contents (represented by an interval of index `I`) of each belonger.
    items: Vec<I>,
    /// Indices of contents of each belonger. Pointers into `content_indices`
    offset_of_start: Vec<usize>,
    /// If step width is fixed, then this field can be optimized away
    offset_of: Vec<usize>,
    // _rep: PhantomData<*const Rep>,
}

impl<I> ItemSet<I>
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
        let mut offset_of_start = Vec::with_capacity(belongers.len() + 1);
        offset_of_start.push(0);
        let mut offset_of = Vec::new();

        let mut next_item = first_item;
        let mut offset_of_index = 0;

        // println!("go");

        for &did in dids {
            let step_size = step(did);

            let mut offset = 0;

            offset_of.push(offset);

            let mut n_holders = 0;
            for (idx, holder) in item_holder_iter(tcx, did).enumerate() {
                // offset_of.push(offset);
                if step_size(idx, holder) {
                    with_content(next_item);
                    next_item += 1;
                    offset += 1;
                }
                offset_of.push(offset);
                // offset_of_index += 1;
                n_holders += 1;
            }
            // println!("{:?}", offset_of);
            items.push(next_item);
            offset_of_index += n_holders + 1;
            offset_of_start.push(offset_of_index);
        }
        // println!("{:?}", offset_of_for_belonger);

        ItemSet {
            belongers,
            items: items.into(),
            offset_of_start,
            offset_of,
            // _rep: PhantomData,
        }

        // for &did in dids {
        //     // println!("go {:?}", did);
        //     let to_step = step(did);
        //     for (idx, holder) in item_holder_iter(tcx, did).enumerate() {
        //         if to_step(idx, holder) {
        //             let _ = this.get_content(did, idx);
        //         }
        //     }
        // }

        // this
    }

    #[inline]
    pub fn belongers(&self) -> impl Iterator<Item = &DefId> {
        self.belongers.keys()
    }

    #[inline]
    pub fn has_entry(&self, belonger: DefId) -> bool {
        self.belongers.contains_key(&belonger)
    }
}

impl<I> ItemSet<I>
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
    pub fn iter(&self) -> impl Iterator<Item = (&DefId, Range<I>)> {
        self.belongers
            .iter()
            .map(|(did, &idx)| (did, self.get_contents_inner(idx)))
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
    fn get_offset_of_inner(&self, inner_idx: usize) -> &[usize] {
        let start = self.offset_of_start[inner_idx];
        let end = self.offset_of_start[inner_idx + 1];
        &self.offset_of[start..end]
    }

    // #[inline]
    // pub fn get_offset_of(&self, belonger: DefId) -> &[usize] {
    //     let inner_idx = self.belongers[&belonger];
    //     self.get_offset_of_inner(inner_idx)
    // }

    #[inline]
    pub fn get_singleton_content(&self, belonger: DefId, idx: usize) -> I {
        let inner_idx = self.belongers[&belonger];
        let offset_of = self.get_offset_of_inner(inner_idx);
        let offset = offset_of[idx];
        assert_eq!(offset_of[idx + 1], offset + 1);
        let start = self.items[inner_idx];
        start + offset as u32
    }
}
