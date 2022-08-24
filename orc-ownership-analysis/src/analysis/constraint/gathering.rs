use rustc_middle::mir::visit::Visitor;

use crate::analysis::OwnershipAnalysisCtxt;

impl<'octxt, 'tcx> Visitor<'tcx> for OwnershipAnalysisCtxt<'octxt, 'tcx> {}
