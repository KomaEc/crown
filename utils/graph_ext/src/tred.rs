

use rustc_data_structures::graph::{scc::Sccs, DirectedGraph, WithNumNodes, WithSuccessors};
use rustc_index::vec::{Idx, IndexVec};

use crate::implementation::forward_star::Graph;

pub fn graph_to_sccs_dag<G, S>(g: &G) -> Sccs<G::Node, S>
where
    G: DirectedGraph + WithNumNodes + WithSuccessors,
    S: Idx + Ord,
{
    Sccs::new(g)
}

pub fn sccs_dag_to_toposorted_forward_star<N, S>(
    g: &Sccs<N, S>,
) -> (Graph<usize, usize>, IndexVec<S, usize>)
where
    N: Idx,
    S: Idx + Ord,
{
    let g = Graph::<_, usize>::from_another_graph_rep(g);
    let toposort = (0..g.num_nodes()).rev().map(S::new);
    let mut revmap = IndexVec::from_elem_n(0, g.num_nodes());
    let res = Graph::new(
        g.num_nodes(),
        toposort.enumerate().flat_map(|(idx, node)| {
            revmap[node] = idx;
            g.predecessor_nodes(node)
                .map(move |pre| (pre.index(), node.index()))
        }),
    );

    (res, revmap)
}
