use graph::implementation::forward_star::Direction;
use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    BasicBlock, Local, Location, Operand, Place, RETURN_PLACE,
};

use crate::{
    boundary_model::BoundaryModel,
    ssa::rename::{HasSSARenameState, SSANameHandler},
    ty_ext::TyExt,
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
        todo!()
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
