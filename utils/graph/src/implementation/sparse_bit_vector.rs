use crate::{DirectedGraph, GraphSuccessors, WithNumEdges, WithNumNodes, WithSuccessors};
use index::{
    bit_set::{HybridBitSet, HybridIter},
    vec::{Idx, IndexVec},
};

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
        let mut edges: IndexVec<N, HybridBitSet<N>> = IndexVec::with_capacity(num_nodes);
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
}
