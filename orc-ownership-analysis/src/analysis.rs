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

use crate::analysis::constraint::{generate_signatures_for_local, Database, OwnershipSig};
use crate::analysis::state::SSAState;
use crate::call_graph::FnSig;

use self::constraint::infer::FnResult;
use self::def::Consume;

// pub mod body_ext;
pub mod constants;
pub mod constraint;
pub mod def;
pub mod dom;
pub mod join_points;
pub mod state;

impl<'tcx> CrateCtxt<'tcx> {
    pub fn crash_me_with_pure_rename(&self) {
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

    pub fn crash_me_with_inference(&self) -> anyhow::Result<()> {
        StandAlone::analyze(self)
    }

    pub fn crash_me_with_whole_program_analysis(&self) -> anyhow::Result<()> {
        WholeProgram::analyze(self).map(|results| {
            let model = &results.model[..];
            for fn_result in results.fn_results.into_values() {
                let _ = WholeProgram::apply_results(fn_result, model);
            }
        })
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

pub trait AnalysisResults {
    type FnSig<'a>: Iterator<Item = Option<&'a [Ownership]>>
    where
        Self: 'a;
    fn try_local_result(
        &self,
        r#fn: DefId,
        local: Local,
        location: Location,
    ) -> Option<&[Ownership]>;
    #[inline]
    fn local_result(&self, r#fn: DefId, local: Local, location: Location) -> &[Ownership] {
        self.try_local_result(r#fn, local, location).unwrap()
    }
    fn fn_sig(&self, r#fn: DefId) -> Self::FnSig<'_>;
    fn print_fn_sigs(&self, tcx: TyCtxt, fns: &[DefId]) {
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

pub trait AnalysisKind {
    /// Analysis results
    type Results = ();
    /// Interprocedural context
    type InterCtxt<'analysis> = ()
    where
        Self: 'analysis;
    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<Self::Results>;
}
pub enum Modular {}
impl AnalysisKind for Modular {
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
            // fn_sigs.insert(did, fn_sig);
            fn_sigs.insert(did, fn_sig);
        }

        fn_sigs
    }

    #[inline]
    fn solve_body<'tcx>(
        body: &Body<'tcx>,
        ssa_state: SSAState,
        crate_ctxt: &CrateCtxt<'tcx>,
        inter_ctxt: <WholeProgram as AnalysisKind>::InterCtxt<'_>,
        gen: &mut OwnershipSigGenerator,
        database: &mut Z3Database,
    ) -> anyhow::Result<FnResult> {
        println!("solving {:?}", body.source.def_id());
        database.solver.push();

        // let ssa_state = prune(body, ssa_state);

        let mut rn = Renamer::new(body, ssa_state);

        let mut infer_cx = InferCtxt::new(crate_ctxt, body, database, gen, inter_ctxt);

        rn.go::<Self, _>(&mut infer_cx);

        let results = FnResult::new(rn, infer_cx);

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

    fn apply_results(fn_result: FnResult, model: &[Ownership]) -> SSAState {
        let FnResult {
            fn_body_sig,
            mut ssa_state,
        } = fn_result;

        let consumes = &mut ssa_state.consume_chain.consumes;
        // we have to do this awkwardly as lending iterator is not ready
        for bb in 0..consumes.len() {
            for consume in consumes[bb].iter_mut() {
                for &mut (local, ref mut consume) in consume.iter_mut() {
                    if consume.is_use() {
                        let outter_most = fn_body_sig[local][consume.r#use].start;
                        if matches!(model[outter_most.index()], Ownership::Owning) {
                            *consume = Consume::new();
                        }
                    }
                }
            }
        }
        ssa_state.name_state.reset();
        ssa_state.join_points.reset();
        ssa_state
    }
}

pub struct WholeProgramResults {
    model: Vec<Ownership>,
    fn_sigs: FxHashMap<DefId, FnSig<Option<Range<OwnershipSig>>>>,
    fn_results: FxHashMap<DefId, FnResult>,
}

impl AnalysisResults for WholeProgramResults {
    type FnSig<'a> = impl Iterator<Item = Option<&'a [Ownership]>> + 'a where Self: 'a;

    #[inline]
    fn try_local_result(
        &self,
        r#fn: DefId,
        local: Local,
        location: Location,
    ) -> Option<&[Ownership]> {
        let fn_result = &self.fn_results[&r#fn];
        let sigs = fn_result.local_sig(local, location)?;
        Some(&self.model[sigs.start.index()..sigs.end.index()])
    }

    #[inline]
    fn fn_sig(&self, r#fn: DefId) -> Self::FnSig<'_> {
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

impl AnalysisKind for WholeProgram {
    type Results = WholeProgramResults;

    type InterCtxt<'analysis> = &'analysis FxHashMap<DefId, FnSig<Option<Range<OwnershipSig>>>>;

    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<Self::Results> {
        type DB<'z3> = Z3Database<'z3>;

        let start = DB::FIRST_AVAILABLE_SIG;
        let mut gen = OwnershipSigGenerator::new(start);

        let config = z3::Config::new();
        let ctx = z3::Context::new(&config);
        let mut database = DB::new(&ctx);

        let fn_sigs = WholeProgram::pre_generate_fn_sigs(&crate_ctxt, &mut gen, &mut database);

        let mut fn_results = FxHashMap::default();
        fn_results.reserve(crate_ctxt.functions().len());

        for &did in crate_ctxt.call_graph.functions() {
            let body = crate_ctxt.tcx.optimized_mir(did);

            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, crate_ctxt.tcx, &crate_ctxt);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            let fn_result = WholeProgram::solve_body(
                body,
                ssa_state,
                crate_ctxt,
                &fn_sigs,
                &mut gen,
                &mut database,
            )?;
            fn_results.insert(did, fn_result);
        }

        let model = WholeProgram::retrieve_model(database, start, gen);

        let results = WholeProgramResults {
            model,
            fn_sigs,
            fn_results,
        };

        results.print_fn_sigs(crate_ctxt.tcx, crate_ctxt.functions());

        Ok(results)
    }
}
pub enum StandAlone {}
impl AnalysisKind for StandAlone {
    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<Self::Results> {
        let mut databases = Vec::with_capacity(crate_ctxt.functions().len());
        for &did in crate_ctxt.functions() {
            println!("solving {:?}", did);
            let body = crate_ctxt.tcx.optimized_mir(did);

            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, crate_ctxt.tcx, &crate_ctxt);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            let mut rn = Renamer::new(body, ssa_state);

            let start = CadicalDatabase::FIRST_AVAILABLE_SIG;
            let mut gen = OwnershipSigGenerator::new(start);
            let mut database = CadicalDatabase::new();
            let mut infer_cx = InferCtxt::new(&crate_ctxt, body, &mut database, &mut gen, ());

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
