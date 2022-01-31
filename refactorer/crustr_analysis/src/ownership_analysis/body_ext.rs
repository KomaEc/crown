use rustc_data_structures::graph::WithSuccessors;
use rustc_index::bit_set::HybridBitSet;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Local};
use smallvec::SmallVec;
use std::collections::VecDeque;

use crate::{ownership_analysis::def_use::DefSites, LocationMap};

const DOMINATOR_FRONTIER_ON_STACK_SIZE: usize = 3;
const PHI_NODE_INSERTED_ON_STACK_SIZE: usize = 3;

pub type DominatorFrontier =
    IndexVec<BasicBlock, SmallVec<[BasicBlock; DOMINATOR_FRONTIER_ON_STACK_SIZE]>>;
pub type PhiNodeInsertionPoint =
    IndexVec<BasicBlock, SmallVec<[Local; PHI_NODE_INSERTED_ON_STACK_SIZE]>>;

/// Extension methods for Body<'tcx>
pub trait BodyExt<'tcx> {
    fn dominator_frontier(&self) -> DominatorFrontier;

    /// Compute points of insertion for phi nodes, extend def_sites if upon insertion
    fn compute_phi_node(&self, def_sites: &mut DefSites) -> PhiNodeInsertionPoint;
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

    fn compute_phi_node(&self, def_sites: &mut DefSites) -> PhiNodeInsertionPoint {
        ComputePhiNode::new(self, def_sites, self.dominator_frontier()).run()
    }
}

struct ComputePhiNode<'cpn, 'tcx> {
    body: &'cpn Body<'tcx>,
    def_sites: &'cpn mut DefSites,
    dominator_frontier: DominatorFrontier,
}

impl<'cpn, 'tcx> ComputePhiNode<'cpn, 'tcx> {
    fn new(
        body: &'cpn Body<'tcx>,
        def_sites: &'cpn mut DefSites,
        dominator_frontier: DominatorFrontier,
    ) -> Self {
        ComputePhiNode {
            body,
            def_sites,
            dominator_frontier,
        }
    }

    fn run(self) -> PhiNodeInsertionPoint {
        for a in self.body.local_decls.indices() {
            let def_sites = std::mem::take(&mut self.def_sites[a]).into_iter();
            let mut newly_added: LocationMap<bool> = LocationMap::new(self.body);
            let mut work_list = VecDeque::from_iter(def_sites);
            while let Some(location) = work_list.pop_front() {
                let bb = location.block;
                for bb_f in &self.dominator_frontier[bb] {
                    
                }
            }
        }
        todo!()
    }
}
