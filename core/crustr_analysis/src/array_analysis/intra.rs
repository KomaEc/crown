use crate::array_analysis::{Constraint, Lambda};
use rustc_middle::{
    mir::{visit::Visitor, Body, Local, Location, Place, Rvalue},
    ty::TyCtxt,
};

pub struct IntraContext<'intracx, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub body: &'intracx Body<'tcx>,
    pub constraint_set: Vec<Constraint>,
}

impl<'intracx, 'tcx> Visitor<'tcx> for IntraContext<'intracx, 'tcx> {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {}
}
