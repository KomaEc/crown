use crate::{ctxt::PointerAnalysisCtxt, ConstraintSet, PointerAnalysisNode};
use union_find::{QuickUnionUf, UnionByRank};
use std::iter::repeat;

pub struct ConstraintSolving<'cs, 'tcx> {
    alias_group: QuickUnionUf<UnionByRank>,
    all_constraints: ConstraintSet,
    ptr_ctxt: PointerAnalysisCtxt<'cs, 'tcx>,
}

impl<'cs, 'tcx> ConstraintSolving<'cs, 'tcx> {
    pub fn new(all_constraints: ConstraintSet, ptr_ctxt: PointerAnalysisCtxt<'cs, 'tcx>) -> Self {
        let alias_group = QuickUnionUf::from_iter(
            repeat(UnionByRank::default()).take(ptr_ctxt.nodes().len()),
        );
        ConstraintSolving {
            alias_group,
            all_constraints,
            ptr_ctxt,
        }
    }

    fn join(&mut self, p: PointerAnalysisNode, q: PointerAnalysisNode) {
        unimplemented!()
    }
}
