use rustc_hir::def_id::LocalDefId;
use rustc_middle::mir::visit::{PlaceContext, Visitor};
use rustc_middle::mir::{Location, Place, PlaceRef, Terminator, TerminatorKind};

use log;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::TyKind::FnDef;
use std::collections::HashSet;

pub struct PlaceTracer<'pt, 'tcx> {
    body_ids: &'pt [LocalDefId],
    tcx: TyCtxt<'tcx>,
    traced: HashSet<PlaceRef<'tcx>>,
}

impl<'pt, 'tcx> PlaceTracer<'pt, 'tcx> {
    pub fn new(body_ids: &'pt [LocalDefId], tcx: TyCtxt<'tcx>) -> Self {
        PlaceTracer {
            body_ids,
            tcx,
            traced: HashSet::new(),
        }
    }
}

impl<'pt, 'tcx> Visitor<'tcx> for PlaceTracer<'pt, 'tcx> {
    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        if !self.traced.contains(&place.as_ref()) {
            tracing::trace!("Found new place: {:?}", place);
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
                fn_span,
            } => {
                if !from_hir_call {
                    eprintln!(
                        "Inner function {:?} at {:?} is not supported",
                        func, fn_span
                    );
                    panic!();
                }
                // assert!(from_hir_call, "Inner functions are not supported");

                if let FnDef(def_id, _) = func.constant().unwrap().ty().kind() {
                    tracing::trace!(
                        "... found {} function call {:?} with def id {:?}",
                        if let Some(local_def_id) = def_id.as_local() {
                            if self.body_ids.contains(&local_def_id) {
                                assert!(
                                    self.body_ids.contains(&local_def_id),
                                    "expect {}",
                                    self.tcx.def_path_str(*def_id)
                                );
                                "local"
                            } else {
                                "linked C"
                            }
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
