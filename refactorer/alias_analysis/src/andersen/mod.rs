pub mod constraint_generation;
mod constraint_solving;
mod node_ctxt;

use crate::andersen::node_ctxt::NodeCtxt;
use graph::implementation::sparse_bit_vector::SparseBitVectorGraph;
use index::{bit_set::HybridBitSet, vec::IndexVec};
use rustc_middle::mir::{Local, Place, PlaceRef};
use std::ops::Index;

pub struct AndersenResult<'tcx> {
    pub pts_graph: PtsGraph,
    pub node_ctxt: NodeCtxt<'tcx>,
}

impl<'tcx> AndersenResult<'tcx> {
    pub fn new(pts_graph: PtsGraph, node_ctxt: NodeCtxt<'tcx>) -> Self {
        AndersenResult {
            pts_graph,
            node_ctxt,
        }
    }
}

pub struct PtsGraph {
    graph: SparseBitVectorGraph<AndersenNode>,
    /// runtime flag. remove later
    finished: bool,
}

impl PtsGraph {
    pub fn new(num_nodes: usize) -> Self {
        PtsGraph {
            graph: SparseBitVectorGraph::new(num_nodes, [].into_iter()),
            finished: false,
        }
    }

    #[inline]
    pub fn pts(&self, p: AndersenNode) -> &HybridBitSet<AndersenNode> {
        self.graph.successor_nodes(p)
    }

    #[inline]
    pub fn pts_mut(&mut self, p: AndersenNode) -> &mut HybridBitSet<AndersenNode> {
        self.graph.successor_nodes_mut(p)
    }

    #[inline]
    pub fn pick2_pts_mut(
        &mut self,
        p: AndersenNode,
        q: AndersenNode,
    ) -> (
        &mut HybridBitSet<AndersenNode>,
        &mut HybridBitSet<AndersenNode>,
    ) {
        self.graph.pick2_successor_nodes_mut(p, q)
    }

    pub fn finish(&mut self) {
        self.finished = true;
        todo!()
    }
}

pub struct ConstraintSet {
    constraints: IndexVec<ConstraintIndex, Constraint>,
}

impl ConstraintSet {
    pub fn new() -> ConstraintSet {
        ConstraintSet {
            constraints: IndexVec::new(),
        }
    }

    #[inline]
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    #[inline]
    pub fn push(&mut self, c: Constraint) -> ConstraintIndex {
        self.constraints.push(c)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.iter()
    }

    #[inline]
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (ConstraintIndex, &Constraint)> {
        self.constraints.iter_enumerated()
    }

    #[inline]
    pub fn universe(&self) -> &IndexVec<ConstraintIndex, Constraint> {
        &self.constraints
    }
}

impl Index<ConstraintIndex> for ConstraintSet {
    type Output = Constraint;

    fn index(&self, i: ConstraintIndex) -> &Self::Output {
        &self.constraints[i]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ConstraintKind {
    /// p = &q
    AddressOf,
    /// p = q
    Copy,
    /// p = *q
    Load,
    /// *p = q
    Store,
}

#[derive(Clone, Copy)]
pub struct Constraint {
    pub constraint_kind: ConstraintKind,
    pub left: AndersenNode,
    pub right: AndersenNode,
}

impl Constraint {
    pub fn new(ck: ConstraintKind, l: AndersenNode, r: AndersenNode) -> Self {
        Constraint {
            constraint_kind: ck,
            left: l,
            right: r,
        }
    }

    /// Test if a constraint mentions deref of [`node`].
    pub fn mention_deref(self, node: AndersenNode) -> bool {
        match self.constraint_kind {
            ConstraintKind::AddressOf => false,
            ConstraintKind::Copy => false,
            ConstraintKind::Load => node == self.right,
            ConstraintKind::Store => node == self.left,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AndersenNodeData<'tcx> {
    Mir(PlaceRef<'tcx>),
    Temporary,
}

impl<'tcx> AndersenNodeData<'tcx> {}

impl<'tcx> From<Local> for AndersenNodeData<'tcx> {
    fn from(local: Local) -> Self {
        let place = Place::from(local);
        AndersenNodeData::Mir(place.as_ref())
    }
}

index::newtype_index! {
    pub struct AndersenNode {
        DEBUG_FORMAT = "AndersenNode({})"
    }
}

pub const INVALID_ANDERSEN_NODE: AndersenNode = AndersenNode::MAX;

index::newtype_index! {
    pub struct ConstraintIndex {
        DEBUG_FORMAT = "AndersenConstraintIndex({})"
    }
}
