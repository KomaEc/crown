use rustc_hir::def_id::DefId;
use rustc_middle::mir::{
    visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
    BasicBlock, Location, Operand, Place, RETURN_PLACE,
};

use crate::{
    boundary_model::BoundaryModel,
    ssa::rename::{HasSSARenameState, SSANameHandler},
    ty_ext::TyExt,
    Boundary,
};

use super::{IntraInfer, PtrPlaceDefResult};

impl<'infercx, 'tcx, Handler: SSANameHandler> BoundaryModel<'tcx>
    for IntraInfer<'infercx, 'tcx, Handler>
{
    fn model_boundary(
        &mut self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // let caller = self.ctxt.func;
        let callee = self.ctxt.call_graph.lookup_function(&callee).unwrap();

        // f.1 = y
        // This should introduce a pseudo constraint rho_f.1 + rho_y' = rho_y
        // We introduce constraint rho_y' ≤ rho_y. Upon instantiation, if rho_f.1 = 1,
        // then add constraints rho_y = 1 and rho_y' = 0; if rho_f.1 = 0, then add constraint
        // rho_y ≤ rho_y'
        // f.1 = S.f
        // This should introduce a pseudo constraint rho_f.1 ≤ rho_S.f
        let arguments = args
            .iter()
            .map(|arg| {
                if arg
                    .ty(self.ctxt.body, self.ctxt.tcx)
                    .is_ptr_but_not_fn_ptr()
                {
                    let place = arg
                        .place()
                        .expect("constant in call arguments is not supported");
                    let res = self.process_ptr_place(&place, location);
                    match &res {
                        PtrPlaceDefResult::Base { old, new } => {
                            self.ctxt.constraint_system.push_le(new.start, old.start);
                        }
                        PtrPlaceDefResult::Proj(_) => {}
                    }
                    Some(res)
                } else {
                    self.visit_operand(arg, location);
                    None
                }
            })
            .collect::<Vec<_>>();

        // x = f.0
        // This should introduce the constraint rho_x = 0 && pseudo constraint rho_x' = rho_f.0
        // We introduce rho_x = 0. Upon instantiation, assert rho_x' if rho_f.0 is known
        // S.f = f.0
        // This should introduce the pseudo constraint rho_S.f = rho_f.0
        let dest = destination
            .map(|(destination, _)| {
                if destination
                    .ty(self.ctxt.body, self.ctxt.tcx)
                    .ty
                    .is_ptr_but_not_fn_ptr()
                {
                    let res = self.process_ptr_place(&destination, location);
                    match res {
                        PtrPlaceDefResult::Base { old, new } => {
                            self.ctxt.constraint_system.assume(old.start, false);
                            Some(new)
                        }
                        PtrPlaceDefResult::Proj(rhos) => Some(rhos),
                    }
                } else {
                    self.visit_place(
                        &destination,
                        PlaceContext::MutatingUse(MutatingUseContext::Call),
                        location,
                    );
                    None
                }
            })
            .flatten();

        self.boundaries.push(Boundary {
            callee,
            dest,
            arguments,
        });
    }

    fn model_return(&mut self, location: Location) {
        // tracing::error!("TODO: process return ssa indices!");

        if self.ctxt.body.local_decls[RETURN_PLACE]
            .ty
            .is_ptr_but_not_fn_ptr()
        {
            // We do not 'process' `RETURN_PLACE` here because there's no need
            // to generate post variables in return clauses
            let ssa_idx = self.ssa_state().r#use(RETURN_PLACE);
            let rhos = self.handle_ptr_use(RETURN_PLACE, ssa_idx, location);
            if let Some(rep_rhos) = &self.ctxt.return_vars_rep {
                for (rep_rho, rho) in std::iter::zip(rep_rhos.clone(), rhos) {
                    self.ctxt.constraint_system.push_eq(rep_rho, rho)
                }
            } else {
                self.ctxt.return_vars_rep = Some(rhos);
            }
        } else {
            self.visit_local(
                RETURN_PLACE,
                PlaceContext::NonMutatingUse(NonMutatingUseContext::Move),
                location,
            );
        }

        tracing::debug!("Generate constraints upon function exit");
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
