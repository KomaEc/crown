use rustc_hir::def_id::DefId;
use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    BasicBlock, Location, Operand, Place,
};

use crate::{
    array_analysis::{infer::Infer, Lambda},
    def_use::IsDefUse,
    ssa::rename::SSANameHandler,
};

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>>
    Infer<'infercx, 'tcx, DefUse, Handler>
{
    pub fn assume(&mut self, lambda: Lambda, value: bool) {
        let assumption = &mut self.ctxt.lambda_ctxt.lambda_map.assumptions[lambda];
        match assumption {
            &mut Some(val) if val ^ value => panic!("conflict in constraint!"),
            _ => *assumption = Some(value),
        }
        log::debug!(
            "generate constraint {:?} = {}",
            lambda,
            value.then_some(1).unwrap_or(0)
        )
    }

    pub fn assume_call_argument(
        &mut self,
        arg: &Operand<'tcx>,
        value: bool,
        location: Location,
    ) -> Lambda {
        let arg = arg.place().unwrap();
        let lambda = self.process_rhs_assume_simple(&arg, location);
        self.assume(lambda, value);
        lambda
    }

    pub fn assume_call_return(&mut self, dest: &Place<'tcx>, value: bool, location: Location) {
        let lambda = self.process_lhs_assume_simple(dest, location);
        self.assume(lambda, value);
    }
}

impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>>
    Infer<'infercx, 'tcx, DefUse, Handler>
{
    /// Basically, this is self.super_terminator(..)
    pub fn default_model_call(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("... default modelling: introducing no constraint");
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (lhs, _) = destination.unwrap();
        self.visit_place(
            &lhs,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        );
    }
}

/// Modelling library calls
impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>>
    Infer<'infercx, 'tcx, DefUse, Handler>
{
    pub fn model_library_call(
        &mut self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        let def_path = self.ctxt.tcx.def_path(callee);

        // if it is a library call in core::ptr
        if def_path
            .data
            .get(0)
            .map(|d| match d.data {
                rustc_hir::definitions::DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
                _ => false,
            })
            .is_some()
        {
            // if it is core::ptr::<..>::..
            if let Some(d) = def_path.data.get(3) {
                match d.data {
                    // if it is core::ptr::<..>::offset
                    rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "offset" => {
                        self.model_ptr_offset(args, destination, location);
                        return;
                    }
                    // if it is core::ptr::<..>::is_null
                    rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "is_null" => {
                        self.model_ptr_is_null(args, destination, location);
                        return;
                    }
                    _ => {}
                }
            }
        }

        // catch all other library calls that is not modelled
        log::debug!("modelling {}", self.ctxt.tcx.def_path_str(callee));
        self.default_model_call(args, destination, location)
    }
    pub fn model_ptr_offset(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling ptr offset");
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (lhs, _) = destination.unwrap();
        self.assume_call_return(&lhs, false, location);
    }

    pub fn model_ptr_is_null(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling ptr is_null");
        self.default_model_call(args, destination, location)
    }
}

