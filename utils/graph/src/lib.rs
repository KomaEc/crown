#![feature(min_specialization)]
#![feature(rustc_private)]

extern crate rustc_index;
extern crate rustc_serialize;

use rustc_index::vec::Idx;

pub mod implementation;
pub mod scc;

// #[cfg(test)]
// mod tests;

pub trait DirectedGraph {
    type Node: Idx;
}

pub trait WithNumNodes: DirectedGraph {
    fn num_nodes(&self) -> usize;
}

pub trait WithNumEdges: DirectedGraph {
    fn num_edges(&self) -> usize;
}

pub trait WithSuccessors: DirectedGraph
where
    Self: for<'graph> GraphSuccessors<'graph, Item = <Self as DirectedGraph>::Node>,
{
    fn successors(&self, node: Self::Node) -> <Self as GraphSuccessors<'_>>::Iter;

    /*
    fn depth_first_search(&self, from: Self::Node) -> iterate::DepthFirstSearch<'_, Self>
    where
        Self: WithNumNodes,
    {
        iterate::DepthFirstSearch::new(self, from)
    }
    */
}

// #[allow(unused_lifetimes)]
pub trait GraphSuccessors<'graph> {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
}

pub trait WithPredecessors: DirectedGraph
where
    Self: for<'graph> GraphPredecessors<'graph, Item = <Self as DirectedGraph>::Node>,
{
    fn predecessors(&self, node: Self::Node) -> <Self as GraphPredecessors<'_>>::Iter;
}

// #[allow(unused_lifetimes)]
pub trait GraphPredecessors<'graph> {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
}

pub trait WithStartNode: DirectedGraph {
    fn start_node(&self) -> Self::Node;
}
