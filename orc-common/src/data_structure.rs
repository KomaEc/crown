//! Mapping a def_id (from a group of def_ids, say functions) to
//! a set of things. This set is represented by an interval of
//! indices.

use std::{marker::PhantomData, ops::Range};

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

/// A frozen, allocation-avoiding representation of `Vec<Vec<I>>`
#[derive(Debug)]
pub struct FrozenVecVec<I> {
    l1_indexing: Vec<usize>,
    l2_indexing: Vec<I>,
}

impl<I> std::ops::Index<usize> for FrozenVecVec<I> {
    type Output = [I];

    fn index(&self, index: usize) -> &Self::Output {
        let end = self.l1_indexing[index + 1];
        let start = self.l1_indexing[index];
        &self.l2_indexing[start..end]
    }
}

impl<I> std::ops::IndexMut<usize> for FrozenVecVec<I> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let end = self.l1_indexing[index + 1];
        let start = self.l1_indexing[index];
        &mut self.l2_indexing[start..end]
    }
}

impl<I> FrozenVecVec<I> {
    pub fn new(len: usize) -> FrozenVecVecConstruction<I> {
        let mut l1_indexing = Vec::with_capacity(len + 1);
        l1_indexing.push(0);
        let l2_indexing = Vec::new();
        let frozen_vec_vec = FrozenVecVec {
            l1_indexing,
            l2_indexing,
        };
        FrozenVecVecConstruction {
            frozen_vec_vec,
            l1_index: 0,
            n_cur_items: 0,
        }
    }
}

#[derive(Debug)]
pub struct FrozenVecVecConstruction<I> {
    frozen_vec_vec: FrozenVecVec<I>,
    l1_index: usize,
    n_cur_items: usize,
}

impl<I> FrozenVecVecConstruction<I> {
    pub fn reserve_item(&mut self, item: I) {
        self.frozen_vec_vec.l2_indexing.push(item);
        self.n_cur_items += 1;
    }

    pub fn push_items(&mut self) {
        self.l1_index += std::mem::take(&mut self.n_cur_items);
        self.frozen_vec_vec.l1_indexing.push(self.l1_index);
    }

    #[inline]
    pub fn done(self) -> FrozenVecVec<I> {
        self.frozen_vec_vec
    }
}

pub trait IsOffsetOfConstruction {
    type OffsetOf: std::fmt::Debug = ();
    fn new(len: usize) -> Self;
    #[inline]
    fn reserve_offset(&mut self, _: usize) {}
    #[inline]
    fn push_offsets(&mut self) {}
    fn done(self) -> Self::OffsetOf;
}

impl IsOffsetOfConstruction for FrozenVecVecConstruction<usize> {
    type OffsetOf = FrozenVecVec<usize>;

    #[inline]
    fn new(len: usize) -> Self {
        FrozenVecVec::new(len)
    }

    #[inline]
    fn reserve_offset(&mut self, offset: usize) {
        self.reserve_item(offset)
    }

    #[inline]
    fn push_offsets(&mut self) {
        self.push_items();
    }

    #[inline]
    fn done(self) -> Self::OffsetOf {
        self.done()
    }

}

impl IsOffsetOfConstruction for () {
    #[inline]
    fn new(_: usize) -> Self {}
    #[inline]
    fn done(self) -> Self::OffsetOf {}
}

pub trait Step {
    type L2Items<I>;
    type OffSetOfConstruction: std::fmt::Debug + IsOffsetOfConstruction =
        FrozenVecVecConstruction<usize>;
    type StepSize = usize;
    fn size(size: Self::StepSize) -> usize;
    fn l2_items<I>(items: Range<I>) -> Self::L2Items<I>;
}
pub trait NotFixed {}
#[derive(Debug)]
pub struct Arbitrary;
impl Step for Arbitrary {
    type L2Items<I> = Range<I>;
    
