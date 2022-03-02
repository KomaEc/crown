use rustc_hir::def_id::DefId;
use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    BasicBlock, Location, Operand, Place,
};

use crate::{
    array_analysis::{assert_fat, assert_thin, infer::Infer, Constraint},
    def_use::IsDefUse,
    ssa::rename::SSANameHandler,
};

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
                        self.model_ptr_offset(args, destination, location)
                    }
                    // if it is core::ptr::<..>::is_null
                    rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "is_null" => {
                        self.model_ptr_is_null(args, destination, location)
                    }
                    _ => unimplemented!(),
                }
            }
        }
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
        let lhs = self.process_lhs(&lhs, location);
        assert_thin(self.ctxt.lambda_ctxt.lambda_map, lhs);
        log::debug!("generate constraint {:?} = 0", lhs);
    }

    pub fn model_ptr_is_null(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling ptr is_null");
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (lhs, _) = destination.unwrap();
        self.visit_place(
            &lhs,
            PlaceContext::MutatingUse(MutatingUseContext::Call),
            location,
        );
        // no constraint is generated
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
            s if s.as_str() == "calloc" => self.model_calloc(args, destination, location),
            s if s.as_str() == "realloc" => self.model_realloc(args, destination, location),
            _ => unimplemented!(),
        }
    }

    pub fn model_calloc(
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
        let lambda = self.process_lhs(&lhs, location);
        assert_fat(self.ctxt.lambda_ctxt.lambda_map, lambda);
        log::debug!("generate constraint {:?} = 1", lambda);
    }

    pub fn model_realloc(
        &mut self,
        args: &Vec<Operand<'tcx>>,
        destination: Option<(Place<'tcx>, BasicBlock)>,
        location: Location,
    ) {
        log::debug!("modelling realloc");
        assert_eq!(args.len(), 2);
        let (rhs, args) = args.split_first().unwrap();
        let rhs = rhs.place().unwrap();
        let rhs = self.process_rhs(&rhs, location);
        assert_fat(self.ctxt.lambda_ctxt.lambda_map, rhs);
        log::debug!("generate constraint {:?} = 1", rhs);
        for arg in args {
            self.visit_operand(arg, location);
        }
        let (lhs, _) = destination.unwrap();
        let lhs = self.process_lhs(&lhs, location);
        let constraint = Constraint(lhs, rhs);
        log::debug!("generate constraint {}", constraint);
        self.ctxt.constraints.push(constraint);
    }
}
