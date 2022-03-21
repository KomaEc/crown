use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor},
        CastKind, Location, Operand, Place, Rvalue,
    },
    ty::TyCtxt,
};

use crate::{
    libcall_model::LibCallModel, ownership_analysis::infer::PtrPlaceDefResult,
    ssa::rename::SSANameHandler,
};

use super::IntraInfer;

impl<'infercx, 'tcx, Handler: SSANameHandler> LibCallModel<'tcx>
    for IntraInfer<'infercx, 'tcx, Handler>
{
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.ctxt.tcx
    }

    fn model_malloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, rustc_middle::mir::BasicBlock)>,
        location: Location,
    ) {
        assert_eq!(args.len(), 1);
        let rhs = args.first().unwrap();
        self.visit_operand(rhs, location);
        let (lhs, _) = destination.unwrap();
        match self.process_ptr_place(&lhs, location) {
            PtrPlaceDefResult::Base { old, new } => {
                /// TODO
                self.ctxt.constraint_system.assume(old.start, false);
                self.ctxt.constraint_system.assume(new.start, true)
            }
            PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, true),
        }
    }

    fn model_free(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, rustc_middle::mir::BasicBlock)>,
        location: Location,
    ) {
        assert_eq!(args.len(), 1);
        let rhs = args.first().unwrap();
        if let Operand::Move(rhs) | Operand::Copy(rhs) = rhs {
            match self.process_ptr_place(rhs, location) {
                PtrPlaceDefResult::Base { old, new } => {
                    self.ctxt.constraint_system.assume(old.start, true);
                    self.ctxt.constraint_system.assume(new.start, false)
                }
                PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, true),
            }
        } else {
            log::debug!("This terminator is not processed due to constant operand type!");
            self.visit_operand(rhs, location)
        }

        let (lhs, _) = destination.unwrap();
        self.visit_place(
            &lhs,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
    }
}
