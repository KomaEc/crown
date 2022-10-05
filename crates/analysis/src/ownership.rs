pub mod infer;

use std::ops::Range;

use anyhow::bail;
use either::Either;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{BasicBlockData, Body, Local, Location, Rvalue, StatementKind};

use self::infer::{FnSummary, InferCtxt};
use crate::{
    call_graph::FnSig,
    ssa::{
        constraint::{
            infer::{Pure, Renamer},
            initialize_local, CadicalDatabase, Database, Gen, Var, Z3Database,
        },
        consume::{initial_definitions, Consume, Voidable},
        dom::compute_dominance_frontier,
        state::SSAState,
        AnalysisResults, FnResult,
    },
    CrateCtxt,
};

impl<'tcx> CrateCtxt<'tcx> {
    pub fn pure_rename(&mut self) {
        for &did in self.fns() {
            println!("renaming {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, self.tcx, self);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            let mut rn = Renamer::new(body, ssa_state);
            rn.go::<false, Pure>(());
            println!("completed");
        }
    }

    pub fn standalone_solve(&mut self) -> anyhow::Result<()> {
        StandAlone::analyze(self)
    }

    pub fn whole_program_analysis(&mut self) -> anyhow::Result<()> {
        let results = WholeProgram::analyze(self)?;

        for did in results.fn_summaries.keys() {
            let body = self.tcx.optimized_mir(did);
            tracing::debug!("@{}", self.tcx.def_path_str(*did));
            for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
                for index in 0..bb_data.statements.len() + bb_data.terminator.iter().count() {
                    let location = Location {
                        block: bb,
                        statement_index: index,
                    };
                    let result = results
                        .fn_result(*did)
                        .unwrap()
                        .location_result(location)
                        .map(|(local, result)| format!("{:?}: {:?}", local, result))
                        .collect::<Vec<_>>()
                        .join(", ");

                    tracing::debug!("@{:?}: {result}", location);
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ownership {
    Owning,
    Transient,
    Unknown,
}

impl From<Option<bool>> for Ownership {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => Ownership::Owning,
            Some(false) => Ownership::Transient,
            None => Ownership::Unknown,
        }
    }
}

impl std::fmt::Display for Ownership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ownership::Owning => write!(f, "&move"),
            Ownership::Transient => write!(f, "&"),
            Ownership::Unknown => write!(f, "&any"),
        }
    }
}

impl Voidable for &[Ownership] {
    const VOID: Self = &[];

    #[inline]
    fn is_void(&self) -> bool {
        self.is_empty()
    }
}

pub trait OwnershipSchemes<'analysis>: AnalysisResults<'analysis, Value = Ownership> {}

impl<'analysis, Results> OwnershipSchemes<'analysis> for Results where
    Results: AnalysisResults<'analysis, Value = Ownership>
{
}

// pub trait LocalUsage {
//     fn base_transfered(&self, model: &[Ownership]) -> bool;
// }

// impl LocalUsage for Consume<Range<OwnershipSig>> {
//     fn base_transfered(&self, model: &[Ownership]) -> bool {
//         let pre = model[self.r#use.start.index()];
//         let post = model[self.def.start.index()];
//         pre == Ownership::Owning && post == Ownership::Transient
//             || pre == Ownership::Transient && post == Ownership::Owning
//     }
// }

pub trait LocalUsage {
    fn state_changed(&self) -> bool;
}

impl LocalUsage for Consume<&[Ownership]> {
    fn state_changed(&self) -> bool {
        for (&r#use, &def) in self.r#use.iter().zip(self.def.iter()) {
            if r#use == def {
                continue;
            }
            if r#use == Ownership::Owning || def == Ownership::Owning {
                return true;
            }
        }
        false
    }
}

impl<'a> FnResult<'a> for (&'a FnSummary, &'a [Ownership]) {
    type LocalResult = &'a [Ownership];

