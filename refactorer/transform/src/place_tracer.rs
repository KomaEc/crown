use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{Location, Place, Rvalue};

use log;

pub struct PlaceTracer;

impl PlaceTracer {
    pub fn new() -> Self {
        PlaceTracer
    }
}

impl<'tcx> Visitor<'tcx> for PlaceTracer {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        log::trace!("{:?}", place);
        self.super_assign(place, rvalue, location)
    }

    /*
    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        let mut context = context;

        if !place.projection.is_empty() {
            if context.is_use() {
                // ^ Only change the context if it is a real use, not a "use" in debuginfo.
                context = if context.is_mutating_use() {
                    PlaceContext::MutatingUse(MutatingUseContext::Projection)
                } else {
                    PlaceContext::NonMutatingUse(NonMutatingUseContext::Projection)
                };
            }
        }

        // println!("{:?}", place);
        log::trace!("{:?}", place);

        self.visit_local(&place.local, context, location);

        self.visit_projection(place.as_ref(), context, location);
    }
    */
}
