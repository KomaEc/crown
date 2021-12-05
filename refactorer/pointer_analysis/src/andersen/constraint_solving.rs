use crate::andersen::{
    ctxt::AndersenAnalysisCtxt, AndersenNode, AndersenResult, ConstraintIndex, ConstraintKind,
    ConstraintSet, PtsGraph,
};
use graph::{implementation::sparse_bit_vector::SparseBitVectorGraph, WithSuccessors};
use index::vec::IndexVec;
use std::collections::VecDeque;

/// Data structure for solving the constraints.
pub struct ConstraintSolving<'tcx> {
    /// Each node is associated with a points-to set.
    pts_graph: PtsGraph,
    /// Each node is associated with a set of complex constraints.
    /// For a node `p`, constraints of the forms `*p = q`, `q = *p` are
    /// considered associated complex constraints.
    associated_complex_constraints: IndexVec<AndersenNode, Vec<ConstraintIndex>>,
    /// The constraint graph. A directed edge from node `q` to `p` means
    /// `p` ⊃ `q`, or `p = q`.
    constraint_graph: SparseBitVectorGraph<AndersenNode>,
    all_constraints: ConstraintSet,
    /// Node context, which says how nodes in the constraint graph
    /// are related to original program variables.
    node_ctxt: AndersenAnalysisCtxt<'tcx>,
}

impl<'tcx> ConstraintSolving<'tcx> {
    pub fn new(all_constraints: ConstraintSet, node_ctxt: AndersenAnalysisCtxt<'tcx>) -> Self {
        let num_nodes = node_ctxt.num_nodes();

        let mut pts_graph = PtsGraph::new(num_nodes);
        let mut associated_complex_constraints =
            IndexVec::from_elem(Vec::new(), node_ctxt.universe());
        let mut constraint_graph: SparseBitVectorGraph<AndersenNode> =
            SparseBitVectorGraph::new(num_nodes, [].into_iter());
        for (cid, constraint) in all_constraints.iter_enumerated() {
            match constraint.constraint_kind {
                // p = &q, then q ∈ pts(p).
                ConstraintKind::AddressOf => {
                    pts_graph.pts_mut(constraint.left).insert(constraint.right);
                }
                // p = q, add a graph edge fropm q to p.
                ConstraintKind::Copy => {
                    constraint_graph.add_edge(constraint.right, constraint.left);
                }
                // p = *q, associate complex constraint to q
                ConstraintKind::Load => {
                    associated_complex_constraints[constraint.right].push(cid);
                }
                // *p = q, associate complex constraint to p
                ConstraintKind::Store => {
                    associated_complex_constraints[constraint.left].push(cid);
                }
            }
        }
        ConstraintSolving {
            pts_graph,
            associated_complex_constraints,
            constraint_graph,
            all_constraints,
            node_ctxt,
        }
    }

    /// Dynamic transitive closure algorithm
    pub fn solve_by_dynamic_transitive_closure(&mut self) {
        // insert all nodes into work list.
        let mut work_list = VecDeque::from_iter(self.node_ctxt.universe().indices());
        while let Some(p) = work_list.pop_front() {
            // for all r ∈ p
            for r in self.pts_graph.pts(p).iter() {
                for constraint in self.associated_complex_constraints[p]
                    .iter()
                    .map(|&cid| self.all_constraints[cid])
                {
                    match constraint.constraint_kind {
                        // propagate subset constraint q ⊃ *p, deduce q ⊃ r
                        ConstraintKind::Load => {
                            assert_eq!(p, constraint.right);
                            let q = constraint.left;
                            if self.constraint_graph.add_edge(r, q) {
                                work_list.push_back(r);
                            }
                        }
                        // propagate subset constraint *p ⊃ q, deduce r ⊃ q
                        ConstraintKind::Store => {
                            assert_eq!(p, constraint.left);
                            let q = constraint.right;
                            if self.constraint_graph.add_edge(q, r) {
                                work_list.push_back(q);
                            }
                        }
                        _ => panic!("impossible"),
                    }
                }
            }

            // propagate along graph edges
            for q in self.constraint_graph.successors(p) {
                let (pts_q, pts_p) = self.pts_graph.pick2_pts_mut(q, p);
                if pts_q.union(pts_p) {
                    work_list.push_back(q);
                }
            }
        }
    }

    pub fn finish(self) -> AndersenResult<'tcx> {
        AndersenResult::new(self.pts_graph, self.node_ctxt)
    }
}