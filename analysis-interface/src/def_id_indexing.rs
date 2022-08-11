//! Mapping a def_id (from a group of def_ids, say functions) to
//! a set of things. This set is represented by an interval of
//! indices.

use std::marker::PhantomData;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;

/// Discretisation of a set of things common to a group of def_ids.
/// Note: if the set of things are well-indexed (say it is itself an `IndexVec`),
/// then it is not recommended to use this data structure.
///
/// # Example Usages
/// Discretise the set of all locals with pointer type of functions in a crate.
pub struct DefIdContentsIndexing<I, Content> {
    items: FxHashMap<DefId, usize>,
    contents: Vec<I>,
    content_indices_start: Vec<usize>,
    content_indices: Vec<usize>,
    _marker: PhantomData<*const Content>,
}
