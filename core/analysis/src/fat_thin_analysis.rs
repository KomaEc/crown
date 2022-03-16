use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use graph::implementation::forward_star;
use rustc_data_structures::graph::{scc::Sccs, WithNumNodes};
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{mir::Local, ty::TyCtxt};

use crate::{
    call_graph::{CallGraph, CallSite, Func},
    def_use::FatThinAnalysisDefUse,
    fat_thin_analysis::solve::{solve, SolveSuccess},
    ssa::rename::{
        handler::{SSADefSites, SSANameSourceMap},
        SSANameHandler,
    },
    Analysis, CVSourceData, CrateAnalysisCtxt,
};

use self::infer::InferEngine;

pub mod infer;
pub mod solve;
#[cfg(test)]
mod test;

impl<'tcx> Analysis<'tcx> for CrateSummary<'tcx> {
    const NAME: &'static str = "Fat/Thin Analysis";
    type DefUse = FatThinAnalysisDefUse;
    type Infer<'infercx, H>
    where
        'tcx: 'infercx,
        H: SSANameHandler,
    = InferEngine<'infercx, 'tcx, H>;
}

/// This structure should hold info about all struct definitions
/// and local nested pointers in the crate
pub struct CrateSummary<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub call_graph: CallGraph,
    // pub lambda_ctxt: CrateLambdaCtxt,
    pub lambda_ctxt: CrateAnalysisCtxt<Lambda, Option<bool>>,
    pub globals: Range<usize>,
    func_summaries: IndexVec<Func, FuncSummary>,
    pub constraints: ConstraintSet,
    boundary_constraints: IndexVec<CallSite, Vec<Constraint>>,
    pub def_sites: IndexVec<Func, SSADefSites<FatThinAnalysisDefUse>>,
    pub ssa_name_source_map: IndexVec<Func, SSANameSourceMap<FatThinAnalysisDefUse>>,
}

/// Pairs of start/end pointers into lambda context and constraints
/// for a given function; Function signature
pub struct FuncSummary {
    pub lambda_ctxt: Range<usize>,
    pub constraints: Range<usize>,
    /// func_sig maps function arguments and return to constraint variables. It follows
    /// the convention of MIR, where the first entry represents return place.
    /// func_sig entries are empty if and only if its type is pointer type of concern
    pub func_sig: Vec<Range<Lambda>>,
}

impl<'tcx> CrateSummary<'tcx> {
    pub fn new<Handler: SSANameHandler<Output = ()>>(
        tcx: TyCtxt<'tcx>,
        adt_defs: &[DefId],
        call_graph: CallGraph,
        extra_handler: Handler,
    ) -> Self {
        let num_funcs = call_graph.num_nodes();
        let lambda_ctxt = CrateAnalysisCtxt::initiate(tcx, adt_defs, &call_graph);
        CrateSummary {
            tcx,
            call_graph,
            globals: Range {
                start: 0,
                end: lambda_ctxt.assumptions.len(),
            },
            lambda_ctxt,
            func_summaries: IndexVec::with_capacity(num_funcs),
            constraints: ConstraintSet::new(),
            boundary_constraints: IndexVec::new(),
            def_sites: IndexVec::with_capacity(num_funcs),
            ssa_name_source_map: IndexVec::with_capacity(num_funcs),
        }
        .log_initial_state()
        .infer_all::<_>(extra_handler)
        .debug_state_after_infer()
    }

