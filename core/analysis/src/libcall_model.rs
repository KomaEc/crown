//! Models for important libc calls and certain other library calls like `is_null()`

use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{
        visit::{MutatingUseContext, PlaceContext, Visitor},
        BasicBlock, Location, Operand, Place,
    },
    ty::TyCtxt,
};

pub trait LibCallModel<'tcx>: Visitor<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx>;

    #[inline]
    fn default_arg(&mut self, operand: &Operand<'tcx>, location: Location) {
        self.visit_operand(operand, location)
    }

    #[inline]
    fn default_dest(&mut self, destination: Option<(Place<'tcx>, BasicBlock)>, location: Location) {
        if let Some((lhs, _)) = destination {
            self.visit_place(
                &lhs,
                PlaceContext::MutatingUse(MutatingUseContext::Call),
                location,
            );
        }
    }

    /// Basically, this is self.super_terminator(..)
    fn default_model_lib_call(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        tracing::debug!("... default modelling: introducing no constraint");
        for arg in args {
            self.default_arg(arg, location)
        }
        self.default_dest(destination, location)
    }

    fn model_library_call(
        &mut self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        let def_path = self.tcx().def_path(callee);

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
                        tracing::debug!("modelling ptr offset");
                        self.model_ptr_offset(args, destination, location);
                        return;
                    }
                    // if it is core::ptr::<..>::is_null
                    rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "is_null" => {
                        tracing::debug!("modelling ptr is_null");
                        self.model_ptr_is_null(args, destination, location);
                        return;
                    }
                    _ => {}
                }
            }
        }

        // catch all other library calls that is not modelled
        tracing::debug!("modelling {}", self.tcx().def_path_str(callee));
        self.default_model_lib_call(args, destination, location)
    }

    fn model_libc_call(
        &mut self,
        callee: DefId,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        let foreign_item = self.tcx().hir().expect_foreign_item(callee.expect_local());
        tracing::debug!("modelling {}", foreign_item.ident);
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
            s => panic!("extern call to {s} is not supported"),
        }
    }

    fn model_ptr_offset(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location);
    }

    fn model_ptr_is_null(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_printf(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_calloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_realloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_malloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_free(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_memmove(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_memcpy(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_memset(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_strncat(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_strcmp(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_strstr(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }

    fn model_strlen(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        self.default_model_lib_call(args, destination, location)
    }
}
