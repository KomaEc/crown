use crate::pointer_analysis::{
    constraint_generation::ConstraintGeneration, Constraint, ConstraintKind, ConstraintSet,
    PointerAnalysisNode, PointerAnalysisNodeData,
};
use rustc_hir::def_id::LocalDefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{Body, Local, Place, PlaceRef},
    ty::TyCtxt,
};
use std::collections::{hash_map::Entry, HashMap, HashSet};

/// Data structure for the node factory
pub struct PointerAnalysisCtxt<'aacx, 'tcx> {
    /// Invariant: [`bodies`] are sorted by def_ids
    crate bodies: &'aacx [&'aacx Body<'tcx>],
    // crate all_function_def_ids: HashSet<DefId>,
    tcx: TyCtxt<'tcx>,
    crate nodes: IndexVec<PointerAnalysisNode, PointerAnalysisNodeData<'tcx>>,
    value_node_map: HashMap<(LocalDefId, PlaceRef<'tcx>), PointerAnalysisNode>,
}

const TEMP_NODE_BASE: usize = 1000000;

impl<'aacx, 'tcx> PointerAnalysisCtxt<'aacx, 'tcx> {
    pub fn into_constraint_generation(self) -> ConstraintGeneration<'aacx, 'tcx> {
        ConstraintGeneration {
            constraints: ConstraintSet::new(),
            ptr_ctxt: self,
        }
    }
}

impl<'aacx, 'tcx> PointerAnalysisCtxt<'aacx, 'tcx> {
    pub fn new(
        all_functions: &'aacx [&'aacx Body<'tcx>],
        tcx: TyCtxt<'tcx>,
    ) -> PointerAnalysisCtxt<'aacx, 'tcx> {
        let mut all_function_def_ids = HashSet::new();
        for body in all_functions {
            all_function_def_ids.insert(body.source.instance.def_id());
        }
        PointerAnalysisCtxt {
            bodies: all_functions,
            // all_function_def_ids,
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
    pub fn nodes(&self) -> &IndexVec<PointerAnalysisNode, PointerAnalysisNodeData<'tcx>> {
        &self.nodes
    }

    fn create_node_from_mir_data(
        &mut self,
        data: (LocalDefId, PlaceRef<'tcx>),
    ) -> PointerAnalysisNode {
        let node = self.nodes.push(data.into());
        self.value_node_map.insert(data, node);

        tracing::trace!(
            "generating node {:?} for place {}",
            node,
            self.node_to_str(node)
        );

        node
    }

    fn get_or_create_from_mir_data(
        &mut self,
        data: (LocalDefId, PlaceRef<'tcx>),
    ) -> PointerAnalysisNode {
        match self.value_node_map.entry(data) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(_) => self.create_node_from_mir_data(data),
        }
    }

    #[inline]
    pub fn generate_from_local(
        &mut self,
        context: LocalDefId,
        local: Local,
    ) -> PointerAnalysisNode {
        self.get_or_create_from_mir_data((context, Place::from(local).as_ref()))
    }

    pub fn lookup_local(&self, context: LocalDefId, local: Local) -> Option<PointerAnalysisNode> {
        self.value_node_map
            .get(&(context, Place::from(local).as_ref()))
            .map(|&p| p)
    }

    pub fn generate_temporary(&mut self, context: LocalDefId) -> PointerAnalysisNode {
        tracing::trace!("generating temporary node");
        self.nodes.push(context.into())
    }

    pub fn find(&self, node: PointerAnalysisNode) -> PointerAnalysisNodeData<'_> {
        self.nodes[node]
    }

    /// FIXME: assume that all place_refs are locals
    pub fn node_to_str(&self, node: PointerAnalysisNode) -> String {
        match self.find(node) {
            PointerAnalysisNodeData::Mir(did, place_ref) => {
                format!(
                    "{}::mir_{}",
                    self.tcx.def_path_str(did.to_def_id()),
                    place_ref.local.index()
                )
            }
            PointerAnalysisNodeData::Temporary(did) => format!(
                "{}::tmp_{}",
                self.tcx.def_path_str(did.to_def_id()),
                node.index() + TEMP_NODE_BASE
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
