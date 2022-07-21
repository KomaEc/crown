use crate::Program;

pub mod body_ext;
pub mod call_graph;
pub mod constants;
pub mod place_ext;
pub mod sssa;


pub struct OwnershipAnalysisCtxt<'octxt, 'tcx> {
    program: &'octxt Program<'tcx>,
}