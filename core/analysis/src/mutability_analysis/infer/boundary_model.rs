use rustc_hir::def_id::DefId;
use rustc_middle::mir::{visit::Visitor, BasicBlock, Location, Operand, Place, RETURN_PLACE};

use crate::{
    boundary_model::BoundaryModel,
    ssa::rename::{HasSSARenameState, SSANameHandler},
    ty_ext::TyExt,
    Boundary,
};

use super::IntraInfer;

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
        let callee = self.ctxt.call_graph.lookup_function(&callee).unwrap();

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
                    if self.ctxt.body.local_decls[place.local]
                        .ty
                        .is_ptr_but_not_fn_ptr()
                    {
                        let mu = self.use_ptr_local(place.local, location);
                        Some(mu)
                    } else {
                        None
                    }
                } else {
                    self.visit_operand(arg, location);
                    None
                }
            })
            .collect::<Vec<_>>();

        let dest = destination
            .map(|(destination, _)| {
                if destination
                    .ty(self.ctxt.body, self.ctxt.tcx)
                    .ty
                    .is_ptr_but_not_fn_ptr()
                {
                    if self.ctxt.body.local_decls[destination.local]
                        .ty
                        .is_ptr_but_not_fn_ptr()
                    {
                        let mu = if let Some(local) = destination.as_local() {
                            self.define_ptr_local(local, location)
                        } else {
                            self.use_ptr_local(destination.local, location)
                        };
                        return Some(mu);
                    }
                }

                None
            })
            .flatten();

        self.boundaries.push(Boundary {
            callee,
            dest,
            arguments,
        });
    }

    fn model_return(&mut self, location: Location) {
        if self.ctxt.body.local_decls[RETURN_PLACE]
            .ty
            .is_ptr_but_not_fn_ptr()
        {
            let ssa_idx = self.ssa_state().r#use(RETURN_PLACE);
            let mu = self.handle_ptr_use(RETURN_PLACE, ssa_idx, location);
            if let &Some(rep_mu) = &self.ctxt.return_vars_rep {
                self.ctxt.constraint_system.push_eq(rep_mu, mu)
            } else {
                self.ctxt.return_vars_rep = Some(mu);
            }
        }
    }
}
