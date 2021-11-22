mod constraint_generation;
mod constraint_solving;

use rustc_index::vec::IndexVec;
use rustc_middle::mir::{
    traversal, Body, ClearCrossCrate, Local, Location, Mutability, Operand, Place, PlaceElem,
    PlaceRef, VarDebugInfoContents,
};

// use graph::implementation;

pub struct ConstraintSet<'tcx> {
    constraints: IndexVec<ConstraintIndex, Constraint<'tcx>>,
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

pub struct Constraint<'tcx> {
    pub constraint_kind: ConstraintKind,
    pub left: &'tcx Place<'tcx>,
    pub right: &'tcx Place<'tcx>,
}

#[derive(Clone, Copy)]
pub struct Node {
    pub idx: i32,
}

pub struct NodeData<'tcx> {
    pub data: &'tcx Place<'tcx>,
}

rustc_index::newtype_index! {
    pub struct ConstraintIndex {
        DEBUG_FORMAT = "AndersenConstraintIndex({})"
    }
}
