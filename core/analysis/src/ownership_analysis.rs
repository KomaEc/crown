use std::{marker::PhantomData, ops::Range};

use rustc_index::vec::IndexVec;
use rustc_middle::ty::TyCtxt;
use smallvec::SmallVec;

use crate::{
    call_graph::{CallGraph, Func},
    def_use::{IsDefUse, OwnershipAnalysisDefUse},
    Analysis,
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
    pub globals: Range<usize>,
    func_summaries: IndexVec<Func, FuncSummary>,
}

pub const NESTED_LEVEL_HINT: usize = 1;

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

impl range_ext::IsRustcIndex for Rho {}
