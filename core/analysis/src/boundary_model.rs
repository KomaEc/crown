use rustc_hir::def_id::DefId;
use rustc_middle::mir::{visit::Visitor, Location, Operand, Place};

pub trait BoundaryModel<'tcx>: Visitor<'tcx> {
    fn model_boundary(
        &mut self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        location: Location,
    );

    fn model_return(&mut self, location: Location);
}
