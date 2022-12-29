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
    pub fn contents(&self, did: &DefId) -> impl Iterator<Item = Range<T>> + '_ {
        let idx = self.did_idx[did];
        self.contents[idx]
            .array_windows()
            .map(|&[start, end]| start..end)
    }
}

pub struct StructFields<T>(pub Discretization<T>);
pub struct FnLocals<T>(pub Discretization<T>);

impl<T: Copy> StructFields<T> {
    /// [`fields()`] returns a slice of [`Range<Var>`] that is in lock-step with [`all_fields()`]
    pub fn fields(&self, did: &DefId) -> impl Iterator<Item = Range<T>> + '_ {
        self.0.contents(did)
    }
}

impl<T: Copy> FnLocals<T> {
    /// [`locals()`] returns a slice of [`Range<Var>`] that is in lock-step with [`local_decls`]
    pub fn locals(&self, did: &DefId) -> impl Iterator<Item = Range<T>> + '_ {
        self.0.contents(did)
    }
}
