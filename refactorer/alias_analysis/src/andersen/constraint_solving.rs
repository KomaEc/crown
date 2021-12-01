use crate::andersen::{node_ctxt::NodeCtxt, AndersenNode, ConstraintIndex, ConstraintSet};
use graph::implementation::sparse_bit_vector::SparseBitVectorGraph;
use index::{bit_set::BitSet, vec::IndexVec};

/// Data structure for solving the constraints.
pub struct ConstraintSolving<'tcx> {
    /// Each node is associated with a points-to set.
    pts_sets: IndexVec<AndersenNode, BitSet<AndersenNode>>,
    /// Each node is associated with a set of complex constraints.
    /// For a node `p`, constraints of the forms `*p = q`, `q = *p` are
    /// considered associated complex constraints.
    associated_complex_constraints: IndexVec<AndersenNode, BitSet<ConstraintIndex>>,
    /// The constraint graph. A directed edge from node `q` to `p` means
    /// `p` ⊃ `q`, or `p = q`.
    constraint_graph: SparseBitVectorGraph<AndersenNode>,
    all_constraints: ConstraintSet,
    /// Node context, which says how nodes in the constraint graph
    /// are related to original program variables.
    node_ctxt: NodeCtxt<'tcx>,
}

impl<'tcx> ConstraintSolving<'tcx> {
    pub fn new(all_constraints: ConstraintSet, node_ctxt: NodeCtxt<'tcx>) -> Self {
        let num_nodes = node_ctxt.num_nodes();
        /// FIXME: initialise!
        let pts_sets = IndexVec::from_elem_n(BitSet::new_empty(num_nodes), num_nodes);
        let associated_complex_constraints = IndexVec::from_elem_n(BitSet::new_empty(num_nodes), num_nodes); 
        let constraint_graph = todo!();
        ConstraintSolving {
            pts_sets,
            associated_complex_constraints,
            constraint_graph,
            all_constraints,
            node_ctxt,
        }
    }
}
