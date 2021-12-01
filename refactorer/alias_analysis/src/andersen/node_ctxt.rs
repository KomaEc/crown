use crate::andersen::AndersenNode;
use index::vec::IndexVec;
use rustc_middle::mir::{Local, Place, PlaceRef};
use std::collections::{hash_map::Entry, HashMap};

use super::AndersenNodeData;

/// Data structure for the node factory
pub struct NodeCtxt<'tcx> {
    node_set: IndexVec<AndersenNode, AndersenNodeData<'tcx>>,
    value_node_map: HashMap<PlaceRef<'tcx>, AndersenNode>,
}

impl<'tcx> NodeCtxt<'tcx> {
    pub fn new() -> NodeCtxt<'tcx> {
        NodeCtxt {
            node_set: IndexVec::new(),
            value_node_map: HashMap::new(),
        }
    }

    fn create_node_from_mir_data(&mut self, data: PlaceRef<'tcx>) -> AndersenNode {
        let node = self.node_set.push(AndersenNodeData::Mir(data));
        self.value_node_map.insert(data, node);

        log::trace!("generating node {:?} for place {:?}", node, data);

        node
    }

    fn get_or_create_from_mir_data(&mut self, data: PlaceRef<'tcx>) -> AndersenNode {
        match self.value_node_map.entry(data) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(_) => self.create_node_from_mir_data(data),
        }
    }

    #[inline(always)]
    pub fn generate_from_local(&mut self, local: Local) -> AndersenNode {
        self.get_or_create_from_mir_data(Place::from(local).as_ref())
    }

    pub fn generate_temporary(&mut self) -> AndersenNode {
        log::trace!("generating temporary node");
        self.node_set.push(AndersenNodeData::Temporary)
    }
}
