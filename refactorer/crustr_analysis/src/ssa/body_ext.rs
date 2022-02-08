//! Extension methods for Body<'tcx>

use rustc_index::bit_set::{BitSet, HybridBitSet};
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Local};
use rustc_middle::ty::TyCtxt;
use rustc_mir_dataflow::{Analysis, ResultsCursor};
use smallvec::SmallVec;
use std::collections::VecDeque;

use crate::def_use::{DefSites, DefSitesGatherer, DefUseCategorisable};
use crate::liveness_analysis::MaybeLiveLocals;

const DOMINATOR_FRONTIER_ON_STACK_SIZE: usize = 3;
const PHI_NODE_INSERTED_ON_STACK_SIZE: usize = 3;

pub type DominanceFrontier =
    IndexVec<BasicBlock, SmallVec<[BasicBlock; DOMINATOR_FRONTIER_ON_STACK_SIZE]>>;
pub type PhiNodeInserted = IndexVec<BasicBlock, SmallVec<[Local; PHI_NODE_INSERTED_ON_STACK_SIZE]>>;

pub trait BodyExt<'tcx> {
    fn dominance_frontier(&self) -> DominanceFrontier;
    fn compute_phi_node<DefUse: DefUseCategorisable>(&self, tcx: TyCtxt<'tcx>) -> PhiNodeInserted;
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

    fn compute_phi_node<DefUse: DefUseCategorisable>(&self, tcx: TyCtxt<'tcx>) -> PhiNodeInserted {
        minimal_ssa_form(
            self,
            DefSitesGatherer::<DefUse>::new(self).gather(),
            self.dominance_frontier(),
            liveness_result(tcx, self),
        )
    }
}

/// Compute phi node insertion points for minimal SSA form.
/// Along with normal construction, check for liveness on entry
/// before inserting phi node to avoid redundant nodes.
pub fn minimal_ssa_form<'tcx>(
    body: &Body<'tcx>,
    mut def_sites: DefSites,
    dominance_frontier: DominanceFrontier,
    mut liveness: ResultsCursor<'_, 'tcx, MaybeLiveLocals>,
) -> PhiNodeInserted {
    let mut inserted: PhiNodeInserted = IndexVec::from_elem(SmallVec::new(), body.basic_blocks());
    let def_sites = def_sites
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
    for a in body.local_decls.indices() {
        let mut already_added = BitSet::new_empty(body.basic_blocks().len());
        let mut work_list = VecDeque::from_iter(def_sites[a].iter().map(|&bb| bb));
        while let Some(bb) = work_list.pop_front() {
            for &bb_f in &dominance_frontier[bb] {
                liveness.seek_to_block_start(bb_f);
                if !already_added.contains(bb_f) && liveness.get().contains(a) {
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
