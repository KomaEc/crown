pub mod infer;
pub mod solve;
#[cfg(test)]
mod test;

use std::{fmt::Display, marker::PhantomData, ops::Range};

use graph::implementation::forward_star::{Direction, Graph};
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
use smallvec::SmallVec;

use crate::{
    call_graph::{CallGraph, CallSite, Func},
    def_use::OwnershipAnalysisDefUse,
    ssa::rename::{SSAIdx, SSANameHandler},
    ty_ext::TyExt,
    Analysis, Boundary, BoundaryE, CrateAnalysisCtxt, FnSigVal, FnSigValE,
};

/*
impl<'analysis, 'tcx> Analysis<'tcx> for AnalysisEngine<'analysis, 'tcx> {
    const NAME: &'static str = "Ownership Analysis";

    type DefUse = OwnershipAnalysisDefUse;

    type Infer<'a, E>
    where
        'tcx: 'a,
        E: SSANameHandler,
    = IntraInfer<'a, 'analysis, 'tcx, E>;
}
*/

#[derive(Clone)]
pub struct FieldDefSourceInfo {
    adt_def: DefId,
    variant_idx: VariantIdx,
    field_idx: usize,
    nested_level: usize,
}

impl Display for FieldDefSourceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FieldDefSourceInfo {
            adt_def,
            variant_idx,
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
    base: Local,
    ssa_idx: SSAIdx,
    nested_level: usize,
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
    global_assumptions: ConstraintDataBase,
    global_source_map: Vec<FieldDefSourceInfo>,
    field_defs: FxHashMap<DefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
}

impl InterCtxt {
    pub fn view(&mut self) -> InterCtxtView<'_> {
        InterCtxtView {
            global_assumptions: &mut self.global_assumptions,
            global_source_map: &self.global_source_map,
            field_defs: &self.field_defs,
        }
    }
}

pub struct InterCtxtView<'view> {
    global_assumptions: &'view mut ConstraintDataBase,
    global_source_map: &'view Vec<FieldDefSourceInfo>,
    field_defs: &'view FxHashMap<DefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
}

pub struct InterSummary {
    // rho_ctxt: CrateAnalysisCtxt<Rho, Option<bool>>,
    // globals: Range<Rho>,
    inter_ctxt: InterCtxt,
    call_graph: CallGraph,
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
        // let rho_ctxt = CrateAnalysisCtxt::initiate(tcx, adt_defs, &call_graph);

        // let mut global_assumptions = IndexVec::new();
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

        let global_assumptions = ConstraintDataBase::new(global_source_map.len(), 0);

        let mut inter_ctxt = InterCtxt {
            global_assumptions,
            global_source_map,
            field_defs,
        };

        let mut engine = AnalysisEngine {
            tcx,
            call_graph: &call_graph,
            inter_ctxt: inter_ctxt.view(),
            func_summaries: IndexVec::with_capacity(num_funcs),
        }
        .log_initial_state();

        engine.infer(extra_handler);

        let func_summaries = engine.func_summaries;

        InterSummary {
            inter_ctxt,
            call_graph,
            func_summaries,
        }
    }
}

pub struct AnalysisEngine<'analysis, 'tcx> {
    tcx: TyCtxt<'tcx>,
    call_graph: &'analysis CallGraph,
    // rho_ctxt: CrateAnalysisCtxt<Rho, Option<bool>>,
    // globals: Range<Rho>,
    // global_assumptions: &'analysis mut IndexVec<Rho, Option<bool>>,
    // global_source_map: &'analysis IndexVec<Rho, FieldDefSourceInfo>,
    // field_defs: &'analysis FxHashMap<DefId, IndexVec<VariantIdx, Vec<Range<Rho>>>>,
    inter_ctxt: InterCtxtView<'analysis>,
    // boundaries: IndexVec<CallSite, BoundaryE<()>>,
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

pub struct IntraSummary {
    // pub all_constraint_vars: Range<Rho>,
    pub constraint_db: ConstraintDataBase,
    // pub all_constraints: Range<usize>,
    // pub func_sig: FnSigValE<Range<Rho>>, //Vec<Range<Rho>>,
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

/// Unit Less or Equal constraint graph per function
/// Node indexing: `[0, 1, globals.start..globals.end, locals.start..locals.end]`
#[derive(Clone)]
struct ULEConstraintGraph {
    local_start: Rho,
    /// Invariant: `graph.num_nodes() == locals.len() + globals.len() + 2`
    /// TODO!!!!!!!!!!!!!!!!!!!!!
    /// use SparseBitVector Graph instead. forward star allows multi-graph!!!!!!
    graph: Graph<Rho, usize>,
}

impl ULEConstraintGraph {
    const ZERO: Rho = Rho::from_u32(0);
    const ONE: Rho = Rho::from_u32(1);

