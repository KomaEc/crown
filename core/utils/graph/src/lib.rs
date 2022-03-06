#![feature(min_specialization)]
#![feature(rustc_private)]

extern crate rustc_data_structures;
extern crate rustc_index;
extern crate rustc_serialize;

pub mod implementation;

#[macro_export]
macro_rules! delegate_graph_via {
    ($S:ident.$f:ident$colon:tt $ty:ty) => {
        impl DirectedGraph for $S {
            type Node = <$ty as DirectedGraph>::Node;
        }

        impl WithNumEdges for $S {
            fn num_edges(&self) -> usize {
                self.$f.num_edges()
            }
        }

        impl WithNumNodes for $S {
            fn num_nodes(&self) -> usize {
                self.$f.num_nodes()
            }
        }

        impl<'graph> GraphSuccessors<'graph> for $S {
            type Item = <$ty as GraphSuccessors<'graph>>::Item;
            type Iter = <$ty as GraphSuccessors<'graph>>::Iter;
        }

        impl WithSuccessors for $S {
            fn successors(&self, node: Self::Node) -> <Self as GraphSuccessors<'_>>::Iter {
                self.$f.successors(node)
            }
        }

        impl<'graph> GraphPredecessors<'graph> for $S {
            type Item = <$ty as GraphPredecessors<'graph>>::Item;
            type Iter = <$ty as GraphPredecessors<'graph>>::Iter;
        }

        impl WithPredecessors for $S {
            fn predecessors(&self, node: Self::Node) -> <Self as GraphPredecessors<'_>>::Iter {
                self.$f.predecessors(node)
            }
        }
    };
}