    type OffSetOfConstruction =
        FrozenVecVecConstruction<usize>;

    type StepSize = usize;

    #[inline]
    fn l2_items<I>(items: Range<I>) -> Self::L2Items<I> {
        items
    }


    #[inline]
    fn size(size: Self::StepSize) -> usize {
        size
    }
}
impl NotFixed for Arbitrary {}
#[derive(Debug)]
pub struct FixedSize<const SIZE: usize>;
impl<const SIZE: usize> Step for FixedSize<SIZE> {
    type L2Items<I> = I;

    type OffSetOfConstruction = ();

    type StepSize = ();

    #[inline]
    fn l2_items<I>(items: Range<I>) -> Self::L2Items<I> {
        items.start
    }

    #[inline]
    fn size((): Self::StepSize) -> usize {
        SIZE
    }
}
#[derive(Debug)]
pub struct Maybe;
impl Step for Maybe {
    type L2Items<I> = I;

    type StepSize = bool;
    
    type OffSetOfConstruction =
        FrozenVecVecConstruction<usize>;

    #[inline]
    fn l2_items<I>(items: Range<I>) -> Self::L2Items<I> {
        items.start
    }


    #[inline]
    fn size(size: Self::StepSize) -> usize {
        size as usize
    }
}
impl NotFixed for Maybe {}

pub trait ResetAcrossDefId {
    fn reset<I>(item: &mut I, to_be: I);
}
#[derive(Debug)]
pub struct Reset;
impl ResetAcrossDefId for Reset {
    #[inline]
    fn reset<I>(item: &mut I, to_be: I) {
        *item = to_be
    }
}
#[derive(Debug)]
pub struct Contiguous;
impl ResetAcrossDefId for Contiguous {
    #[inline]
    fn reset<I>(_: &mut I, _: I) {}
}

/// Allocation-avoiding representation of `FxHashMap<DefId, Vec<Vec<I>>>`
/// where `I` is some index type. A consecutive set of `I` is represented
/// as `Range<I>`.
/// TODO: specialise for `FixedSize`
#[derive(Debug)]
pub struct HashMapDefIdVecRange<I, St: Step = Arbitrary, Re: ResetAcrossDefId = Contiguous> {
    fx_hash_map: FxHashMap<DefId, usize>,
    /// Sets of contents (represented by an interval of index `I`) of each did.
    items: Vec<I>,
    // /// Indices of contents of each def_id. Pointers into `content_indices`
    // offset_of_start: Vec<usize>,
    // offset_of: Vec<usize>,
    /// This can be optimised away if `FixedSize`
    // offset_of: FrozenVecVec<usize>,
    offset_of: <<St as Step>::OffSetOfConstruction as IsOffsetOfConstruction>::OffsetOf,
    _marker: PhantomData<*const (St, Re)>,
}

