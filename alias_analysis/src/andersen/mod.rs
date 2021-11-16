

mod constraint_generation;

use rustc_middle::mir::{
    traversal, Body, ClearCrossCrate, Local, Location, Mutability, Operand, Place, PlaceElem,
    PlaceRef, VarDebugInfoContents,
};
use rustc_index::vec::IndexVec;


pub struct ConstraintSet<'tcx> {
    constraints: IndexVec<ConstraintIndex, Constraint<'tcx>>,
}

#[derive(Clone, Copy, Debug)]
pub enum ConstraintKind {
    AddressOf,
    Copy,
    Assign,
    Dereference,
}

pub struct Constraint<'tcx> {
    pub constraint_kind: ConstraintKind,
    pub left: &'tcx Place<'tcx>,
    pub right: &'tcx Place<'tcx>,
}

rustc_index::newtype_index! {
    pub struct ConstraintIndex {
        DEBUG_FORMAT = "AndersenConstraintIndex({})"
    }
}
