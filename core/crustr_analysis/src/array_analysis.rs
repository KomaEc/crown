use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use graph::implementation::forward_star;
use rustc_data_structures::graph::{scc::Sccs, WithNumNodes};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::Local,
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;

use crate::{
    array_analysis::solve::{solve, SolveSuccess},
    call_graph::{CallGraph, CallSite, Func},
    ty_ext::TyExt,
};

pub mod infer;
pub mod solve;
#[cfg(test)]
mod test;

/// This structure should hold info about all struct definitions
/// and local nested pointers in the crate
pub struct CrateSummary<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub call_graph: CallGraph,
    lambda_ctxt: CrateLambdaCtxt,
    globals: Range<usize>,
    func_summaries: IndexVec<Func, FuncSummary>,
    constraints: Vec<Constraint>,
    boundary_constraints: IndexVec<CallSite, Vec<Constraint>>,
}

/// Pairs of start/end pointers into lambda context and constraints
/// for a given function
#[derive(Clone)]
pub struct FuncSummary {
    pub lambda_ctxt: Range<usize>,
    pub constraints: Range<usize>,
}

impl<'tcx> CrateSummary<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, adt_defs: &[DefId], call_graph: CallGraph) -> Self {
        let num_funcs = call_graph.num_nodes();
        let lambda_ctxt = CrateLambdaCtxt::initiate(tcx, adt_defs, &call_graph);
        CrateSummary {
            tcx,
            call_graph,
            globals: Range {
                start: 0,
                end: lambda_ctxt.lambda_map.len(),
            },
            lambda_ctxt,
            func_summaries: IndexVec::with_capacity(num_funcs),
            constraints: Vec::new(),
            boundary_constraints: IndexVec::new(),
        }
    }

    pub fn iterate_to_fixpoint(&mut self) -> Result<(), ()> {
        let call_graph_sccs = Sccs::<Func, usize>::new(&self.call_graph);
        // it seems that the scc algorithm will rank sccs in post order, namely if there
        // is a dependency S1 -> S2, then S2 < S1 (well it is natural to be this case,
        // from the Tarjan's algorithm's perspective). Here we follow this assumption.
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
                        } = self.func_summaries[func].clone();
                        match solve(
                            &mut self.lambda_ctxt.lambda_map.assumptions,
                            self.globals.clone(),
                            locals,
                            &self.constraints[constraints_range],
                            self.call_graph
                                .graph
                                .adjacent_edges(func, forward_star::Direction::Outgoing)
                                .map(|(call_site, _)| {
                                    self.boundary_constraints[call_site].iter().map(|&c| c)
                                })
                                .flatten(),
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

    pub fn debug(&self) {
        log::debug!("Initialising crate summary");
        for (&adt_did, x) in &self.lambda_ctxt.field_defs {
            for (variant_idx, y) in x.iter_enumerated() {
                for (field_idx, z) in y.iter().enumerate() {
                    let adt_def = self.tcx.adt_def(adt_did);
                    let field_def = &adt_def.variants[variant_idx].fields[field_idx];
                    let field_def_str = format!("{}.{}", self.tcx.type_of(adt_did), field_def.name);
                    log::debug!(
                        "for field {}: {}:",
                        field_def_str,
                        self.tcx.type_of(field_def.did)
                    );
                    for (idx, lambda) in z.iter().enumerate() {
                        log::debug!("{:*<1$}{2} ==> {3:?}", "", idx, field_def_str, lambda)
                    }
                }
            }
        }
    }
}

/// A bidirectional map between constraint variables lambdas and the language constructs
/// we care about
pub struct CrateLambdaCtxt {
    pub lambda_map: LambdaMap<Option<bool>>, //IndexVec<Lambda, LambdaData>,
    /// did of adt_def -> variant_idx -> field_idx -> nested_level -> lambda
    pub field_defs: FxHashMap<DefId, IndexVec<VariantIdx, Vec<Vec<Lambda>>>>,
    pub func_ctxt: IndexVec<Func, FuncLambdaCtxt>,
}

pub struct FuncLambdaCtxt {
    pub local: IndexVec<Local, Vec<Lambda>>,
    pub local_nested: IndexVec<Local, Vec<Lambda>>,
}