impl<I, St, Re> HashMapDefIdVecRange<I, St, Re>
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
    St: Step,
    Re: ResetAcrossDefId,
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
        G: FnMut(St::L2Items<I>),
        S: FnMut(DefId) -> P,
        P: Fn(usize, ItemHolder) -> St::StepSize,
        It: Iterator<Item = ItemHolder>,
    {
        let fx_hash_map: FxHashMap<DefId, usize> = dids
            .iter()
            .enumerate()
            .map(|(idx, did)| (*did, idx))
            .collect();

        let mut items = Vec::with_capacity(fx_hash_map.len() + 1);
        items.push(first_item);
        // let mut offset_of_start = Vec::with_capacity(fx_hash_map.len() + 1);
        // offset_of_start.push(0);
        // let mut offset_of = Vec::new();

        let mut offset_of =
            St::OffSetOfConstruction::new(
                fx_hash_map.len(),
            );

        let mut next_item = first_item;
        // let mut offset_of_index = 0;

        // println!("go");

        for &did in dids {
            Re::reset(&mut next_item, first_item);

            let step_size = step(did);

            let mut offset = 0;

            // offset_of.push(offset);

            offset_of.reserve_offset(offset);

            // let mut n_holders = 0;
            for (idx, holder) in item_holder_iter(tcx, did).enumerate() {
                let size = St::size(step_size(idx, holder));
                if size > 0 {
                    with_content(St::l2_items(Range {
                        start: next_item,
                        end: next_item + size as u32,
                    }));
                }
                next_item += size as u32;
                offset += size;
                // if step_size(idx, holder) {
                //     with_content(St::l2_items(Range { start: next_item, end: next_item + 1 }));
                //     // with_content(next_item);
                //     next_item += 1;
                //     offset += 1;
                // }
                // offset_of.push(offset);
                offset_of.reserve_offset(offset);
                // n_holders += 1;
            }
            // println!("{:?}", offset_of.frozen_vec_vec.l2_indexing);
            // println!("{:?}", offset_of);
            items.push(next_item);
            // offset_of_index += n_holders + 1;
            // offset_of_start.push(offset_of_index);
            offset_of.push_offsets();
        }
        let offset_of = offset_of.done();
        // println!("{:?}", offset_of.frozen_vec_vec.l1_indexing);
        // println!("{:?}", offset_of_start);

        HashMapDefIdVecRange {
            fx_hash_map,
            items,
            // offset_of_start,
            // offset_of,
            offset_of,
            _marker: PhantomData,
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
            .map(|(did, &idx)| (did, self.level_one_items_inner(idx)))
    }

    #[inline]
    fn level_one_items_inner(&self, did_idx: usize) -> Range<I> {
        let start = self.items[did_idx];
        let end = self.items[did_idx + 1];
        Range { start, end }
    }

    #[inline]
    pub fn level_one_items(&self, did: DefId) -> Range<I> {
        let did_idx = self.fx_hash_map[&did];
        self.level_one_items_inner(did_idx)
    }
}

impl<I, Re> HashMapDefIdVecRange<I, Arbitrary, Re>
where
    I: std::ops::Add<u32, Output = I>
        + Clone
        + Copy
        + PartialOrd,
    Re: ResetAcrossDefId,
{
    #[inline]
    pub fn get_items(&self, did: DefId, idx: usize) -> Range<I> {
        let did_idx = self.fx_hash_map[&did];
        let offset_of = &self.offset_of[did_idx];
        let level_one_start = self.items[did_idx];
        let start = level_one_start + offset_of[idx] as u32;
        let end = level_one_start + offset_of[idx + 1] as u32;
        assert!(start < end);
        Range { start, end }
    }
}

impl<I, Re> HashMapDefIdVecRange<I, Maybe, Re>
where
    I: std::ops::Add<u32, Output = I>
        + Clone
        + Copy
        + PartialEq,
    Re: ResetAcrossDefId,
{
    #[inline]
    pub fn try_get_item(&self, did: DefId, idx: usize) -> Option<I> {
        let did_idx = self.fx_hash_map[&did];
        let offset_of = &self.offset_of[did_idx];
        let offset = offset_of[idx];
        if offset_of[idx + 1] == offset + 1 {
            let start = self.items[did_idx];
            Some(start + offset as u32)
        } else {
            None
        }
    }

    /// Panic if item does not exist
    #[inline]
    pub fn get_item(&self, did: DefId, idx: usize) -> I {
        self.try_get_item(did, idx).unwrap()
    }
}

impl<I, const N: usize, Re> HashMapDefIdVecRange<I, FixedSize<N>, Re>
where
    I: std::ops::Add<u32, Output = I>
        + Clone
        + Copy,
    Re: ResetAcrossDefId,
{
    #[inline]
    pub fn get_item(&self, did: DefId, idx: usize) -> I {
        let did_idx = self.fx_hash_map[&did];
        let start = self.items[did_idx];
        start + (idx * N) as u32
    }
}
