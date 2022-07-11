use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        Location, Operand, Place,
    },
    ty::TyCtxt,
};

use crate::{
    libcall_model::LibCallModel, ownership_analysis::infer::PtrPlaceDefResult,
    ssa::rename::SSANameHandler, ty_ext::TyExt,
};

use super::IntraInfer;

impl<'infercx, 'tcx, Handler: SSANameHandler> IntraInfer<'infercx, 'tcx, Handler> {
    fn lend_arg_assume_non_constant(&mut self, arg: &Operand<'tcx>, location: Location) {
        let arg = arg.place().unwrap();
        match self.process_ptr_place(&arg, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.push_eq(old.start, new.start)
            }
            PtrPlaceDefResult::Proj(_) => {}
        }
    }

    fn send_owning_arg_assume_non_constant(&mut self, arg: &Operand<'tcx>, location: Location) {
        let ptr = arg
            .place()
            .expect("The first argument of realloc should not be constant");
        match self.process_ptr_place(&ptr, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, true);
                self.ctxt.constraint_system.assume(new.start, false)
            }
            PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, true),
        }
    }

    fn receive_transient_dest(&mut self, ret: Place<'tcx>, location: Location) {
        match self.process_ptr_place(&ret, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, false);
                self.ctxt.constraint_system.assume(new.start, false)
            }
            PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, false),
        }
    }

    fn receive_owning_dest(&mut self, ret: Place<'tcx>, location: Location) {
        match self.process_ptr_place(&ret, location) {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, false);
                self.ctxt.constraint_system.assume(new.start, true)
            }
            PtrPlaceDefResult::Proj(f) => self.ctxt.constraint_system.assume(f.start, true),
        }
    }
}

impl<'infercx, 'tcx, Handler: SSANameHandler> LibCallModel<'tcx>
    for IntraInfer<'infercx, 'tcx, Handler>
{
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.ctxt.tcx
    }

    fn default_arg(&mut self, arg: &Operand<'tcx>, location: Location) {
        if let Some(arg) = arg.place() {
            if arg
                .ty(self.ctxt.body, self.ctxt.tcx)
                .ty
                .is_ptr_but_not_fn_ptr()
            {
                match self.process_ptr_place(&arg, location) {
                    PtrPlaceDefResult::Base { old, new } => {
                        self.ctxt.constraint_system.push_le(new.start, old.start)
                    }
                    PtrPlaceDefResult::Proj(_) => {}
                }
                return;
            }
        }
        self.visit_operand(arg, location)
    }

    fn model_ptr_is_null(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        lhs: Place<'tcx>,
        location: Location,
    ) {
        assert_eq!(args.len(), 1);
        let rhs = args.first().unwrap();
        self.lend_arg_assume_non_constant(rhs, location);
        self.visit_place(
            &lhs,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
    }

    /// the receiver of ptr_offset must not have ownership
    fn model_ptr_offset(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        ptr: Place<'tcx>,
        location: Location,
    ) {
        for arg in args {
            self.default_arg(arg, location)
        }
        self.receive_transient_dest(ptr, location);
    }

    fn model_malloc(&mut self, args: &Vec<Operand<'tcx>>, lhs: Place<'tcx>, location: Location) {
        assert_eq!(args.len(), 1);
        let rhs = args.first().unwrap();
        self.visit_operand(rhs, location);
        self.receive_owning_dest(lhs, location);
    }

    fn model_calloc(&mut self, args: &Vec<Operand<'tcx>>, lhs: Place<'tcx>, location: Location) {
        for arg in args {
            self.visit_operand(arg, location);
        }
        self.receive_owning_dest(lhs, location);
    }

    fn model_realloc(&mut self, args: &Vec<Operand<'tcx>>, lhs: Place<'tcx>, location: Location) {
        // assert_eq!(args.len(), 2);
        let ([ptr, sz], empty) = args.split_array_ref();
        assert!(empty.is_empty());
        self.send_owning_arg_assume_non_constant(ptr, location);
        self.visit_operand(sz, location);
        self.receive_owning_dest(lhs, location);
    }

    fn model_free(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        location: Location,
    ) {
        assert_eq!(args.len(), 1);
        let rhs = args.first().unwrap();
        self.send_owning_arg_assume_non_constant(rhs, location);
        self.default_dest(destination, location);
    }

    /// Polymorphic signature
    /// `memmove: (&'rho i8, &'transient i8, _) -> &'rho i8
    fn model_memmove(&mut self, args: &Vec<Operand<'tcx>>, ret: Place<'tcx>, location: Location) {
        let ([dest, src, num], _) = args.split_array_ref();
        let dest = dest.place().unwrap();
        let dest = self.process_ptr_place(&dest, location);
        let dest_to_be_processed = match dest {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.push_le(new.start, old.start);
                old.start
            }
            PtrPlaceDefResult::Proj(f) => f.start,
        };
        // self.lend_arg_assume_non_constant(dest, location);
        self.lend_arg_assume_non_constant(src, location);
        self.default_arg(num, location);
        let ret = self.process_ptr_place(&ret, location);
        let ret_to_be_processed = match ret {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, false);
                new.start
            }
            PtrPlaceDefResult::Proj(f) => f.start,
        };
        // dest and ret should have the same signature
        self.ctxt
            .constraint_system
            .push_eq(dest_to_be_processed, ret_to_be_processed);

        // self.receive_transient_dest(ret, location);
    }

    fn model_memcpy(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        location: Location,
    ) {
        self.model_memmove(args, destination, location)
    }

    fn model_memset(&mut self, args: &Vec<Operand<'tcx>>, ret: Place<'tcx>, location: Location) {
        let ([ptr, value, num], _) = args.split_array_ref();
        let ptr = ptr.place().unwrap();
        let ptr = self.process_ptr_place(&ptr, location);
        let ptr_to_be_processed = match ptr {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.push_le(new.start, old.start);
                old.start
            }
            PtrPlaceDefResult::Proj(f) => f.start,
        };
        self.default_arg(value, location);
        self.default_arg(num, location);
        let ret = self.process_ptr_place(&ret, location);
        let ret_to_be_processed = match ret {
            PtrPlaceDefResult::Base { old, new } => {
                self.ctxt.constraint_system.assume(old.start, false);
                new.start
            }
            PtrPlaceDefResult::Proj(f) => f.start,
        };
        self.ctxt
            .constraint_system
            .push_eq(ptr_to_be_processed, ret_to_be_processed);
    }

    fn model_strncat(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        location: Location,
    ) {
        self.model_memmove(args, destination, location)
    }

    fn model_strcmp(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        location: Location,
    ) {
        let ([str1, str2], _) = args.split_array_ref();
        self.lend_arg_assume_non_constant(str1, location);
        self.lend_arg_assume_non_constant(str2, location);
        self.default_dest(destination, location)
    }

    fn model_strlen(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Place<'tcx>,
        location: Location,
    ) {
        let str = args.first().unwrap();
        self.lend_arg_assume_non_constant(str, location);
        self.default_dest(destination, location)
    }

    fn model_strstr(&mut self, args: &Vec<Operand<'tcx>>, ret: Place<'tcx>, location: Location) {
        let ([str1, str2], _) = args.split_array_ref();
        self.lend_arg_assume_non_constant(str1, location);
        self.lend_arg_assume_non_constant(str2, location);
        self.receive_transient_dest(ret, location);
    }
}
