pub mod implementation;


pub trait DirectedGraph {
    type Node;
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
}

#[allow(unused_lifetimes)]
pub trait GraphSuccessors<'graph> {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
}
