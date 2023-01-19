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
use super::{total_deref_level, AnalysisKind, Ownership, Precision};
use crate::{
    call_graph::FnSig,
    ownership::{
        infer::{FnSummary, InferCtxt},
        Param,
    },
    ptr::Measurable,
    ssa::{
        constraint::{
            infer::Renamer, initialize_local, Database, Gen, GlobalAssumptions, Var, Z3Database,
        },
        consume::Consume,
        state::SSAState,
        AnalysisResults, FnResults,
    },
    struct_ctxt::{RestrictedStructCtxt, StructCtxt},
    type_qualifier::output_params::OutputParams,
    CrateCtxt,
};

/// whole program analysis
pub enum WholeProgramAnalysis {}

impl<'analysis, 'db, 'tcx> AnalysisKind<'analysis, 'db, 'tcx> for WholeProgramAnalysis {
    type Results = WholeProgramResults<'tcx>;

    type InterCtxt = &'analysis InterCtxt;

    type DB = Z3Database<'db>;

    fn analyze(
        mut crate_ctxt: CrateCtxt<'tcx>,
        output_params: &OutputParams,
    ) -> anyhow::Result<Self::Results> {
        let required_precision = std::cmp::min(
            crate_ctxt.fns().iter().copied().fold(0, |acc, did| {
                let body = crate_ctxt.tcx.optimized_mir(did);
                std::cmp::max(acc, total_deref_level(body) + 1)
            }),
            3,
        );

        // first stage
        let mut results = solve_crate(&mut crate_ctxt, Left((output_params, required_precision)))?;

        // second stage
        for _ in 1..2 * required_precision {
            results = solve_crate(&mut crate_ctxt, Right(results))?;
        }

        let (model, fn_locals, struct_fields) = results;

        Ok(WholeProgramResults {
            model,
            fn_locals,
            struct_fields,
            struct_ctxt: crate_ctxt.struct_ctxt,
        })
    }
}

pub struct FnLocals {
    fn_sigs: InterCtxt,
    fn_summaries: indexmap::IndexMap<DefId, (FnSummary, Precision)>,
}

pub type IntermediateResults = (Vec<Ownership>, FnLocals, GlobalAssumptions);

pub struct WholeProgramResults<'tcx> {
    model: Vec<Ownership>,
    fn_locals: FnLocals,
    struct_fields: GlobalAssumptions,
    struct_ctxt: StructCtxt<'tcx>,
}

