//! Pointer nullability analysis
//!
//! Nullable pointers are used in C to emulate `Option<T>`, but all pointers in C are semantically
//! nullable, even though these semantics are not always necessary. This module implements some
//! analysis to determine whether a pointer is ever treated as nullable, so that when a raw pointer
//! is translated into a more idiomatic pointer type like `&T` or `Box<T>`, we can avoid using
//! `Option` to keep the output code clean.
//!
//! We use the following rules to determine, for a given pointer, whether it is nullable:
//! - If a pointer is dereferenced, we can deduce that the pointer is not nullable at that location,
//! and has not been nullable since its last assignment.
//! - If `core::ptr::is_null` is called on a pointer, then it is nullable, and has been since its
//! last assignment.

use rustc_hir::{def_id::LocalDefId, definitions::DefPathData};
use rustc_middle::{
    mir::{
        Constant, ConstantKind, Field, Local, PlaceRef, ProjectionElem, Terminator, TerminatorKind,
    },
    ty::{TyCtxt, TyKind},
};
use rustc_mir_dataflow::JoinSemiLattice;

use crate::usage_analysis::{
    self, Analysis, AnalysisResult, Domain, IntermediateResult, UsageAnalysis,
};

// defer to CrateResults instead of exposing it to avoid having to make everything in
// usage_analysis public
pub struct CrateResults<'tcx, 'a>(usage_analysis::CrateResults<'tcx, 'a, NullAnalysis>);

impl<'tcx, 'a> CrateResults<'tcx, 'a> {
    pub fn collect(tcx: TyCtxt<'tcx>, fns: &'a [LocalDefId], structs: &'a [LocalDefId]) -> Self {
        CrateResults(usage_analysis::CrateResults::collect(tcx, fns, structs))
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
pub enum Nullability {
    Nullable,
    NonNullable,
}

impl JoinSemiLattice for Nullability {
    fn join(&mut self, other: &Self) -> bool {
        if *self == Nullability::NonNullable && *other == Nullability::Nullable {
            *self = Nullability::Nullable;
            return true;
        }
        return false;
    }
}

impl Into<bool> for Nullability {
    fn into(self) -> bool {
        self == Nullability::Nullable
    }
}

impl AnalysisResult for Nullability {
    const DEFAULT: Self = Nullability::Nullable;
}

pub struct NullAnalysis;

impl Analysis for NullAnalysis {
    type Result = Nullability;

    fn check_place<'tcx>(
        cx: &UsageAnalysis<'tcx, '_, Self>,
        state: &mut Domain<Self::Result>,
        place: PlaceRef<'tcx>,
    ) {
        // For every deref in this place, record that the base of the deref was not null
        for (idx, proj) in place.projection.iter().enumerate() {
            if let ProjectionElem::Deref = proj {
                let ptr_place = PlaceRef {
                    local: place.local,
                    projection: &place.projection[..idx],
                };
                *state.result_for(cx.tcx, cx.body, ptr_place) =
                    IntermediateResult::Definite(Nullability::NonNullable);
            }
        }
    }

    fn call<'tcx>(
        cx: &UsageAnalysis<'tcx, '_, Self>,
        state: &mut Domain<Self::Result>,
        terminator: &Terminator<'tcx>,
    ) {
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
        // ::is_null
        let is_is_null = def_path
            .data
            .get(3)
            .map(|d| matches!(d.data, DefPathData::ValueNs(s) if s.as_str() == "is_null"))
            .unwrap_or(false);
        if in_core && in_ptr && is_is_null {
            let place = args[0].place().expect("null check on constant");
            *state.result_for(cx.tcx, cx.body, place.as_ref()) =
                Nullability::Nullable.to_intermediate();
            return;
        }

        if let DefPathData::ValueNs(name) = def_path.data.last().unwrap().data {
            let mut non_nullable_arg = |n: usize| {
                if let Some(local) = args[n].place().unwrap().as_local() {
                    state.locals[local][0] = Nullability::NonNullable.to_intermediate();
                }
            };
            match name.as_str() {
                "strlen" | "free" => {
                    non_nullable_arg(0);
                    return;
                }
                "strcat" | "strncat" | "strcmp" | "strncmp" | "strstr" => {
                    non_nullable_arg(0);
                    non_nullable_arg(1);
                    return;
                }
                _ => {}
            }
        }
    }
}
