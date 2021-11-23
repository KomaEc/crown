use graph::implementation::forward_star::Graph;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, Body, Local, Location, Place, PlaceRef, ProjectionElem, Rvalue,
    SourceInfo, Statement, StatementKind, Terminator, TerminatorKind, UserTypeProjection,
};

use crate::{
    andersen::node_generation::NodeGeneration,
    andersen::{AndersenNode, ConstraintSet},
};

/*
use crate::andersen::{Constraint, ConstraintKind, ConstraintSet};

pub fn generate_constraints<'tcx>(body: &'tcx Body) -> ConstraintSet<'tcx> {
    for (_bb, _bb_data) in reverse_postorder(body) {}
    unimplemented!()
}
*/

// pub type PtsGraph = Graph<AndersenNode, ()>;

/// 'cg = the duration of the constraint generation
pub struct ConstraintGeneration<'tcx, 'cg> {
    constraints: &'cg mut ConstraintSet,
    body: &'cg Body<'tcx>,
    node_gen: NodeGeneration<'cg>,
}

impl<'tcx, 'cg> Visitor<'tcx> for ConstraintGeneration<'tcx, 'cg> {}
