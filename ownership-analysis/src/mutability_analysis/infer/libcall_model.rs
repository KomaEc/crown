use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        BasicBlock, Location, Operand, Place,
    },
    ty::TyCtxt,
};

use super::IntraInfer;
use crate::{libcall_model::LibCallModel, ssa::rename::SSANameHandler};

impl<'infercx, 'tcx, Handler: SSANameHandler> IntraInfer<'infercx, 'tcx, Handler> {
    fn mutating_use_ptr_place(&mut self, place: Place, location: Location) {
        if let Some(local) = place.as_local() {
            let mu = self.use_ptr_local(local, location);
            self.ctxt.constraint_system.assume(mu, true)
        }
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> LibCallModel<'tcx>
    for IntraInfer<'infercx, 'tcx, Handler>
{
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.ctxt.tcx
    }

    /*
    fn model_ptr_is_null(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        assert_eq!(args.len(), 1);
        let rhs = args.first().unwrap();
        let rhs = rhs.place().expect("testing variable nullness only");
        self.nonmutating_use_ptr_place(rhs, location);
        let (lhs, _) = destination.unwrap();
        self.visit_place(
            &lhs,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
    }
    */

    fn model_memmove(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        let ([dest, src, num], _) = args.split_array_ref();
        let dest = dest.place().unwrap();
        self.mutating_use_ptr_place(dest, location);
        self.visit_operand(src, location);
        self.visit_operand(num, location);
        let (lhs, _) = destination.unwrap();
        self.visit_place(
            &lhs,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
    }

    fn model_memcpy(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.model_memmove(args, destination, location)
    }

    fn model_memset(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        let ([ptr, value, num], _) = args.split_array_ref();
        let ptr = ptr.place().unwrap();
        self.mutating_use_ptr_place(ptr, location);
        self.visit_operand(value, location);
        self.visit_operand(num, location);
        let (lhs, _) = destination.unwrap();
        self.visit_place(
            &lhs,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
    }

    fn model_strncat(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.model_memmove(args, destination, location)
    }
}
