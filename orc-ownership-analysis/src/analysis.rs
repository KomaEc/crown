use anyhow::bail;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::{Body, Local, Location};
use rustc_middle::ty::TyCtxt;
use std::ops::Range;

use crate::analysis::constraint::infer::{InferCtxt, Pure, Renamer};
use crate::analysis::constraint::{CadicalDatabase, OwnershipSigGenerator, Z3Database};
use crate::analysis::def::initial_definitions;
use crate::analysis::dom::compute_dominance_frontier;
use crate::CrateCtxt;

use crate::analysis::constraint::{generate_signatures_for_local, OwnershipSig};
use crate::analysis::state::SSAState;
use crate::call_graph::FnSig;

use self::constraint::infer::FnSummary;
use self::def::{Consume, HasInvalid};

pub mod constants;
pub mod constraint;
pub mod def;
pub mod dom;
pub mod join_points;
pub mod state;

impl<'tcx> CrateCtxt<'tcx> {
    pub fn pure_rename(&self) {
        for &did in self.functions() {
            println!("renaming {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, self.tcx, self);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            let mut rn = Renamer::new(body, ssa_state);
            rn.go::<Pure, ()>(());
            println!("completed");
        }
    }

    pub fn standalone_solve(&self) -> anyhow::Result<()> {
        StandAlone::analyze(self)
    }

    pub fn whole_program_analysis(&self) -> anyhow::Result<()> {
        let results = WholeProgram::analyze(self)?;

        for did in results.fn_summaries.keys() {
            let body = self.tcx.optimized_mir(did);
            println!("@{:?}", did);
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

                    println!("@{:?}: {result}", location);
                }
            }
        }

