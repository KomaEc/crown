use graph::{DirectedGraph, WithNumNodes, WithSuccessors};

pub struct WorkList<'wl, G>
where
    G: ?Sized + DirectedGraph + WithNumNodes + WithSuccessors,
{
    pub constraint_graph: Option<&'wl G>,
}
