use graph::implementation::forward_star::Direction;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    Local, Location, Operand, Place, RETURN_PLACE,
};

use crate::{
    boundary_model::BoundaryModel,
    fat_thin_analysis::BoundaryConstraint,
    ssa::rename::{HasSSANameHandler, HasSSARenameState, SSANameHandler},
    ty_ext::TyExt,
};

use super::InferEngine;

impl<'infercx, 'tcx, Handler: SSANameHandler> BoundaryModel<'tcx>
    for InferEngine<'infercx, 'tcx, Handler>
{
    fn model_boundary(
        &mut self,
        _callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        location: Location,
    ) {
        assert_eq!(
            self.ctxt.call_graph.call_sites.len(),
            self.ctxt.call_graph.graph.edges.len()
        );
        let (call_site, edge_data) = self
            .ctxt
            .call_graph
            .graph
            .adjacent_edges(self.ctxt.lambda_ctxt.func, Direction::Outgoing)
            .find(|&(call_site, _)| self.ctxt.call_graph.call_sites[call_site] == location)
            .unwrap();
        /*
        debug_assert_eq!(edge_data.source, self.ctxt.lambda_ctxt.func);
        debug_assert_eq!(
            edge_data.target,
            self.ctxt.call_graph.lookup_function(&callee_did).unwrap()
        );
        */
        for (idx, arg) in args.iter().enumerate() {
            if arg
                .ty(self.ctxt.body, self.ctxt.tcx)
                .is_ptr_but_not_fn_ptr()
            {
                let place = arg
                    .place()
                    .expect("constant in call arguments is not supported");
                let lambdas = self.use_ptr_place(&place, location);
                self.boundary_constraints[call_site].push(BoundaryConstraint::Argument {
                    caller: lambdas,
                    callee: Local::from_usize(idx + 1),
                });
                tracing::debug!(
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
        if destination
            .ty(self.ctxt.body, self.ctxt.tcx)
            .ty
            .is_ptr_but_not_fn_ptr()
        {
            let lambdas = self.try_define_ptr_place(&destination, location);
            self.boundary_constraints[call_site].push(BoundaryConstraint::Return {
                caller: lambdas,
                callee: Place::return_place().local,
            });
            tracing::debug!(
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

    fn model_return(&mut self, location: Location) {
        if self.ctxt.body.local_decls[RETURN_PLACE]
            .ty
            .is_ptr_but_not_fn_ptr()
        {
            let ssa_idx = self.ssa_state().r#use(RETURN_PLACE);
            self.ssa_name_handler()
                .handle_use(RETURN_PLACE, ssa_idx, location);
            self.return_ssa_idx.push(ssa_idx);
        }
    }
}