    type LocationResults = impl Iterator<Item = (Local, Consume<&'a [Ownership]>)>;

    #[inline]
    fn local_result(&self, local: Local, location: Location) -> Option<Consume<Self::LocalResult>> {
        let local_result = self.0.local_result(local, location)?;
        Some(local_result.map_valid(|sigs| &self.1[sigs.start.index()..sigs.end.index()]))
    }

    #[inline]
    fn location_result(&'a self, location: Location) -> Self::LocationResults {
        self.0.location_result(location).map(|(local, result)| {
            (
                local,
                result.map_valid(|sigs| &self.1[sigs.start.index()..sigs.end.index()]),
            )
        })
    }
}

pub trait AnalysisKind<'analysis, 'db> {
    /// Analysis results
    type Results;
    /// Interprocedural context
    type InterCtxt;
    type DB: Database;
    fn analyze(crate_ctxt: &mut CrateCtxt) -> anyhow::Result<Self::Results>;
}
pub enum Modular {}
impl<'analysis, 'db> AnalysisKind<'analysis, 'db> for Modular {
    type Results = ();
    type InterCtxt = ();
    type DB = ();
    fn analyze(_: &mut CrateCtxt) -> anyhow::Result<Self::Results> {
        // TODO implement this
        anyhow::bail!("modular analysis is not implemented")
    }
}
pub enum WholeProgram {}

impl WholeProgram {
    fn pre_generate_fn_sigs(
        crate_ctxt: &CrateCtxt,
        gen: &mut Gen,
        database: &mut Z3Database,
    ) -> FxHashMap<DefId, FnSig<Option<Range<Var>>>> {
        let mut fn_sigs = FxHashMap::default();
        fn_sigs.reserve(crate_ctxt.fns().len());
        for &did in crate_ctxt.call_graph.fns() {
            let body = crate_ctxt.tcx.optimized_mir(did);
            let fn_sig = {
                let mut local_decls = body.local_decls.iter();
                let return_local_decl = local_decls.next().unwrap();
                let ret = initialize_local(return_local_decl, gen, database, crate_ctxt);

                let args = local_decls
                    .take(body.arg_count)
                    .map(|local_decl| initialize_local(local_decl, gen, database, crate_ctxt))
                    .collect();

                FnSig { ret, args }
            };
            println!("generating signatures for {:?}: {:?}", did, fn_sig);
            fn_sigs.insert(did, fn_sig);
        }

        fn_sigs
    }

    #[inline]
    fn initial_ssa<'a, 'tcx>(crate_ctxt: &'a CrateCtxt<'tcx>, body: &'a Body<'tcx>) -> SSAState {
        let dominance_frontier = compute_dominance_frontier(body);
        let definitions = initial_definitions(body, crate_ctxt.tcx, crate_ctxt);
        SSAState::new(body, &dominance_frontier, definitions)
    }

    fn solve_body<'tcx, const STRICT: bool>(
        body: &Body<'tcx>,
        ssa_state: SSAState,
        crate_ctxt: &CrateCtxt<'tcx>,
        inter_ctxt: <WholeProgram as AnalysisKind>::InterCtxt,
        gen: &mut Gen,
        database: &mut Z3Database,
    ) -> anyhow::Result<FnSummary> {
        database.solver.push();

        let mut rn = Renamer::new(body, ssa_state);

        let mut infer_cx = InferCtxt::new(crate_ctxt, body, database, gen, inter_ctxt);

        rn.go::<STRICT, Self>(&mut infer_cx);

        let results = FnSummary::new(rn, infer_cx);

        match database.solver.check() {
            z3::SatResult::Unsat => {
                let fn_name = crate_ctxt.tcx.def_path_str(body.source.def_id());
                println!("failed: {fn_name}");
                database.solver.pop(1);
            }
            z3::SatResult::Unknown => bail!("z3 status: unknown"),
            z3::SatResult::Sat => {}
        }

        Ok(results)
    }

    fn retrieve_model(database: Z3Database, gen: Gen) -> Vec<Ownership> {
        assert!(matches!(database.solver.check(), z3::SatResult::Sat));
        let z3_model = database.solver.get_model().unwrap();
        let mut model = Vec::with_capacity(gen.next().index());

        assert_eq!(Var::MIN.index(), 1);

        model.push(Ownership::Unknown);

        for sig in Var::MIN..gen.next() {
            let value = z3_model
                .eval(&database.z3_ast[sig], false)
                .unwrap()
                .as_bool();
            model.push(value.into());
        }

        model
    }

    /// This is incomplete at this moment
    fn apply_model<'a, 'tcx>(
        body: &'a Body<'tcx>,
        fn_summary: FnSummary,
        model: &[Ownership],
    ) -> SSAState {
        let mut state_changed_locations: FxHashSet<Location> = FxHashSet::default();
        let fn_results = (&fn_summary, model);

        for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
            let BasicBlockData { statements, .. } = bb_data;
            let mut index = 0;
            let mut deref_copy: Option<Location> = None;
            for statement in statements {
                let location = Location {
                    block: bb,
                    statement_index: index,
                };
                if let StatementKind::Assign(box (_, rvalue)) = &statement.kind
                    && let Rvalue::CopyForDeref(_) = rvalue
                {
                    let base_location = deref_copy.take().or(Some(location));
                    deref_copy = base_location;
                    index += 1;
                    continue;
                }
                if let Some(base_location) = deref_copy.take() {
                    let StatementKind::Assign(box (_, _)) = &statement.kind else { unreachable!() };

                    let location_result = fn_results
                        .location_result(base_location)
                        .chain(fn_results.location_result(location));
                    for (_, result) in location_result {
                        if result.state_changed() {
                            state_changed_locations.insert(location);
                            state_changed_locations.insert(base_location);
                        }
                    }

                    index += 1;
                    continue;
                }

                let location_result = fn_results.location_result(location);
                for (_, result) in location_result {
                    if result.state_changed() {
                        state_changed_locations.insert(location);
                    }
                }

                index += 1;
            }
        }

