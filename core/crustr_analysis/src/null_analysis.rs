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

use std::fmt::Display;

use rustc_hir::{def_id::LocalDefId, definitions::DefPathData};
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        ConstantKind, Local, Place, PlaceRef, ProjectionElem, Rvalue, StatementKind,
        TerminatorKind, START_BLOCK,
    },
    ty::{TyCtxt, TyKind},
};
use rustc_mir_dataflow::{
    fmt::DebugWithContext, Analysis, AnalysisDomain, Engine, JoinSemiLattice, Results,
    ResultsRefCursor,
};
use tracing::debug;

pub struct NullAnalysisResults<'tcx> {
    tcx: TyCtxt<'tcx>,
    def_id: LocalDefId,
    results: Results<'tcx, NullAnalysis<'tcx>>,
}

impl<'tcx> NullAnalysisResults<'tcx> {
    pub fn collect(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> Self {
        let body = tcx.optimized_mir(def_id);
        let engine = Engine::new_generic(tcx, &body, NullAnalysis::new(tcx));
        let results = engine.iterate_to_fixpoint();
        NullAnalysisResults {
            tcx,
            def_id,
            results,
        }
    }
}

impl Display for NullAnalysisResults<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_name = self.tcx.def_path_str(self.def_id.to_def_id());
        let body = self.tcx.optimized_mir(self.def_id);
        if body.arg_count == 0 {
            write!(f, "fn {fn_name} has no arguments")?;
            return Ok(());
        }
        let mut cursor = ResultsRefCursor::new(body, &self.results);
        cursor.seek_to_block_start(START_BLOCK);
        let results = cursor.get();
        let arg_results = body
            .args_iter()
            .filter(|local| body.local_decls[*local].ty.is_unsafe_ptr())
            .map(|local| {
                let span = body.local_decls[local].source_info.span;
                let binding_name = self.tcx.sess.source_map().span_to_snippet(span).unwrap();
                (binding_name, results[local])
            })
            .collect::<Vec<_>>();
        write!(f, "fn {fn_name} has: {arg_results:?}")?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nullability {
    Nullable,
    NonNullable,
    Unknown,
}

impl JoinSemiLattice for Nullability {
    fn join(&mut self, other: &Self) -> bool {
        use Nullability::*;
        match (&self, other) {
            (Nullable, _) => return false,
            (NonNullable, Nullable) => {
                *self = Nullable;
                return true;
            }
            (NonNullable, _) => return false,
            (Unknown, Unknown) => return false,
            (Unknown, other) => {
                *self = *other;
                return true;
            }
        }
    }
}

pub struct NullAnalysis<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> NullAnalysis<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        NullAnalysis { tcx }
    }
}

impl<'tcx> DebugWithContext<NullAnalysis<'tcx>> for IndexVec<Local, Nullability> {}

impl<'tcx> AnalysisDomain<'tcx> for NullAnalysis<'tcx> {
    type Domain = IndexVec<Local, Nullability>;
    type Direction = rustc_mir_dataflow::Backward;

    const NAME: &'static str = "pointer_nullability";

    fn bottom_value(&self, body: &rustc_middle::mir::Body<'tcx>) -> Self::Domain {
        // not all locals are pointers, but idk what happens if we use non contiguous indices in
        // here, so just make it big enough to hold all of them
        let locals = body.local_decls.len();
        IndexVec::from_iter(std::iter::repeat(Nullability::Unknown).take(locals))
    }

    fn initialize_start_block(
        &self,
        _body: &rustc_middle::mir::Body<'tcx>,
        _state: &mut Self::Domain,
    ) {
    }
}

impl<'tcx> Analysis<'tcx> for NullAnalysis<'tcx> {
    fn apply_statement_effect(
        &self,
        state: &mut Self::Domain,
        statement: &rustc_middle::mir::Statement<'tcx>,
        _location: rustc_middle::mir::Location,
    ) {
        match &statement.kind {
            StatementKind::Assign(box (place, Rvalue::Use(operand)))
                if place.projection.is_empty() && operand.place().is_some() =>
            {
                let lhs = place.as_local().expect("projections aren't supported yet");
                if let Some(rhs) = operand.place().as_ref().map(Place::as_local).flatten() {
                    state[rhs] = state[lhs];
                }
            }
            StatementKind::Assign(box (place, Rvalue::Use(operand))) => {
                match place.as_ref() {
                    PlaceRef {
                        local,
                        projection: [ProjectionElem::Deref, ..],
                    } => {
                        debug!(?local, "lhs deref");
                        state[local] = Nullability::NonNullable;
                    }
                    PlaceRef {
                        local,
                        projection: [],
                    } => {
                        debug!(?local, "lhs assign");
                        state[local] = Nullability::Unknown;
                    }
                    _ => (),
                }
                if let Some(PlaceRef {
                    local,
                    projection: [ProjectionElem::Deref, ..],
                }) = operand.place().as_ref().map(Place::as_ref)
                {
                    debug!(?local, "rhs deref");
                    state[local] = Nullability::NonNullable;
                }
            }
            _ => {}
        }
    }

    fn apply_terminator_effect(
        &self,
        state: &mut Self::Domain,
        terminator: &rustc_middle::mir::Terminator<'tcx>,
        _location: rustc_middle::mir::Location,
    ) {
        if let TerminatorKind::Call { func, args, .. } = &terminator.kind {
            let constant = func.constant().unwrap();
            let constant = match constant.literal {
                ConstantKind::Ty(v) => v,
                _ => return,
            };
            let def_id = match constant.ty.kind() {
                TyKind::FnDef(def_id, _) => def_id,
                _ => return,
            };
            let def_path = self.tcx.def_path(*def_id);
            // ::core ...
            let in_core = self.tcx.crate_name(def_path.krate).as_str() == "core";
            // ::ptr ...
            let in_ptr = def_path
                .data
                .get(0)
                .map(|d| match d.data {
                    DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
                    _ => false,
                })
                .unwrap_or(false);
            // ::{const_ptr, mut_ptr}::{impl} ...
            // ::is_null
            let is_is_null = def_path
                .data
                .get(3)
                .map(|d| match d.data {
                    DefPathData::ValueNs(s) if s.as_str() == "is_null" => true,
                    _ => false,
                })
                .unwrap_or(false);
            if in_core && in_ptr && is_is_null {
                let place = args[0].place().expect("null check on constant");
                let local = place.as_local().expect("projections aren't supported yet");
                state[local] = Nullability::Nullable;
            }
        }
    }

    fn apply_call_return_effect(
        &self,
        _state: &mut Self::Domain,
        _block: rustc_middle::mir::BasicBlock,
        _return_places: rustc_mir_dataflow::CallReturnPlaces<'_, 'tcx>,
    ) {
    }
}
