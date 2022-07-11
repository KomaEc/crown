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

use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::{def_id::LocalDefId, definitions::DefPathData, FnRetTy, ItemKind};
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{
        BasicBlock, Body, Constant, ConstantKind, Field, Local, PlaceRef, ProjectionElem, Rvalue,
        StatementKind, TerminatorKind, VarDebugInfoContents, START_BLOCK,
    },
    ty::{TyCtxt, TyKind},
};
use rustc_mir_dataflow::{
    fmt::DebugWithContext, Analysis, AnalysisDomain, Engine, JoinSemiLattice, Results,
    ResultsRefCursor,
};
use tracing::{debug_span, trace};

fn get_struct_field<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    place: PlaceRef<'tcx>,
) -> Option<(LocalDefId, Field, usize)> {
    let field_idx = place
        .projection
        .iter()
        .rev()
        .enumerate()
        .find(|(_i, elem)| matches!(elem, ProjectionElem::Field(_, _)));
    if let Some((idx, ProjectionElem::Field(field, _ty))) = field_idx {
        let n_derefs = idx;
        let struct_place = PlaceRef {
            local: place.local,
            projection: &place.projection[..place.projection.len() - idx - 1],
        };
        let struct_def_id = struct_place
            .ty(body, tcx)
            .ty
            .ty_adt_def()
            .unwrap()
            .did
            .as_local()
            .unwrap();
        Some((struct_def_id, *field, n_derefs))
    } else {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Nullability {
    Nullable,
    NonNullable,
    IntraDeps(FxHashSet<IntraDep>),
    InterDeps(FxHashSet<InterDep>),
    Unknown,
}

impl Nullability {
    fn is_definite(&self) -> bool {
        matches!(self, Nullability::Nullable | Nullability::NonNullable)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InterDep {
    FnArg {
        fn_def: LocalDefId,
        arg: Local,
        nested_level: usize,
    },
    StructField {
        struct_def: LocalDefId,
        field_idx: Field,
        nested_level: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntraDep {
    bb_id: BasicBlock,
    local: Local,
    nested_level: usize,
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
            (IntraDeps(_), other) if other.is_definite() => {
                *self = other.clone();
                return true;
            }
            (IntraDeps(left), IntraDeps(right)) => {
                let union = left.union(&right).cloned().collect();
                if &union != left {
                    *self = IntraDeps(union);
                    return true;
                } else {
                    return false;
                }
            }
            (IntraDeps(_), _) => return false,
            (InterDeps(left), InterDeps(right)) => {
                let union = left.union(&right).cloned().collect();
                if &union != left {
                    *self = InterDeps(union);
                    return true;
                } else {
                    return false;
                }
            }
            (InterDeps(_), Unknown) => return false,
            (InterDeps(_), other) => {
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

type CrateFuncResults<'tcx, 'a> = IndexVec<LocalDefId, Option<FuncResults<'tcx, 'a>>>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FnReturnResults(pub IndexVec<LocalDefId, Option<IndexVec<usize, Nullability>>>);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CrateStructResults(
    pub FxHashMap<LocalDefId, IndexVec<Field, IndexVec<usize, Nullability>>>,
);

impl JoinSemiLattice for FnReturnResults {
    fn join(&mut self, other: &Self) -> bool {
        let mut changed = false;
        for (def_id, other) in other.0.iter_enumerated() {
            if let Some(other) = other {
                changed |= self.0[def_id].as_mut().unwrap().join(other);
            }
        }
        changed
    }
}

impl JoinSemiLattice for CrateStructResults {
    fn join(&mut self, other: &Self) -> bool {
        let mut changed = false;
        for (&def_id, struct_results) in &other.0 {
            if !self.0.contains_key(&def_id) {
                changed = true;
            }
            let lhs = self.0.entry(def_id).or_default();
            for (field_idx, field_results) in struct_results.iter_enumerated() {
                if lhs.get(field_idx).is_none() {
                    changed = true;
                }
                lhs.ensure_contains_elem(field_idx, || IndexVec::new());
                let lhs = &mut lhs[field_idx];
                for (nested_level, result) in field_results.iter_enumerated() {
                    if lhs.get(nested_level).is_none() {
                        changed = true;
                    }
                    lhs.ensure_contains_elem(nested_level, || Nullability::Unknown);
                    if lhs[nested_level] != *result {
                        changed = true;
                        lhs[nested_level] = result.clone();
                    }
                }
            }
        }
        changed
    }
}

pub struct CrateResults<'tcx, 'a> {
    pub fn_results: CrateFuncResults<'tcx, 'a>,
    pub struct_results: CrateStructResults,
}

impl<'tcx, 'a> CrateResults<'tcx, 'a> {
    pub fn collect(tcx: TyCtxt<'tcx>, fns: &'a [LocalDefId], structs: &'a [LocalDefId]) -> Self {
        let mut fn_results = IndexVec::new();
        for &def_id in fns {
            fn_results.insert(def_id, FuncResults::collect(tcx, def_id, fns, structs));
        }

        let mut fn_results_iter = fn_results.iter().flatten();
        let first = fn_results_iter.next().unwrap();
        let fn_ret_results = fn_results_iter
            .map(|results| &results.start_results.fn_returns)
            .fold(first.start_results.fn_returns.clone(), |mut acc, x| {
                acc.join(&x);
                acc
            });
        for &def_id in fns {
            let ret_result = fn_ret_results.0[def_id].clone().unwrap();
            fn_results[def_id].as_mut().unwrap().start_results.locals[Local::from_usize(0)]
                .join(&ret_result);
        }

        let struct_results =
            fn_results
                .iter()
                .flatten()
                .fold(CrateStructResults::default(), |mut acc, x| {
                    acc.join(&x.start_results.structs);
                    acc
                });
        let mut results = CrateResults {
            fn_results,
            struct_results,
        };
        resolve_deps(&mut results);
        results
    }
}

impl<'tcx> crate::api::AnalysisResults for CrateResults<'tcx, '_> {
    fn local_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool> {
        Some(
            self.fn_results[func].as_ref().unwrap().start_results.locals[local][ptr_depth]
                == Nullability::Nullable,
        )
    }

    fn local_result_at(
        &self,
        func: LocalDefId,
        local: Local,
        _loc: rustc_middle::mir::Location,
        ptr_depth: usize,
    ) -> Option<bool> {
        self.local_result(func, local, ptr_depth)
    }

    fn field_result(&self, def_id: LocalDefId, field: Field, ptr_depth: usize) -> Option<bool> {
        Some(self.struct_results.0.get(&def_id).unwrap()[field][ptr_depth] == Nullability::Nullable)
    }

    fn sig_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool> {
        self.local_result(func, local, ptr_depth)
    }
}

fn resolve_deps(results: &mut CrateResults) {
    for i in 0..results.fn_results.len() {
        let def_id = LocalDefId::new(i);
        if results.fn_results[def_id].is_none() {
            continue;
        }
        resolve_fn_deps(results, def_id);
    }
}

fn resolve_fn_deps(results: &mut CrateResults, id: LocalDefId) {
    let mut unfinished = true;
    while unfinished {
        let mut start_results = results.fn_results[id]
            .as_mut()
            .unwrap()
            .start_results
            .clone();
        unfinished = false;
        for local_nullabilities in start_results.locals.iter_mut() {
            for nested_nullability in local_nullabilities {
                if let Nullability::InterDeps(deps) = nested_nullability {
                    let result = resolve_dep(results, deps, Some(id));
                    if result.is_none()
                        && deps
                            .iter()
                            .any(|d| matches!(d, InterDep::FnArg { fn_def, .. } if *fn_def == id))
                    {
                        // function depends on itself, skip this local now but keep looping until
                        // it is resolved
                        unfinished = true;
                    } else if result.is_none() {
                        // cyclic dep between fns so we assume the worst
                        *nested_nullability = Nullability::Nullable;
                    } else {
                        *nested_nullability = result.unwrap();
                    }
                } else if *nested_nullability == Nullability::Unknown {
                    *nested_nullability = Nullability::Nullable;
                }
            }
        }
        results.fn_results[id].as_mut().unwrap().start_results = start_results;
    }
}

fn resolve_struct_deps(results: &mut CrateResults, id: LocalDefId) {
    let mut struct_results = std::mem::take(results.struct_results.0.get_mut(&id).unwrap());
    for field in &mut struct_results {
        for nested_nullability in field.iter_mut() {
            if let Nullability::InterDeps(deps) = nested_nullability {
                // this unwrap is wrong but i cba to fix it yet bc it's rare
                *nested_nullability = resolve_dep(results, deps, None).unwrap();
            } else if *nested_nullability == Nullability::Unknown {
                *nested_nullability = Nullability::Nullable;
            }
        }
    }
    *results.struct_results.0.get_mut(&id).unwrap() = struct_results;
}

fn resolve_dep(
    results: &mut CrateResults,
    deps: &FxHashSet<InterDep>,
    skip: Option<LocalDefId>,
) -> Option<Nullability> {
    let mut ret = Nullability::Unknown;
    for result_ref in deps.iter() {
        let other_nullability;
        other_nullability = match result_ref {
            InterDep::FnArg {
                fn_def,
                arg,
                nested_level,
            } => {
                if results.fn_results[*fn_def].is_none() {
                    // we don't have results for this fn. maybe it is extern
                    continue;
                }
                // avoid stack overflow for self-reference
                if Some(*fn_def) != skip {
                    resolve_fn_deps(results, *fn_def);
                }
                results.fn_results[*fn_def]
                    .as_ref()
                    .unwrap()
                    .start_results
                    .locals[*arg][*nested_level]
                    .clone()
            }
            InterDep::StructField {
                struct_def,
                field_idx,
                nested_level,
            } => {
                resolve_struct_deps(results, *struct_def);
                results.struct_results.0[struct_def][*field_idx][*nested_level].clone()
            }
        };

        use Nullability::*;
        match (&ret, other_nullability) {
            (InterDeps(_), _) => panic!("how"),
            (_, InterDeps(_)) => return None, // cyclic
            (_, NonNullable) => ret = NonNullable,
            (Unknown, Nullable) => ret = Nullable,
            _ => {}
        }
    }
    Some(ret)
}

pub struct FuncResults<'tcx, 'a> {
    tcx: TyCtxt<'tcx>,
    def_id: LocalDefId,
    pub results: Results<'tcx, NullAnalysis<'tcx, 'a>>,
    start_results: Domain,
}

impl<'tcx, 'a> FuncResults<'tcx, 'a> {
    fn collect(
        tcx: TyCtxt<'tcx>,
        def_id: LocalDefId,
        fns: &'a [LocalDefId],
        structs: &'a [LocalDefId],
    ) -> Self {
        let fn_name = tcx.def_path_str(def_id.to_def_id());
        let _guard = debug_span!("null analysis", ?fn_name).entered();
        let body = tcx.optimized_mir(def_id);
        let engine = Engine::new_generic(tcx, &body, NullAnalysis::new(tcx, def_id, fns, structs));
        let results = engine.iterate_to_fixpoint();

        let mut cursor = ResultsRefCursor::new(&body, &results);
        cursor.seek_to_block_start(START_BLOCK);
        let mut start_results = cursor.get().clone();

        for (local, local_result) in start_results.locals.iter_enumerated_mut() {
            for (nested_level, nested_result) in local_result.iter_enumerated_mut() {
                if let Nullability::IntraDeps(deps) = nested_result {
                    trace!(?local, nested_level, "intra resolve");
                    *nested_result = deps.iter().fold(Nullability::Unknown, |mut acc, dep| {
                        acc.join(&resolve_intra_dep(&mut cursor, dep.clone()));
                        acc
                    });
                }
            }
        }

        FuncResults {
            tcx,
            def_id,
            results,
            start_results,
        }
    }
}

fn resolve_intra_dep<'tcx>(
    cursor: &mut ResultsRefCursor<'_, '_, 'tcx, NullAnalysis<'tcx, '_>>,
    dep: IntraDep,
) -> Nullability {
    cursor.seek_to_block_start(dep.bb_id);
    let target = cursor.get().locals[dep.local][dep.nested_level].clone();
    match target {
        Nullability::IntraDeps(deps) => {
            return deps.into_iter().fold(Nullability::Unknown, |mut acc, dep| {
                acc.join(&resolve_intra_dep(cursor, dep));
                acc
            });
        }
        Nullability::Unknown => {
            return cursor.analysis().body.predecessors()[dep.bb_id]
                .iter()
                .fold(Nullability::Unknown, |mut acc, &pred| {
                    let dep = IntraDep { bb_id: pred, ..dep };
                    acc.join(&resolve_intra_dep(cursor, dep));
                    acc
                });
        }
        x => return x,
    }
}

impl Display for FuncResults<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_name = self.tcx.def_path_str(self.def_id.to_def_id());
        let body = self.tcx.optimized_mir(self.def_id);
        let local_results = self
            .start_results
            .locals
            .iter_enumerated()
            .map(|(idx, nullability)| {
                let name = body
                    .var_debug_info
                    .iter()
                    .find(|v| matches!(v.value, VarDebugInfoContents::Place(p) if p.local == idx))
                    .map(|v| v.name);
                (idx, name, nullability)
            })
            .filter(|(_, _, nullability)| !nullability.is_empty())
            .collect::<Vec<_>>();
        write!(f, "fn {fn_name} has: {local_results:?}")?;
        Ok(())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Domain {
    // we use IndexVec<usize, _> instead of Vec<_> so we can have impl JoinSemiLattice for free
    locals: IndexVec<Local, IndexVec<usize, Nullability>>,
    structs: CrateStructResults,
    fn_returns: FnReturnResults,
}

impl Domain {
    /// panics if the place is not a raw pointer
    fn result_for<'tcx>(
        &mut self,
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        place: PlaceRef<'tcx>,
    ) -> &mut Nullability {
        if let Some((struct_def_id, field, n_derefs)) = get_struct_field(tcx, body, place) {
            let struct_results = self.structs.0.entry(struct_def_id).or_default();
            struct_results.ensure_contains_elem(field, IndexVec::new);
            struct_results[field].ensure_contains_elem(n_derefs, || Nullability::Unknown);
            &mut self.structs.0.get_mut(&struct_def_id).unwrap()[field][n_derefs]
        } else {
            assert!(place
                .projection
                .iter()
                .all(|e| matches!(e, ProjectionElem::Deref)));
            let n_derefs = place.projection.len();
            &mut self.locals[place.local][n_derefs]
        }
    }
}

impl JoinSemiLattice for Domain {
    fn join(&mut self, other: &Self) -> bool {
        let mut changed = false;
        changed |= self.locals.join(&other.locals);
        changed |= self.structs.join(&other.structs);
        changed |= self.fn_returns.join(&other.fn_returns);
        changed
    }
}

impl<'tcx> DebugWithContext<NullAnalysis<'tcx, '_>> for Domain {}

pub struct NullAnalysis<'tcx, 'a> {
    tcx: TyCtxt<'tcx>,
    body: &'tcx Body<'tcx>,
    fns: &'a [LocalDefId],
    structs: &'a [LocalDefId],
}

impl<'tcx, 'a> NullAnalysis<'tcx, 'a> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        def_id: LocalDefId,
        fns: &'a [LocalDefId],
        structs: &'a [LocalDefId],
    ) -> Self {
        let body = tcx.optimized_mir(def_id);
        NullAnalysis {
            tcx,
            body,
            fns,
            structs,
        }
    }

    /// If a `Place` contains a deref of a local, that local is non-nullable. We have to check this
    /// pretty much everywhere, so this function is here to make the rest of the code quieter.
    fn check_place(&self, state: &mut Domain, place: PlaceRef<'tcx>) {
        for (idx, proj) in place.projection.iter().enumerate() {
            if let ProjectionElem::Deref = proj {
                let ptr_place = PlaceRef {
                    local: place.local,
                    projection: &place.projection[..idx],
                };
                *state.result_for(self.tcx, self.body, ptr_place) = Nullability::NonNullable;
            }
        }
    }
}

impl<'tcx> AnalysisDomain<'tcx> for NullAnalysis<'tcx, '_> {
    type Domain = Domain;
    type Direction = rustc_mir_dataflow::Backward;

    const NAME: &'static str = "pointer_nullability";

    fn bottom_value(&self, body: &rustc_middle::mir::Body<'tcx>) -> Self::Domain {
        let locals = IndexVec::from_iter(body.local_decls.iter().map(|decl| {
            IndexVec::from_iter(
                decl.ty
                    .walk()
                    .take_while(|generic_arg| generic_arg.expect_ty().is_unsafe_ptr())
                    .map(|_| Nullability::Unknown),
            )
        }));

        let hir_ptr_depth = |mut ty: &rustc_hir::Ty| {
            let mut nested_depth = 0;
            while let rustc_hir::TyKind::Ptr(ref mut_ty) = ty.kind {
                nested_depth += 1;
                ty = mut_ty.ty;
            }
            nested_depth
        };
        let structs = CrateStructResults(FxHashMap::from_iter(self.structs.iter().map(|&def_id| {
            let ItemKind::Struct(ref variant_data, _) = self.tcx.hir().expect_item(def_id).kind else { panic!() };
            let fields = variant_data.fields().iter().map(|f| {
                IndexVec::from_iter(std::iter::repeat(Nullability::Unknown).take(hir_ptr_depth(f.ty)))
            });
            (def_id, IndexVec::from_iter(fields))
        })));
        let mut fn_returns = FnReturnResults(IndexVec::new());
        self.fns.iter()
            .map(|&def_id| {
                let ItemKind::Fn(ref sig, _, _) = self.tcx.hir().expect_item(def_id).kind else { panic!() };
                let FnRetTy::Return(ref ty) = sig.decl.output else { return (def_id, IndexVec::new()) };
                (def_id, IndexVec::from_iter(std::iter::repeat(Nullability::Unknown).take(hir_ptr_depth(ty))))
            })
            .for_each(|(def_id, entry)| { fn_returns.0.insert(def_id, entry); });
        Domain {
            locals,
            structs,
            fn_returns,
        }
    }

    fn initialize_start_block(
        &self,
        _body: &rustc_middle::mir::Body<'tcx>,
        _state: &mut Self::Domain,
    ) {
    }
}

impl<'tcx> Analysis<'tcx> for NullAnalysis<'tcx, '_> {
    fn apply_statement_effect(
        &self,
        state: &mut Self::Domain,
        statement: &rustc_middle::mir::Statement<'tcx>,
        loc: rustc_middle::mir::Location,
    ) {
        match &statement.kind {
            StatementKind::Assign(box (
                l_place,
                Rvalue::Use(operand) | Rvalue::Cast(_, operand, _),
            )) if operand.place().is_some() => {
                // assigning a pointer to another -> lhs' nullability transfers to rhs (because we
                // are analysing backwards)
                let l_place = l_place.as_ref();
                let r_place = operand.place().unwrap().as_ref();
                let _guard = debug_span!("known assign", ?l_place, ?r_place, ?loc).entered();
                self.check_place(state, l_place);
                self.check_place(state, r_place);

                if !l_place.ty(self.body, self.tcx).ty.is_unsafe_ptr() {
                    return;
                }
                let mut l_result = state.result_for(self.tcx, self.body, l_place).clone();
                if let Some((struct_def, field_idx, nested_level)) =
                    get_struct_field(self.tcx, self.body, l_place)
                {
                    if !l_result.is_definite() {
                        let dep = InterDep::StructField {
                            struct_def,
                            field_idx,
                            nested_level,
                        };
                        l_result.join(&Nullability::InterDeps([dep].into_iter().collect()));
                    }
                } else {
                    if l_result == Nullability::Unknown {
                        let dep = IntraDep {
                            bb_id: loc.block,
                            local: l_place.local,
                            nested_level: l_place.projection.len(),
                        };
                        l_result = Nullability::IntraDeps([dep].into_iter().collect());
                    }
                }
                trace!(?l_result);
                *state.result_for(self.tcx, self.body, r_place) = l_result;
            }
            StatementKind::Assign(box (l_place, _)) => {
                // assigning through a projection - don't know about nullability anymore, unless
                // one of the projections is a deref of a local
                if !l_place.ty(self.body, self.tcx).ty.is_unsafe_ptr() {
                    return;
                }
                *state.result_for(self.tcx, self.body, l_place.as_ref()) = Nullability::Unknown;
                self.check_place(state, l_place.as_ref());
            }
            _ => {}
        }
    }

    fn apply_terminator_effect(
        &self,
        _state: &mut Self::Domain,
        _terminator: &rustc_middle::mir::Terminator<'tcx>,
        _location: rustc_middle::mir::Location,
    ) {
    }

    fn apply_call_return_effect(
        &self,
        state: &mut Self::Domain,
        bb_id: rustc_middle::mir::BasicBlock,
        _return_places: rustc_mir_dataflow::CallReturnPlaces<'_, 'tcx>,
    ) {
        let terminator = self.body.basic_blocks()[bb_id].terminator.as_ref().unwrap();
        if let TerminatorKind::InlineAsm { .. } = terminator.kind {
            return;
        }
        let TerminatorKind::Call {
            func,
            args,
            destination,
            ..
        } = &terminator.kind else { return };
        if let Some((dest_place, _)) = destination {
            self.check_place(state, dest_place.as_ref());
        }
        let Some(Constant {
            literal: ConstantKind::Ty(constant),
            ..
        }) = func.constant() else { return };
        let TyKind::FnDef(def_id, _) = constant.ty.kind() else { return };
        let def_path = self.tcx.def_path(*def_id);
        // ::core ...
        let in_core = self.tcx.crate_name(def_path.krate).as_str() == "core";
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
            self.check_place(state, place.as_ref());
            *state.result_for(self.tcx, self.body, place.as_ref()) = Nullability::Nullable;
            let local = place.as_local().expect("projections aren't supported yet");
            state.locals[local][0] = Nullability::Nullable;
            return;
        }

        let Some(def_id) = def_id.as_local() else { return };

        if let DefPathData::ValueNs(name) = def_path.data.last().unwrap().data {
            let mut non_nullable_arg = |n: usize| {
                if let Some(local) = args[n].place().unwrap().as_local() {
                    state.locals[local][0] = Nullability::NonNullable;
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
            if !place.ty(self.body, self.tcx).ty.is_unsafe_ptr() {
                continue;
            }
            self.check_place(state, place.as_ref());
            let dep = InterDep::FnArg {
                fn_def: def_id,
                arg: Local::from_usize(idx + 1),
                nested_level: 0,
            };
            let result = state.result_for(self.tcx, self.body, place.as_ref());
            result.join(&Nullability::InterDeps([dep].into_iter().collect()));
        }

        let is_raw_ptr = self
            .tcx
            .fn_sig(def_id)
            .no_bound_vars()
            .unwrap()
            .output()
            .is_unsafe_ptr();
        if self.tcx.is_foreign_item(def_id) || !is_raw_ptr {
            return;
        }

        // lhs transfers to rhs as in normal assignment
        let Some((dest_place, _)) = destination else { return };
        let l_result = state.result_for(self.tcx, self.body, dest_place.as_ref());
        // TODO: handle more levels of nesting, both here and in assignments
        state.fn_returns.0[def_id].as_mut().unwrap()[0] = l_result.clone();
    }
}
