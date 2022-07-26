use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{Field, Local, Location, Place, ProjectionElem},
    ty::TyCtxt,
};
use rustc_mir_dataflow::JoinSemiLattice;

use crate::{
    api::AnalysisResults,
    ownership_analysis,
    usage_analysis::{self, Domain, IntermediateResult, UsageAnalysisContext},
};

// defer to CrateResults instead of exposing it to avoid having to make everything in
// usage_analysis public
pub struct CrateResults<'tcx, 'a>(usage_analysis::CrateResults<'tcx, 'a, MutabilityAnalysis<'a>>);

impl<'tcx, 'a> CrateResults<'tcx, 'a> {
    pub fn collect(
        tcx: TyCtxt<'tcx>,
        fns: &'a [LocalDefId],
        structs: &'a [LocalDefId],
        ownership: &'a ownership_analysis::InterSummary,
    ) -> Self {
        CrateResults(usage_analysis::CrateResults::collect(
            tcx,
            fns,
            structs,
            MutabilityAnalysis { ownership },
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
struct MutabilityAnalysis<'o> {
    ownership: &'o ownership_analysis::InterSummary,
}

impl usage_analysis::Analysis for MutabilityAnalysis<'_> {
    type Result = Mutability;

    fn check_places<'tcx>(
        &self,
        cx: &UsageAnalysisContext<'tcx, '_>,
        state: &mut Domain<Self::Result>,
        l_place: Option<Place<'tcx>>,
        r_place: Option<Place<'tcx>>,
        loc: Location,
    ) {
        let mut mutable = |place: Place<'tcx>| {
            for (base, proj) in place.iter_projections() {
                if matches!(proj, ProjectionElem::Deref) {
                    tracing::trace!(place=?base, "mutable");
                    *state.result_for(cx.tcx, cx.body, base) =
                        IntermediateResult::Definite(Mutability::Mutable);
                }
            }
        };

        tracing::trace!(?l_place, ?r_place, "hello");
        let Some(l_place) = l_place else { return };
        mutable(l_place);

        let Some(r_place) = r_place else { return };
        let l_ownership = self
            .ownership
            .place_result(cx.tcx, cx.def_id, loc, l_place.as_ref());
        let r_ownership = self
            .ownership
            .place_result(cx.tcx, cx.def_id, loc, r_place.as_ref());
        tracing::trace!(?l_ownership, ?r_ownership);
        // if moving out of a pointer, the pointer is mutable
        if l_ownership == Some(true) && r_ownership == Some(true) {
            tracing::trace!("transfer ⇒ mutable");
            mutable(r_place);
        }
    }
}
