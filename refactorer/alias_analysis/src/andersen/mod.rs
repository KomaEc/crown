pub mod constraint_generation;
mod constraint_solving;
mod node_generation;

use std::ops::Index;

use index::vec::IndexVec;
use rustc_middle::mir::{Place, PlaceRef};

pub struct ConstraintSet {
    constraints: IndexVec<ConstraintIndex, Constraint>,
}

impl ConstraintSet {
    pub fn new() -> ConstraintSet {
        ConstraintSet {
            constraints: IndexVec::new(),
        }
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

pub struct Constraint {
    pub constraint_kind: ConstraintKind,
    pub left: AndersenNode,
    pub right: AndersenNode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AndersenNodeData<'tcx> {
    data: PlaceRef<'tcx>,
}

impl<'tcx> AndersenNodeData<'tcx> {
    pub fn as_place_ref(self) -> PlaceRef<'tcx> {
        self.data
    }
}

impl<'tcx> From<PlaceRef<'tcx>> for AndersenNodeData<'tcx> {
    fn from(place_ref: PlaceRef<'tcx>) -> Self {
        AndersenNodeData { data: place_ref }
    }
}

impl<'tcx> From<&'tcx Place<'tcx>> for AndersenNodeData<'tcx> {
    fn from(place: &'tcx Place<'tcx>) -> Self {
        AndersenNodeData {
            data: place.as_ref(),
        }
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
