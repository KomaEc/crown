use rustc_hir::def_id::LocalDefId;
use rustc_middle::mir::visit::{PlaceContext, Visitor};
use rustc_middle::mir::{Location, Place, PlaceRef, Terminator, TerminatorKind};

use log;
use rustc_middle::ty::TyKind::FnDef;
use std::collections::HashSet;

pub struct PlaceTracer<'pt, 'tcx> {
    body_ids: &'pt Vec<LocalDefId>,
    traced: HashSet<PlaceRef<'tcx>>,
}

impl<'pt, 'tcx> PlaceTracer<'pt, 'tcx> {
    pub fn new(body_ids: &'pt Vec<LocalDefId>) -> Self {
        PlaceTracer {
            body_ids,
            traced: HashSet::new(),
        }
    }
}

impl<'pt, 'tcx> Visitor<'tcx> for PlaceTracer<'pt, 'tcx> {
    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        if !self.traced.contains(&place.as_ref()) {
            log::trace!("Found new place: {:?}", place);
            self.traced.insert(place.as_ref());
        }
        self.super_place(place, context, location)
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        match terminator.kind {
            TerminatorKind::Call {
                ref func,
                args: _,
                destination: _,
                cleanup: _,
                from_hir_call,
                fn_span: _,
            } => {
                assert!(from_hir_call, "Inner functions are not supported");

                if let FnDef(def_id, _) = func.constant().unwrap().ty().kind() {
                    log::trace!(
                        "... found {} function call {:?} with def id {:?}",
                        if let Some(local_def_id) = def_id.as_local() {
                            assert!(self.body_ids.contains(&local_def_id));
                            "local"
                        } else {
                            "library"
                        },
                        func.constant().unwrap(),
                        def_id
                    );
                } else {
                    unreachable!("Should be a function definition")
                }
            }
            _ => {}
        }
        self.super_terminator(terminator, location)
    }
}