/// Modelling extern libc calls
impl<'infercx, 'tcx, DefUse: IsDefUse, Handler: SSANameHandler<Output = ()>>
    Infer<'infercx, 'tcx, DefUse, Handler>
{
    pub fn model_libc_call(
        &mut self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        let foreign_item = self
            .ctxt
            .tcx
            .hir()
            .expect_foreign_item(callee.expect_local());
        match foreign_item.ident {
            s if s.as_str() == "printf" => self.model_printf(args, destination, location),
            s if s.as_str() == "calloc" => self.model_calloc(args, destination, location),
            s if s.as_str() == "realloc" => self.model_realloc(args, destination, location),
            s if s.as_str() == "malloc" => self.model_malloc(args, destination, location),
            s if s.as_str() == "free" => self.model_free(args, destination, location),
            s if s.as_str() == "memmove" => self.model_memmove(args, destination, location),
            s if s.as_str() == "memcpy" => self.model_memcpy(args, destination, location),
            s if s.as_str() == "memset" => self.model_memset(args, destination, location),
            s if s.as_str() == "strncat" => self.model_strncat(args, destination, location),
            s if s.as_str() == "strcmp" => self.model_strcmp(args, destination, location),
            s if s.as_str() == "strstr" => self.model_strstr(args, destination, location),
            s if s.as_str() == "strlen" => self.model_strlen(args, destination, location),
            s => unimplemented!("extern call to {s} is not supported"),
        }
    }

    fn model_printf(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling printf");
        self.default_model_call(args, destination, location)
    }

    fn model_calloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling calloc");
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
        log::debug!("modelling realloc");
        assert_eq!(args.len(), 2);
        let (rhs, args) = args.split_first().unwrap();
        let rhs = self.assume_call_argument(rhs, true, location);
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (lhs, _) = destination.unwrap();
        let lhs = self.process_lhs_assume_simple(&lhs, location);
        self.ctxt.constraints.push_le(lhs, rhs);
    }

    /// Modelling: the return value of `malloc` is definitely thin
    fn model_malloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling malloc");
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (dest, _) = destination.unwrap();
        self.assume_call_return(&dest, false, location)
    }

    fn model_free(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling free");
        self.default_model_call(args, destination, location)
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
        log::debug!("modelling memmove");
        assert_eq!(args.len(), 3);
        let (dest, args) = args.split_first().unwrap();
        let dest = dest.place().unwrap();
        let dest = self.process_rhs_assume_simple(&dest, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (ret, _) = destination.unwrap();
        let ret = self.process_lhs_assume_simple(&ret, location);
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
        log::debug!("modelling memcpy");
        assert_eq!(args.len(), 3);
        let (dest, args) = args.split_first().unwrap();
        let dest = dest.place().unwrap();
        let dest = self.process_rhs_assume_simple(&dest, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (ret, _) = destination.unwrap();
        let ret = self.process_lhs_assume_simple(&ret, location);
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
        log::debug!("modelling memset");
        assert_eq!(args.len(), 3);
        let (ptr, args) = args.split_first().unwrap();
        let ptr = ptr.place().unwrap();
        let ptr = self.process_rhs_assume_simple(&ptr, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (ret, _) = destination.unwrap();
        let ret = self.process_lhs_assume_simple(&ret, location);
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
        log::debug!("modelling strncat");
        assert_eq!(args.len(), 3);
        let (dest, args) = args.split_first().unwrap();
        let dest = dest.place().unwrap();
        let dest = self.process_rhs_assume_simple(&dest, location);
        for arg in args {
            self.visit_operand(arg, location)
        }
        let (ret, _) = destination.unwrap();
        let ret = self.process_lhs_assume_simple(&ret, location);
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

    /// Spec: `int strcmp ( const char * str1, const char * str2 );`
    ///
    /// Modelling: default.
    fn model_strcmp(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling strcmp");
        self.default_model_call(args, destination, location)
        /*
        // Modelling: `str1`, `str2` should both be fat.
        assert_eq!(args.len(), 2);
        let ([str1, str2], _) = args.split_array_ref::<2>();
        let _ = self.assume_call_argument(str1, true, location);
        let _ = self.assume_call_argument(str2, true, location);
        let (dest, _) = destination.unwrap();
        self.visit_place(
            &dest,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
        */
    }

    /// Spec: `const char * strstr ( const char * str1, const char * str2 );`
    ///
    /// Modelling: default.
    fn model_strstr(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling strstr");
        self.default_model_call(args, destination, location)
        /*
        // Modelling: `str1`, `str2` should both be fat, return value should be unknown.
        // TODO: is this appropriate? Should the return value be thin instead?
        assert_eq!(args.len(), 2);
        let ([str1, str2], _) = args.split_array_ref::<2>();
        let _ = self.assume_call_argument(str1, true, location);
        let _ = self.assume_call_argument(str2, true, location);
        let (dest, _) = destination.unwrap();
        log::warn!("The return ptr of strstr is unconstrained. Should it be assumed thin instead?");
        self.visit_place(
            &dest,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
        */
    }

    /// Spec: `size_t strlen ( const char * str );`
    ///
    /// Modelling: default.
    fn model_strlen(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling strlen");
        self.default_model_call(args, destination, location)
        /*
        // Modelling: `str` should be fat.
        assert_eq!(args.len(), 1);
        let str = args.first().unwrap();
        let _ = self.assume_call_argument(str, true, location);
        let (dest, _) = destination.unwrap();
        self.visit_place(
            &dest,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        )
        */
    }
}
