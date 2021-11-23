use rustc_infer::traits::query::CanonicalProjectionGoal;
use rustc_middle::mir::{
    visit::Visitor, BasicBlock, BasicBlockData, Body, Local, Location, Operand, Place, PlaceRef,
    ProjectionElem, Rvalue, SourceInfo, Statement, StatementKind, Terminator, TerminatorKind,
    UserTypeProjection,
};

use log;
use rustc_middle::ty::TyCtxt;

pub struct PlaceSimplifier<'tcx, 'ps> {
    tcx: TyCtxt<'tcx>,
    body: &'ps mut Body<'tcx>,
}

impl<'tcx, 'ps> PlaceSimplifier<'tcx, 'ps> {}
