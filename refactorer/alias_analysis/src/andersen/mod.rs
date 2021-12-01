pub mod constraint_generation;
mod constraint_solving;
mod node_ctxt;

use std::ops::Index;

use index::vec::IndexVec;
use rustc_middle::mir::{Local, Place, PlaceRef};

pub struct ConstraintSet {
    constraints: IndexVec<ConstraintIndex, Constraint>,
}

impl ConstraintSet {
    pub fn new() -> ConstraintSet {
        ConstraintSet {
            constraints: IndexVec::new(),
        }
    }

    pub fn push(&mut self, c: Constraint) -> ConstraintIndex {
        self.constraints.push(c)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.iter()
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

/*
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
*/

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
