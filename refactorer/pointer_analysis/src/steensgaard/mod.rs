//! Steensgaard's pointer analysis algorithm.

pub mod constraint_solving;

use union_find::QuickUnionUf;

use crate::ctxt::PointerAnalysisCtxt;

pub struct SteensgaardResult<'sr, 'tcx> {
    pub alias_group: QuickUnionUf<()>,
    pub ptr_ctxt: PointerAnalysisCtxt<'sr, 'tcx>,
}