impl From<(IndexVec<Local, Vec<Lambda>>, IndexVec<Local, Vec<Lambda>>)> for FuncLambdaCtxt {
    fn from(
        (local, local_nested): (IndexVec<Local, Vec<Lambda>>, IndexVec<Local, Vec<Lambda>>),
    ) -> Self {
        FuncLambdaCtxt {
            local,
            local_nested,
        }
    }
}

impl CrateLambdaCtxt {
    pub fn initiate(tcx: TyCtxt, adt_defs: &[DefId], call_graph: &CallGraph) -> Self {
        let mut lambda_map = LambdaMap::new();

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
                                    ty.walk()
                                        .filter_map(|generic_arg| {
                                            if let GenericArgKind::Type(ty) = generic_arg.unpack() {
                                                Some(ty)
                                            } else {
                                                None
                                            }
                                        })
                                        .take_while(|ty| ty.is_ptr_of_concerned())
                                        .enumerate()
                                        .map(|(nested_level, _)| {
                                            lambda_map.push(
                                                None,
                                                LambdaSourceData::FieldDef {
                                                    adt_def: did,
                                                    variant_idx,
                                                    field_idx,
                                                    nested_level,
                                                },
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<IndexVec<_, _>>(),
                )
            })
            .collect::<FxHashMap<_, _>>();

        CrateLambdaCtxt {
            lambda_map,
            field_defs,
            func_ctxt: IndexVec::with_capacity(call_graph.num_nodes()),
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

#[derive(Clone, Debug)]
pub enum BoundaryConstraint {
    Argument { caller: Lambda, callee: Local },
    Return { caller: Lambda, callee: Local },
}

/// The language constructs that a constraint variable λ corresponds to
#[derive(Clone, Debug)]
pub enum LambdaSourceData {
    /// A SSA scalar variable
    LocalScalar {
        func: Func,
        base: Local,
        ssa_idx: usize,
    },
    /// field definition
    FieldDef {
        adt_def: DefId,
        variant_idx: VariantIdx,
        field_idx: usize,
        nested_level: usize,
    },
    /// A local nested pointer type.
    /// For example, if a local `_1` has type `*mut *mut *mut i32`, then
    /// we should have entries for `*_1` and `**_1`
    LocalNested {
        func: Func,
        base: Local,
        nested_level: usize,
    },
}

impl Display for LambdaSourceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LambdaSourceData::LocalScalar {
                func: body,
                base,
                ssa_idx,
            } => f.write_fmt(format_args!("({:?}, {:?}^{})", body, base, ssa_idx)),
            LambdaSourceData::FieldDef {
                adt_def,
                variant_idx: _,
                field_idx,
                nested_level,
            } => f.write_fmt(format_args!(
                "{:*<1$}{2:?}.{3}",
                "", nested_level, adt_def, field_idx
            )),
            LambdaSourceData::LocalNested {
                func: body,
                base,
                nested_level,
            } => f.write_fmt(format_args!(
                "({:?}, {:*<2$}{3:?})",
                body, "", nested_level, base
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LambdaMap<Domain: Clone + Copy> {
    pub assumptions: IndexVec<Lambda, Domain>,
    pub data_map: IndexVec<Lambda, LambdaSourceData>,
}

impl<Domain: Clone + Copy> LambdaMap<Domain> {
    pub fn new() -> Self {
        LambdaMap {
            assumptions: IndexVec::new(),
            data_map: IndexVec::new(),
        }
    }

    pub fn push(&mut self, domain: Domain, data: LambdaSourceData) -> Lambda {
        let _lambda = self.assumptions.push(domain);
        let lambda = self.data_map.push(data);
        debug_assert!(_lambda == lambda);
        lambda
    }

    pub fn len(&self) -> usize {
        let res = self.assumptions.len();
        debug_assert_eq!(res, self.data_map.len());
        res
    }

    pub fn next_index(&self) -> Lambda {
        let res = self.assumptions.next_index();
        debug_assert_eq!(res, self.data_map.next_index());
        res
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Lambda {
        DEBUG_FORMAT = "λ_({})"
    }
}