    pub fn new(n_globals: usize, n_locals: usize) -> Self {
        let mut graph = Graph::new(
            2 + n_globals + n_locals,
            (2..2 + n_globals + n_locals).flat_map(|idx| {
                let idx = Rho::new(idx);
                [(idx, Self::ONE), (Self::ZERO, idx), (idx, idx)].into_iter()
            }),
        );
        graph.add_edge(Self::ONE, Self::ONE);
        graph.add_edge(Self::ZERO, Self::ZERO);
        ULEConstraintGraph {
            local_start: Rho::new(2 + n_globals),
            graph,
        }
    }

    pub fn generate_local(&mut self) -> Rho {
        let new = self.graph.add_node();
        log::debug!("new node variable generated {:?}", new);
        self.add_fact(new, new);
        self.add_fact(0u32.into(), new);
        self.add_fact(new, 1u32.into());
        new
    }

    #[inline]
    pub fn add_fact(&mut self, x: Rho, y: Rho) -> bool {
        log::debug!("adding fact {:?} ≤ {:?}", x, y);
        self.graph.add_edge_without_dup(x, y).is_some()
    }

    pub fn show(&self) {
        for x in self.graph.nodes() {
            for y in self.graph.successor_nodes(x) {
                log::debug!("{:?} ≤ {:?}", x, y)
            }
        }
    }
}

#[derive(Clone)]
pub struct ConstraintDataBase {
    eq_constraints: Vec<OwnershipTransferConstraint>,
    le_constraints: ULEConstraintGraph,
}

impl ConstraintDataBase {
    pub fn new(n_globals: usize, n_locals: usize) -> Self {
        Self {
            eq_constraints: Vec::new(),
            le_constraints: ULEConstraintGraph::new(n_globals, n_locals),
        }
    }

    pub fn generate_local(&mut self) -> Rho {
        self.le_constraints.generate_local()
    }

    pub fn push_le(&mut self, x: Rho, y: Rho) {
        log::debug!("generate constraint {:?} ≤ {:?}", x, y);
        self.le_constraints.add_fact(x, y);
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

    pub fn assume(&mut self, x: Rho, value: bool) {
        assert_ne!(x, ULEConstraintGraph::ZERO);
        assert_ne!(x, ULEConstraintGraph::ONE);
        log::debug!("assume that {:?} = {}", x, value.then_some(1).unwrap_or(0));
        value
            .then(|| {
                self.le_constraints.add_fact(ULEConstraintGraph::ONE, x);
            })
            .unwrap_or_else(|| {
                self.le_constraints.add_fact(x, ULEConstraintGraph::ZERO);
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
    pub fn join_global_facts(&mut self, other: &Self) -> bool {
        assert_eq!(
            self.le_constraints.local_start,
            other.le_constraints.local_start
        );
        let mut changed = false;
        for x in 0u32.into()..other.le_constraints.local_start {
            for y in other.le_constraints.graph.successor_nodes(x) {
                changed = changed || self.le_constraints.add_fact(x, y)
            }
        }
        changed
    }

    pub fn saturate(self) -> Self {
        let ConstraintDataBase {
            eq_constraints,
            mut le_constraints,
        } = self;

        let mut removed = vec![false; eq_constraints.len()];

        loop {
            let mut changed = false;
            let mut sccs = Sccs::<_, u32>::new(&le_constraints.graph);
            // x + y = z
            for (
                &OwnershipTransferConstraint {
                    old_rhs: z,
                    new_lhs: x,
                    new_rhs: y,
                },
                removed,
            ) in std::iter::zip(&eq_constraints, &mut removed)
            {
                // let this_removed = &mut removed[idx];

                if !*removed {
                    log::debug!("solving equality constraint {:?} + {:?} = {:?}", x, y, z);

                    let x_rep = sccs.scc(x);
                    let y_rep = sccs.scc(y);
                    let z_rep = sccs.scc(z);
                    let zero_rep = sccs.scc(ULEConstraintGraph::ZERO);
                    // let one_rep = sccs.scc(ULEConstraintGraph::ONE);

                    if x_rep == zero_rep {
                        le_constraints.add_fact(z, y);
                        *removed = true;
                    } else if y_rep == zero_rep {
                        le_constraints.add_fact(z, x);
                        *removed = true;
                    } else if DepthFirstSearch::new(&sccs)
                        .with_start_node(z_rep)
                        .any(|idx| idx == y_rep)
                    {
                        le_constraints.add_fact(x, ULEConstraintGraph::ZERO);
                        *removed = true;
                    } else if DepthFirstSearch::new(&sccs)
                        .with_start_node(z_rep)
                        .any(|idx| idx == x_rep)
                    {
                        le_constraints.add_fact(y, ULEConstraintGraph::ZERO);
                        *removed = true;
                    }

                    // change happend
                    if *removed {
                        // recompute sccs
                        sccs = Sccs::<_, u32>::new(&le_constraints.graph);
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        let eq_constraints = std::iter::zip(eq_constraints.into_iter(), removed.into_iter())
            .filter_map(|(c, removed)| (!removed).then(|| c))
            .collect::<Vec<_>>();

        ConstraintDataBase {
            eq_constraints,
            le_constraints,
        }
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Rho {
        DEBUG_FORMAT = "ρ_({})"
    }
}

impl range_ext::IsConstraintVariable for Rho {}
