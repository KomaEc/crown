use rustc_data_structures::graph::WithSuccessors;
use rustc_index::bit_set::{BitSet, HybridBitSet};
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Local};
use smallvec::SmallVec;
use std::collections::VecDeque;

use crate::ownership_analysis::def_use::{DefSites, DefSitesGatherer};

const DOMINATOR_FRONTIER_ON_STACK_SIZE: usize = 3;
const PHI_NODE_INSERTED_ON_STACK_SIZE: usize = 3;

pub type DominatorFrontier =
    IndexVec<BasicBlock, SmallVec<[BasicBlock; DOMINATOR_FRONTIER_ON_STACK_SIZE]>>;
pub type PhiNodeInserted = IndexVec<BasicBlock, SmallVec<[Local; PHI_NODE_INSERTED_ON_STACK_SIZE]>>;

/// Extension methods for Body<'tcx>
pub trait BodyExt<'tcx> {
    fn dominator_frontier(&self) -> DominatorFrontier;

    /// Compute points of insertion for phi nodes, extend def_sites if upon insertion
    fn compute_phi_node(&self) -> PhiNodeInserted;
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

    fn compute_phi_node(&self) -> PhiNodeInserted {
        ComputePhiNode::new(
            self,
            DefSitesGatherer::new(self).gather(),
            self.dominator_frontier(),
        )
        .run()
    }
}

struct ComputePhiNode<'cpn, 'tcx> {
    body: &'cpn Body<'tcx>,
    def_sites: DefSites,
    dominator_frontier: DominatorFrontier,
}

impl<'cpn, 'tcx> ComputePhiNode<'cpn, 'tcx> {
    fn new(
        body: &'cpn Body<'tcx>,
        def_sites: DefSites,
        dominator_frontier: DominatorFrontier,
    ) -> Self {
        ComputePhiNode {
            body,
            def_sites,
            dominator_frontier,
        }
    }

    fn run(mut self) -> PhiNodeInserted {
        let mut inserted: PhiNodeInserted =
            IndexVec::from_elem(SmallVec::new(), self.body.basic_blocks());
        let def_sites = self
            .def_sites
            .iter_mut()
            .map(|sites| {
                let mut sites = sites
                    .iter_mut()
                    .map(|location| location.block)
                    .collect::<Vec<_>>();
                sites.sort();
                sites.dedup();
                sites
            })
            .collect::<IndexVec<Local, _>>();
        for a in self.body.local_decls.indices() {
            let mut already_added = BitSet::new_empty(self.body.basic_blocks().len());
            let mut work_list = VecDeque::from_iter(def_sites[a].iter().map(|&bb| bb));
            while let Some(bb) = work_list.pop_front() {
                for &bb_f in &self.dominator_frontier[bb] {
                    // wrong! new visited
                    if !already_added.contains(bb_f) {
                        inserted[bb_f].push(a);
                        assert!(already_added.insert(bb_f));
                        if !def_sites[a].contains(&bb_f) {
                            work_list.push_back(bb_f);
                        }
                    }
                }
            }
        }
        inserted
    }
}
