use rustc_middle::ty::TyCtxt;

use crate::CrateInfo;

pub mod body_ext;
pub mod constants;
pub mod def_sites;
pub mod join_points;
pub mod place_ext;
pub mod state;
pub mod ty_ext;
pub mod constraint;
// pub mod sssa;

pub struct OwnershipAnalysisCtxt<'octxt, 'tcx> {
    program: &'octxt CrateInfo<'tcx>,
}

impl<'octxt, 'tcx> OwnershipAnalysisCtxt<'octxt, 'tcx> {
    pub fn new(program: &'octxt CrateInfo<'tcx>) -> Self {
        Self { program }
    }
}
