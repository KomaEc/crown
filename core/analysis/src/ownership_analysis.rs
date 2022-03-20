pub mod infer;
pub mod solve;
#[cfg(test)]
mod test;

use std::{fmt::Display, ops::Range, marker::PhantomData};

use range_ext::IsRustcIndexDefinedCV;
use rustc_data_structures::graph::WithNumNodes;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::ty::TyCtxt;
use smallvec::SmallVec;

use crate::{
    call_graph::{CallGraph, CallSite, Func},
    def_use::OwnershipAnalysisDefUse,
    ssa::rename::SSANameHandler,
    Analysis, Boundary, BoundaryE, CrateAnalysisCtxt, FnSigVal, FnSigValE, ULEConstraintGraph,
};

use self::infer::InferEngine;

impl<'tcx> Analysis<'tcx> for AnalysisEngine<'tcx> {
    const NAME: &'static str = "Ownership Analysis";

    type DefUse = OwnershipAnalysisDefUse;

    type Infer<'a, E>
    where
        'tcx: 'a,
        E: SSANameHandler,
    = InferEngine<'a, 'tcx, E>;
}

pub struct CrateSummary {
    rho_ctxt: CrateAnalysisCtxt<Rho, Option<bool>>,
    globals: Range<Rho>,
    func_summaries: IndexVec<Func, FuncSummary>,
}

impl CrateSummary {
    pub fn new<'tcx, Handler: SSANameHandler<Output = ()>>(
        tcx: TyCtxt<'tcx>,
        adt_defs: &[DefId],
        call_graph: CallGraph,
        extra_handler: Handler,
    ) -> Self {
        let num_funcs = call_graph.num_nodes();
        let rho_ctxt = CrateAnalysisCtxt::initiate(tcx, adt_defs, &call_graph);
        let mut engine = AnalysisEngine {
            tcx,
            call_graph,
            globals: Range {
                start: 0u32.into(),
                end: rho_ctxt.assumptions.next_index(),
            },
            rho_ctxt,
            func_summaries: IndexVec::with_capacity(num_funcs),
        }
        .log_initial_state();
        engine.infer(extra_handler);
        CrateSummary {
            rho_ctxt: engine.rho_ctxt,
            globals: engine.globals,
            func_summaries: engine.func_summaries,
        }
    }
}

pub struct AnalysisEngine<'tcx> {
    tcx: TyCtxt<'tcx>,
    call_graph: CallGraph,
    rho_ctxt: CrateAnalysisCtxt<Rho, Option<bool>>,
    globals: Range<Rho>,
    // boundaries: IndexVec<CallSite, BoundaryE<()>>,
    func_summaries: IndexVec<Func, FuncSummary>,
}

impl<'tcx> AnalysisEngine<'tcx> {
    fn log_initial_state(self) -> Self {
        #[cfg(debug_assertions)]
        {
            log::debug!("Initialising crate summary");
            for (&adt_did, x) in &self.rho_ctxt.field_defs {
                for (variant_idx, y) in x.iter_enumerated() {
                    for (field_idx, z) in y.iter().enumerate() {
                        let adt_def = self.tcx.adt_def(adt_did);
                        let field_def = &adt_def.variants[variant_idx].fields[field_idx];
                        let field_def_str =
                            format!("{}.{}", self.tcx.type_of(adt_did), field_def.name);
                        log::debug!(
                            "for field {}: {}:",
                            field_def_str,
                            self.tcx.type_of(field_def.did)
                        );
                        for (idx, lambda) in z.clone().enumerate() {
                            log::debug!("{:*<1$}{2} ==> {3:?}", "", idx, field_def_str, lambda)
                        }
                    }
                }
            }
        }
        self
    }
}

pub struct FuncSummary {
    // pub all_constraint_vars: Range<Rho>,
    pub constraint_db: ConstraintDataBase<Saturated>,
    // pub all_constraints: Range<usize>,
    pub func_sig: FnSigValE<Range<Rho>>, //Vec<Range<Rho>>,
}

/// old_rhs = new_lhs + new_rhs
pub struct OwnershipTransferConstraint {
    old_rhs: Rho,
    new_lhs: Rho,
    new_rhs: Rho,
}

impl Display for OwnershipTransferConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} = {:?} + {:?}", self.old_rhs, self.new_lhs, self.new_rhs))
    }
}

pub struct MaybeSaturated;
pub struct Saturated;

pub struct ConstraintDataBase<State> {
    eq_constraints: Vec<OwnershipTransferConstraint>,
    le_constraints: ULEConstraintGraph<Rho>,
    _marker: PhantomData<State>
}

impl ConstraintDataBase<MaybeSaturated> {
    pub fn new(globals: Range<Rho>, locals: Range<Rho>) -> Self {
        Self {
            eq_constraints: Vec::new(),
            le_constraints: ULEConstraintGraph::new(globals, locals),
            _marker: PhantomData
        }
    }

    pub fn push_le(&mut self, x: Rho, y: Rho) {
        log::debug!("generate constraint {:?} ≤ {:?}", x, y);
        self.le_constraints.add_fact(x, y)
    }

    pub fn push_eq(&mut self, x: Rho, y: Rho) {
        self.push_le(x, y);
        self.push_le(y, x);
    }

    pub fn push_transfer(&mut self, old_rhs: Rho, new_lhs: Rho, new_rhs: Rho) {
        let constraint = OwnershipTransferConstraint {
            old_rhs,
            new_lhs,
            new_rhs,
        };
        log::debug!("generate constraint {}", &constraint);
        self.eq_constraints.push(constraint);

        self.push_le(new_lhs, old_rhs);
        self.push_le(new_rhs, old_rhs);
    }

    pub fn saturate(self) -> ConstraintDataBase<Saturated> {
        loop {
            todo!()
        }
    }
}

/*
pub enum Constraint {
    GE(Rho, SmallVec<[Rho; 1]>),
    Eq(Rho, SmallVec<[Rho; 2]>),
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constraint::GE(x, ys) => f.write_fmt(format_args!(
                "{:?} ≥ {}",
                x,
                ys.iter()
                    .map(|y| format!("{y:?}"))
                    .collect::<Vec<_>>()
                    .join(" + ")
            )),
            Constraint::Eq(x, ys) => f.write_fmt(format_args!(
                "{:?} = {}",
                x,
                ys.iter()
                    .map(|y| format!("{y:?}"))
                    .collect::<Vec<_>>()
                    .join(" + ")
            )),
        }
    }
}

pub struct ConstraintSet {
    data: Vec<Constraint>,
}

impl std::ops::Deref for ConstraintSet {
    type Target = Vec<Constraint>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl ConstraintSet {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.data.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Constraint> {
        self.data.into_iter()
    }

    pub fn push_ge(&mut self, lhs: Rho, rhs: impl Iterator<Item = Rho>) -> usize {
        let idx = self.len();
        self.data.push(Constraint::GE(lhs, rhs.collect()));
        #[cfg(debug_assertions)]
        log::debug!("generate constraint {}", self.data[idx]);
        idx
    }

    pub fn push_eq(&mut self, lhs: Rho, rhs: impl Iterator<Item = Rho>) -> usize {
        // self.push_le(lhs, rhs.clone());
        // self.push_ge(lhs, rhs);
        let idx = self.len();
        self.data.push(Constraint::Eq(lhs, rhs.collect()));
        #[cfg(debug_assertions)]
        log::debug!("generate constraint {}", self.data[idx]);
        idx
    }
}
*/

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Rho {
        DEBUG_FORMAT = "ρ_({})"
    }
}

impl range_ext::IsConstraintVariable for Rho {}
