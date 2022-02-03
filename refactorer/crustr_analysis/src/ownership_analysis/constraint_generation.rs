//! Constraint Generation

use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    BasicBlock, Body, Location, Operand, Place, Terminator, TerminatorKind,
};
use rustc_span::Span;

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

pub trait BodySummarisable<'tcx>: Visitor<'tcx> {
    fn super_call(
        &mut self,
        func: &Operand<'tcx>,
        args: &Vec<Operand<'tcx>>,
        destination: &Option<(Place<'tcx>, BasicBlock)>,
        _cleanup: Option<BasicBlock>,
        _from_hir_call: bool,
        _fn_span: Span,
        location: Location,
    ) {
        self.visit_operand(func, location);
        for arg in args {
            self.visit_operand(arg, location);
        }
        if let Some((destination, _)) = destination {
            self.visit_place(
                destination,
                PlaceContext::MutatingUse(MutatingUseContext::Call),
                location,
            );
        }
    }

    fn visit_call(
        &mut self,
        func: &Operand<'tcx>,
        args: &Vec<Operand<'tcx>>,
        destination: &Option<(Place<'tcx>, BasicBlock)>,
        cleanup: Option<BasicBlock>,
        from_hir_call: bool,
        fn_span: Span,
        location: Location,
    ) {
        self.super_call(
            func,
            args,
            destination,
            cleanup,
            from_hir_call,
            fn_span,
            location,
        )
    }

    fn super_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        if let TerminatorKind::Call {
            func,
            args,
            destination,
            cleanup,
            from_hir_call,
            fn_span,
        } = &terminator.kind
        {
            self.visit_call(
                func,
                args,
                destination,
                *cleanup,
                *from_hir_call,
                *fn_span,
                location,
            )
        } else {
            Visitor::super_terminator(self, terminator, location)
        }
    }
}
