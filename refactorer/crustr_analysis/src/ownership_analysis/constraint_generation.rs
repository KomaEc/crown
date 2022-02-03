//! Constraint Generation

use rustc_middle::mir::{visit::Visitor, Body};

/// Generate ownership constraints for a function body, perform
/// SSA renaming in the meantime
pub struct ConstraintGenerationForBody<'me, 'tcx> {
    body: &'me Body<'tcx>,
}

impl<'me, 'tcx> Visitor<'tcx> for ConstraintGenerationForBody<'me, 'tcx> {
    fn visit_body(&mut self, body: &Body<'tcx>) {
        self.super_body(body)
    }
}

pub trait BodySummarisable<'tcx>: Visitor<'tcx> {}