    pub fn iterate_to_fixpoint(&mut self) -> Result<(), ()> {
        let boundary_constraints = IndexVec::from_fn_n(
            |func| {
                self.call_graph
                    .graph
                    .adjacent_edges(func, forward_star::Direction::Outgoing)
                    .map(|(call_site, _)| self.boundary_constraints[call_site].iter().map(|&c| c))
                    .flatten()
                    .collect::<Vec<_>>()
            },
            self.call_graph.functions.len(),
        );

        let call_graph_sccs = Sccs::<Func, usize>::new(&self.call_graph);
        let mut scc_nodes = vec![Vec::new(); call_graph_sccs.num_sccs()];
        for func in self.call_graph.graph.nodes() {
            scc_nodes[call_graph_sccs.scc(func)].push(func)
        }

        'globally_changed: loop {
            for scc_node in &scc_nodes {
                // TODO: use worklist algorithm for inner loop
                'locally_changed: loop {
                    let mut locally_changed = false;
                    for &func in scc_node {
                        #[cfg(debug_assertions)]
                        {
                            log::debug!("processing {:?}", self.call_graph.functions[func])
                        }
                        let FuncSummary {
                            lambda_ctxt: locals,
                            constraints: constraints_range,
                            ..
                        } = &self.func_summaries[func];

                        let locals = locals.clone();
                        let constraints_range = constraints_range.clone();

                        match solve(
                            &mut self.lambda_ctxt.assumptions,
                            self.globals.clone(),
                            locals,
                            &self.constraints[constraints_range],
                            &boundary_constraints[func],
                        )? {
                            SolveSuccess::Unchanged => {}
                            SolveSuccess::LocallyChanged => locally_changed = true,
                            SolveSuccess::GloballyChanged => continue 'globally_changed,
                        }
                    }
                    if locally_changed {
                        continue 'locally_changed;
                    } else {
                        break;
                    }
                }
            }
            break;
        }
        Ok(())
    }

    pub fn source_data_to_str(&self, src_data: CVSourceData) -> String {
        match src_data {
            CVSourceData::Local {
                func,
                base,
                ssa_idx,
                nested_level,
            } => {
                let did = self.call_graph.functions[func];
                format!(
                    "{:*<1$}{2:?}^{3} in {4}",
                    "",
                    nested_level,
                    base,
                    ssa_idx,
                    self.tcx.def_path_str(did)
                )
            }
            CVSourceData::FieldDef {
                adt_def,
                variant_idx,
                field_idx,
                nested_level,
            } => {
                let adt_def = self.tcx.adt_def(adt_def);
                let variant_def = &adt_def.variants[variant_idx];
                let field_def = &variant_def.fields[field_idx];
                format!(
                    "{:*<1$}{2}.{3}",
                    "", nested_level, variant_def.name, field_def.name
                )
            }
        }
    }

    fn log_initial_state(self) -> Self {
        #[cfg(debug_assertions)]
        {
            log::debug!("Initialising crate summary");
            for (&adt_did, x) in &self.lambda_ctxt.field_defs {
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

    fn debug_state_after_infer(self) -> Self {
        #[cfg(debug_assertions)]
        {
            assert_eq!(self.ssa_name_source_map.len(), self.call_graph.num_nodes());
            assert_eq!(self.func_summaries.len(), self.call_graph.num_nodes());
        }
        self
    }

    pub fn error_state(&self) {
        log::error!("All constraints:");
        for constraint in self.constraints.iter() {
            // log::debug!("{}", constraint)
            log::error!("{}", constraint)
        }

        for (lambda, &solution) in self.lambda_ctxt.assumptions.iter_enumerated() {
            // log::debug!(
            log::error!(
                "{: <7} = {: <2} at {}",
                &format!("{:?}", lambda),
                solution
                    .map(|fat| { fat.then_some("1").unwrap_or("0") })
                    .unwrap_or("?"),
                self.source_data_to_str(self.lambda_ctxt.source_map[lambda].clone())
            )
        }
    }
}

/// λ1 ≤ λ2
#[derive(Clone, Copy, Debug)]
pub struct Constraint(Lambda, Lambda);

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} ≤ {:?}", self.0, self.1))
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

    pub fn push_le(&mut self, lhs: Lambda, rhs: Lambda) {
        let constraint = Constraint(lhs, rhs);
        log::debug!("generate constraint {}", constraint);
        self.data.push(constraint);
    }

    pub fn push_eq(&mut self, lhs: Lambda, rhs: Lambda) {
        self.push_le(lhs, rhs);
        self.push_le(rhs, lhs);
    }
}

#[derive(Clone, Debug)]
pub enum BoundaryConstraint {
    Argument {
        caller: Range<Lambda>,
        callee: Local,
    },
    Return {
        caller: Range<Lambda>,
        callee: Local,
    },
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Lambda {
        DEBUG_FORMAT = "λ_({})"
    }
}

impl range_ext::IsConstraintVariable for Lambda {}
