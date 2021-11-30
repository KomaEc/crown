use crate::andersen::AndersenNode;
use index::vec::IndexVec;
use std::collections::{hash_map::Entry, HashMap};

use super::AndersenNodeData;

/// Data structure for the node factory
pub struct NodeGeneration<'tcx> {
    node_set: IndexVec<AndersenNode, AndersenNodeData<'tcx>>,
    value_node_map: HashMap<AndersenNodeData<'tcx>, AndersenNode>,
}

impl<'tcx> NodeGeneration<'tcx> {
    pub fn new() -> NodeGeneration<'tcx> {
        NodeGeneration {
            node_set: IndexVec::new(),
            value_node_map: HashMap::new(),
        }
    }

    fn create_node(&mut self, data: AndersenNodeData<'tcx>) -> AndersenNode {
        let node = self.node_set.push(data);
        self.value_node_map.insert(data, node);

        log::trace!(
            "node generation: generating node {:?} for place {:?}",
            node,
            data.as_place_ref()
        );

        node
    }

    fn get_or_create(&mut self, data: AndersenNodeData<'tcx>) -> AndersenNode {
        match self.value_node_map.entry(data) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(_) => self.create_node(data),
        }
    }

    #[inline(always)]
    pub fn generate(&mut self, data: AndersenNodeData<'tcx>) -> AndersenNode {
        self.get_or_create(data)
    }
}
