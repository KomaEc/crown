mod state;

use std::ops::Range;

use anyhow::bail;
use either::Either::{self, Left, Right};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{Body, Local, Location},
    ty::TyCtxt,
};

use self::state::{initial_inter_ctxt, initial_ssa_state, refine_state};
use super::{max_deref_level, AnalysisKind, Ownership, Precision};
use crate::{
    call_graph::FnSig,
    ownership::{
        infer::{FnSummary, InferCtxt},
        Param,
    },
    ssa::{
        constraint::{infer::Renamer, initialize_local, Database, Gen, Var, Z3Database},
        consume::Consume,
        state::SSAState,
        AnalysisResults, FnResult,
    },
    type_qualifier::noalias::NoAliasParams,
    CrateCtxt,
};

/// whole program analysis
pub enum WholeProgramAnalysis {}

impl<'analysis, 'db> AnalysisKind<'analysis, 'db> for WholeProgramAnalysis {
    type Results = WholeProgramAnalysisResults;

    type InterCtxt = &'analysis WholeProgramInterCtxt;

    type DB = Z3Database<'db>;

    fn analyze(
        crate_ctxt: &mut CrateCtxt,
        noalias_params: &NoAliasParams,
    ) -> anyhow::Result<Self::Results> {
        let required_precision = crate_ctxt.fns().iter().copied().fold(0, |acc, did| {
            let body = crate_ctxt.tcx.optimized_mir(did);
            std::cmp::max(acc, max_deref_level(body) + 1)
        });

        // first stage
        let mut results = solve_crate(crate_ctxt, Left(noalias_params))?;

        // second stage
        for _ in 1..required_precision {
            results = solve_crate(crate_ctxt, Right(results))?;
        }

        Ok(results)
    }
}

pub struct WholeProgramAnalysisResults {
    model: Vec<Ownership>,
    fn_sigs: WholeProgramInterCtxt,
    fn_summaries: FxHashMap<DefId, (FnSummary, Precision)>,
}

fn solve_body<'tcx>(
    body: &Body<'tcx>,
    ssa_state: SSAState,
    crate_ctxt: &CrateCtxt<'tcx>,
    precision: Precision,
    inter_ctxt: <WholeProgramAnalysis as AnalysisKind>::InterCtxt,
    gen: &mut Gen,
    database: &mut Z3Database,
) -> anyhow::Result<(FnSummary, Precision)> {
    if precision == 0 {
        return Ok((
            FnSummary {
                fn_body_sig: rustc_index::vec::IndexVec::new(),
                ssa_state: ssa_state.mk_dummy(),
            },
            precision,
        ));
    }

    database.solver.push();

    let mut rn = Renamer::new(body, ssa_state);

    let mut infer_cx = InferCtxt::new(
        crate_ctxt.with_precision(precision),
        body,
        database,
        gen,
        inter_ctxt,
    );

    rn.go::<WholeProgramAnalysis>(&mut infer_cx);

    let results = FnSummary::new(rn, infer_cx);

    match database.solver.check() {
        z3::SatResult::Unsat => {
            let fn_name = crate_ctxt.tcx.def_path_str(body.source.def_id());
            println!("failed: {fn_name}");
            database.solver.pop(1);
            Ok((results, precision - 1))
        }
        z3::SatResult::Unknown => bail!("z3 status: unknown"),
        z3::SatResult::Sat => Ok((results, precision)),
    }
}

