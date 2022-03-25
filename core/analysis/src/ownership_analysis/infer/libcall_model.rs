use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        BasicBlock, Location, Operand, Place,
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

    fn model_ptr_is_null(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        assert_eq!(args.len(), 1);
        let rhs = args.first().unwrap();
        let rhs = rhs.place().unwrap();
        match self.process_ptr_place(&rhs, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.push_eq(old.start, new.start)
            }
            PtrPlaceDefResult::Proj(_) => {}
        }
        let (lhs, _) = destination.unwrap();
        self.visit_place(
            &lhs,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
    }

    fn model_malloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        assert_eq!(args.len(), 1);
        let rhs = args.first().unwrap();
        self.visit_operand(rhs, location);
        let (lhs, _) = destination.unwrap();
        match self.process_ptr_place(&lhs, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, false);
                self.ctxt.constraint_system.assume(new.start, true)
            }
            PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, true),
        }
    }

    fn model_calloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (lhs, _) = destination.unwrap();
        match self.process_ptr_place(&lhs, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, false);
                self.ctxt.constraint_system.assume(new.start, true)
            }
            PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, true),
        }
    }

    fn model_realloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // assert_eq!(args.len(), 2);
        let ([ptr, sz], empty) = args.split_array_ref();
        assert!(empty.is_empty());
        let ptr = ptr
            .place()
            .expect("The first argument of realloc should not be constant");
        match self.process_ptr_place(&ptr, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, true);
                self.ctxt.constraint_system.assume(new.start, false)
            }
            PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, true),
        }
        self.visit_operand(sz, location);
        let (lhs, _) = destination.unwrap();
        match self.process_ptr_place(&lhs, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, false);
                self.ctxt.constraint_system.assume(new.start, true)
            }
            PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, true),
        }
    }

    fn model_free(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
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
