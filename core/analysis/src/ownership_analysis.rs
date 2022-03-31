pub mod infer;
#[cfg(test)]
mod test;

use std::{collections::VecDeque, fmt::Display, ops::Range};

use range_ext::IsRustcIndexDefinedCV;
use rustc_data_structures::graph::{iterate::DepthFirstSearch, scc::Sccs, WithNumNodes};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::LocalDefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::Local,
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;

use crate::{
    call_graph::{CallGraph, Func},
    def_use::OwnershipAnalysisDefUse,
    ssa::rename::{
        handler::{SSADefSites, SSANameSourceMap},
        SSAIdx, SSANameHandler,
    },
    ty_ext::TyExt,
    Boundary, FuncSig, Inner, Surface, ULEConstraintGraph, UnitAnalysisCV,
};

use self::infer::PtrPlaceDefResult;

use range_ext::RangeExt;

#[derive(Clone)]
pub struct FieldDefSourceInfo {
    pub adt_def: LocalDefId,
    pub variant_idx: VariantIdx,
    pub field_idx: usize,
    pub nested_level: usize,
}

impl Display for FieldDefSourceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FieldDefSourceInfo {
            adt_def,
            variant_idx: _,
            field_idx,
            nested_level,
        } = *self;
        f.write_fmt(format_args!(
            "{:*<1$}{2:?}.{3}",
            "",
            nested_level,
            adt_def.to_def_id(),
            field_idx
        ))
    }
}

#[derive(Clone)]
pub struct LocalSourceInfo {
    pub base: Local,
    pub ssa_idx: SSAIdx,
    pub nested_level: usize,
}

impl Display for LocalSourceInfo {
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

pub struct InterCtxt {
    pub global_assumptions: ConstraintDatabase,
    global_source_map: Vec<FieldDefSourceInfo>,
    pub field_defs: FxHashMap<LocalDefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
}

impl InterCtxt {
    pub fn view(&mut self) -> InterCtxtView<'_> {
        InterCtxtView {
            global_assumptions: &mut self.global_assumptions,
            // global_source_map: &self.global_source_map,
            field_defs: &self.field_defs,
        }
    }
}
pub struct InterCtxtView<'view> {
    global_assumptions: &'view mut ConstraintDatabase,
    field_defs: &'view FxHashMap<LocalDefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
}

pub struct InterSummary {
    pub inter_ctxt: InterCtxt,
    call_graph: CallGraph,
    func_sigs: IndexVec<Func, FuncSig<Surface, Option<bool>>>,
    func_summaries: IndexVec<Func, IntraSummary>,
}

