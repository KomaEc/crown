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

use rustc_hash::FxHashSet;
use rustc_hir::{def_id::LocalDefId, definitions::DefPathData};
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{
        Constant, ConstantKind, Local, Place, PlaceRef, ProjectionElem, Rvalue, StatementKind,
        TerminatorKind, START_BLOCK,
    },
    ty::{TyCtxt, TyKind},
};
use rustc_mir_dataflow::{
    fmt::DebugWithContext, Analysis, AnalysisDomain, Engine, JoinSemiLattice, Results,
    ResultsRefCursor,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Nullability {
    Nullable,
    NonNullable,
    DependsOn(FxHashSet<Dependency>),
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dependency {
    FnArg {
        fn_def: LocalDefId,
        arg_idx: usize,
        nested_level: usize,
    },
    StructField {
        struct_def: LocalDefId,
        field_idx: usize,
        nested_level: usize,
    },
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
            (DependsOn(left), DependsOn(right)) => {
                let union = left.union(&right).cloned().collect();
                if &union != left {
                    *self = DependsOn(union);
                    return true;
                } else {
                    return false;
                }
            }
            (DependsOn(_), Unknown) => return false,
            (DependsOn(_), other) => {
                *self = other.clone();
                return true;
            }
            (Unknown, Unknown) => return false,
            (Unknown, other) => {
                *self = other.clone();
                return true;
            }
        }
    }
}

pub struct CrateResults<'tcx> {
    pub fn_results: IndexVec<LocalDefId, Option<FuncResults<'tcx>>>,
}

impl<'tcx> CrateResults<'tcx> {
    pub fn collect(tcx: TyCtxt<'tcx>, fns: &[LocalDefId]) -> Self {
        let mut fn_results = IndexVec::new();
        for &def_id in fns {
            fn_results.insert(def_id, FuncResults::collect(tcx, def_id));
        }
        resolve_deps(&mut fn_results);
        CrateResults { fn_results }
    }
}

fn resolve_deps(fn_results: &mut IndexVec<LocalDefId, Option<FuncResults<'_>>>) {
    for i in 0..fn_results.len() {
        let def_id = LocalDefId::new(i);
        if fn_results[def_id].is_none() {
            continue;
        }
        resolve_deps_for(fn_results, def_id);
    }
}

fn resolve_deps_for(results: &mut IndexVec<LocalDefId, Option<FuncResults<'_>>>, id: LocalDefId) {
    let mut args = std::mem::take(&mut results[id].as_mut().unwrap().args);
    for arg_nullability in args.iter_mut() {
        let Some(Nullability::DependsOn(deps)) = arg_nullability else { continue };
        let mut final_nullability = Nullability::Unknown;
        'each_dep: for result_ref in deps.iter() {
            let other_nullability;
            match result_ref {
                Dependency::FnArg {
                    fn_def, arg_idx, ..
                } => {
                    if results[*fn_def].is_none() {
                        // we don't have results for this fn. maybe it is extern
                        continue 'each_dep;
                    }
                    resolve_deps_for(results, *fn_def);
                    other_nullability = results[*fn_def]
                        .as_ref()
                        .unwrap()
                        .args
                        .get(*arg_idx)
                        .expect("circular dependency")
                        .as_ref()
                        .expect("non-pointer dependency");
                }
                Dependency::StructField { .. } => todo!(),
            }

            use Nullability::*;
            match (&final_nullability, other_nullability) {
                (DependsOn(_), _) => panic!("how"),
                (_, DependsOn(_)) => panic!("how"),
                (_, NonNullable) => final_nullability = NonNullable,
                (Unknown, Nullable) => final_nullability = Nullable,
                _ => {}
            }
        }
        *arg_nullability = Some(final_nullability);
    }
    results[id].as_mut().unwrap().args = args;
}

pub struct FuncResults<'tcx> {
    tcx: TyCtxt<'tcx>,
    def_id: LocalDefId,
    pub results: Results<'tcx, NullAnalysis<'tcx>>,
    args: Vec<Option<Nullability>>,
}

