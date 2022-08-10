//! Playing with ORC and rustc

use rustc_hash::FxHashSet;
use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    Body, Location, Place, Rvalue,
};

use crate::{
    analysis::{place_ext::PlaceExt, OwnershipAnalysisCtxt},
    CrateInfo,
};

impl<'tcx> CrateInfo<'tcx> {
    pub fn verify_shape_of_place(&self) {
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
        for did in self.functions() {
            let body = self.tcx.optimized_mir(did);
            Vis.visit_body(body);
        }
    }

    pub fn compute_percentage_of_non_address_taking_functions(&self) {
        struct Vis;
        impl<'tcx> Visitor<'tcx> for Vis {
            fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, _location: Location) {
                if let Rvalue::AddressOf(rustc_ast::Mutability::Mut, _) = rvalue {
                    panic!("{:?} to be catched", rvalue)
                }
            }
        }
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let n_address_taking_functions = self
            .functions()
            .iter()
            .filter(|&did| {
                let body = self.tcx.optimized_mir(did);
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Vis.visit_body(body)))
                    .is_ok()
            })
            .count();
        std::panic::set_hook(prev_hook);
        let percentage = n_address_taking_functions as f64 / self.functions().len() as f64;
        println!("-------------stat: percntage of non address taking functions-----------------");
        println!("                   {percentage}");
    }

    pub fn inspect_place_abs(&self) {
        struct Vis<'me, 'tcx>(
            &'me OwnershipAnalysisCtxt<'me, 'tcx>,
            &'me Body<'tcx>,
            FxHashSet<Place<'tcx>>,
        );
        impl<'me, 'tcx> Visitor<'tcx> for Vis<'me, 'tcx> {
            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                context: PlaceContext,
                _location: Location,
            ) {
                let visited = &mut self.2;
                if visited.contains(&place) {
                    return;
                }
                visited.insert(*place);
                let octxt = self.0;
                let body = self.1;
                let (PlaceContext::MutatingUse(..) | PlaceContext::NonMutatingUse(..)) = context else { return };
                if place.projection.len() < 2 {
                    return;
                }
                let Some(place_abs) = place.r#abstract(body, &octxt) else { return };
                tracing::debug!("{:?} -> {}", place, place_abs)
            }
        }
        let octxt = OwnershipAnalysisCtxt::new(&*self);
        for did in self.functions() {
            let body = self.tcx.optimized_mir(did);
            let mut vis = Vis(&octxt, body, FxHashSet::default());
            vis.visit_body(body);
        }
    }

    pub fn print_mir(&self) {
        self.functions().iter().for_each(|&fn_did| {
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
