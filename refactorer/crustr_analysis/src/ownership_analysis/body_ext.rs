use rustc_data_structures::graph::WithSuccessors;
use rustc_index::bit_set::HybridBitSet;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body};
use smallvec::SmallVec;

pub const DOMINATOR_FRONTIER_ON_STACK_SIZE: usize = 3;

pub type DominatorFrontier = IndexVec<BasicBlock, SmallVec<[BasicBlock; DOMINATOR_FRONTIER_ON_STACK_SIZE]>>;

/// Extension methods for Body<'tcx>
pub trait BodyExt<'tcx> {
    fn dominator_frontier(&self) -> DominatorFrontier;
}

impl<'tcx> BodyExt<'tcx> for Body<'tcx> {
    fn dominator_frontier(&self) -> DominatorFrontier {
        use core::cmp::Ordering;

        let dominators = self.dominators();
        let mut df = IndexVec::from_elem(
            HybridBitSet::new_empty(self.basic_blocks().len()),
            self.basic_blocks(),
        );
        // compute df_local
        for bb in self.basic_blocks().indices() {
            for succ in self.successors(bb) {
                if dominators.immediate_dominator(succ) != bb {
                    df[bb].insert(succ);
                }
            }
        }
        // union df_up
        let bbs_in_reverse_post_order = {
            let mut bbs = self.basic_blocks().indices().collect::<Vec<_>>();
            bbs.sort_by(|&lhs, &rhs| {
                dominators
                    .rank_partial_cmp(lhs, rhs)
                    .map(Ordering::reverse)
                    .unwrap_or_else(|| Ordering::Equal)
            });
            bbs
        };
        for bb in bbs_in_reverse_post_order {
            assert!(dominators.is_reachable(bb), "block is unreachable!");
            let pred = dominators.immediate_dominator(bb);
            if bb != pred {
                let (df_bb, df_pred) = df.pick2_mut(bb, pred);
                df_pred.union(df_bb);
            }
        }
        IndexVec::from_iter(df.iter().map(|set| set.iter().collect::<SmallVec<_>>()))
    }
}