        for fn_result in results.fn_summaries.into_values() {
            let _ = WholeProgram::apply_model(fn_result, &results.model[..]);
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

impl HasInvalid for &[Ownership] {
    const INVALID: Self = &[];

    #[inline]
    fn is_invalid(&self) -> bool {
        self.is_empty()
    }
}

pub trait FnResult<'a> {
    type LocalResult;
    type LocationResults: Iterator<Item = (Local, Consume<Self::LocalResult>)>;

    fn local_result(&self, local: Local, location: Location) -> Option<Consume<Self::LocalResult>>;

    fn location_result(&'a self, location: Location) -> Self::LocationResults;
}

impl<'a> FnResult<'a> for (&'a FnSummary, &'a [Ownership]) {
    type LocalResult = &'a [Ownership];

    type LocationResults = impl Iterator<Item = (Local, Consume<&'a [Ownership]>)>;

    #[inline]
    fn local_result(&self, local: Local, location: Location) -> Option<Consume<Self::LocalResult>> {
        let local_result = self.0.local_result(local, location)?;
        Some(local_result.map(|sigs| &self.1[sigs.start.index()..sigs.end.index()]))
    }

    #[inline]
    fn location_result(&'a self, location: Location) -> Self::LocationResults {
        self.0.location_result(location).map(|(local, result)| {
            (
                local,
                result.map(|sigs| &self.1[sigs.start.index()..sigs.end.index()]),
            )
        })
    }
}

pub trait AnalysisResults<'a> {
    type FnSig: Iterator<Item = Option<&'a [Ownership]>>;
    type FnResult: FnResult<'a, LocalResult = &'a [Ownership]>;

    fn fn_result(&'a self, r#fn: DefId) -> Option<Self::FnResult>;

    fn fn_sig(&'a self, r#fn: DefId) -> Self::FnSig;
    fn print_fn_sigs(&'a self, tcx: TyCtxt, fns: &[DefId]) {
        for &did in fns {
            let mut fn_sig = self.fn_sig(did);
            let ret = fn_sig.next().unwrap();
            let ret = if let Some(sig) = ret {
                format!("{}", sig[0])
            } else {
                "_".to_owned()
            };
            let args = fn_sig
                .map(|sig| {
                    if let Some(sig) = sig {
                        format!("{}", sig[0])
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

pub trait AnalysisKind<'analysis> {
    /// Analysis results
    type Results;
    /// Interprocedural context
    type InterCtxt;
    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<Self::Results>;
}
pub enum Modular {}
impl<'analysis> AnalysisKind<'analysis> for Modular {
    type Results = ();
    type InterCtxt = ();
    fn analyze(_: &CrateCtxt) -> anyhow::Result<Self::Results> {
        // TODO implement this
        anyhow::bail!("modular analysis is not implemented")
    }
}
pub enum WholeProgram {}

impl WholeProgram {
    fn pre_generate_fn_sigs(
        crate_ctxt: &CrateCtxt,
        gen: &mut OwnershipSigGenerator,
        database: &mut Z3Database,
    ) -> FxHashMap<DefId, FnSig<Option<Range<OwnershipSig>>>> {
        let mut fn_sigs = FxHashMap::default();
        fn_sigs.reserve(crate_ctxt.functions().len());
        for &did in crate_ctxt.call_graph.functions() {
            let body = crate_ctxt.tcx.optimized_mir(did);
            let fn_sig = {
                let mut local_decls = body.local_decls.iter();
                let return_local_decl = local_decls.next().unwrap();
                let ret =
                    generate_signatures_for_local(return_local_decl, gen, database, crate_ctxt);

                let args = local_decls
                    .take(body.arg_count)
                    .map(|local_decl| {
                        generate_signatures_for_local(local_decl, gen, database, crate_ctxt)
                    })
                    .collect();

                FnSig { ret, args }
            };
            println!("generating signatures for {:?}: {:?}", did, fn_sig);
            fn_sigs.insert(did, fn_sig);
        }

        fn_sigs
    }

    #[inline]
    fn initial_state<'tcx>(crate_ctxt: &CrateCtxt<'tcx>, body: &Body<'tcx>) -> SSAState {
        let dominance_frontier = compute_dominance_frontier(body);
        let definitions = initial_definitions(body, crate_ctxt.tcx, crate_ctxt);
        SSAState::new(body, &dominance_frontier, definitions)
    }

    #[inline]
    fn solve_body<'tcx>(
        body: &Body<'tcx>,
        ssa_state: SSAState,
        crate_ctxt: &CrateCtxt<'tcx>,
        inter_ctxt: <WholeProgram as AnalysisKind>::InterCtxt,
        gen: &mut OwnershipSigGenerator,
        database: &mut Z3Database,
    ) -> anyhow::Result<FnSummary> {
        println!("solving {:?}", body.source.def_id());
        database.solver.push();

        // let ssa_state = prune(body, ssa_state);

        let mut rn = Renamer::new(body, ssa_state);

        let mut infer_cx = InferCtxt::new(crate_ctxt, body, database, gen, inter_ctxt);

        rn.go::<Self, _>(&mut infer_cx);

        let results = FnSummary::new(rn, infer_cx);

        match database.solver.check() {
            z3::SatResult::Unsat => {
                println!("failed.");
                database.solver.pop(1);
            }
            z3::SatResult::Unknown => bail!("z3 status: unknown"),
            z3::SatResult::Sat => {}
        }

        Ok(results)
    }

    #[inline]
    fn retrieve_model(
        database: Z3Database,
        start: OwnershipSig,
        gen: OwnershipSigGenerator,
    ) -> Vec<Ownership> {
        let z3_model = database.solver.get_model().unwrap();
        let mut model = Vec::with_capacity(gen.next().index());

        for _ in 0..start.index() {
            model.push(Ownership::Unknown);
        }
        for sig in start..gen.next() {
            let value = z3_model
                .eval(&database.z3_ast[sig], true)
                .unwrap()
                .as_bool();
            model.push(value.into());
        }

        model
    }

    fn apply_model(fn_summary: FnSummary, model: &[Ownership]) -> SSAState {
        let FnSummary {
            fn_body_sig,
            mut ssa_state,
        } = fn_summary;

        let consumes = &mut ssa_state.consume_chain.consumes;
        // we have to do this awkwardly as lending iterator is not ready
        for bb in 0..consumes.len() {
            for consumes in consumes[bb].iter_mut() {
                for &mut (local, ref mut consume) in consumes.iter_mut() {
                    if consume.is_use() {
                        let outter_most = fn_body_sig[local][consume.r#use].start;
                        if matches!(model[outter_most.index()], Ownership::Owning) {
                            consume.mk_def();
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
}

pub struct WholeProgramResults {
    model: Vec<Ownership>,
    fn_sigs: FxHashMap<DefId, FnSig<Option<Range<OwnershipSig>>>>,
    fn_summaries: FxHashMap<DefId, FnSummary>,
}

impl<'a> AnalysisResults<'a> for WholeProgramResults {
    type FnSig = impl Iterator<Item = Option<&'a [Ownership]>>;

    type FnResult = (&'a FnSummary, &'a [Ownership]);

    fn fn_result(&'a self, r#fn: DefId) -> Option<Self::FnResult> {
        let fn_summary = self.fn_summaries.get(&r#fn)?;
        Some((fn_summary, &self.model[..]))
    }

    #[inline]
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

impl<'analysis> AnalysisKind<'analysis> for WholeProgram {
    type Results = WholeProgramResults;

    type InterCtxt = &'analysis FxHashMap<DefId, FnSig<Option<Range<OwnershipSig>>>>;

    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<Self::Results> {
        type DB<'z3> = Z3Database<'z3>;

        let start = OwnershipSig::MIN;
        let mut gen = OwnershipSigGenerator::new(start);

        let config = z3::Config::new();
        let ctx = z3::Context::new(&config);
        let mut database = DB::new(&ctx);

        let inter_ctxt = WholeProgram::pre_generate_fn_sigs(crate_ctxt, &mut gen, &mut database);

        let mut fn_summaries = FxHashMap::default();
        fn_summaries.reserve(crate_ctxt.functions().len());

        for &did in crate_ctxt.call_graph.functions() {
            let body = crate_ctxt.tcx.optimized_mir(did);
            let ssa_state = WholeProgram::initial_state(crate_ctxt, body);
            let fn_summary = WholeProgram::solve_body(
                body,
                ssa_state,
                crate_ctxt,
                &inter_ctxt,
                &mut gen,
                &mut database,
            )?;
            fn_summaries.insert(did, fn_summary);
        }

        let model = WholeProgram::retrieve_model(database, start, gen);

        let results = WholeProgramResults {
            model,
            fn_sigs: inter_ctxt,
            fn_summaries,
        };

        results.print_fn_sigs(crate_ctxt.tcx, crate_ctxt.functions());

        Ok(results)
    }
}
pub enum StandAlone {}
impl<'analysis> AnalysisKind<'analysis> for StandAlone {
    type Results = ();
    type InterCtxt = ();
    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<Self::Results> {
        let mut databases = Vec::with_capacity(crate_ctxt.functions().len());
        for &did in crate_ctxt.functions() {
            println!("solving {:?}", did);
            let body = crate_ctxt.tcx.optimized_mir(did);

            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, crate_ctxt.tcx, crate_ctxt);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            let mut rn = Renamer::new(body, ssa_state);

            let start = OwnershipSig::MIN;
            let mut gen = OwnershipSigGenerator::new(start);
            let mut database = CadicalDatabase::new();
            let mut infer_cx = InferCtxt::new(crate_ctxt, body, &mut database, &mut gen, ());

            rn.go::<Self, _>(&mut infer_cx);
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
