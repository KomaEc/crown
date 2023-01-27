// use orc_common::data_structure::vec_array::VecArray;
// use rustc_data_structures::graph::WithStartNode;
// use rustc_index::vec::IndexVec;
// use rustc_middle::mir::{BasicBlock, Body};

// pub struct DomTree {
//     pub idoms: IndexVec<BasicBlock, Option<BasicBlock>>,
//     /// BasicBlock -> Array of BasicBlock
//     pub tree: VecArray<BasicBlock>,
// }

// impl DomTree {
//     pub fn new(body: &Body) -> Self {
//         let dominators = body.basic_blocks.dominators();
//         let mut tree = VecArray::new(body.basic_blocks().len());
//         let mut idoms = IndexVec::with_capacity(body.basic_blocks().len());
//         assert_eq!(body.basic_blocks.start_node(), BasicBlock::from_u32(0));
//         idoms.push(None);
//         for bb in body.basic_blocks().indices().skip(1) {
//             assert_eq!(bb, idoms.push(Some(dominators.immediate_dominator(bb))));
//         }
//         for bb in body.basic_blocks().indices() {
//             tree.push_array(dominators.dominators(bb));
//         }
//         let tree = tree.done();
//         DomTree { idoms, tree }
//     }
// }

use rustc_index::{bit_set::HybridBitSet, vec::IndexVec};
use rustc_middle::mir::{BasicBlock, Body};
use smallvec::SmallVec;

use crate::ssa::constants::SIZE_DOM_FRONTIER;

pub type DominanceFrontier = IndexVec<BasicBlock, SmallVec<[BasicBlock; SIZE_DOM_FRONTIER]>>;

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
                    runner = dominators.immediate_dominator(runner)
                }
            }
        }
    }
    df.iter()
        .map(|set| set.iter().collect::<SmallVec<_>>())
        .collect()
}
