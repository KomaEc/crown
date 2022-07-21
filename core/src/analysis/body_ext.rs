//! Extensions to mir Body

use crate::analysis::{constants::DOM_FRONTIER_SIZE_HINT, place_ext::PlaceExt};
use rustc_index::{bit_set::HybridBitSet, vec::IndexVec};
use rustc_middle::mir::{visit::Visitor, BasicBlock, Body, Local, Place};
use smallvec::SmallVec;

pub type DominanceFrontier = IndexVec<BasicBlock, SmallVec<[BasicBlock; DOM_FRONTIER_SIZE_HINT]>>;

pub trait BodyExt {
    fn cal_dom_frontier(&self) -> DominanceFrontier;
    fn max_deref_levels(&self) -> IndexVec<Local, u32>;
}

impl<'tcx> BodyExt for Body<'tcx> {
    fn cal_dom_frontier(&self) -> DominanceFrontier {
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

    fn max_deref_levels(&self) -> IndexVec<Local, u32> {
        struct MirVis(IndexVec<Local, u32>);
        impl<'tcx> Visitor<'tcx> for MirVis {
            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                _: rustc_middle::mir::visit::PlaceContext,
                _: rustc_middle::mir::Location,
            ) {
                let place_abs = place.r#abstract();
                self.0[place_abs.base_local] =
                    std::cmp::max(self.0[place_abs.base_local], place_abs.num_derefs);
            }
        }
        let mut vis = MirVis(IndexVec::from_elem_n(0, self.local_decls.len()));
        vis.visit_body(self);
        vis.0
    }
}
