use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{Field, Local, Place, ProjectionElem},
    ty::TyCtxt,
};
use rustc_mir_dataflow::JoinSemiLattice;

use crate::usage_analysis::{self, Domain, IntermediateResult, UsageAnalysisContext};

// defer to CrateResults instead of exposing it to avoid having to make everything in
// usage_analysis public
pub struct CrateResults<'tcx, 'a>(usage_analysis::CrateResults<'tcx, 'a, MutabilityAnalysis>);

impl<'tcx, 'a> CrateResults<'tcx, 'a> {
    pub fn collect(tcx: TyCtxt<'tcx>, fns: &'a [LocalDefId], structs: &'a [LocalDefId]) -> Self {
        CrateResults(usage_analysis::CrateResults::collect(
            tcx,
            fns,
            structs,
            MutabilityAnalysis,
        ))
    }

    pub fn show(&self, tcx: TyCtxt<'tcx>) {
        self.0.show(tcx)
    }
}

impl<'tcx, 'a> crate::api::AnalysisResults for CrateResults<'tcx, 'a> {
    fn local_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool> {
        self.0.local_result(func, local, ptr_depth)
    }

    fn local_result_at(
        &self,
        func: LocalDefId,
        local: Local,
        loc: rustc_middle::mir::Location,
        ptr_depth: usize,
    ) -> Option<bool> {
        self.0.local_result_at(func, local, loc, ptr_depth)
    }

    fn field_result(&self, def_id: LocalDefId, field: Field, ptr_depth: usize) -> Option<bool> {
        self.0.field_result(def_id, field, ptr_depth)
    }

    fn sig_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool> {
        self.0.sig_result(func, local, ptr_depth)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mutability {
    Mutable,
    Immutable,
}

impl JoinSemiLattice for Mutability {
    fn join(&mut self, other: &Self) -> bool {
        if *self == Mutability::Immutable && *other == Mutability::Mutable {
            *self = Mutability::Mutable;
            return true;
        }
        return false;
    }
}

impl Into<bool> for Mutability {
    fn into(self) -> bool {
        self == Mutability::Mutable
    }
}

impl usage_analysis::AnalysisResult for Mutability {
    const DEFAULT: Self = Mutability::Immutable;
}

#[derive(Clone)]
struct MutabilityAnalysis;

impl usage_analysis::Analysis for MutabilityAnalysis {
    type Result = Mutability;

    fn check_places<'tcx>(
        &self,
        cx: &UsageAnalysisContext<'tcx, '_>,
        state: &mut Domain<Self::Result>,
        l_place: Option<Place<'tcx>>,
        _r_place: Option<Place<'tcx>>,
    ) {
        tracing::trace!(?l_place, ?_r_place, "hello");
        let Some(l_place) = l_place else { return };
        for (base, proj) in l_place.iter_projections() {
            if matches!(proj, ProjectionElem::Deref) {
                tracing::trace!(place=?base, "mutable");
                *state.result_for(cx.tcx, cx.body, base) =
                    IntermediateResult::Definite(Mutability::Mutable);
            }
        }
    }
}
