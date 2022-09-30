use std::ops::Range;

use rustc_middle::{
    mir::{visit::Visitor, BasicBlock, Location, Operand, Place},
    ty::TyCtxt,
};

use crate::{
    fat_thin_analysis::{infer::InferEngine, Lambda},
    libcall_model::LibCallModel,
    ssa::rename::SSANameHandler,
    utils::range_ext::RangeExt,
};

impl<'infercx, 'tcx, Handler: SSANameHandler> InferEngine<'infercx, 'tcx, Handler> {
    pub fn assume_call_argument(
        &mut self,
        arg: &Operand<'tcx>,
        value: bool,
        location: Location,
    ) -> Lambda {
        let arg = arg.place().unwrap();
        let lambda = self.use_place_assume_simple_ptr(&arg, location);
        self.ctxt.lambda_ctxt.assume(lambda, value);
        lambda
    }

    pub fn assume_call_return(&mut self, dest: &Place<'tcx>, value: bool, location: Location) {
        let lambda = self.define_place_assume_simple_ptr(dest, location);
        self.ctxt.lambda_ctxt.assume(lambda, value);
    }
}

/// Modelling library calls
impl<'infercx, 'tcx, Handler: SSANameHandler> LibCallModel<'tcx>
    for InferEngine<'infercx, 'tcx, Handler>
{
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.ctxt.tcx
    }

    fn model_ptr_offset(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // tracing::debug!("modelling ptr offset");
        assert_eq!(args.len(), 2);
        let ([p, n], _) = args.split_array_ref();
        let rhs = p.place().expect("input to offset should not be a constant");
        let (lhs, _) = destination.unwrap();

        let Range {
            start: rhs_outtermost,
            end: rhs_end,
        } = self.use_ptr_place(&rhs, location);
        self.visit_operand(n, location);
        let Range {
            start: lhs_outtermost,
            end: lhs_end,
        } = self.try_define_ptr_place(&lhs, location);

        let lhs_inners = Range {
            start: lhs_outtermost + 1,
            end: lhs_end,
        };
        let rhs_inners = Range {
            start: rhs_outtermost + 1,
            end: rhs_end,
        };

        assert_eq!(lhs_inners.len(), rhs_inners.len());

        for (lhs, rhs) in std::iter::zip(lhs_inners, rhs_inners) {
            self.ctxt.constraints.push_le(lhs, rhs)
        }
        self.ctxt.lambda_ctxt.assume(lhs_outtermost, false);
    }

    fn model_calloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // tracing::debug!("modelling calloc");
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (lhs, _) = destination.unwrap();
        self.assume_call_return(&lhs, true, location);
    }

    fn model_realloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // tracing::debug!("modelling realloc");
        assert_eq!(args.len(), 2);
        let (rhs, args) = args.split_first().unwrap();
        let rhs = self.assume_call_argument(rhs, true, location);
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (lhs, _) = destination.unwrap();
        let lhs = self.define_place_assume_simple_ptr(&lhs, location);
        self.ctxt.constraints.push_le(lhs, rhs);
    }

    /// Modelling: the return value of `malloc` is definitely thin
    fn model_malloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // tracing::debug!("modelling malloc");
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (dest, _) = destination.unwrap();
        self.assume_call_return(&dest, false, location)
    }

    /// Spec: `void * memmove ( void * destination, const void * source, size_t num );`
    /// where `destination` is returned.
    ///
    /// Modelling: return value = destination (as they should be the same thing)
    fn model_memmove(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // tracing::debug!("modelling memmove");
        assert_eq!(args.len(), 3);
        let (dest, args) = args.split_first().unwrap();
        let dest = dest.place().unwrap();
        let dest = self.use_place_assume_simple_ptr(&dest, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (ret, _) = destination.unwrap();
        let ret = self.define_place_assume_simple_ptr(&ret, location);
        self.ctxt.constraints.push_eq(ret, dest);
        /*
        // Modelling: `destination`, `source`, and return value should all be fat.
        assert_eq!(args.len(), 3);
        let ([dest, src, num], _) = args.split_array_ref::<3>();
        let _ = self.assume_call_argument(dest, true, location);
        let _ = self.assume_call_argument(src, true, location);
        self.visit_operand(num, location);
        let (dest, _) = destination.unwrap();
        self.assume_call_return(&dest, true, location);
        */
    }

    /// Spec: `void * memcpy ( void * destination, const void * source, size_t num );`
    /// where `destination` is returned.
    ///
    /// Modelling: identical to `memmove`.
    fn model_memcpy(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // tracing::debug!("modelling memcpy");
        assert_eq!(args.len(), 3);
        let (dest, args) = args.split_first().unwrap();
        let dest = dest.place().unwrap();
        let dest = self.use_place_assume_simple_ptr(&dest, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (ret, _) = destination.unwrap();
        let ret = self.define_place_assume_simple_ptr(&ret, location);
        self.ctxt.constraints.push_eq(ret, dest);
        /*
        assert_eq!(args.len(), 3);
        let ([dest, src, num], _) = args.split_array_ref::<3>();
        let _ = self.assume_call_argument(dest, true, location);
        let _ = self.assume_call_argument(src, true, location);
        self.visit_operand(num, location);
        let (dest, _) = destination.unwrap();
        self.assume_call_return(&dest, true, location);
        */
    }

    /// Spec: `void * memset ( void * ptr, int value, size_t num );`
    /// where `ptr` is returned.
    ///
    /// Modelling: identical to `memmove`.
    fn model_memset(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // tracing::debug!("modelling memset");
        assert_eq!(args.len(), 3);
        let (ptr, args) = args.split_first().unwrap();
        let ptr = ptr.place().unwrap();
        let ptr = self.use_place_assume_simple_ptr(&ptr, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (ret, _) = destination.unwrap();
        let ret = self.define_place_assume_simple_ptr(&ret, location);
        self.ctxt.constraints.push_eq(ret, ptr);
        /*
        // Modelling: `ptr` and return value should both be fat.
        assert_eq!(args.len(), 3);
        let (ptr, args) = args.split_first().unwrap();
        let _ = self.assume_call_argument(ptr, true, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (dest, _) = destination.unwrap();
        self.assume_call_return(&dest, true, location);
        */
    }

    /// Spec: `char * strncat ( char * destination, const char * source, size_t num );`
    /// where destination is returned.
    ///
    /// Modelling: identical to `memmove`.
    fn model_strncat(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        // tracing::debug!("modelling strncat");
        assert_eq!(args.len(), 3);
        let (dest, args) = args.split_first().unwrap();
        let dest = dest.place().unwrap();
        let dest = self.use_place_assume_simple_ptr(&dest, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (ret, _) = destination.unwrap();
        let ret = self.define_place_assume_simple_ptr(&ret, location);
        self.ctxt.constraints.push_eq(ret, dest);
        /*
        // Modelling: identical to `memmove`.
        assert_eq!(args.len(), 3);
        let ([dest, src, num], _) = args.split_array_ref::<3>();
        let _ = self.assume_call_argument(dest, true, location);
        let _ = self.assume_call_argument(src, true, location);
        self.visit_operand(num, location);
        let (dest, _) = destination.unwrap();
        self.assume_call_return(&dest, true, location);
        */
    }
}
