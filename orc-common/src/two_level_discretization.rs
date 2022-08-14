//! Mapping a def_id (from a group of def_ids, say functions) to
//! a set of things. This set is represented by an interval of
//! indices.

use std::ops::Range;

use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::TyCtxt;

/// Discretisation of a set of things common to a group of def_ids.
/// Note: if the set of things are well-indexed (say it is itself an `IndexVec`),
/// then it is not recommended to use this data structure.
///
/// # Example Usages
/// Discretise the set of all locals with pointer type of functions in a crate.
#[derive(Debug)]
pub struct TwoLevelDiscretization<I> {
    belongers: FxHashMap<DefId, usize>,
    /// Sets of contents (represented by an interval of index `I`) of each belonger.
    contents: Vec<I>,
    /// Indices of contents of each belonger. Pointers into `content_indices`
    content_indices_start: Vec<usize>,
    content_indices: Vec<usize>,
}

impl<I> TwoLevelDiscretization<I>
where
    I: std::ops::AddAssign<u32>
        + std::ops::Add<u32, Output = I>
        + Clone
        + Copy
        + PartialOrd
        + Ord
        + PartialEq
        + Eq,
{
    #[inline]
    pub fn new<'tcx, ContentHolder, F, G, P, It>(
        tcx: TyCtxt<'tcx>,
        dids: &[DefId],
        first_content: I,
        content_holder_iter: F,
        mut with_content: G,
        to_step: P,
    ) -> Self
    where
        ContentHolder: 'tcx,
        F: Fn(TyCtxt<'tcx>, DefId) -> It,
        G: FnMut(I),
        P: Fn(TyCtxt<'tcx>, &ContentHolder) -> bool,
        It: Iterator<Item = &'tcx ContentHolder>,
    {
        let belongers: FxHashMap<DefId, usize> = dids
            .iter()
            .enumerate()
            .map(|(idx, did)| (*did, idx))
            .collect();

        let mut contents = Vec::with_capacity(belongers.len() + 1);
        contents.push(first_content);
        let mut content_indices_start = Vec::with_capacity(belongers.len() + 1);
        content_indices_start.push(0);
        let mut content_indices = Vec::new();

        for &did in dids {
            let mut content = unsafe { *contents.last().unwrap_unchecked() };
            let mut start = unsafe { *content_indices_start.last().unwrap_unchecked() };
            let mut content_index = 0;
            for holder in content_holder_iter(tcx, did) {
                with_content(content);
                content_indices.push(content_index);
                if to_step(tcx, holder) {
                    content += 1;
                    content_index += 1;
                }
                start += 1;
            }
            contents.push(content);
            content_indices_start.push(start);
        }

        TwoLevelDiscretization {
            belongers,
            contents,
            content_indices_start,
            content_indices,
        }
    }

    #[inline]
    fn get_contents_inner(&self, inner_idx: usize) -> Range<I> {
        let start = self.contents[inner_idx];
        let end = self.contents[inner_idx + 1];
        Range { start, end }
    }

    #[inline]
    pub fn get_contents(&self, belonger: DefId) -> Range<I> {
        let inner_idx = self.belongers[&belonger];
        self.get_contents_inner(inner_idx)
    }

    #[inline]
    pub fn get_content(&self, belonger: DefId, idx: usize) -> I {
        let inner_idx = self.belongers[&belonger];
        let offset = self.content_indices[self.content_indices_start[inner_idx] + idx];
        let Range { start, end } = self.get_contents_inner(inner_idx);
        assert!(start + (offset as u32) < end);
        start + (offset as u32)
    }
}
