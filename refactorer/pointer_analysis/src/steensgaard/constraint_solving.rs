use crate::{
    ctxt::PointerAnalysisCtxt, steensgaard::AbstractLocation, ConstraintSet, PointerAnalysisNode,
    PointerAnalysisNodeData,
};
use index::vec::IndexVec;
use std::iter::repeat;
use union_find::QuickUnionUf;

pub struct ConstraintSolving<'cs, 'tcx> {
    locations: QuickUnionUf<AbstractLocation>,
    pts: IndexVec<PointerAnalysisNode, usize>,
    all_constraints: ConstraintSet,
    ptr_ctxt: PointerAnalysisCtxt<'cs, 'tcx>,
    delayed: Vec<PointerAnalysisNode>,
}

impl<'cs, 'tcx> ConstraintSolving<'cs, 'tcx> {
    pub fn new(all_constraints: ConstraintSet, ptr_ctxt: PointerAnalysisCtxt<'cs, 'tcx>) -> Self {
        let num_nodes = ptr_ctxt.nodes().len();
        let locations =
            QuickUnionUf::from_iter(repeat(AbstractLocation::default()).take(2 * num_nodes));
        // let pts = IndexVec::from_elem(usize::MAX, ptr_ctxt.nodes());
        // p -> *p, *p -> nothing
        let pts = IndexVec::from_iter(
            (num_nodes..2 * num_nodes).chain(repeat(usize::MAX).take(num_nodes)),
        );
        // TODO: extend locations!
        ConstraintSolving {
            locations,
            pts,
            all_constraints,
            ptr_ctxt,
            delayed: Vec::new(),
        }
    }

    fn join(&mut self, key0: usize, key1: usize) {
        unimplemented!()
    }
}
