use std::ops::Range;

use rustc_index::vec::IndexVec;
use rustc_middle::ty::TyCtxt;

use crate::{
    call_graph::{CallGraph, Func},
    def_use::OwnershipAnalysisDefUse,
    Analysis, CrateAnalysisCtxt,
};

impl<'tcx> Analysis<'tcx> for CrateSummary<'tcx> {
    const NAME: &'static str = "Ownership Analysis";

    type DefUse = OwnershipAnalysisDefUse;

    type Infer<'a, E>
    where
        'tcx: 'a,
        E: crate::ssa::rename::SSANameHandler,
    = crate::ssa::rename::implementations::PlainRenamer<'a, 'tcx, Self::DefUse, E>;
}

pub struct CrateSummary<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub call_graph: CallGraph,
    pub rho_ctxt: CrateAnalysisCtxt<Rho, Option<bool>>,
    pub globals: Range<Rho>,
    pub func_summaries: IndexVec<Func, FuncSummary>,
    pub constraints: ConstraintSet,
}

pub struct FuncSummary {
    pub all_constraint_vars: Range<Rho>,
    pub all_constraints: Range<usize>,
    pub func_sig: Vec<Range<Rho>>,
}

pub enum Constraint {
    LE(Rho, Vec<Rho>),
    GE(Rho, Vec<Rho>),
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

    pub fn push_le(&mut self, lhs: Rho, rhs: Vec<Rho>) -> usize {
        let idx = self.len();
        self.data.push(Constraint::LE(lhs, rhs));
        idx
    }

    pub fn push_ge(&mut self, lhs: Rho, rhs: Vec<Rho>) -> usize {
        let idx = self.len();
        self.data.push(Constraint::GE(lhs, rhs));
        idx
    }

    pub fn push_eq(&mut self, lhs: Rho, rhs: Vec<Rho>) {
        self.push_le(lhs, rhs.clone());
        self.push_ge(lhs, rhs);
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Rho {
        DEBUG_FORMAT = "ρ_({})"
    }
}

impl range_ext::IsConstraintVariable for Rho {}