        let FnSummary {
            fn_body_sig,
            mut ssa_state,
        } = fn_summary;

        let consumes = &mut ssa_state.consume_chain.consumes;
        // we have to do this awkwardly as lending iterator is not ready
        for bb in 0..consumes.len() {
            for (statement_index, consumes) in consumes[bb].iter_mut().enumerate() {
                for &mut (local, ref mut consume) in consumes.iter_mut() {
                    if consume.is_use() {
                        let location = Location {
                            block: bb.into(),
                            statement_index,
                        };
                        let Either::Left(_) = body.stmt_at(location) else {
                            unreachable!("function args and return are assumed to be local. rustc changes this property somehow")
                        };
                        let outter_most = fn_body_sig[local][consume.r#use].start;
                        if matches!(model[outter_most.index()], Ownership::Owning)
                            || state_changed_locations.contains(&location)
                        {
                            consume.enable_def();
                        }
                    }
                }
            }
        }
        ssa_state.name_state.reset();
        ssa_state.join_points.reset();
        ssa_state.consume_chain.reset();
        ssa_state
    }

    fn solve_crate<const STRICT: bool>(
        crate_ctxt: &mut CrateCtxt,
        previous_results: Option<WholeProgramResults>,
    ) -> anyhow::Result<WholeProgramResults> {
        let mut gen = Gen::new();

        let config = z3::Config::new();
        let ctx = z3::Context::new(&config);
        let mut database = <Self as AnalysisKind>::DB::new(&ctx);

        let mut fn_summaries = FxHashMap::default();
        fn_summaries.reserve(crate_ctxt.fns().len());

        let fn_sigs = if let Some(previous_results) = previous_results {
            crate_ctxt.struct_topology.next_stage(crate_ctxt.tcx);
            let (inter_ctxt, fns) =
                previous_results.next_stage(crate_ctxt, &mut gen, &mut database);
            for (did, ssa_state) in fns {
                let body = crate_ctxt.tcx.optimized_mir(did);
                let fn_summary = WholeProgram::solve_body::<STRICT>(
                    body,
                    ssa_state,
                    crate_ctxt,
                    &inter_ctxt,
                    &mut gen,
                    &mut database,
                )?;
                fn_summaries.insert(did, fn_summary);
            }
            inter_ctxt
        } else {
            let inter_ctxt =
                WholeProgram::pre_generate_fn_sigs(crate_ctxt, &mut gen, &mut database);
            for &did in crate_ctxt.call_graph.fns() {
                let body = crate_ctxt.tcx.optimized_mir(did);
                let ssa_state = WholeProgram::initial_ssa(crate_ctxt, body);
                let fn_summary = WholeProgram::solve_body::<STRICT>(
                    body,
                    ssa_state,
                    crate_ctxt,
                    &inter_ctxt,
                    &mut gen,
                    &mut database,
                )?;
                fn_summaries.insert(did, fn_summary);
            }
            inter_ctxt
        };

        let model = WholeProgram::retrieve_model(database, gen);

        let results = WholeProgramResults {
            model,
            fn_sigs,
            fn_summaries,
        };

        results.print_fn_sigs(crate_ctxt.tcx, crate_ctxt.fns());

        Ok(results)
    }
}

pub struct WholeProgramResults {
    model: Vec<Ownership>,
    fn_sigs: FxHashMap<DefId, FnSig<Option<Range<Var>>>>,
    fn_summaries: FxHashMap<DefId, FnSummary>,
}

