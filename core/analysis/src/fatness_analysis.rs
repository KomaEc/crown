use rustc_hir::{def_id::LocalDefId, definitions::DefPathData};
use rustc_middle::{
    mir::{
        Constant, ConstantKind, Field, Local, Terminator, TerminatorKind,
    },
    ty::{TyCtxt, TyKind},
};
use rustc_mir_dataflow::JoinSemiLattice;

use crate::usage_analysis::{
    self, Analysis, AnalysisResult, Domain, UsageAnalysis,
};

// defer to CrateResults instead of exposing it to avoid having to make everything in
// usage_analysis public
pub struct CrateResults<'tcx, 'a>(usage_analysis::CrateResults<'tcx, 'a, FatnessAnalysis>);

impl<'tcx, 'a> CrateResults<'tcx, 'a> {
    pub fn collect(tcx: TyCtxt<'tcx>, fns: &'a [LocalDefId], structs: &'a [LocalDefId]) -> Self {
        CrateResults(usage_analysis::CrateResults::collect(tcx, fns, structs, FatnessAnalysis))
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
pub enum Fatness {
    Fat,
    Thin,
}

impl JoinSemiLattice for Fatness {
    fn join(&mut self, other: &Self) -> bool {
        if *self == Fatness::Thin && *other == Fatness::Fat {
            tracing::warn!("interesting case");
            *self = Fatness::Fat;
            return true;
        }
        return false;
    }
}

impl Into<bool> for Fatness {
    fn into(self) -> bool {
        self == Fatness::Fat
    }
}

impl AnalysisResult for Fatness {
    const DEFAULT: Self = Fatness::Thin;
}

#[derive(Clone)]
struct FatnessAnalysis;

impl Analysis for FatnessAnalysis {
    type Result = Fatness;

    fn call<'tcx>(
        cx: &UsageAnalysis<'tcx, '_, Self>,
        state: &mut Domain<Self::Result>,
        terminator: &Terminator<'tcx>,
    ) {
        // if .offset() is called on a pointer, then it is fat
        let TerminatorKind::Call {
            func,
            args,
            ..
        } = &terminator.kind else { return };
        let Some(Constant {
            literal: ConstantKind::Ty(constant),
            ..
        }) = func.constant() else { return };
        let TyKind::FnDef(def_id, _) = constant.ty.kind() else { return };
        let def_path = cx.tcx.def_path(*def_id);
        // ::core ...
        let in_core = cx.tcx.crate_name(def_path.krate).as_str() == "core";
        // ::ptr ...
        let in_ptr = def_path
            .data
            .get(0)
            .map(|d| matches!(d.data, DefPathData::TypeNs(s) if s.as_str() == "ptr"))
            .unwrap_or(false);
        // ::{const_ptr, mut_ptr}::{impl} ...
        // ::offset
        let is_offset = def_path
            .data
            .get(3)
            .map(|d| matches!(d.data, DefPathData::ValueNs(s) if s.as_str() == "offset"))
            .unwrap_or(false);
        if in_core && in_ptr && is_offset {
            let place = args[0].place().expect("pointer offset on constant");
            *state.result_for(cx.tcx, cx.body, place.as_ref()) =
                Fatness::Fat.to_intermediate();
            return;
        }
    }
}
