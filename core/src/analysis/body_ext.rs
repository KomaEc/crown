//! Extensions to mir Body

use crate::analysis::{constants::DOM_FRONTIER_SIZE_HINT, place_ext::PlaceExt};
use rustc_index::{bit_set::HybridBitSet, vec::IndexVec};
use rustc_middle::mir::{visit::Visitor, BasicBlock, Body, Local, Place};
use smallvec::SmallVec;

pub type DominanceFrontier = IndexVec<BasicBlock, SmallVec<[BasicBlock; DOM_FRONTIER_SIZE_HINT]>>;

pub trait BodyExt {
    fn calculate_dominance_frontier(&self) -> DominanceFrontier;
}

impl<'tcx> BodyExt for Body<'tcx> {
    fn calculate_dominance_frontier(&self) -> DominanceFrontier {
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
