use rustc_middle::mir::{visit::Visitor, BasicBlock, Location, Operand, Place};

pub trait BoundaryModel<'tcx>: Visitor<'tcx> {
    fn model_boundary(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    );

    fn model_return(&mut self, location: Location);
}
