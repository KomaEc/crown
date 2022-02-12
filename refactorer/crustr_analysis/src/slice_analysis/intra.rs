use rustc_index::vec::IndexVec;
use rustc_middle::{ty::TyCtxt, mir::{Body, Local, visit::Visitor, Place, Rvalue, Location}};
use crate::slice_analysis::{Constraint, ConstraintVar};

pub struct IntraContext<'intracx, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub body: &'intracx Body<'tcx>,
    pub constraint_set: Vec<Constraint>,
}

impl<'intracx, 'tcx> Visitor<'tcx> for IntraContext<'intracx, 'tcx> {
    fn visit_assign(&mut self, place: &Place< 'tcx>, rvalue: &Rvalue< 'tcx>, location: Location) {
        
    }
}