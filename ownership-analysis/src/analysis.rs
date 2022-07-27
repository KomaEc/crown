use crate::CrateInfo;

pub mod body_ext;
pub mod constants;
pub mod place_ext;
// pub mod sssa;

pub struct OwnershipAnalysisCtxt<'octxt, 'tcx> {
    program: &'octxt CrateInfo<'tcx>,
}

impl<'octxt, 'tcx> OwnershipAnalysisCtxt<'octxt, 'tcx> {
    pub fn new(program: &'octxt CrateInfo<'tcx>) -> Self {
        Self { program }
    }
}
