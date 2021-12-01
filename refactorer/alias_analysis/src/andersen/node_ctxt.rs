use crate::andersen::AndersenNode;
use index::vec::IndexVec;
use rustc_middle::mir::{Local, Place, PlaceRef};
use std::collections::{hash_map::Entry, HashMap};

use super::AndersenNodeData;

/// Data structure for the node factory
pub struct NodeCtxt<'tcx> {
    universe: IndexVec<AndersenNode, AndersenNodeData<'tcx>>,
    value_node_map: HashMap<PlaceRef<'tcx>, AndersenNode>,
}

impl<'tcx> NodeCtxt<'tcx> {
    pub fn new() -> NodeCtxt<'tcx> {
        NodeCtxt {
            universe: IndexVec::new(),
            value_node_map: HashMap::new(),
        }
    }

    #[inline]
    pub fn num_nodes(&self) -> usize {
        self.universe.len()
    }

    #[inline]
    pub fn universe(&self) -> &IndexVec<AndersenNode, AndersenNodeData<'tcx>> {
        &self.universe
    }

    fn create_node_from_mir_data(&mut self, data: PlaceRef<'tcx>) -> AndersenNode {
        let node = self.universe.push(AndersenNodeData::Mir(data));
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
        self.universe.push(AndersenNodeData::Temporary)
    }

    pub fn find(&self, node: AndersenNode) -> AndersenNodeData<'_> {
        self.universe[node]
    }

    pub fn to_string(&self, node: AndersenNode) -> String {
        match self.find(node) {
            /// FIXME: assume that all place_refs are locals
            AndersenNodeData::Mir(place_ref) => format!("mir_{}", place_ref.local.index()),
            AndersenNodeData::Temporary => format!("tmp_{}", node.index()),
        }
    }
}
