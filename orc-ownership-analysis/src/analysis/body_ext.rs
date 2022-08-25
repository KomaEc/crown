//! Extensions to mir Body

use crate::analysis::constants::SIZE_DOM_FRONTIER;
use rustc_index::{bit_set::HybridBitSet, vec::IndexVec};
use rustc_middle::mir::{BasicBlock, Body};
use smallvec::SmallVec;

pub(crate) type DominanceFrontier = IndexVec<BasicBlock, SmallVec<[BasicBlock; SIZE_DOM_FRONTIER]>>;

pub(crate) trait BodyExt {
    fn compute_dominance_frontier(&self) -> DominanceFrontier;
}

impl<'tcx> BodyExt for Body<'tcx> {
    fn compute_dominance_frontier(&self) -> DominanceFrontier {
        let dominators = self.basic_blocks.dominators();
        let mut df = IndexVec::from_elem(
            HybridBitSet::new_empty(self.basic_blocks().len()),
            self.basic_blocks(),
        );

        for bb in self.basic_blocks().indices() {
            let preds = &self.basic_blocks.predecessors()[bb];
            if preds.len() >= 2 {
                // if `bb` is a join point
                for &bb_p in preds {
                    let mut runner = bb_p;
                    while runner != dominators.immediate_dominator(bb) {
                        df[runner].insert(bb);
                        runner = dominators.immediate_dominator(runner)
                    }
                }
            }
        }
        df.iter()
            .map(|set| set.iter().collect::<SmallVec<_>>())
            .collect()
    }
}
