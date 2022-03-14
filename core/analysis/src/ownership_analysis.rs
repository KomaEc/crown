use std::ops::Range;

use rustc_index::vec::IndexVec;
use rustc_middle::ty::TyCtxt;

use crate::{
    call_graph::{CallGraph, Func},
    def_use::OwnershipAnalysisDefUse,
    Analysis, CrateAnalysisCtxt,
};

impl<'tcx> Analysis<'tcx> for CrateSummary<'tcx> {
    const NAME: &'static str = "Ownership Analysis";

    type DefUse = OwnershipAnalysisDefUse;

    type Infer<'a, E>
    where
        'tcx: 'a,
        E: crate::ssa::rename::SSANameHandler,
    = crate::ssa::rename::implementations::PlainRenamer<'a, 'tcx, Self::DefUse, E>;
}

pub struct CrateSummary<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub call_graph: CallGraph,
    pub rho_ctxt: CrateAnalysisCtxt<Rho, Option<bool>>,
    pub globals: Range<usize>,
    pub func_summaries: IndexVec<Func, FuncSummary>,
}

pub struct FuncSummary {
    pub lambda_ctxt: Range<usize>,
    pub constraints: Range<usize>,
    pub func_sig: Vec<Range<Rho>>,
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Rho {
        DEBUG_FORMAT = "ρ_({})"
    }
}

impl range_ext::IsConstraintVariable for Rho {}
