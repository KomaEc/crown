use anyhow::bail;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::Local;
use smallvec::SmallVec;
use std::ops::Range;

use crate::analysis::constraint::infer::{InferCtxt, Pure, Renamer};
use crate::analysis::constraint::prune::prune;
use crate::analysis::constraint::{CadicalDatabase, OwnershipSigGenerator, Z3Database};
use crate::analysis::def::initial_definitions;
use crate::analysis::dom::compute_dominance_frontier;
use crate::CrateCtxt;

use crate::analysis::constraint::{generate_signatures_for_local, Database, OwnershipSig};
use crate::analysis::state::SSAState;
use crate::call_graph::FnSig;

use self::def::RichLocation;

// pub mod body_ext;
pub mod constants;
pub mod constraint;
pub mod def;
pub mod dom;
pub mod join_points;
pub mod state;
pub mod ty;

impl<'tcx> CrateCtxt<'tcx> {
    pub fn crash_me_with_pure_rename(&self) {
        for &did in self.functions() {
            println!("renaming {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, self.tcx, self);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            let mut rn = Renamer::new(body, ssa_state);
            // let mut rn = Renamer::new(body, &dominance_frontier, definitions);
            rn.go::<Pure, ()>(());
            println!("completed");
        }
    }

    pub fn crash_me_with_inference(&self) -> anyhow::Result<()> {
        StandAlone::analyze(self)
    }

    pub fn crash_me_with_whole_program_analysis(&self) -> anyhow::Result<()> {
        WholeProgram::analyze(self)
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
            Ownership::Unknown => write!(f, "&?"),
        }
    }
}

pub trait AnalysisResults {
    fn local_sigs(
        &self,
        r#rn: DefId,
        local: Local,
        location: RichLocation,
    ) -> SmallVec<[Ownership; 2]>;
    fn fn_sigs(&self, r#rn: DefId) -> FnSig<Option<SmallVec<[Ownership; 2]>>>;
}

// pub struct WholeProgramAnalysisResults<'results> {
//     z3_database: Z3Database<'results>,
//     z3_model: Model<'results>,
//     fn_sigs: FxHashMap<DefId, FnSig<Option<Range<OwnershipSig>>>>,
//     fn_bodies: FxHashMap<DefId, LocalSigs>,
//     consume_chains: FxHashMap<DefId, ConsumeChain>,
// }

pub trait AnalysisKind {
    /// Interprocedural Context
    type InterCtxt<'analysis>
    where
        Self: 'analysis;
    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<()>;
}
pub enum Modular {}
impl AnalysisKind for Modular {
    type InterCtxt<'analysis> = ();

    fn analyze(_: &CrateCtxt) -> anyhow::Result<()> {
        // TODO implement this
        anyhow::bail!("modular analysis is not implemented")
    }
}
pub enum WholeProgram {}
impl AnalysisKind for WholeProgram {
    type InterCtxt<'analysis> = &'analysis Vec<FnSig<Option<Range<OwnershipSig>>>>; //&'analysis FxHashMap<DefId, FnSig<Option<Range<OwnershipSig>>>>;

    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<()> {
        type DB<'z3> = Z3Database<'z3>;

        let start = DB::FIRST_AVAILABLE_SIG;
        let mut gen = OwnershipSigGenerator::new(start);

        let config = z3::Config::new();
        let ctx = z3::Context::new(&config);
        let mut database = DB::new(&ctx);

        let mut fn_sigs = Vec::with_capacity(crate_ctxt.functions().len()); // FxHashMap::default();
        for &did in crate_ctxt.call_graph.functions() {
            let body = crate_ctxt.tcx.optimized_mir(did);
            let fn_sig = {
                let mut local_decls = body.local_decls.iter();
                let return_local_decl = local_decls.next().unwrap();
                let ret = generate_signatures_for_local(
                    return_local_decl,
                    &mut gen,
                    &mut database,
                    crate_ctxt,
                );

                let args = local_decls
                    .take(body.arg_count)
                    .map(|local_decl| {
                        generate_signatures_for_local(
                            local_decl,
                            &mut gen,
                            &mut database,
                            crate_ctxt,
                        )
                    })
                    .collect();

                FnSig { ret, args }
            };
            println!("generating signatures for {:?}: {:?}", did, fn_sig);
            // fn_sigs.insert(did, fn_sig);
            fn_sigs.push(fn_sig);
        }

        let mut local_sigs = Vec::with_capacity(crate_ctxt.call_graph.functions().len());
        let mut consume_chains = Vec::with_capacity(crate_ctxt.call_graph.functions().len());

        for &did in crate_ctxt.call_graph.functions() {
            println!("solving {:?}", did);

            database.solver.push();

            let body = crate_ctxt.tcx.optimized_mir(did);

            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, crate_ctxt.tcx, crate_ctxt);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);

            let ssa_state = prune(body, ssa_state);

            let mut rn = Renamer::new(body, ssa_state);

            let mut infer_cx = InferCtxt::new(crate_ctxt, body, &mut database, &mut gen, &fn_sigs);

            rn.go::<Self, _>(&mut infer_cx);

            local_sigs.push(infer_cx.local_sigs);
            consume_chains.push(rn.state.consume_chain);

            match database.solver.check() {
                z3::SatResult::Unsat => {
                    println!("failed.");
                    database.solver.pop(1);
                }
                z3::SatResult::Unknown => bail!("z3 status: unknown"),
                z3::SatResult::Sat => {}
            }
        }

        let z3_model = database.solver.get_model().unwrap();

        for (did, fn_sig) in crate_ctxt
            .call_graph
            .functions()
            .iter()
            .copied()
            .zip(fn_sigs)
        {
            let fn_sig = fn_sig.repack(|sigs| {
                if let Some(sigs) = sigs {
                    sigs.map(|sig| {
                        match z3_model
                            .eval(&database.z3_ast[sig], true)
                            .unwrap()
                            .as_bool()
                        {
                            Some(true) => "&move",
                            Some(false) => "&",
                            None => "&any",
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
                } else {
                    "_".to_owned()
                }
            });

            let def_path_str = crate_ctxt.tcx.def_path_str(did);
            println!("{def_path_str}: {fn_sig}")
        }

        Ok(())
    }
}
pub enum StandAlone {}
impl AnalysisKind for StandAlone {
    type InterCtxt<'analysis> = ();

    fn analyze(crate_ctxt: &CrateCtxt) -> anyhow::Result<()> {
        let mut databases = Vec::with_capacity(crate_ctxt.functions().len());
        for &did in crate_ctxt.functions() {
            println!("solving {:?}", did);
            let body = crate_ctxt.tcx.optimized_mir(did);

            let dominance_frontier = compute_dominance_frontier(body);
            let definitions = initial_definitions(body, crate_ctxt.tcx, crate_ctxt);
            let ssa_state = SSAState::new(body, &dominance_frontier, definitions);
            // let mut rn = Renamer::new(body, &dominance_frontier, definitions);
            let mut rn = Renamer::new(body, ssa_state);

            let start = CadicalDatabase::FIRST_AVAILABLE_SIG;
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
