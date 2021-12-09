use crate::andersen::{
    constraint_generation::ConstraintGeneration, AndersenNode, AndersenNodeData, Constraint,
    ConstraintKind, ConstraintSet,
};
use index::vec::IndexVec;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{Body, Local, Place, PlaceRef},
    ty::TyCtxt,
};
use std::cell::Ref;
use std::collections::{hash_map::Entry, HashMap};

/// Data structure for the node factory
pub struct AndersenAnalysisCtxt<'aacx, 'tcx> {
    crate all_functions: &'aacx [Ref<'aacx, Body<'tcx>>],
    tcx: TyCtxt<'tcx>,
    nodes: IndexVec<AndersenNode, AndersenNodeData<'tcx>>,
    value_node_map: HashMap<(LocalDefId, PlaceRef<'tcx>), AndersenNode>,
}

impl<'aacx, 'tcx> AndersenAnalysisCtxt<'aacx, 'tcx> {
    pub fn into_constraint_generation(self) -> ConstraintGeneration<'aacx, 'tcx> {
        ConstraintGeneration {
            constraints: ConstraintSet::new(),
            aa_ctxt: self,
        }
    }
}

impl<'aacx, 'tcx> AndersenAnalysisCtxt<'aacx, 'tcx> {
    pub fn new(
        all_functions: &'aacx [Ref<'aacx, Body<'tcx>>],
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

        log::trace!(
            "generating node {:?} for place {}",
            node,
            self.node_to_str(node)
        );

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

    pub fn lookup_local(&self, context: LocalDefId, local: Local) -> Option<AndersenNode> {
        self.value_node_map
            .get(&(context, Place::from(local).as_ref()))
            .map(|&p| p)
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
                format!(
                    "{}::mir_{}",
                    self.tcx.def_path_str(did.to_def_id()),
                    place_ref.local.index()
                )
            }
            AndersenNodeData::Temporary(did) => format!(
                "{}::tmp_{}",
                self.tcx.def_path_str(did.to_def_id()),
                node.index() + 100000
            ),
        }
    }

    pub fn constraint_to_str(&self, constraint: Constraint) -> String {
        let lhs = self.node_to_str(constraint.left);
        let rhs = self.node_to_str(constraint.right);
        match constraint.constraint_kind {
            ConstraintKind::AddressOf => format!("{} = &{}", lhs, rhs),
            ConstraintKind::Copy => format!("{} = {}", lhs, rhs),
            ConstraintKind::Load => format!("{} = *{}", lhs, rhs),
            ConstraintKind::Store => format!("*{} = {}", lhs, rhs),
        }
    }
}
