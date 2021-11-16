
use rustc_middle::mir::{Body, traversal::reverse_postorder};

use crate::{
    andersen::{ConstraintSet, ConstraintKind, Constraint},
};

pub fn generate<'tcx>(body: &'tcx Body) -> ConstraintSet<'tcx> {
    for (_bb, _bb_data) in reverse_postorder(body) {

    }
    unimplemented!()
}