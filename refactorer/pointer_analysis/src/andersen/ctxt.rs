use crate::andersen::{AndersenNode, AndersenNodeData};
use index::vec::IndexVec;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{Local, Place, PlaceRef},
    ty::TyCtxt,
};
use std::collections::{hash_map::Entry, HashMap};

/// Data structure for the node factory
pub struct AndersenAnalysisCtxt<'aacx, 'tcx> {
    pub all_functions: &'aacx [LocalDefId],
    tcx: TyCtxt<'tcx>,
    nodes: IndexVec<AndersenNode, AndersenNodeData<'tcx>>,
    value_node_map: HashMap<(LocalDefId, PlaceRef<'tcx>), AndersenNode>,
}

impl<'aacx, 'tcx> AndersenAnalysisCtxt<'aacx, 'tcx> {
    pub fn new(
        all_functions: &'aacx [LocalDefId],
        tcx: TyCtxt<'tcx>,
    ) -> AndersenAnalysisCtxt<'aacx, 'tcx> {
        AndersenAnalysisCtxt {
            all_functions,
            tcx,
            nodes: IndexVec::new(),
            value_node_map: HashMap::new(),
        }
    }

    #[inline]
    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }

    #[inline]
    pub fn all_functions(&self) -> impl Iterator<Item = LocalDefId> + '_ {
        self.all_functions.iter().map(|&v| v)
    }

    #[inline]
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn nodes(&self) -> &IndexVec<AndersenNode, AndersenNodeData<'tcx>> {
        &self.nodes
    }

    fn create_node_from_mir_data(&mut self, data: (LocalDefId, PlaceRef<'tcx>)) -> AndersenNode {
        let node = self.nodes.push(data.into());
        self.value_node_map.insert(data, node);

        log::trace!("generating node {:?} for place {:?}", node, data);

        node
    }

    fn get_or_create_from_mir_data(&mut self, data: (LocalDefId, PlaceRef<'tcx>)) -> AndersenNode {
        match self.value_node_map.entry(data) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(_) => self.create_node_from_mir_data(data),
        }
    }

    #[inline]
    pub fn generate_from_local(&mut self, context: LocalDefId, local: Local) -> AndersenNode {
        self.get_or_create_from_mir_data((context, Place::from(local).as_ref()))
    }

    pub fn lookup_local(&mut self, context: LocalDefId, local: Local) -> Option<AndersenNode> {
        match self
            .value_node_map
            .entry((context, Place::from(local).as_ref()))
        {
            Entry::Occupied(entry) => Some(*entry.get()),
            Entry::Vacant(_) => None,
        }
    }

    pub fn generate_temporary(&mut self, context: LocalDefId) -> AndersenNode {
        log::trace!("generating temporary node");
        self.nodes.push(context.into())
    }

    pub fn find(&self, node: AndersenNode) -> AndersenNodeData<'_> {
        self.nodes[node]
    }

    /// FIXME: assume that all place_refs are locals
    pub fn node_to_str(&self, node: AndersenNode) -> String {
        match self.find(node) {
            AndersenNodeData::Mir(did, place_ref) => {
                format!("{}::mir_{}", self.tcx.def_path_str(did.to_def_id()), place_ref.local.index())
            }
            AndersenNodeData::Temporary(did) => format!("{}::tmp_{}", self.tcx.def_path_str(did.to_def_id()), node.index()),
        }
    }
}
