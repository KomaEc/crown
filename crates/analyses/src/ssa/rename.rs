use std::ops::{Index, IndexMut};

use rustc_index::{Idx, IndexVec};

use crate::ssa::SSAIdx;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Renamed<T> {
    item: T,
    ssa_idx: SSAIdx,
}

pub struct RenamedMap<K: Idx, V> {
    map: IndexVec<K, IndexVec<SSAIdx, V>>,
}

impl<K: Idx, V> Index<K> for RenamedMap<K, V> {
    type Output = IndexVec<SSAIdx, V>;

    fn index(&self, index: K) -> &Self::Output {
        &self.map[index]
    }
}

impl<K: Idx, V> IndexMut<K> for RenamedMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.map[index]
    }
}

impl<K: Idx, V> Index<Renamed<K>> for RenamedMap<K, V> {
    type Output = V;

    fn index(&self, index: Renamed<K>) -> &Self::Output {
        &self[index.item][index.ssa_idx]
    }
}

impl<K: Idx, V> IndexMut<Renamed<K>> for RenamedMap<K, V> {
    fn index_mut(&mut self, index: Renamed<K>) -> &mut Self::Output {
        &mut self[index.item][index.ssa_idx]
    }
}
