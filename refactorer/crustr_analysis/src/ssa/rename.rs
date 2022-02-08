use rustc_middle::{
    mir::{visit::Visitor, Body, BasicBlock, BasicBlockData},
    ty::TyCtxt,
};

/// A `Renamer` will not really rename variables in a body (and
/// hence it's a subtrait of `Visitor` rather than `MutVisitor`),
/// but emit side effects (such as constraint variables generation
/// in some analysis) when rename happens.
pub trait Renamer<'tcx>: Visitor<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx>;

    fn visit_body(&mut self, body: &Body<'tcx>) {
        use std::cmp::Ordering;
        let dominators = body.dominators();
        let mut bb_in_dominance_order = body.basic_blocks().indices().collect::<Vec<_>>();
        bb_in_dominance_order.sort_by(|&bb1, &bb2| {
            dominators
                .rank_partial_cmp(bb1, bb2)
                .map(Ordering::reverse)
                .unwrap_or(Ordering::Equal)
        });
        for bb in bb_in_dominance_order {
            self.super_basic_block_data(bb, &body.basic_blocks()[bb])
        }
    }

    fn visit_basic_block_data(&mut self, bb: BasicBlock, data: BasicBlockData) {

    }
}
