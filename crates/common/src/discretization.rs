use std::ops::Range;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;

use crate::data_structure::vec_vec::VecVec;

pub struct Discretization<Idx> {
    pub did_idx: FxHashMap<DefId, usize>,
    /// [`DefId`] -> entity -> [`std::ops::Range<Idx>`]
    pub contents: VecVec<Idx>,
}

impl<Idx: Copy> Discretization<Idx> {
    #[inline]
    pub fn contents_iter(&self, did: &DefId) -> impl Iterator<Item = Range<Idx>> + '_ {
        let idx = self.did_idx[did];
        self.contents[idx]
            .array_windows()
            .map(|&[start, end]| start..end)
    }

    #[inline]
    pub fn content(&self, did: &DefId, idx: usize) -> Range<Idx> {
        let outer_idx = self.did_idx[did];
        self.contents[outer_idx][idx]..self.contents[outer_idx][idx + 1]
    }
}

pub struct StructFields<Idx>(pub Discretization<Idx>);
pub struct FnLocals<Idx>(pub Discretization<Idx>);

impl<Idx: Copy> StructFields<Idx> {
    /// [`fields()`] returns a slice of [`Range<T>`] that is in lock-step with [`all_fields()`]
    #[inline]
    pub fn fields(&self, did: &DefId) -> impl Iterator<Item = Range<Idx>> + '_ {
        self.0.contents_iter(did)
    }

    #[inline]
    pub fn field(&self, did: &DefId, f: usize) -> Range<Idx> {
        self.0.content(did, f)
    }
}

impl<Idx: Copy> FnLocals<Idx> {
    /// [`locals()`] returns a slice of [`Range<Var>`] that is in lock-step with [`local_decls`]
    /// #[inline]
    pub fn locals(&self, did: &DefId) -> impl Iterator<Item = Range<Idx>> + '_ {
        self.0.contents_iter(did)
    }

    #[inline]
    pub fn local(&self, did: &DefId, local: usize) -> Range<Idx> {
        self.0.content(did, local)
    }
}