impl InterSummary {
    pub fn new<'tcx, Handler: SSANameHandler<Output = ()>>(
        tcx: TyCtxt<'tcx>,
        adt_defs: &[LocalDefId],
        call_graph: CallGraph,
        extra_handler: Handler,
    ) -> Self {
        let num_funcs = call_graph.num_nodes();

        let mut global_source_map = Vec::new();
        let field_defs = adt_defs
            .iter()
            .map(|&did| {
                (
                    did,
                    tcx.adt_def(did)
                        .variants
                        .iter_enumerated()
                        .map(|(variant_idx, variant_def)| {
                            variant_def
                                .fields
                                .iter()
                                .enumerate()
                                .map(|(field_idx, field_def)| {
                                    let ty = tcx.type_of(field_def.did);

                                    let start = global_source_map.len() + 2;

                                    ty.walk()
                                        .filter_map(|generic_arg| {
                                            if let GenericArgKind::Type(ty) = generic_arg.unpack() {
                                                Some(ty)
                                            } else {
                                                None
                                            }
                                        })
                                        .take_while(|ty| ty.is_ptr_but_not_fn_ptr())
                                        .enumerate()
                                        .for_each(|(nested_level, _)| {
                                            // global_assumptions.push(None);
                                            global_source_map.push(FieldDefSourceInfo {
                                                adt_def: did,
                                                variant_idx,
                                                field_idx,
                                                nested_level,
                                            });
                                        });

                                    let end = global_source_map.len() + 2;

                                    Range {
                                        start: start.into(),
                                        end: end.into(),
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<IndexVec<_, _>>(),
                )
            })
            .collect::<FxHashMap<_, _>>();

        let global_assumptions = ConstraintDatabase::new(global_source_map.len(), 0);

        let mut inter_ctxt = InterCtxt {
            global_assumptions,
            global_source_map,
            field_defs,
        };

        // let mut boundaries = IndexVec::from_elem_n(Vec::new(), call_graph.num_edges());

        let mut engine = AnalysisEngine {
            tcx,
            call_graph: &call_graph,
            inter_ctxt: inter_ctxt.view(),
            // boundaries: &mut boundaries,
            func_sigs: IndexVec::with_capacity(num_funcs),
            intra_summaries: IndexVec::with_capacity(num_funcs),
        }
        .log_initial_state();

        engine.infer(extra_handler);

        // let func_summaries = engine.func_summaries;
        let AnalysisEngine {
            func_sigs,
            intra_summaries: func_summaries,
            ..
        } = engine;

        InterSummary {
            inter_ctxt,
            call_graph,
            func_sigs,
            func_summaries,
        }
        .debug_bidirectionality()
    }

    #[inline]
    fn debug_bidirectionality(self) -> Self {
        #[cfg(debug_assertions)]
        {
            for (rho, field_def) in self.field_def_source_iter_enumerated() {
                let &FieldDefSourceInfo {
                    adt_def,
                    variant_idx,
                    field_idx,
                    nested_level,
                } = field_def;
                assert_eq!(
                    rho,
                    self.inter_ctxt.field_defs[&adt_def][variant_idx][field_idx].start
                        + nested_level
                )
            }

            for summary in &self.func_summaries {
                for (rho, local) in summary.local_source_iter_enumerated() {
                    let &LocalSourceInfo {
                        base,
                        ssa_idx,
                        nested_level,
                    } = local;
                    assert_eq!(rho, summary.locals[base][ssa_idx].start + nested_level)
                }
            }
        }

        self
    }

    pub fn get_field_def_source(&self, x: Rho) -> &FieldDefSourceInfo {
        let idx = x.index() - 2;
        &self.inter_ctxt.global_source_map[idx]
    }

    pub fn field_def_source_iter_enumerated(
        &self,
    ) -> impl Iterator<Item = (Rho, &FieldDefSourceInfo)> {
        self.inter_ctxt
            .global_source_map
            .iter()
            .enumerate()
            .map(|(idx, field_def)| {
                let rho = Rho::new(idx + 2);
                (rho, field_def)
            })
    }

    pub fn resolve(&mut self) -> Result<(), Vec<Rho>> {
        'outter: loop {
            // first propagate global assumptions to individual functions
            for summary in self.func_summaries.iter_mut() {
                summary
                    .constraint_system
                    .join_global_facts(&self.inter_ctxt.global_assumptions, None);
            }

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

                    log::debug!("Solving {:?}", self.call_graph.functions[func]);

                    let summary = &mut self.func_summaries[func];

                    summary.instantiate(&self.func_sigs);

                    let constraint_system = &mut summary.constraint_system;
                    // before solving, global facts should be joined
                    debug_assert!(!constraint_system
                        .join_global_facts(&self.inter_ctxt.global_assumptions, None));

                    let func_constraint_sccs = constraint_system.saturate()?;

                    if self
                        .inter_ctxt
                        .global_assumptions
                        .join_global_facts(&*constraint_system, Some(&func_constraint_sccs))
                    {
                        continue 'outter;
                    }

                    if !summary
                        .update_surface_func_sig(&func_constraint_sccs, &mut self.func_sigs[func])
                    {
                        continue;
                    }

                    for caller in self.call_graph.graph.predecessor_nodes(func) {
                        if self.call_graph.sccs_data.sccs.scc(caller) == group_idx
                            && !in_queue[caller]
                        {
                            in_queue[caller] = true;
                            work_list.push_back(caller)
                        }
                    }
                }
            }

            break;
        }

        Ok(())
    }

    pub fn field_def_value_with<F>(&self, f: F)
    where
        F: Fn(Rho, Option<bool>),
    {
        let sccs = Sccs::<_, u32>::new(&self.inter_ctxt.global_assumptions.le_constraints.graph);
        let one_rep = sccs.scc(Rho::new(1));
        let zero_rep = sccs.scc(Rho::new(0));
        for x in self
            .inter_ctxt
            .global_assumptions
            .le_constraints
            .graph
            .nodes()
            .skip(2)
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

    pub fn field_def_known_to_be_owning(&self) -> Vec<Rho> {
        DepthFirstSearch::new(&self.inter_ctxt.global_assumptions.le_constraints.graph)
            .with_start_node(Rho::ONE)
            .filter(|&rho| rho > Rho::ONE)
            .collect()
    }

    pub fn show_result(&self) {
        self.field_def_value_with(|rho, value| {
            log::debug!(
                "GLOBAL::{:?} = {}",
                rho,
                value
                    .map(|value| value.then_some("1").unwrap_or("0"))
                    .unwrap_or("?")
            )
        });

        for (func, summary) in self.func_summaries.iter_enumerated() {
            summary.local_value_with(|rho, value| {
                log::debug!(
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
                            Some(true) => "&own".to_owned(),
                            Some(false) => "&transient".to_owned(),
                            None => "&polymorphic".to_owned(),
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>();

            let (ret, args) = sig_strs.split_first().unwrap();

            log::debug!("{:?}: ({}) -> {}", did, args.join(", "), ret)
        }
    }
}

pub struct AnalysisEngine<'analysis, 'tcx> {
    tcx: TyCtxt<'tcx>,
    call_graph: &'analysis CallGraph,
    inter_ctxt: InterCtxtView<'analysis>,
    // boundaries: &'analysis mut IndexVec<CallSite, Vec<OwnershipAnalysisBoundary>>,
    func_sigs: IndexVec<Func, FuncSig<Surface, Option<bool>>>,
    intra_summaries: IndexVec<Func, IntraSummary>,
}

impl<'me, 'tcx> AnalysisEngine<'me, 'tcx> {
    fn log_initial_state(self) -> Self {
        #[cfg(debug_assertions)]
        {
            log::debug!("Initialising crate summary");
            for (&adt_did, x) in self.inter_ctxt.field_defs {
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
                        for (idx, rho) in z.clone().enumerate() {
                            log::debug!("{:*<1$}{2} ==> GLOBAL::{3:?}", "", idx, field_def_str, rho)
                        }
                    }
                }
            }
        }
        self
    }
}

pub type OwnershipAnalysisBoundary = Boundary<Option<Range<Rho>>, Option<PtrPlaceDefResult>>;

pub struct IntraSummary {
    constraint_system: ConstraintDatabase,
    /// For non pointer locals, the length of inner vec is 0
    locals: IndexVec<Local, IndexVec<SSAIdx, Range<Rho>>>,
    intra_source_map: Vec<LocalSourceInfo>,
    boundaries: Vec<OwnershipAnalysisBoundary>,
    pub ssa_name_source_map: SSANameSourceMap<OwnershipAnalysisDefUse>,
    pub ssa_def_sites: SSADefSites<OwnershipAnalysisDefUse>,
    fn_sig: FuncSig<Inner, Rho>,
}

impl IntraSummary {
    pub fn instantiate(&mut self, surfaces: &IndexVec<Func, FuncSig<Surface, Option<bool>>>) {
        log::debug!("Instantiating boundary constraints");
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
                let dest = dest.as_ref().unwrap();
                for (dest, &ret) in dest.clone().zip(ret.iter()) {
                    if let Some(value) = ret {
                        self.constraint_system.assume(dest, value);
                    }
                }
            }

            for (parameter, argument) in parameters.iter().zip(arguments) {
                // is of ptr type
                if !parameter.is_empty() {
                    match argument.as_ref().unwrap() {
                        PtrPlaceDefResult::Base { old, new } => {
                            assert_eq!(parameter.len(), new.len());
                            let (&parameter_outtermost, parameter_inner) =
                                parameter.split_first().unwrap();
                            match parameter_outtermost {
                                Some(true) => {
                                    self.constraint_system.assume(old.start, true);
                                    self.constraint_system.assume(new.start, false)
                                }
                                Some(false) => self.constraint_system.push_le(old.start, new.start),
                                None => {}
                            }
                            for (argument, parameter) in new.clone().skip(1).zip(parameter_inner) {
                                if let &Some(value) = parameter {
                                    self.constraint_system.assume(argument, value)
                                }
                            }
                        }
                        PtrPlaceDefResult::Proj(rhos) => {
                            let mut arg_para_pair_iter = rhos.clone().zip(parameter);
                            let (arg_outtermost, para_outtermost) =
                                arg_para_pair_iter.next().unwrap();
                            if let &Some(true) = para_outtermost {
                                self.constraint_system.assume(arg_outtermost, true)
                            }
                            for (arg, para) in arg_para_pair_iter {
                                if let &Some(value) = para {
                                    self.constraint_system.assume(arg, value)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update_surface_func_sig(
        &self,
        constraint_graph_sccs: &Sccs<Rho, u32>,
        surface: &mut FuncSig<Surface, Option<bool>>,
    ) -> bool {
        let mut changed = false;

        for (inner, surface) in std::iter::zip(&self.fn_sig.sig, &mut surface.sig) {
            assert_eq!(inner.len(), surface.len());
            for (rho, value) in std::iter::zip(inner.clone(), surface.iter_mut()) {
                let rho_rep = constraint_graph_sccs.scc(rho);
                let zero_rep = constraint_graph_sccs.scc(Rho::ZERO);
                let one_rep = constraint_graph_sccs.scc(Rho::ONE);
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

        #[cfg(debug_assertions)]
        {
            if changed {
                // let (ret, args) = func_sig.sig.split_first().unwrap();
                let sig_strs = surface
                    .sig
                    .iter()
                    .map(|arg| {
                        arg.iter()
                            .map(|ptr| match ptr {
                                Some(true) => "&own".to_owned(),
                                Some(false) => "&transient".to_owned(),
                                None => "&polymorphic".to_owned(),
                            })
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .collect::<Vec<_>>();

                let (ret, args) = sig_strs.split_first().unwrap();

                log::debug!("signature updated to: ({}) -> {}", args.join(", "), ret)
            }
        }

        changed
    }

    pub fn get_local_source(&self, x: Rho) -> &LocalSourceInfo {
        assert!(x >= self.constraint_system.le_constraints.local_start);
        let idx = x.index() - self.constraint_system.le_constraints.local_start.index();
        &self.intra_source_map[idx]
    }

    pub fn local_source_iter_enumerated(&self) -> impl Iterator<Item = (Rho, &LocalSourceInfo)> {
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
        F: Fn(Rho, Option<bool>),
    {
        let sccs = Sccs::<_, u32>::new(&self.constraint_system.le_constraints.graph);
        let one_rep = sccs.scc(Rho::new(1));
        let zero_rep = sccs.scc(Rho::new(0));
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

/// old_rhs = new_lhs + new_rhs
#[derive(Clone)]
pub struct OwnershipTransferConstraint {
    old_rhs: Rho,
    new_lhs: Rho,
    new_rhs: Rho,
}

impl Display for OwnershipTransferConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?} = {:?} + {:?}",
            self.old_rhs, self.new_lhs, self.new_rhs
        ))
    }
}

#[derive(Clone)]
pub struct ConstraintDatabase {
    eq_constraints: Vec<OwnershipTransferConstraint>,
    le_constraints: ULEConstraintGraph<Rho>,
}

impl ConstraintDatabase {
    pub fn new(n_globals: usize, n_locals: usize) -> Self {
        Self {
            eq_constraints: Vec::new(),
            le_constraints: ULEConstraintGraph::new(n_globals, n_locals),
        }
    }

    #[inline]
    pub fn new_var(&mut self) -> Rho {
        self.le_constraints.add_node()
    }

    pub fn push_le(&mut self, x: Rho, y: Rho) {
        log::debug!("generate constraint {:?} ≤ {:?}", x, y);
        self.le_constraints.add_relation(x, y);
    }

    #[inline]
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

    pub fn assume(&mut self, x: Rho, value: bool) {
        assert_ne!(x, Rho::ZERO);
        assert_ne!(x, Rho::ONE);
        log::debug!("assume that {:?} = {}", x, value.then_some(1).unwrap_or(0));
        value
            .then(|| {
                self.le_constraints.add_relation(Rho::ONE, x);
            })
            .unwrap_or_else(|| {
                self.le_constraints.add_relation(x, Rho::ZERO);
            })
    }

    pub fn show(&self) {
        let sccs = Sccs::<_, u32>::new(&self.le_constraints.graph);
        let one_rep = sccs.scc(Rho::new(1));
        let zero_rep = sccs.scc(Rho::new(0));
        for x in self.le_constraints.graph.nodes().skip(2) {
            let x_rep = sccs.scc(x);
            if x_rep == zero_rep {
                log::debug!("{:?} = 0", x)
            } else if x_rep == one_rep {
                log::debug!("{:?} = 1", x)
            } else {
                log::debug!("{:?} = ?", x)
            }
        }
    }

    /// Assumption: eq_constraints do not involve pure global facts!!!!!
    /// TODO: cleverer? changed only when facts can not be deduced
    pub fn join_global_facts(&mut self, other: &Self, absolute: Option<&Sccs<Rho, u32>>) -> bool {
        assert_eq!(
            self.le_constraints.local_start,
            other.le_constraints.local_start
        );
        let local_start = self.le_constraints.local_start;
        let mut changed = false;
        for x in 1u32.into()..local_start {
            if x < local_start {
                for y in other.le_constraints.graph.successor_nodes(x) {
                    if y > Rho::ZERO && x != y && y < local_start {
                        changed = changed || self.le_constraints.add_relation(x, y)
                    }
                }

                if let Some(sccs) = absolute {
                    let x_rep = sccs.scc(x);
                    let zero_rep = sccs.scc(Rho::ZERO);
                    let one_rep = sccs.scc(Rho::ONE);
                    if x_rep == zero_rep {
                        changed = changed || self.le_constraints.add_relation(x, Rho::ZERO);
                    } else if x_rep == one_rep {
                        changed = changed || self.le_constraints.add_relation(Rho::ONE, x);
                    }
                }
            }
        }
        changed
    }

    #[inline]
    pub fn consistent(&self, sccs: &Sccs<Rho, u32>) -> bool {
        self.le_constraints.consistent(sccs)
    }

    pub fn saturate(&mut self) -> Result<Sccs<Rho, u32>, Vec<Rho>> {
        let mut removed = vec![false; self.eq_constraints.len()];

        let mut sccs = Sccs::<_, u32>::new(&self.le_constraints.graph);
        if !self.consistent(&sccs) {
            return Err(self.le_constraints.explain(Rho::ONE, Rho::ZERO));
        }
        loop {
            let mut changed = false;
            // x + y = z
            for (
                &OwnershipTransferConstraint {
                    old_rhs: z,
                    new_lhs: x,
                    new_rhs: y,
                },
                removed,
            ) in std::iter::zip(&self.eq_constraints, &mut removed)
            {
                // let this_removed = &mut removed[idx];

                if !*removed {
                    log::debug!("solving equality constraint {:?} + {:?} = {:?}", x, y, z);

                    let x_rep = sccs.scc(x);
                    let y_rep = sccs.scc(y);
                    let z_rep = sccs.scc(z);
                    let zero_rep = sccs.scc(Rho::ZERO);
                    // let one_rep = sccs.scc(ULEConstraintGraph::ONE);

                    if x_rep == zero_rep {
                        log::debug!("     {:?} = 0, {:?} = {:?} + {:?}", x, z, x, y);
                        log::debug!("---------------------------------------------");
                        log::debug!("     {:?} = {:?}", z, y);
                        self.le_constraints.add_relation(z, y);
                        *removed = true;
                    } else if y_rep == zero_rep {
                        log::debug!("     {:?} = 0, {:?} = {:?} + {:?}", y, z, x, y);
                        log::debug!("---------------------------------------------");
                        log::debug!("     {:?} = {:?}", z, x);
                        self.le_constraints.add_relation(z, x);
                        *removed = true;
                    } else if DepthFirstSearch::new(&sccs)
                        .with_start_node(z_rep)
                        .any(|rep| rep == y_rep)
                    {
                        log::debug!("     {:?} ≤ {:?}, {:?} = {:?} + {:?}", z, y, z, x, y);
                        log::debug!("---------------------------------------------");
                        log::debug!("     {:?} = 0", x);
                        self.le_constraints.add_relation(x, Rho::ZERO);
                        *removed = true;
                    } else if DepthFirstSearch::new(&sccs)
                        .with_start_node(z_rep)
                        .any(|rep| rep == x_rep)
                    {
                        log::debug!("     {:?} ≤ {:?}, {:?} = {:?} + {:?}", z, x, z, x, y);
                        log::debug!("---------------------------------------------");
                        log::debug!("     {:?} = 0", y);
                        self.le_constraints.add_relation(y, Rho::ZERO);
                        *removed = true;
                    }

                    // change happend
                    if *removed {
                        // recompute sccs
                        sccs = Sccs::<_, u32>::new(&self.le_constraints.graph);
                        changed = true;

                        if !self.consistent(&sccs) {
                            return Err(self.le_constraints.explain(Rho::ONE, Rho::ZERO));
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }

        let mut removed = removed.into_iter();

        self.eq_constraints.retain(|_| !removed.next().unwrap());

        Ok(sccs)
    }
}

pub fn explain_error(reason: Vec<Rho>) {
    assert!(reason.len() >= 2);
    assert_eq!(reason[0], Rho::ONE);
    assert_eq!(*reason.last().unwrap(), Rho::ZERO);

    log::debug!("A chain of inequalities that leads to this conflict:");
    for &[x, y] in reason.array_windows() {
        log::debug!("{:?} ≤ {:?}", x, y)
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Rho {
        DEBUG_FORMAT = "ρ_({})"
    }
}

impl UnitAnalysisCV for Rho {
    const ZERO: Self = Rho::from_u32(0);
    const ONE: Self = Rho::from_u32(1);
}

impl range_ext::IsConstraintVariable for Rho {}
