pub mod infer;
#[cfg(test)]
mod test;

use std::{collections::VecDeque, fmt::Display, ops::Range};

use range_ext::IsRustcIndexDefinedCV;
use rustc_data_structures::graph::{iterate::DepthFirstSearch, scc::Sccs, WithNumNodes};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::Local,
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;

use crate::{
    call_graph::{CallGraph, Func},
    ssa::rename::{SSAIdx, SSANameHandler},
    ty_ext::TyExt,
    Boundary, FnSig, Inner, Surface, ULEConstraintGraph, UnitAnalysisCV,
};

use self::infer::PtrPlaceDefResult;

use range_ext::RangeExt;

#[derive(Clone)]
pub struct FieldDefSourceInfo {
    pub adt_def: DefId,
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
            "", nested_level, adt_def, field_idx
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
    global_assumptions: ConstraintDatabase,
    global_source_map: Vec<FieldDefSourceInfo>,
    field_defs: FxHashMap<DefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
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
    field_defs: &'view FxHashMap<DefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
}

pub struct InterSummary {
    inter_ctxt: InterCtxt,
    call_graph: CallGraph,
    func_sigs: IndexVec<Func, FnSig<Surface, Option<bool>>>,
    func_summaries: IndexVec<Func, IntraSummary>,
}

impl InterSummary {
    pub fn new<'tcx, Handler: SSANameHandler<Output = ()>>(
        tcx: TyCtxt<'tcx>,
        adt_defs: &[DefId],
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
            func_summaries: IndexVec::with_capacity(num_funcs),
        }
        .log_initial_state();

        engine.infer(extra_handler);

        // let func_summaries = engine.func_summaries;
        let AnalysisEngine {
            func_sigs,
            func_summaries,
            ..
        } = engine;

        InterSummary {
            inter_ctxt,
            call_graph,
            func_sigs,
            func_summaries,
        }
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

    pub fn resolve(&mut self) -> Result<(), ()> {
        // let mut global_constraint_sccs =
        //     Sccs::<_, u32>::new(&self.inter_ctxt.global_assumptions.le_constraints.graph);
        'outter: loop {
            // first propagate global assumptions to individual functions
            for summary in self.func_summaries.iter_mut() {
                summary
                    .constraint_system
                    .join_global_facts(&self.inter_ctxt.global_assumptions);
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
                    debug_assert!(
                        !constraint_system.join_global_facts(&self.inter_ctxt.global_assumptions)
                    );


                    let func_constraint_sccs = constraint_system.saturate();

                    if !constraint_system.consistent(&func_constraint_sccs) {
                        return Err(());
                    }

                    if self
                        .inter_ctxt
                        .global_assumptions
                        .join_global_facts(&*constraint_system)
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
}

pub struct AnalysisEngine<'analysis, 'tcx> {
    tcx: TyCtxt<'tcx>,
    call_graph: &'analysis CallGraph,
    inter_ctxt: InterCtxtView<'analysis>,
    // boundaries: &'analysis mut IndexVec<CallSite, Vec<OwnershipAnalysisBoundary>>,
    func_sigs: IndexVec<Func, FnSig<Surface, Option<bool>>>,
    func_summaries: IndexVec<Func, IntraSummary>,
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

pub type OwnershipAnalysisBoundary = Boundary<Option<PtrPlaceDefResult>, Option<PtrPlaceDefResult>>;

pub struct IntraSummary {
    constraint_system: ConstraintDatabase,
    /// For non pointer locals, the length of inner vec is 0
    locals: IndexVec<Local, IndexVec<SSAIdx, Range<Rho>>>,
    intra_source_map: Vec<LocalSourceInfo>,
    boundaries: Vec<OwnershipAnalysisBoundary>,
    fn_sig: FnSig<Inner, Rho>,
}

impl IntraSummary {
    pub fn instantiate(
        &mut self,
        surfaces: &IndexVec<Func, FnSig<Surface, Option<bool>>>
    ) {
        for &Boundary {
            callee,
            ref dest,
            ref arguments,
        } in &self.boundaries
        {
            let surface = &surfaces[callee];
            let (ret, parameters) = surface.sig.split_first().unwrap();
            todo!()
        }
    }

    pub fn update_surface_func_sig(
        &self,
        constraint_graph_sccs: &Sccs<Rho, u32>,
        surface: &mut FnSig<Surface, Option<bool>>,
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

    /*
    pub fn get_sccs(&mut self) -> &Sccs<Rho, u32> {
        self.sccs_cache.get_or_insert_with(|| Sccs::<_, u32>::new(&self.le_constraints.graph))
    }
    */

    /// Assumption: eq_constraints do not involve pure global facts!!!!!
    /// TODO: cleverer? changed only when facts can not be deduced
    pub fn join_global_facts(&mut self, other: &Self) -> bool {
        assert_eq!(
            self.le_constraints.local_start,
            other.le_constraints.local_start
        );
        let local_start = self.le_constraints.local_start;
        let mut changed = false;
        for x in 0u32.into()..local_start {
            if x < local_start {
                for y in other.le_constraints.graph.successor_nodes(x) {
                    if y < local_start {
                        changed = changed || self.le_constraints.add_relation(x, y)
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

    pub fn saturate(&mut self) -> Sccs<Rho, u32> {
        let mut removed = vec![false; self.eq_constraints.len()];

        let mut sccs = Sccs::<_, u32>::new(&self.le_constraints.graph);
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
                        self.le_constraints.add_relation(z, y);
                        *removed = true;
                    } else if y_rep == zero_rep {
                        self.le_constraints.add_relation(z, x);
                        *removed = true;
                    } else if DepthFirstSearch::new(&sccs)
                        .with_start_node(z_rep)
                        .any(|rep| rep == y_rep)
                    {
                        self.le_constraints.add_relation(x, Rho::ZERO);
                        *removed = true;
                    } else if DepthFirstSearch::new(&sccs)
                        .with_start_node(z_rep)
                        .any(|rep| rep == x_rep)
                    {
                        self.le_constraints.add_relation(y, Rho::ZERO);
                        *removed = true;
                    }

                    // change happend
                    if *removed {
                        // recompute sccs
                        sccs = Sccs::<_, u32>::new(&self.le_constraints.graph);
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        let mut removed = removed.into_iter();

        self.eq_constraints.retain(|_| !removed.next().unwrap());

        sccs
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
