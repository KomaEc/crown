mod infer;

use std::collections::VecDeque;

use once_cell::unsync::OnceCell;
use range_ext::IsRustcIndexDefinedCV;
use rustc_data_structures::graph::{scc::Sccs, WithNumNodes};
use rustc_hir::def_id::LocalDefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{Field, Local, Location},
    ty::TyCtxt,
};

use crate::{
    call_graph::{CallGraph, Func},
    def_use::MutabilityAnalysisDefUse,
    ssa::rename::{
        handler::{SSADefSites, SSANameSourceMap},
        SSAIdx, SSANameHandler,
    },
    Boundary, FuncSig, Inner, Surface, ULEConstraintGraph, UnitAnalysisCV,
};

use range_ext::RangeExt;

#[derive(Clone)]
pub struct LocalSourceInfo {
    pub base: Local,
    pub ssa_idx: SSAIdx,
    pub nested_level: usize,
}

impl std::fmt::Display for LocalSourceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let LocalSourceInfo {
            base,
            ssa_idx,
            nested_level,
        } = *self;
        f.write_fmt(format_args!(
            "{:*<1$}{2:?}^{3}",
            "", nested_level, base, ssa_idx
        ))
    }
}

pub struct InterSummary {
    pub call_graph: CallGraph,
    pub func_sigs: IndexVec<Func, FuncSig<Surface, Option<bool>>>,
    pub func_summaries: IndexVec<Func, IntraSummary>,
    pub approximate_mu_ctxt: OnceCell<IndexVec<Func, IndexVec<Mu, bool>>>,
}
impl InterSummary {
    pub fn new<'tcx, Handler: SSANameHandler<Output = ()>>(
        tcx: TyCtxt<'tcx>,
        _adt_defs: &[LocalDefId],
        call_graph: CallGraph,
        extra_handler: Handler,
    ) -> Self {
        let num_funcs = call_graph.num_nodes();
        let mut engine = AnalysisEngine {
            tcx,
            call_graph: &call_graph,
            // boundaries: &mut boundaries,
            func_sigs: IndexVec::with_capacity(num_funcs),
            intra_summaries: IndexVec::with_capacity(num_funcs),
        };

        engine.infer(extra_handler);

        let AnalysisEngine {
            func_sigs,
            intra_summaries: func_summaries,
            ..
        } = engine;

        InterSummary {
            call_graph,
            func_sigs,
            func_summaries,
            approximate_mu_ctxt: OnceCell::new(),
        }
        .debug_bidirectionality()
    }

    #[inline]
    fn debug_bidirectionality(self) -> Self {
        #[cfg(debug_assertions)]
        {
            for summary in &self.func_summaries {
                for (rho, local) in summary.local_source_iter_enumerated() {
                    let &LocalSourceInfo {
                        base,
                        ssa_idx,
                        nested_level,
                    } = local;
                    assert_eq!(rho, summary.locals[base][ssa_idx] + nested_level)
                }
            }
        }

        self
    }

    pub fn resolve(&mut self) -> Result<(), Vec<Mu>> {
        let mut in_queue = IndexVec::from_elem(true, &self.call_graph.graph.nodes);

        for (group_idx, func_group) in self
            .call_graph
            .sccs_data
            .ranked_by_post_order
            .iter_enumerated()
        {
            let mut work_list = func_group.iter().map(|&func| func).collect::<VecDeque<_>>();
            while let Some(func) = work_list.pop_front() {
                in_queue[func] = false;

                tracing::debug!("Solving {:?}", self.call_graph.functions[func]);

                let summary = &mut self.func_summaries[func];

                summary.instantiate(&self.func_sigs);

                let constraint_system = &mut summary.constraint_system;

                let func_constraint_sccs = Sccs::new(&constraint_system.le_constraints.graph);
                if !constraint_system.consistent(&func_constraint_sccs) {
                    return Err(constraint_system.le_constraints.explain(Mu::ONE, Mu::ZERO));
                }

                if !summary
                    .update_surface_func_sig(&func_constraint_sccs, &mut self.func_sigs[func])
                {
                    continue;
                }

                for caller in self.call_graph.graph.predecessor_nodes(func) {
                    if self.call_graph.sccs_data.sccs.scc(caller) == group_idx && !in_queue[caller]
                    {
                        in_queue[caller] = true;
                        work_list.push_back(caller)
                    }
                }
            }
        }

        self.approximate_mu_ctxt
            .set(
                self.func_summaries
                    .iter()
                    .map(|summary| {
                        let constraint_system = &summary.constraint_system;
                        let sccs = Sccs::<_, u32>::new(&constraint_system.le_constraints.graph);
                        let one_rep = sccs.scc(Mu::ONE);
                        constraint_system
                            .le_constraints
                            .graph
                            .nodes()
                            .map(|mu| sccs.scc(mu) == one_rep)
                            .collect()
                    })
                    .collect(),
            )
            .unwrap();
        Ok(())
    }

    pub fn show_result(&self) {
        for (func, summary) in self.func_summaries.iter_enumerated() {
            summary.local_value_with(|rho, value| {
                tracing::debug!(
                    "{:?}::{:?} = {}",
                    self.call_graph.functions[func],
                    rho,
                    value
                        .map(|value| value.then_some("1").unwrap_or("0"))
                        .unwrap_or("?")
                )
            })
        }

        for (func, func_sig) in self.func_sigs.iter_enumerated() {
            let did = self.call_graph.functions[func];
            // let (ret, args) = func_sig.sig.split_first().unwrap();
            let sig_strs = func_sig
                .sig
                .iter()
                .map(|arg| {
                    arg.iter()
                        .map(|ptr| match ptr {
                            Some(true) => "*mut".to_owned(),
                            Some(false) => "*const".to_owned(),
                            None => "*polymorphic".to_owned(),
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>();

            let (ret, args) = sig_strs.split_first().unwrap();

            tracing::debug!("{:?}: ({}) -> {}", did, args.join(", "), ret)
        }
    }
}

impl crate::api::AnalysisResults for InterSummary {
    fn local_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool> {
        let func = self.call_graph.lookup_function(&func.to_def_id()).unwrap();
        if ptr_depth > 0 {
            tracing::warn!("mutability analysis doesn't support nested level > 0");
        }
        let mc = &self.approximate_mu_ctxt.get().unwrap()[func];
        let mut results = self.func_summaries[func].locals[local]
            .iter()
            .map(|&mu| mc[mu]);
        let first = results.next().unwrap();
        assert!(
            results.all(|r| r == first),
            "no single value for local_result"
        );
        Some(first)
    }

    fn local_result_at(
        &self,
        func: LocalDefId,
        local: Local,
        loc: Location,
        ptr_depth: usize,
    ) -> Option<bool> {
        let func = self.call_graph.lookup_function(&func.to_def_id()).unwrap();
        if ptr_depth > 0 {
            tracing::warn!("mutability analysis doesn't support nested level > 0");
        }
        let source_map = &self.func_summaries[func].ssa_name_source_map;
        let ssa_idx = source_map
            .try_def(local, loc)
            .or_else(|| source_map.try_use(local, loc))
            .unwrap();
        let mu = self.func_summaries[func].locals[local][ssa_idx].clone();
        Some(self.approximate_mu_ctxt.get().unwrap()[func][mu])
    }

    fn field_result(&self, _def_id: LocalDefId, _field: Field, _ptr_depth: usize) -> Option<bool> {
        tracing::warn!("mutability analysis doesn't support struct fields");
        None
    }

    fn sig_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool> {
        let func = self.call_graph.lookup_function(&func.to_def_id()).unwrap();
        self.func_sigs[func].sig[local.as_usize()][ptr_depth]
    }
}

pub type MutabilityAnalysisBoundary = Boundary<Option<Mu>, Option<Mu>>;

pub struct IntraSummary {
    constraint_system: ConstraintDatabase,
    /// For non pointer locals, the length of inner vec is 0
    pub locals: IndexVec<Local, IndexVec<SSAIdx, Mu>>,
    intra_source_map: Vec<LocalSourceInfo>,
    boundaries: Vec<MutabilityAnalysisBoundary>,
    pub ssa_name_source_map: SSANameSourceMap<MutabilityAnalysisDefUse>,
    pub ssa_def_sites: SSADefSites<MutabilityAnalysisDefUse>,
    fn_sig: FuncSig<Inner, Mu>,
}

impl IntraSummary {
    pub fn instantiate(&mut self, surfaces: &IndexVec<Func, FuncSig<Surface, Option<bool>>>) {
        for &Boundary {
            callee,
            ref dest,
            ref arguments,
        } in &self.boundaries
        {
            let surface = &surfaces[callee];
            let (ret, parameters) = surface.sig.split_first().unwrap();
            // is of ptr type
            if !ret.is_empty() {
                let &dest = dest.as_ref().unwrap();
                if let Some(value) = ret[0] {
                    self.constraint_system.assume(dest, value);
                }
            }

            for (parameter, &argument) in parameters.iter().zip(arguments) {
                if !parameter.is_empty() {
                    let mu = argument.unwrap();
                    if let Some(value) = parameter[0] {
                        self.constraint_system.assume(mu, value)
                    }
                }
            }
        }
    }

    pub fn update_surface_func_sig(
        &self,
        constraint_graph_sccs: &Sccs<Mu, u32>,
        surface: &mut FuncSig<Surface, Option<bool>>,
    ) -> bool {
        let mut changed = false;

        for (inner, surface) in std::iter::zip(&self.fn_sig.sig, &mut surface.sig) {
            assert_eq!(inner.len(), surface.len());
            for (rho, value) in std::iter::zip(inner.clone(), surface.iter_mut()) {
                let rho_rep = constraint_graph_sccs.scc(rho);
                let zero_rep = constraint_graph_sccs.scc(Mu::ZERO);
                let one_rep = constraint_graph_sccs.scc(Mu::ONE);
                match value {
                    &mut Some(value) => {
                        let rep = value.then_some(one_rep).unwrap_or(zero_rep);
                        assert_eq!(rho_rep, rep);
                    }
                    None => {
                        if rho_rep == zero_rep {
                            changed = true;
                            *value = Some(false);
                        } else if rho_rep == one_rep {
                            changed = true;
                            *value = Some(true);
                        }
                    }
                }
            }
        }

        changed
    }

    pub fn get_local_source(&self, x: Mu) -> &LocalSourceInfo {
        assert!(x >= self.constraint_system.le_constraints.local_start);
        let idx = x.index() - self.constraint_system.le_constraints.local_start.index();
        &self.intra_source_map[idx]
    }

    pub fn local_source_iter_enumerated(&self) -> impl Iterator<Item = (Mu, &LocalSourceInfo)> {
        self.intra_source_map
            .iter()
            .enumerate()
            .map(|(idx, source)| {
                let rho = self.constraint_system.le_constraints.local_start + idx;
                (rho, source)
            })
    }

    pub fn local_value_with<F>(&self, f: F)
    where
        F: Fn(Mu, Option<bool>),
    {
        let sccs = Sccs::<_, u32>::new(&self.constraint_system.le_constraints.graph);
        let one_rep = sccs.scc(Mu::new(1));
        let zero_rep = sccs.scc(Mu::new(0));
        for x in self
            .constraint_system
            .le_constraints
            .graph
            .nodes()
            .skip(self.constraint_system.le_constraints.local_start.index())
        {
            let x_rep = sccs.scc(x);
            if x_rep == zero_rep {
                f(x, Some(false));
            } else if x_rep == one_rep {
                f(x, Some(true));
            } else {
                f(x, None);
            }
        }
    }
}

pub struct AnalysisEngine<'analysis, 'tcx> {
    tcx: TyCtxt<'tcx>,
    call_graph: &'analysis CallGraph,
    func_sigs: IndexVec<Func, FuncSig<Surface, Option<bool>>>,
    intra_summaries: IndexVec<Func, IntraSummary>,
}

pub struct ConstraintDatabase {
    le_constraints: ULEConstraintGraph<Mu>,
}

impl ConstraintDatabase {
    pub fn new(n_globals: usize, n_locals: usize) -> Self {
        Self {
            le_constraints: ULEConstraintGraph::new(n_globals, n_locals),
        }
    }

    #[inline]
    pub fn new_var(&mut self) -> Mu {
        self.le_constraints.add_node()
    }

    pub fn push_le(&mut self, x: Mu, y: Mu) {
        tracing::debug!("generate constraint {:?} ≤ {:?}", x, y);
        self.le_constraints.add_relation(x, y);
    }

    pub fn push_eq(&mut self, x: Mu, y: Mu) {
        self.le_constraints.add_relation(x, y);
        self.le_constraints.add_relation(y, x);
    }

    pub fn assume(&mut self, x: Mu, value: bool) {
        assert_ne!(x, Mu::ZERO);
        assert_ne!(x, Mu::ONE);
        tracing::debug!("assume that {:?} = {}", x, value.then_some(1).unwrap_or(0));
        value
            .then(|| {
                self.le_constraints.add_relation(Mu::ONE, x);
            })
            .unwrap_or_else(|| {
                self.le_constraints.add_relation(x, Mu::ZERO);
            })
    }

    #[inline]
    pub fn consistent(&self, sccs: &Sccs<Mu, u32>) -> bool {
        self.le_constraints.consistent(sccs)
    }
}

pub fn explain_error(reason: Vec<Mu>) {
    assert!(reason.len() >= 2);
    assert_eq!(reason[0], Mu::ONE);
    assert_eq!(*reason.last().unwrap(), Mu::ZERO);

    tracing::debug!("A chain of inequalities that leads to this conflict:");
    for &[x, y] in reason.array_windows() {
        tracing::debug!("{:?} ≤ {:?}", x, y)
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Mu {
        DEBUG_FORMAT = "μ_({})"
    }
}

impl UnitAnalysisCV for Mu {
    const ZERO: Self = Mu::from_u32(0);
    const ONE: Self = Mu::from_u32(1);
}

impl range_ext::IsConstraintVariable for Mu {}