impl<'tcx> FuncResults<'tcx> {
    fn collect(tcx: TyCtxt<'tcx>, def_id: LocalDefId) -> Self {
        let body = tcx.optimized_mir(def_id);
        let engine = Engine::new_generic(tcx, &body, NullAnalysis::new(tcx));
        let results = engine.iterate_to_fixpoint();

        let mut cursor = ResultsRefCursor::new(&body, &results);
        cursor.seek_to_block_start(START_BLOCK);
        let start_results = cursor.get();
        let args = body
            .args_iter()
            .map(|local| {
                if body.local_decls[local].ty.is_unsafe_ptr() {
                    Some(start_results[local].clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        FuncResults {
            tcx,
            def_id,
            results,
            args,
        }
    }
}

impl Display for FuncResults<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_name = self.tcx.def_path_str(self.def_id.to_def_id());
        let body = self.tcx.optimized_mir(self.def_id);
        let arg_results = self
            .args
            .iter()
            .enumerate()
            .filter_map(|(idx, maybe_nullability)| {
                maybe_nullability.as_ref().map(|nullability| {
                    let span = body.local_decls[(idx + 1).into()].source_info.span;
                    let binding_name = self.tcx.sess.source_map().span_to_snippet(span).unwrap();
                    (binding_name, nullability)
                })
            })
            .collect::<Vec<_>>();
        write!(f, "fn {fn_name} has: {arg_results:?}")?;
        Ok(())
    }
}

pub struct NullAnalysis<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> NullAnalysis<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        NullAnalysis { tcx }
    }

    /// If a `Place` contains a deref of a local, that local is non-nullable. We have to check this
    /// pretty much everywhere, so this function is here to make the rest of the code quieter.
    fn check_place(&self, state: &mut IndexVec<Local, Nullability>, place: PlaceRef) {
        if let PlaceRef {
            local,
            projection: [ProjectionElem::Deref, ..],
        } = place
        {
            state[local] = Nullability::NonNullable;
        }
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
            StatementKind::Assign(box (
                place,
                Rvalue::Use(operand) | Rvalue::Cast(_, operand, _),
            )) if place.projection.is_empty() && operand.place().is_some() => {
                // assigning a pointer to another -> lhs' nullability transfers to rhs (because we
                // are analysing backwards)
                self.check_place(state, operand.place().unwrap().as_ref());
                let lhs = place.as_local().unwrap();
                if let Some(rhs) = operand.place().as_ref().map(Place::as_local).flatten() {
                    state[rhs] = state[lhs].clone();
                }
            }
            StatementKind::Assign(box (lhs_place, Rvalue::Use(operand))) => {
                // assigning through a projection - don't know about nullability anymore, unless
                // one of the projections is a deref of a local
                if let PlaceRef {
                    local,
                    projection: [],
                } = lhs_place.as_ref()
                {
                    state[local] = Nullability::Unknown;
                }
                self.check_place(state, lhs_place.as_ref());
                if let Some(rhs_place) = operand.place() {
                    self.check_place(state, rhs_place.as_ref());
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
            let constant = match func.constant() {
                Some(Constant {
                    literal: ConstantKind::Ty(v),
                    ..
                }) => v,
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
                return;
            }

            let Some(def_id) = def_id.as_local() else { return };

            if let DefPathData::ValueNs(name) = def_path.data.last().unwrap().data {
                let mut non_nullable_arg = |n: usize| {
                    if let Some(local) = args[n].place().unwrap().as_local() {
                        state[local] = Nullability::NonNullable;
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

            for (idx, place) in args
                .iter()
                .enumerate()
                .filter_map(|(idx, op)| op.place().map(|place| (idx, place)))
            {
                self.check_place(state, place.as_ref());
                if let PlaceRef {
                    local,
                    projection: [],
                } = place.as_ref()
                {
                    let dep = Dependency::FnArg {
                        fn_def: def_id,
                        arg_idx: idx,
                        nested_level: 0,
                    };
                    state[local].join(&Nullability::DependsOn([dep].into_iter().collect()));
                }
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
