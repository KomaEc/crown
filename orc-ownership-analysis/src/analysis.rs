use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::Local;
use smallvec::SmallVec;
use std::marker::PhantomData;
use std::ops::Range;

use crate::analysis::body_ext::BodyExt;
use crate::analysis::constraint::infer::{InferCtxt, Pure, Renamer, WithCtxt};
use crate::analysis::constraint::CadicalDatabase;
use crate::analysis::def::initial_definitions;
use crate::CrateCtxt;

use self::constraint::{Database, OwnershipSig};
use self::state::SSAIdx;

pub mod body_ext;
pub mod constants;
pub mod constraint;
pub mod def;
// pub mod dom;
pub mod join_points;
pub mod state;
pub mod ty_ext;

pub struct FnSig<T> {
    ret: T,
    args: SmallVec<[T; 4]>,
}

impl<T> FnSig<T> {
    fn new(ret: T, args: SmallVec<[T; 4]>) -> Self {
        FnSig { ret, args }
    }
}

impl<T: Default> Default for FnSig<T> {
    fn default() -> Self {
        Self {
            ret: Default::default(),
            args: Default::default(),
        }
    }
}

impl<'tcx> CrateCtxt<'tcx> {
    pub fn crash_me_with_pure_rename(&self) {
        for &did in self.functions() {
            println!("renaming {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = body.compute_dominance_frontier();
            let definitions = initial_definitions(body, self.tcx, self);
            let mut rn = Renamer::new(body, &dominance_frontier, definitions);
            rn.go::<Pure, ()>(());
            println!("completed");
        }
    }

    pub fn crash_me_with_inference(&self) -> anyhow::Result<()> {
        let mut databases = Vec::with_capacity(self.functions().len());
        for &did in self.functions() {
            println!("solving {:?}", did);
            let body = self.tcx.optimized_mir(did);
            let dominance_frontier = body.compute_dominance_frontier();
            let definitions = initial_definitions(body, self.tcx, self);
            let mut infer_cx = InferCtxt::new(self, body, &definitions, CadicalDatabase::new());
            let mut rn = Renamer::new(body, &dominance_frontier, definitions);
            rn.go::<WithCtxt, _>(&mut infer_cx);
            match infer_cx.database.solver.solve() {
                Some(true) => {
                    println!("succeeded");
                    // for sig in infer_cx.all_sigs() {
                    //     let value = infer_cx.database.solver.value(sig.into_lit());
                    //     // println!("{sig} = {:?}", value)
                    //     if let Some(value) = value {
                    //         println!("{sig} = {}", value as u32);
                    //     } else {
                    //         println!("{sig} = any")
                    //     }
                    // }
                }
                Some(false) => println!("failed"), // anyhow::bail!("failed in solving ownership constraints"),
                None => anyhow::bail!("timeout"),
            }
            databases.push(infer_cx.database);
        }
        Ok(())
    }
}

pub trait AnalysisKind {
    type Constraints<DB>;
}
pub enum Modular {}
impl AnalysisKind for Modular {
    type Constraints<DB> = Vec<DB>; // FxHashMap<DefId, DB>;
}
pub enum WholeProgram {}
impl AnalysisKind for WholeProgram {
    type Constraints<DB> = DB;
}

pub struct Analysis<'analysis, 'tcx, DB, Kind: AnalysisKind> {
    crate_ctxt: &'analysis CrateCtxt<'tcx>,
    db: Kind::Constraints<DB>,
    _kind: PhantomData<*const Kind>,
}

impl<'analysis, 'tcx, DB> Analysis<'analysis, 'tcx, DB, WholeProgram>
where
    'tcx: 'analysis,
    DB: Database,
{
    pub fn new(crate_ctxt: &'analysis CrateCtxt<'tcx>, database: DB) -> Self {
        let mut analysis_ctxt = AnalysisCtxt::default();
        for &did in crate_ctxt.functions() {
            let body = crate_ctxt.tcx.optimized_mir(did);
            let dominance_frontier = body.compute_dominance_frontier();
            let definitions = initial_definitions(body, crate_ctxt.tcx, crate_ctxt);
        }
        todo!()
        // Self {
        //     crate_ctxt,
        //     db: database,
        //     _kind: PhantomData,
        // }
    }
}

pub struct AnalysisCtxt<DB> {
    /// DefId -> Local -> SSAIdx -> Range<OwnershipSig>
    local_sigs: FxHashMap<DefId, IndexVec<Local, IndexVec<SSAIdx, Range<OwnershipSig>>>>,
    /// DefId -> FnSig
    fn_sigs: FxHashMap<DefId, FnSig<Option<Range<OwnershipSig>>>>,
    database: DB,
}

impl<DB: Database> AnalysisCtxt<DB> {
    pub fn new(database: DB) -> Self {
        AnalysisCtxt {
            local_sigs: Default::default(),
            fn_sigs: Default::default(),
            database,
        }
    }
}