impl WholeProgramResults {
    pub fn next_stage<'tcx>(
        self,
        crate_ctxt: &CrateCtxt<'tcx>,
        gen: &mut Gen,
        database: &mut Z3Database,
    ) -> (
        FxHashMap<DefId, FnSig<Option<Range<Var>>>>,
        impl Iterator<Item = (DefId, SSAState)> + 'tcx,
    ) {
        let mut inter_ctxt = FxHashMap::default();
        inter_ctxt.reserve(self.fn_sigs.len());

        for (did, original) in self.fn_sigs.into_iter() {
            let body = crate_ctxt.tcx.optimized_mir(did);
            let fn_sig = {
                let mut local_decls = body.local_decls.iter();
                let return_local_decl = local_decls.next().unwrap();
                let ret = initialize_local(return_local_decl, gen, database, crate_ctxt);

                let args = local_decls
                    .take(body.arg_count)
                    .map(|local_decl| initialize_local(local_decl, gen, database, crate_ctxt))
                    .collect();

                FnSig { ret, args }
            };

            for (pre, now) in original.iter().zip(fn_sig.iter()) {
                if let Some(pre) = pre.clone() {
                    let now = now.clone().unwrap();
                    assert!(
                        pre.end.index() - pre.start.index() <= now.end.index() - now.start.index()
                    );
                    for (pre, now) in pre.zip(now) {
                        match self.model[pre.index()] {
                            Ownership::Owning => {
                                database.push_assume::<crate::ssa::constraint::Debug>(
                                    (),
                                    now,
                                    true,
                                );
                            }
                            Ownership::Transient | Ownership::Unknown => {}
                        }
                    }
                }
            }

            println!("generating signatures for {:?}: {:?}", did, fn_sig);

            inter_ctxt.insert(did, fn_sig);
        }

        let tcx = crate_ctxt.tcx;

        let fns = self.fn_summaries.into_iter().map(move |(did, fn_summary)| {
            (
                did,
                WholeProgram::apply_model(tcx.optimized_mir(did), fn_summary, &self.model[..]),
            )
        });

        (inter_ctxt, fns)
    }
}

impl<'a> AnalysisResults<'a> for WholeProgramResults {
    type Value = Ownership;

    type FnSig = impl Iterator<Item = Option<&'a [Ownership]>>;

    type FnResult = (&'a FnSummary, &'a [Ownership]);

    fn fn_result(&'a self, r#fn: DefId) -> Option<Self::FnResult> {
        let fn_summary = self.fn_summaries.get(&r#fn)?;
        Some((fn_summary, &self.model[..]))
    }

    fn fn_sig(&'a self, r#fn: DefId) -> Self::FnSig {
        let fn_sigs = &self.fn_sigs[&r#fn];
        let ret = fn_sigs
            .ret
            .as_ref()
            .map(|sigs| &self.model[sigs.start.index()..sigs.end.index()]);

        let args = fn_sigs.args.iter().map(|arg| {
            arg.as_ref()
                .map(|sigs| &self.model[sigs.start.index()..sigs.end.index()])
        });

        std::iter::once(ret).chain(args)
    }
}

impl<'analysis, 'db> AnalysisKind<'analysis, 'db> for WholeProgram {
    type Results = WholeProgramResults;

    /// TODO refactor this to be `&'analysis FxHashMap<DefId, FnSig<Range<OwnershipSig>>>`
    type InterCtxt = &'analysis FxHashMap<DefId, FnSig<Option<Range<Var>>>>;

    type DB = Z3Database<'db>;

    fn analyze(crate_ctxt: &mut CrateCtxt) -> anyhow::Result<Self::Results> {
        // first stage

        let results = WholeProgram::solve_crate::<false>(crate_ctxt, None)?;

        // second stage
        let mut results = results;

        for _ in 0..2 {
            results = WholeProgram::solve_crate::<false>(crate_ctxt, Some(results))?;
        }

        Ok(results)
    }
}
pub enum StandAlone {}
impl<'analysis, 'db> AnalysisKind<'analysis, 'db> for StandAlone {
    type Results = ();
    type InterCtxt = ();
    type DB = CadicalDatabase;
    fn analyze(crate_ctxt: &mut CrateCtxt) -> anyhow::Result<Self::Results> {
        let mut databases = Vec::with_capacity(crate_ctxt.fns().len());
        for &did in crate_ctxt.fns() {
            println!("solving {:?}", did);
            let body = crate_ctxt.tcx.optimized_mir(did);

            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, crate_ctxt.tcx, crate_ctxt);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            let mut rn = Renamer::new(body, ssa_state);

            let mut gen = Gen::new();
            let mut database = CadicalDatabase::new();
            let mut infer_cx = InferCtxt::new(crate_ctxt, body, &mut database, &mut gen, ());

            rn.go::<false, Self>(&mut infer_cx);
            match database.solver.solve() {
                Some(true) => println!("succeeded"),
                Some(false) => println!("failed"),
                None => anyhow::bail!("timeout"),
            }
            databases.push(database);
        }
        Ok(())
    }
}
