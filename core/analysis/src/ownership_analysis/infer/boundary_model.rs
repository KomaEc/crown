use graph::implementation::forward_star::Direction;
use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    BasicBlock, Local, Location, Operand, Place, RETURN_PLACE,
};

use crate::{
    boundary_model::BoundaryModel,
    ssa::rename::{HasSSARenameState, SSANameHandler},
    ty_ext::TyExt, Boundary,
};

use super::IntraInfer;

impl<'infercx, 'inter, 'tcx, Handler: SSANameHandler> BoundaryModel<'tcx>
    for IntraInfer<'infercx, 'inter, 'tcx, Handler>
{
    fn model_boundary(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        let (call_site, edge_data) = self
            .ctxt
            .call_graph
            .graph
            .adjacent_edges(self.ctxt.func, Direction::Outgoing)
            .find(|&(call_site, _)| self.ctxt.call_graph.call_sites[call_site] == location)
            .unwrap();

        for (idx, arg) in args.iter().enumerate() {
            if arg
                .ty(self.ctxt.body, self.ctxt.tcx)
                .is_ptr_but_not_fn_ptr()
            {
                let place = arg
                    .place()
                    .expect("constant in call arguments is not supported");
                // let lambdas = self.use_ptr_place(&place, location);
                let rhos = self.process_ptr_place(&place, location);
                self.boundaries[call_site].push(Boundary::Parameter {
                    caller: rhos,
                    callee: Local::from_usize(idx + 1),
                });
                log::debug!(
                    "generate boundary constraint ({:?}, {:?}) ≤ ({:?}. {:?})",
                    edge_data.target,
                    Local::from_usize(idx + 1),
                    edge_data.source,
                    arg
                )
            } else {
                self.visit_operand(arg, location)
            }
        }
        if let Some((destination, _)) = destination {
            if destination
                .ty(self.ctxt.body, self.ctxt.tcx)
                .ty
                .is_ptr_but_not_fn_ptr()
            {
                let rhos = self.process_ptr_place(&destination, location);
                let rhos = match rhos {
                    crate::ownership_analysis::infer::PlaceProcessResult::Base { old, new } => {
                        todo!();
                        new
                    },
                    crate::ownership_analysis::infer::PlaceProcessResult::Proj(rhos) => rhos,
                };
                self.boundaries[call_site].push(Boundary::Return(rhos));
                log::debug!(
                    "generate boundary constraint ({:?}, {:?}) ≤ ({:?}, {:?})",
                    edge_data.source,
                    destination,
                    edge_data.target,
                    Place::return_place().local,
                )
            } else {
                self.visit_place(
                    &destination,
                    PlaceContext::MutatingUse(MutatingUseContext::Call),
                    location,
                )
            }
        }
    }

    fn model_return(&mut self, location: Location) {
        log::error!("TODO: process return ssa indices!");
        log::debug!("Generate constraints upon function exit");
        for (local, local_decl) in self.ctxt.body.local_decls.iter_enumerated().skip(1) {
            if local_decl.ty.is_ptr_but_not_fn_ptr() {
                let ssa_idx = self.ssa_state().r#use(local);
                // don't go through extra handlers since it is not an actual use
                let rhos = self.ctxt.handle_use(local, ssa_idx, location);
                self.ctxt.constraint_system.assume(rhos.start, false);
            }
        }
    }
}
