mod constraint_generation;
mod constraint_solving;
mod node_generation;

use std::ops::Index;

use index::vec::IndexVec;
use rustc_middle::mir::{
    traversal, Body, ClearCrossCrate, Local, Location, Mutability, Operand, Place, PlaceElem,
    PlaceRef, VarDebugInfoContents,
};

// use graph::implementation;

pub struct ConstraintSet {
    constraints: IndexVec<ConstraintIndex, Constraint>,
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

/*
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AndersenNode {
    pub idx: usize,
}
*/
index::newtype_index! {
    pub struct AndersenNode {
        DEBUG_FORMAT = "AndersenNode({})"
    }
}

pub struct AndersenNodeData<'tcx> {
    pub data: &'tcx Place<'tcx>,
}

index::newtype_index! {
    pub struct ConstraintIndex {
        DEBUG_FORMAT = "AndersenConstraintIndex({})"
    }
}