fn solve_crate(
    crate_ctxt: &mut CrateCtxt,
    previous_results: Either<&NoAliasParams, WholeProgramAnalysisResults>, //Option<WholeProgramResults>,
) -> anyhow::Result<WholeProgramAnalysisResults> {
    let mut gen = Gen::new();

    let config = z3::Config::new();
    let ctx = z3::Context::new(&config);
    let mut database = <WholeProgramAnalysis as AnalysisKind>::DB::new(&ctx);

    let mut fn_summaries = FxHashMap::default();
    fn_summaries.reserve(crate_ctxt.fns().len());

    let fn_sigs = match previous_results {
        Left(noalias_params) => {
            let inter_ctxt =
                initial_inter_ctxt(crate_ctxt, noalias_params, &mut gen, &mut database);
            for &did in crate_ctxt.call_graph.fns() {
                let body = crate_ctxt.tcx.optimized_mir(did);
                let max_ptr_depth = max_deref_level(body) + 1;
                let ssa_state = initial_ssa_state(crate_ctxt, body);
                let fn_summary = solve_body(
                    body,
                    ssa_state,
                    crate_ctxt,
                    // TODO
                    max_ptr_depth,
                    &inter_ctxt,
                    &mut gen,
                    &mut database,
                )?;
                fn_summaries.insert(did, fn_summary);
            }
            inter_ctxt
        }
        Right(previous_results) => {
            crate_ctxt.struct_topology.next_stage(crate_ctxt.tcx);
            let (inter_ctxt, fns) =
                previous_results.next_stage(crate_ctxt, &mut gen, &mut database);
            for (did, ssa_state, precision) in fns {
                let body = crate_ctxt.tcx.optimized_mir(did);
                let fn_summary = solve_body(
                    body,
                    ssa_state,
                    crate_ctxt,
                    // TODO
                    precision,
                    &inter_ctxt,
                    &mut gen,
                    &mut database,
                )?;
                fn_summaries.insert(did, fn_summary);
            }
            inter_ctxt
        }
    };

    let model = retrieve_model(database, gen);

    let results = WholeProgramAnalysisResults {
        model,
        fn_sigs,
        fn_summaries,
    };

    results.print_fn_sigs(crate_ctxt.tcx, crate_ctxt.fns());

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

impl<'a> AnalysisResults<'a> for WholeProgramAnalysisResults {
    type Value = Ownership;

    type Param = Param<&'a [Ownership]>;

    type FnSig = impl Iterator<Item = Option<Self::Param>>;

    type FnResult = (&'a FnSummary, &'a [Ownership]);

    fn fn_result(&'a self, r#fn: DefId) -> Option<Self::FnResult> {
        let (fn_summary, _) = self.fn_summaries.get(&r#fn)?;
        Some((fn_summary, &self.model[..]))
    }

    fn fn_sig(&'a self, r#fn: DefId) -> Self::FnSig {
        let fn_sigs = &self.fn_sigs[&r#fn];
        let ret = fn_sigs
            .ret
            .clone()
            .map(|sigs| sigs.map(|sigs| &self.model[sigs.start.index()..sigs.end.index()]));

        let args = fn_sigs.args.iter().map(|arg| {
            arg.clone()
                .map(|sigs| sigs.map(|sigs| &self.model[sigs.start.index()..sigs.end.index()]))
        });

        std::iter::once(ret).chain(args)
    }

    fn print_fn_sigs(&'a self, tcx: TyCtxt, fns: &[DefId]) {
        fn display_value<Value: std::fmt::Display>(value: &[Value]) -> String {
            value
                .iter()
                .map(|value| format!("{value}"))
                .collect::<Vec<_>>()
                .join(" ")
        }

        for &did in fns {
            let mut fn_sig = self.fn_sig(did);
            let ret = fn_sig.next().unwrap();
            let ret = if let Some(sig) = ret {
                // format!("{:?}", sig)
                display_value(sig.expect_normal())
            } else {
                "_".to_owned()
            };
            let args = fn_sig
                .map(|sig| {
                    if let Some(sig) = sig {
                        match sig {
                            Param::Output(output_param) => {
                                "&uniq ".to_owned()
                                    + &display_value(&output_param.r#use[1..])
                                    + " ↓ &uniq "
                                    + &display_value(&output_param.def[1..])
                            }
                            Param::Normal(param) => display_value(param),
                        }
                    } else {
                        "_".to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            let fn_path = tcx.def_path_str(did);
            println!("{fn_path}: ({args}) -> {ret}")
        }
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
    fn location_results(&'a self, location: Location) -> Self::LocationResults {
        self.0.location_results(location).map(|(local, result)| {
            (
                local,
                result.map_valid(|sigs| &self.1[sigs.start.index()..sigs.end.index()]),
            )
        })
    }
}

pub type WholeProgramInterCtxt = FxHashMap<DefId, FnSig<Option<Param<Range<Var>>>>>;

impl WholeProgramAnalysisResults {
    pub fn next_stage<'tcx>(
        self,
        crate_ctxt: &CrateCtxt<'tcx>,
        gen: &mut Gen,
        database: &mut Z3Database,
    ) -> (
        WholeProgramInterCtxt,
        impl Iterator<Item = (DefId, SSAState, Precision)> + 'tcx,
    ) {
        let mut inter_ctxt = FxHashMap::default();
        inter_ctxt.reserve(self.fn_sigs.len());

        for (did, original) in self.fn_sigs.into_iter() {
            let body = crate_ctxt.tcx.optimized_mir(did);
            let precision = self.fn_summaries[&did].1;

            let fn_sig = original
                .iter()
                .zip(&body.local_decls)
                .map(|(original, local_decl)| {
                    // let Some(original) = original else { return None; };
                    // FIXME correctness?
                    let original = original.as_ref()?;
                    match original {
                        Param::Output(output_params) => {
                            let r#use =
                            // TODO
                                initialize_local(local_decl, gen, database, crate_ctxt.with_precision(precision))?;
                            let def =
                            // TODO
                                initialize_local(local_decl, gen, database, crate_ctxt.with_precision(precision))?;

                            for (pre, now) in output_params
                                .r#use
                                .clone()
                                .zip(r#use.clone())
                                .chain(output_params.def.clone().zip(def.clone()))
                            {
                                if let Ownership::Owning = self.model[pre.index()] {
                                    database.push_assume::<crate::ssa::constraint::Debug>(
                                        (),
                                        now,
                                        true,
                                    );
                                }
                            }

                            Some(Param::Output(Consume { r#use, def }))
                        }
                        Param::Normal(params) => {
                            let now =
                            // TODO
                                initialize_local(local_decl, gen, database, crate_ctxt.with_precision(precision))?;

                            for (pre, now) in params.clone().zip(now.clone()) {
                                if let Ownership::Owning = self.model[pre.index()] {
                                    database.push_assume::<crate::ssa::constraint::Debug>(
                                        (),
                                        now,
                                        true,
                                    );
                                }
                            }

                            Some(Param::Normal(now))
                        }
                    }
                })
                .collect();

            println!("generating signatures for {:?}: {:?}", did, fn_sig);

            inter_ctxt.insert(did, fn_sig);
        }

        let tcx = crate_ctxt.tcx;

        let state_iter =
            self.fn_summaries
                .into_iter()
                .map(move |(did, (fn_summary, precision))| {
                    (
                        did,
                        refine_state(tcx.optimized_mir(did), fn_summary, &self.model[..]),
                        precision,
                    )
                });

        (inter_ctxt, state_iter)
    }

    pub fn trace(&self, tcx: TyCtxt) {
        for did in self.fn_summaries.keys() {
            let body = tcx.optimized_mir(did);
            tracing::debug!("@{}", tcx.def_path_str(*did));
            for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
                for index in 0..bb_data.statements.len() + bb_data.terminator.iter().count() {
                    let location = Location {
                        block: bb,
                        statement_index: index,
                    };
                    let result = self
                        .fn_result(*did)
                        .unwrap()
                        .location_results(location)
                        .map(|(local, result)| format!("{:?}: {:?}", local, result))
                        .collect::<Vec<_>>()
                        .join(", ");

                    tracing::debug!("@{:?}: {result}", location);
                }
            }
        }
    }
}
