use std::ops::Range;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;

use crate::data_structure::vec_vec::VecVec;

pub struct Discretization<T> {
    pub did_idx: FxHashMap<DefId, usize>,
    /// [`DefId`] -> entity -> [`std::ops::Range<Var>`]
    pub contents: VecVec<T>,
}

impl<T: Copy> Discretization<T> {
    #[inline]
    pub fn contents_iter(&self, did: &DefId) -> impl Iterator<Item = Range<T>> + '_ {
        let idx = self.did_idx[did];
        self.contents[idx]
            .array_windows()
            .map(|&[start, end]| start..end)
    }

    #[inline]
    pub fn content(&self, did: &DefId, idx: usize) -> Range<T> {
        let outer_idx = self.did_idx[did];
        self.contents[outer_idx][idx]..self.contents[outer_idx][idx + 1]
    }
}

pub struct StructFields<T>(pub Discretization<T>);
pub struct FnLocals<T>(pub Discretization<T>);

impl<T: Copy> StructFields<T> {
    /// [`fields()`] returns a slice of [`Range<Var>`] that is in lock-step with [`all_fields()`]
    #[inline]
    pub fn fields(&self, did: &DefId) -> impl Iterator<Item = Range<T>> + '_ {
        self.0.contents_iter(did)
    }

    #[inline]
    pub fn field(&self, did: &DefId, f: usize) -> Range<T> {
        self.0.content(did, f)
    }
}

impl<T: Copy> FnLocals<T> {
    /// [`locals()`] returns a slice of [`Range<Var>`] that is in lock-step with [`local_decls`]
    /// #[inline]
    pub fn locals(&self, did: &DefId) -> impl Iterator<Item = Range<T>> + '_ {
        self.0.contents_iter(did)
    }

    #[inline]
    pub fn local(&self, did: &DefId, local: usize) -> Range<T> {
        self.0.content(did, local)
    }
}
