use rustc_index::vec::IndexVec;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, Body, Local, Location, Place, PlaceRef, ProjectionElem, Rvalue,
    SourceInfo, Statement, StatementKind, Terminator, TerminatorKind, UserTypeProjection,
};

use crate::andersen::AndersenNode;

pub struct NodeGeneration<'ng> {
    node_set: &'ng mut IndexVec<AndersenNode, ()>,
}

impl<'ng> NodeGeneration<'ng> {
    pub fn generate_node<'tcx>(&mut self, place: &'tcx Place<'tcx>) -> AndersenNode {
        unimplemented!()
    }
}
