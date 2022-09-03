use orc_common::data_structure::vec_array::VecArray;
use rustc_data_structures::graph::WithStartNode;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body};

pub(crate) struct DomTree {
    pub(crate) idoms: IndexVec<BasicBlock, Option<BasicBlock>>,
    /// BasicBlock -> Array of BasicBlock
    pub(crate) tree: VecArray<BasicBlock>,
}

impl DomTree {
    pub(crate) fn new<'tcx>(body: &Body<'tcx>) -> Self {
        let dominators = body.basic_blocks.dominators();
        let mut tree = VecArray::new(body.basic_blocks().len());
        let mut idoms = IndexVec::with_capacity(body.basic_blocks().len());
        assert_eq!(body.basic_blocks.start_node(), BasicBlock::from_u32(0));
        idoms.push(None);
        for bb in body.basic_blocks().indices().skip(1) {
            assert_eq!(bb, idoms.push(Some(dominators.immediate_dominator(bb))));
        }
        for bb in body.basic_blocks().indices() {
            tree.push_array(dominators.dominators(bb));
        }
        let tree = tree.done();
        DomTree { idoms, tree }
    }
}
