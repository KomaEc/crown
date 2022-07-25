//! Playing with ORC and rustc

use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    Location, Place,
};

use crate::Program;

impl<'tcx> Program<'tcx> {
    pub fn inspect_nested_places(&self) {
        struct Vis;
        impl<'tcx> Visitor<'tcx> for Vis {
            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                context: PlaceContext,
                _location: Location,
            ) {
                let (PlaceContext::MutatingUse(..) | PlaceContext::NonMutatingUse(..)) = context else { return };

                let mut derefed = false;
                let mut offsetted = false;

                for projection_elem in place.projection {
                    match projection_elem {
                        rustc_middle::mir::ProjectionElem::Deref => {
                            if derefed {
                                panic!("nested derefs found: {:?}", place)
                            }
                            if offsetted {
                                panic!("deref offseted place: {:?}", place)
                            }
                            derefed = true;
                        }
                        rustc_middle::mir::ProjectionElem::Field(_, _) => {
                            offsetted = true;
                        }
                        _ => continue,
                    }
                }
            }
        }
        for &did in &self.call_graph.functions {
            let body = self.tcx.optimized_mir(did);
            Vis.visit_body(body);
        }
    }

    pub fn print_mir(&self) {
        self.call_graph.functions.iter().for_each(|&fn_did| {
            let body = self.tcx.optimized_mir(fn_did);
            rustc_middle::mir::pretty::write_mir_fn(
                self.tcx,
                body,
                &mut |_, _| Ok(()),
                &mut std::io::stdout(),
            )
            .unwrap();
        });
    }
}