impl<'tcx> WholeProgramResults<'tcx> {
    pub fn fields<'a>(
        &'a self,
        r#struct: &DefId,
    ) -> impl Iterator<Item = &'a [Ownership]> + 'a + common::captures::Captures<'tcx> {
        self.struct_fields
            .fields(&self.struct_ctxt, r#struct)
            .map(|range| &self.model[range.start.index()..range.end.index()])
    }

    pub fn precision(&self, did: &DefId) -> Precision {
        self.fn_locals.fn_summaries[did].1
    }

    pub fn trace(&self, tcx: TyCtxt) {
        for &did in &self.struct_ctxt.post_order {
            tracing::debug!("{}: {{", tcx.def_path_str(did));
            for (field_def, field_results) in
                itertools::izip!(tcx.adt_def(did).all_fields(), self.fields(&did))
            {
                tracing::debug!("   {}: {:?}", field_def.ident(tcx), field_results);
            }
            tracing::debug!("}}");
        }

        for did in self.fn_locals.fn_summaries.keys() {
            let body = tcx.optimized_mir(did);
            tracing::debug!("@{}", tcx.def_path_str(*did));
            for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
                for index in 0..bb_data.statements.len() + bb_data.terminator.iter().count() {
                    let location = Location {
                        block: bb,
                        statement_index: index,
                    };
                    let result = self
                        .fn_results(*did)
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

fn solve_body<'tcx>(
    body: &Body<'tcx>,
    ssa_state: SSAState,
    crate_ctxt: &CrateCtxt<'tcx>,
    precision: Precision,
    inter_ctxt: <WholeProgramAnalysis as AnalysisKind>::InterCtxt,
    global_assumptions: &GlobalAssumptions,
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

    let mut rn = Renamer::new(body, ssa_state, crate_ctxt.tcx);

    print!(
        "Solving {} with precision {}... ",
        crate_ctxt.tcx.def_path_str(body.source.def_id()),
        std::cmp::min(precision, crate_ctxt.struct_ctxt.max_ptr_chased()),
    );

    let mut infer_cx = InferCtxt::new(
        crate_ctxt,
        precision,
        body,
        database,
        gen,
        inter_ctxt,
        global_assumptions,
    );

    rn.go::<WholeProgramAnalysis>(&mut infer_cx);

    let results = FnSummary::new(rn, infer_cx);

    match database.solver.check() {
        z3::SatResult::Unsat => {
            println!("\u{274C}");
            database.solver.pop(1);
            Ok((results, precision - 1))
        }
        z3::SatResult::Unknown => bail!("z3 status: unknown"),
        z3::SatResult::Sat => {
            println!("\u{2705}");
            Ok((results, precision))
        }
    }
}

fn solve_crate(
    crate_ctxt: &mut CrateCtxt,
    previous_results: Either<(&OutputParams, Precision), IntermediateResults>,
) -> anyhow::Result<IntermediateResults> {
    let mut gen = Gen::new();

    let config = z3::Config::new();
    let ctx = z3::Context::new(&config);
    let mut database = <WholeProgramAnalysis as AnalysisKind>::DB::new(&ctx);

    let global_assumptions = GlobalAssumptions::new(&*crate_ctxt, &mut gen, &mut database);

    let mut fn_summaries = indexmap::IndexMap::default();
    fn_summaries.reserve(crate_ctxt.fns().len());

    let fn_sigs = match previous_results {
        Left((noalias_params, required_precision)) => {
            let inter_ctxt =
                initial_inter_ctxt(crate_ctxt, noalias_params, &mut gen, &mut database);
            for &did in crate_ctxt.fn_ctxt.fns() {
                let body = crate_ctxt.tcx.optimized_mir(did);
                let ssa_state = initial_ssa_state(crate_ctxt, body);
                let fn_summary = solve_body(
                    body,
                    ssa_state,
                    crate_ctxt,
                    required_precision,
                    &inter_ctxt,
                    &global_assumptions,
                    &mut gen,
                    &mut database,
                )?;
                fn_summaries.insert(did, fn_summary);
            }
            inter_ctxt
        }
        Right(previous_results) => {
            crate_ctxt.struct_ctxt.increase_precision(crate_ctxt.tcx);
            let (inter_ctxt, fns) =
                previous_results
                    .1
                    .refine(previous_results.0, crate_ctxt, &mut gen, &mut database);
            for (did, ssa_state, precision) in fns {
                let body = crate_ctxt.tcx.optimized_mir(did);
                let fn_summary = solve_body(
                    body,
                    ssa_state,
                    crate_ctxt,
                    precision,
                    &inter_ctxt,
                    &global_assumptions,
                    &mut gen,
                    &mut database,
                )?;
                fn_summaries.insert(did, fn_summary);
            }
            inter_ctxt
        }
    };

    let model = retrieve_model(database, gen);

    let fn_locals = FnLocals {
        fn_sigs,
        fn_summaries,
    };

    let intermediate_results = (model, fn_locals, global_assumptions);
    show_fn_sigs(
        &intermediate_results.0,
        &intermediate_results.1,
        crate_ctxt.tcx,
        crate_ctxt.fns(),
    );

    Ok(intermediate_results)
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

impl<'analysis_results, 'tcx: 'analysis_results> AnalysisResults<'analysis_results>
    for WholeProgramResults<'tcx>
{
    type Value = Ownership;

    type Param = Param<&'analysis_results [Ownership]>;

    type FnSig = impl Iterator<Item = Option<Self::Param>>;

    type FnResults = (
        &'analysis_results FnSummary,
        &'analysis_results [Ownership],
        RestrictedStructCtxt<'analysis_results, 'tcx>,
    );

    fn fn_results(&'analysis_results self, r#fn: DefId) -> Option<Self::FnResults> {
        let (fn_summary, precision) = self.fn_locals.fn_summaries.get(&r#fn)?;
        Some((
            fn_summary,
            &self.model[..],
            self.struct_ctxt.with_max_precision(*precision),
        ))
    }

    fn fn_sig(&'analysis_results self, r#fn: DefId) -> Self::FnSig {
        get_fn_sig(&self.model, &self.fn_locals, r#fn)
    }

    fn print_fn_sigs(&'analysis_results self, tcx: TyCtxt, fns: &[DefId]) {
        show_fn_sigs(&self.model, &self.fn_locals, tcx, fns);
    }
}

fn get_fn_sig<'a>(
    model: &'a [Ownership],
    fn_locals: &'a FnLocals,
    r#fn: DefId,
) -> impl Iterator<Item = Option<Param<&'a [Ownership]>>> + 'a {
    let fn_sigs = &fn_locals.fn_sigs[&r#fn];
    let ret = fn_sigs
        .ret
        .clone()
        .map(|sigs| sigs.map(|sigs| &model[sigs.start.index()..sigs.end.index()]));

    let args = fn_sigs.args.iter().map(|arg| {
        arg.clone()
            .map(|sigs| sigs.map(|sigs| &model[sigs.start.index()..sigs.end.index()]))
    });

    std::iter::once(ret).chain(args)
}

fn show_fn_sigs(model: &[Ownership], fn_locals: &FnLocals, tcx: TyCtxt, fns: &[DefId]) {
    fn display_value<Value: std::fmt::Display>(value: &[Value]) -> String {
        value
            .iter()
            .map(|value| format!("{value}"))
            .collect::<Vec<_>>()
            .join(" ")
    }

    for &did in fns {
        let mut fn_sig = get_fn_sig(model, fn_locals, did);
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

impl<'a, 'tcx> FnResults<'a>
    for (
        &'a FnSummary,
        &'a [Ownership],
        RestrictedStructCtxt<'a, 'tcx>,
    )
{
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

pub type InterCtxt = FxHashMap<DefId, FnSig<Option<Param<Range<Var>>>>>;

impl FnLocals {
    pub fn refine<'tcx>(
        self,
        model: Vec<Ownership>,
        crate_ctxt: &CrateCtxt<'tcx>,
        gen: &mut Gen,
        database: &mut Z3Database,
    ) -> (
        InterCtxt,
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
                        Param::Output(_) => {
                            let r#use = initialize_local(
                                local_decl,
                                gen,
                                database,
                                crate_ctxt.struct_ctxt.with_max_precision(precision),
                            )?;
                            assert!(!r#use.is_empty());
                            database.push_assume::<crate::ssa::constraint::Debug>(
                                (),
                                r#use.start,
                                true,
                            );
                            let def = initialize_local(
                                local_decl,
                                gen,
                                database,
                                crate_ctxt.struct_ctxt.with_max_precision(precision),
                            )?;
                            assert!(!def.is_empty());
                            database.push_assume::<crate::ssa::constraint::Debug>(
                                (),
                                def.start,
                                true,
                            );

                            Some(Param::Output(Consume { r#use, def }))
                        }
                        Param::Normal(_) => {
                            let now = initialize_local(
                                local_decl,
                                gen,
                                database,
                                crate_ctxt.struct_ctxt.with_max_precision(precision),
                            )?;

                            Some(Param::Normal(now))
                        }
                    }
                })
                .collect();

            inter_ctxt.insert(did, fn_sig);
        }

        let tcx = crate_ctxt.tcx;

        let state_iter =
            self.fn_summaries
                .into_iter()
                .map(move |(did, (fn_summary, precision))| {
                    (
                        did,
                        refine_state(tcx.optimized_mir(did), fn_summary, &model),
                        precision,
                    )
                });

        (inter_ctxt, state_iter)
    }
}
