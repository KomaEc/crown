use std::fmt::{Debug, Display};

use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::{def_id::LocalDefId, FnRetTy, ItemKind};
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{
        Body, Field, Local, Location, Place, PlaceRef, ProjectionElem, Rvalue, StatementKind,
        Terminator, TerminatorKind, VarDebugInfoContents, START_BLOCK,
    },
    ty::TyCtxt,
};
use rustc_mir_dataflow::{
    fmt::DebugWithContext, AnalysisDomain, Engine, JoinSemiLattice, Results, ResultsRefCursor,
};
use tracing::{debug_span, trace};

use analysis_interface::get_struct_field;

pub(crate) trait AnalysisResult:
    Clone + std::fmt::Debug + Eq + Into<bool> + JoinSemiLattice
{
    // The result to use in case analysis finishes and it is still unknown.
    const DEFAULT: Self;

    fn to_intermediate(self) -> IntermediateResult<Self> {
        IntermediateResult::Definite(self)
    }
}

pub(crate) trait Analysis: Clone + Sized {
    type Result: AnalysisResult;

    fn check_places<'tcx>(
        &self,
        _cx: &UsageAnalysisContext<'tcx, '_>,
        _state: &mut Domain<Self::Result>,
        _l_place: Option<Place<'tcx>>,
        _r_place: Option<Place<'tcx>>,
        _loc: Location,
    ) {
    }

    fn call<'tcx>(
        &self,
        _cx: &UsageAnalysisContext<'tcx, '_>,
        _state: &mut Domain<Self::Result>,
        _terminator: &Terminator<'tcx>,
        _loc: Location,
    ) {
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum IntermediateResult<R: AnalysisResult> {
    Definite(R),
    InterDeps(FxHashSet<InterDep>),
    Unknown,
}

impl<R: AnalysisResult> IntermediateResult<R> {
    fn is_definite(&self) -> bool {
        matches!(self, IntermediateResult::Definite(_))
    }

    fn unwrap_definite(&self) -> R {
        match self {
            IntermediateResult::Definite(v) => v.clone(),
            IntermediateResult::Unknown => R::DEFAULT,
            _ => panic!("result was not resolved properly"),
        }
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

impl<R: AnalysisResult> JoinSemiLattice for IntermediateResult<R> {
    fn join(&mut self, other: &Self) -> bool {
        use IntermediateResult::*;
        match (&self, other) {
            (Definite(l), Definite(r)) => {
                let mut l = l.clone();
                let changed = l.join(r);
                *self = Definite(l);
                return changed;
            }
            (Definite(_), _) => return false,
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

type CrateFuncResults<'tcx, 'a, R> = IndexVec<LocalDefId, Option<FuncResults<'tcx, 'a, R>>>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct FnReturnResults<R: AnalysisResult>(
    pub IndexVec<LocalDefId, Option<IndexVec<usize, IntermediateResult<R>>>>,
);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CrateStructResults<R: AnalysisResult>(
    pub FxHashMap<LocalDefId, IndexVec<Field, IndexVec<usize, IntermediateResult<R>>>>,
);

impl<R: AnalysisResult> Default for CrateStructResults<R> {
    fn default() -> Self {
        CrateStructResults(FxHashMap::default())
    }
}

impl<R: AnalysisResult> JoinSemiLattice for FnReturnResults<R> {
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

impl<R: AnalysisResult> JoinSemiLattice for CrateStructResults<R> {
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
                    lhs.ensure_contains_elem(nested_level, || IntermediateResult::Unknown);
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

pub(crate) struct CrateResults<'tcx, 'a, A: Analysis> {
    pub fn_results: CrateFuncResults<'tcx, 'a, A>,
    pub struct_results: CrateStructResults<A::Result>,
}

impl<'tcx, 'a, A: Analysis> CrateResults<'tcx, 'a, A> {
    pub fn collect(
        tcx: TyCtxt<'tcx>,
        fns: &'a [LocalDefId],
        structs: &'a [LocalDefId],
        analysis: A,
    ) -> Self {
        let mut fn_results = IndexVec::new();
        for &def_id in fns {
            fn_results.insert(
                def_id,
                FuncResults::collect(tcx, def_id, fns, structs, analysis.clone()),
            );
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

        let struct_results = fn_results.iter().flatten().fold(
            CrateStructResults(Default::default()),
            |mut acc, x| {
                acc.join(&x.start_results.structs);
                acc
            },
        );
        let mut results = CrateResults {
            fn_results,
            struct_results,
        };
        resolve_deps(&mut results);
        results
    }

    pub fn show(&self, tcx: TyCtxt<'tcx>) {
        for (struct_def_id, struct_result) in self.struct_results.0.iter() {
            let struct_name = tcx.def_path_str(struct_def_id.to_def_id());
            let struct_def = &tcx.adt_def(*struct_def_id).variants()[0u32.into()];
            for (field, field_result) in struct_result.iter_enumerated() {
                let field_name = &struct_def.fields[field.as_usize()].name;
                println!("{struct_name}.{field_name} has {field_result:?}");
            }
        }
        for result in self.fn_results.iter().filter_map(|v| v.as_ref()) {
            println!("{result}");
        }
    }
}

impl<'tcx, A: Analysis> analysis_interface::AnalysisResults for CrateResults<'tcx, '_, A> {
    fn local_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool> {
        Some(
            self.fn_results[func].as_ref().unwrap().start_results.locals[local][ptr_depth]
                .unwrap_definite()
                .into(),
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
        Some(
            self.struct_results.0.get(&def_id).unwrap()[field][ptr_depth]
                .unwrap_definite()
                .into(),
        )
    }

    fn sig_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool> {
        self.local_result(func, local, ptr_depth)
    }
}

fn resolve_deps<A: Analysis>(results: &mut CrateResults<A>) {
    for i in 0..results.fn_results.len() {
        let def_id = LocalDefId::new(i);
        if results.fn_results[def_id].is_none() {
            continue;
        }
        resolve_fn_deps(results, def_id);
    }
}

fn resolve_fn_deps<A: Analysis>(results: &mut CrateResults<A>, id: LocalDefId) {
    let mut unfinished = true;
    while unfinished {
        let mut start_results = results.fn_results[id]
            .as_mut()
            .unwrap()
            .start_results
            .clone();
        unfinished = false;
        for local_results in start_results.locals.iter_mut() {
            for nested_result in local_results {
                if let IntermediateResult::InterDeps(deps) = nested_result {
                    let result = resolve_dep(results, &deps, Some(id));
                    if result.is_none()
                        && deps
                            .iter()
                            .any(|d| matches!(d, InterDep::FnArg { fn_def, .. } if *fn_def == id))
                    {
                        // function depends on itself, skip this local now but keep looping until
                        // it is resolved
                        unfinished = true;
                    } else if result.is_none() {
                        // cyclic dep between fns so we just throw the whole thing out
                        *nested_result = A::Result::DEFAULT.to_intermediate();
                    } else {
                        *nested_result = result.unwrap();
                    }
                }
            }
        }
        results.fn_results[id].as_mut().unwrap().start_results = start_results;
    }
}

fn resolve_struct_deps<A: Analysis>(results: &mut CrateResults<A>, id: LocalDefId) {
    let mut struct_results = std::mem::take(results.struct_results.0.get_mut(&id).unwrap());
    for field in &mut struct_results {
        for nested_result in field.iter_mut() {
            if let IntermediateResult::InterDeps(deps) = nested_result {
                // this unwrap is wrong but i cba to fix it yet bc it's rare
                *nested_result = resolve_dep(results, deps, None).unwrap();
            }
        }
    }
    *results.struct_results.0.get_mut(&id).unwrap() = struct_results;
}

fn resolve_dep<A: Analysis>(
    results: &mut CrateResults<A>,
    deps: &FxHashSet<InterDep>,
    skip: Option<LocalDefId>,
) -> Option<IntermediateResult<A::Result>> {
    let mut ret = IntermediateResult::Unknown;
    for result_ref in deps.iter() {
        let other_result = match result_ref {
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

        use IntermediateResult::*;
        match (&mut ret, other_result) {
            (InterDeps(_), _) => panic!("how"),
            (_, InterDeps(_)) => return None, // cyclic
            (l @ (Definite(_) | Unknown), r @ (Definite(_) | Unknown)) => {
                l.join(&r);
            }
        }
    }
    Some(ret)
}

pub(crate) struct FuncResults<'tcx, 'a, A: Analysis> {
    tcx: TyCtxt<'tcx>,
    def_id: LocalDefId,
    _results: Results<'tcx, UsageAnalysis<'tcx, 'a, A>>,
    start_results: Domain<A::Result>,
}

impl<'tcx, 'a, A: Analysis> FuncResults<'tcx, 'a, A> {
    fn collect(
        tcx: TyCtxt<'tcx>,
        def_id: LocalDefId,
        fns: &'a [LocalDefId],
        structs: &'a [LocalDefId],
        analysis: A,
    ) -> Self {
        let fn_name = tcx.def_path_str(def_id.to_def_id());
        let _guard = debug_span!("fn", ?fn_name).entered();
        let body = tcx.optimized_mir(def_id);
        let cx = UsageAnalysisContext {
            tcx,
            def_id,
            body,
            fns,
            structs,
        };
        let engine = Engine::new_generic(tcx, &body, UsageAnalysis(cx, analysis));
        let results = engine.iterate_to_fixpoint();

        let mut cursor = ResultsRefCursor::new(&body, &results);
        cursor.seek_to_block_start(START_BLOCK);
        let start_results = cursor.get().clone();

        FuncResults {
            tcx,
            def_id,
            _results: results,
            start_results,
        }
    }
}

impl<A: Analysis> Display for FuncResults<'_, '_, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_name = self.tcx.def_path_str(self.def_id.to_def_id());
        let body = self.tcx.optimized_mir(self.def_id);
        let local_results = self
            .start_results
            .locals
            .iter_enumerated()
            .map(|(idx, results)| {
                let name = body
                    .var_debug_info
                    .iter()
                    .find(|v| matches!(v.value, VarDebugInfoContents::Place(p) if p.local == idx))
                    .map(|v| v.name);
                (idx, name, results)
            })
            .filter(|(_, _, results)| !results.is_empty())
            .collect::<Vec<_>>();
        write!(f, "fn {fn_name} has: {local_results:?}")?;
        Ok(())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub(crate) struct Domain<R: AnalysisResult> {
    // we use IndexVec<usize, _> instead of Vec<_> so we can have impl JoinSemiLattice for free
    pub(crate) locals: IndexVec<Local, IndexVec<usize, IntermediateResult<R>>>,
    structs: CrateStructResults<R>,
    fn_returns: FnReturnResults<R>,
}

impl<R: AnalysisResult> Domain<R> {
    /// panics if the place is not a raw pointer
    pub(crate) fn result_for<'tcx>(
        &mut self,
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        place: PlaceRef<'tcx>,
    ) -> &mut IntermediateResult<R> {
        trace!(?place, "result for");
        if let Some((struct_def_id, field, n_derefs)) = get_struct_field(tcx, body, place) {
            let struct_results = self.structs.0.entry(struct_def_id).or_default();
            struct_results.ensure_contains_elem(field, IndexVec::new);
            struct_results[field].ensure_contains_elem(n_derefs, || IntermediateResult::Unknown);
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

impl<R: AnalysisResult> JoinSemiLattice for Domain<R> {
    fn join(&mut self, other: &Self) -> bool {
        let mut changed = false;
        changed |= self.locals.join(&other.locals);
        changed |= self.structs.join(&other.structs);
        changed |= self.fn_returns.join(&other.fn_returns);
        changed
    }
}

pub(crate) struct UsageAnalysisContext<'tcx, 'a> {
    pub tcx: TyCtxt<'tcx>,
    pub def_id: LocalDefId,
    pub body: &'tcx Body<'tcx>,
    fns: &'a [LocalDefId],
    structs: &'a [LocalDefId],
}

pub(crate) struct UsageAnalysis<'tcx, 'a, A: Analysis>(UsageAnalysisContext<'tcx, 'a>, A);

impl<'tcx, A: Analysis> UsageAnalysis<'tcx, '_, A> {
    fn check_places(
        &self,
        state: &mut Domain<A::Result>,
        l_place: Option<Place<'tcx>>,
        r_place: Option<Place<'tcx>>,
        loc: Location,
    ) {
        self.1.check_places(&self.0, state, l_place, r_place, loc);
    }

    fn call(&self, state: &mut Domain<A::Result>, terminator: &Terminator<'tcx>, loc: Location) {
        self.1.call(&self.0, state, terminator, loc);
    }
}

impl<'tcx, A: Analysis> DebugWithContext<UsageAnalysis<'tcx, '_, A>> for Domain<A::Result> {}

impl<'tcx, A: Analysis> AnalysisDomain<'tcx> for UsageAnalysis<'tcx, '_, A> {
    type Domain = Domain<A::Result>;
    type Direction = rustc_mir_dataflow::Backward;

    const NAME: &'static str = "usage analysis";

    fn bottom_value(&self, body: &rustc_middle::mir::Body<'tcx>) -> Self::Domain {
        let locals = IndexVec::from_iter(body.local_decls.iter().map(|decl| {
            IndexVec::from_iter(
                decl.ty
                    .walk()
                    .take_while(|generic_arg| generic_arg.expect_ty().is_unsafe_ptr())
                    .map(|_| IntermediateResult::Unknown),
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
        let structs = CrateStructResults(FxHashMap::from_iter(self.0.structs.iter().map(|&def_id| {
            let ItemKind::Struct(ref variant_data, _) = self.0.tcx.hir().expect_item(def_id).kind else { panic!() };
            let fields = variant_data.fields().iter().map(|f| {
                IndexVec::from_iter(std::iter::repeat(IntermediateResult::Unknown).take(hir_ptr_depth(f.ty)))
            });
            (def_id, IndexVec::from_iter(fields))
        })));
        let mut fn_returns = FnReturnResults(IndexVec::new());
        self.0.fns.iter()
            .map(|&def_id| {
                let ItemKind::Fn(ref sig, _, _) = self.0.tcx.hir().expect_item(def_id).kind else { panic!() };
                let FnRetTy::Return(ref ty) = sig.decl.output else { return (def_id, IndexVec::new()) };
                (def_id, IndexVec::from_iter(std::iter::repeat(IntermediateResult::Unknown).take(hir_ptr_depth(ty))))
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

impl<'tcx, A: Analysis> rustc_mir_dataflow::Analysis<'tcx> for UsageAnalysis<'tcx, '_, A> {
    fn apply_statement_effect(
        &self,
        state: &mut Self::Domain,
        statement: &rustc_middle::mir::Statement<'tcx>,
        loc: rustc_middle::mir::Location,
    ) {
        let _guard = debug_span!("stmt", ?loc).entered();
        match &statement.kind {
            StatementKind::Assign(box (
                l_place,
                Rvalue::Use(operand) | Rvalue::Cast(_, operand, _),
            )) if operand.place().is_some() => {
                let r_place = operand.place().unwrap();
                let _guard = debug_span!("known assign", ?l_place, ?r_place).entered();

                self.check_places(state, Some(*l_place), Some(r_place), loc);
                if !l_place.ty(self.0.body, self.0.tcx).ty.is_unsafe_ptr() {
                    return;
                }
                let mut l_result = state
                    .result_for(self.0.tcx, self.0.body, l_place.as_ref())
                    .clone();
                if let Some((struct_def, field_idx, nested_level)) =
                    get_struct_field(self.0.tcx, self.0.body, l_place.as_ref())
                {
                    if !l_result.is_definite() {
                        let dep = InterDep::StructField {
                            struct_def,
                            field_idx,
                            nested_level,
                        };
                        l_result.join(&IntermediateResult::InterDeps([dep].into_iter().collect()));
                    }
                }
                trace!(?l_result);
                // If you want the general result for all locations of a pointer, this is correct.
                // Previously it used `=` instead of `join`, which seemed like it should give more
                // specific results at each location, but I think it is only useful with a very
                // complex rewrite scheme which may not be necessary. With `=` it broke mutability
                // rewrite, so I changed it.
                state
                    .result_for(self.0.tcx, self.0.body, r_place.as_ref())
                    .join(&l_result);
            }
            StatementKind::Assign(box (l_place, _)) => {
                self.check_places(state, Some(*l_place), None, loc);
                if !l_place.ty(self.0.body, self.0.tcx).ty.is_unsafe_ptr() {
                    return;
                }
                *state.result_for(self.0.tcx, self.0.body, l_place.as_ref()) =
                    IntermediateResult::Unknown;
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
        let loc = self.0.body.terminator_loc(bb_id);
        let terminator = self.0.body.basic_blocks()[bb_id]
            .terminator
            .as_ref()
            .unwrap();
        if let TerminatorKind::InlineAsm { .. } = terminator.kind {
            return;
        }
        let TerminatorKind::Call {
            func,
            args,
            destination,
            ..
        } = &terminator.kind else { return };
        self.check_places(state, Some(*destination), None, loc);
        for arg in args {
            if let Some(place) = arg.place() {
                self.check_places(state, None, Some(place), loc);
            }
        }
        let Some((def_id, _)) = func.const_fn_def() else { return };
        self.call(state, terminator, loc);

        let Some(def_id) = def_id.as_local() else {
            return;
        };

        for (idx, place) in args
            .iter()
            .enumerate()
            .filter_map(|(idx, op)| op.place().map(|place| (idx, place)))
        {
            if !place.ty(self.0.body, self.0.tcx).ty.is_unsafe_ptr() {
                continue;
            }
            let dep = InterDep::FnArg {
                fn_def: def_id,
                arg: Local::from_usize(idx + 1),
                nested_level: 0,
            };
            let result = state.result_for(self.0.tcx, self.0.body, place.as_ref());
            result.join(&IntermediateResult::InterDeps([dep].into_iter().collect()));
        }

        let is_raw_ptr = self
            .0
            .tcx
            .fn_sig(def_id)
            .no_bound_vars()
            .unwrap()
            .output()
            .is_unsafe_ptr();
        if self.0.tcx.is_foreign_item(def_id) || !is_raw_ptr {
            return;
        }

        // lhs transfers to rhs as in normal assignment
        let l_result = state.result_for(self.0.tcx, self.0.body, destination.as_ref());
        // TODO: handle more levels of nesting, both here and in assignments
        state.fn_returns.0[def_id].as_mut().unwrap()[0] = l_result.clone();
    }
}
