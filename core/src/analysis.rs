use crate::Program;

pub mod body_ext;
pub mod constants;
pub mod place_ext;
// pub mod sssa;

pub struct OwnershipAnalysisCtxt<'octxt, 'tcx> {
    program: &'octxt Program<'tcx>,
}

impl<'octxt, 'tcx> OwnershipAnalysisCtxt<'octxt, 'tcx> {
    pub fn new(program: &'octxt Program<'tcx>) -> Self {
        Self { program }
    }
}
