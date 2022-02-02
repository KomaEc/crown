use rustc_index::bit_set::{BitSet, HybridBitSet};
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Local};
use rustc_middle::ty::TyCtxt;
use rustc_mir_dataflow::{
    impls::MaybeLiveLocals,
    Analysis, ResultsCursor,
};
use smallvec::SmallVec;
use std::collections::VecDeque;

use crate::ownership_analysis::def_use::{DefSites, DefSitesGatherer};

const DOMINATOR_FRONTIER_ON_STACK_SIZE: usize = 3;
const PHI_NODE_INSERTED_ON_STACK_SIZE: usize = 3;

pub type DominanceFrontier =
    IndexVec<BasicBlock, SmallVec<[BasicBlock; DOMINATOR_FRONTIER_ON_STACK_SIZE]>>;
pub type PhiNodeInserted = IndexVec<BasicBlock, SmallVec<[Local; PHI_NODE_INSERTED_ON_STACK_SIZE]>>;

/// Extension methods for Body<'tcx>
pub trait BodyExt<'tcx> {
    fn dominance_frontier(&self) -> DominanceFrontier;
    fn compute_phi_node(&self, tcx: TyCtxt<'tcx>) -> PhiNodeInserted;
}

impl<'tcx> BodyExt<'tcx> for Body<'tcx> {
    fn dominance_frontier(&self) -> DominanceFrontier {
        let dominators = self.dominators();
        let mut df = IndexVec::from_elem(
            HybridBitSet::new_empty(self.basic_blocks().len()),
            self.basic_blocks(),
        );

        for bb in self.basic_blocks().indices() {
            let preds = self.predecessors()[bb].clone();
            if preds.len() >= 2 {
                // if `bb` is a join point
                for bb_p in preds {
                    let mut runner = bb_p;
                    while runner != dominators.immediate_dominator(bb) {
                        df[runner].insert(bb);
                        runner = dominators.immediate_dominator(runner)
                    }
                }
            }
        }
        IndexVec::from_iter(df.iter().map(|set| set.iter().collect::<SmallVec<_>>()))
    }

    fn compute_phi_node(&self, tcx: TyCtxt<'tcx>) -> PhiNodeInserted {
        ComputePhiNode {
            body: self,
            def_sites: DefSitesGatherer::new(self).gather(),
            dominance_frontier: self.dominance_frontier(),
            liveness: liveness_result(tcx, self)
        }
        .run()
    }
}

struct ComputePhiNode<'cpn, 'tcx> {
    pub body: &'cpn Body<'tcx>,
    pub def_sites: DefSites,
    pub dominance_frontier: DominanceFrontier,
    pub liveness: ResultsCursor<'cpn, 'tcx, MaybeLiveLocals>
}

impl<'cpn, 'tcx> ComputePhiNode<'cpn, 'tcx> {
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
                for &bb_f in &self.dominance_frontier[bb] {
                    self.liveness.seek_to_block_start(bb_f);
                    if !already_added.contains(bb_f) && self.liveness.get().contains(a) {
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

fn liveness_result<'a, 'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &'a Body<'tcx>,
) -> ResultsCursor<'a, 'tcx, MaybeLiveLocals> {
    let liveness = MaybeLiveLocals
        .into_engine(tcx, body)
        .iterate_to_fixpoint()
        .into_results_cursor(body);
    liveness
}