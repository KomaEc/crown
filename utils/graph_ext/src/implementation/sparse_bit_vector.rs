#[cfg(test)]
mod tests;

use rustc_data_structures::graph::{
    DirectedGraph, GraphSuccessors, WithNumEdges, WithNumNodes, WithSuccessors,
};
use rustc_index::{
    bit_set::{HybridBitSet, HybridIter},
    vec::{Idx, IndexVec},
};

#[derive(Clone)]
pub struct SparseBitVectorGraph<N: Idx> {
    edges: IndexVec<N, HybridBitSet<N>>,
}

impl<N: Idx> DirectedGraph for SparseBitVectorGraph<N> {
    type Node = N;
}

impl<N: Idx> WithNumNodes for SparseBitVectorGraph<N> {
    fn num_nodes(&self) -> usize {
        self.edges.len()
    }
}

impl<N: Idx> WithNumEdges for SparseBitVectorGraph<N> {
    fn num_edges(&self) -> usize {
        self.edges.iter().map(|bs| bs.iter().count()).sum()
    }
}

impl<'graph, N: Idx> GraphSuccessors<'graph> for SparseBitVectorGraph<N> {
    type Item = N;

    type Iter = HybridIter<'graph, N>;
}

impl<N: Idx> WithSuccessors for SparseBitVectorGraph<N> {
    fn successors(&self, node: N) -> <Self as GraphSuccessors<'_>>::Iter {
        self.edges[node].iter()
    }
}

impl<N: Idx> SparseBitVectorGraph<N> {
    pub fn new(num_nodes: usize, edge_pairs: impl Iterator<Item = (N, N)>) -> Self {
        let mut edges: IndexVec<N, HybridBitSet<N>> =
            IndexVec::from_elem_n(HybridBitSet::new_empty(num_nodes), num_nodes);
        for (src, dst) in edge_pairs {
            edges[src].insert(dst);
        }
        SparseBitVectorGraph { edges }
    }

    /// Return [`true`] if changed
    /// #[inline]
    pub fn add_edge(&mut self, src: N, dst: N) -> bool {
        self.edges[src].insert(dst)
    }

    pub fn has_edge(&self, src: N, dst: N) -> bool {
        self.edges[src].contains(dst)
    }

    #[inline]
    pub fn successor_nodes(&self, src: N) -> &HybridBitSet<N> {
        &self.edges[src]
    }

    #[inline]
    pub fn successor_nodes_mut(&mut self, src: N) -> &mut HybridBitSet<N> {
        &mut self.edges[src]
    }

    #[inline]
    pub fn pick2_successor_nodes_mut(
        &mut self,
        n1: N,
        n2: N,
    ) -> (&mut HybridBitSet<N>, &mut HybridBitSet<N>) {
        self.edges.pick2_mut(n1, n2)
    }

    pub fn collect_edges(&self) -> Vec<(N, N)> {
        self.edges
            .clone()
            .into_iter_enumerated()
            .flat_map(|(src, dests)| dests.iter().map(|dest| (src, dest)).collect::<Vec<_>>())
            .collect()
    }

    pub fn reverse(&self) -> Self {
        Self::new(
            self.num_nodes(),
            self.collect_edges()
                .into_iter()
                .map(|(src, dest)| (dest, src)),
        )
    }
}
