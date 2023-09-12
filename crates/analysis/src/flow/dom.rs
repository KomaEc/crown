use rustc_index::{bit_set::HybridBitSet, IndexVec};
use rustc_middle::mir::{BasicBlock, Body};
use smallvec::SmallVec;

pub type DominanceFrontier = IndexVec<BasicBlock, SmallVec<[BasicBlock; 3]>>;

pub fn compute_dominance_frontier(body: &Body) -> DominanceFrontier {
    let dominators = body.basic_blocks.dominators();
    let mut df = IndexVec::from_elem(
        HybridBitSet::new_empty(body.basic_blocks.len()),
        &body.basic_blocks,
    );

    for bb in body.basic_blocks.indices() {
        let preds = &body.basic_blocks.predecessors()[bb];
        if preds.len() >= 2 {
            // if `bb` is a join point
            for &bb_p in preds {
                let mut runner = bb_p;
                while !dominators.dominates(runner, bb) {
                    df[runner].insert(bb);
                    let Some(dom) = dominators.immediate_dominator(runner) else {
                        break;
                    };
                    runner = dom;
                }
            }
        }
    }
    df.iter()
        .map(|set| set.iter().collect::<SmallVec<_>>())
        .collect()
}
