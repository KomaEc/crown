use rustc_middle::mir::{traversal::reverse_postorder, Body};

use crate::andersen::{Constraint, ConstraintKind, ConstraintSet};

pub fn generate_constraints<'tcx>(body: &'tcx Body) -> ConstraintSet<'tcx> {
    for (_bb, _bb_data) in reverse_postorder(body) {}
    unimplemented!()
}
