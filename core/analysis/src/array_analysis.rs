use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::Range,
};

use graph::implementation::forward_star;
use rustc_data_structures::graph::{scc::Sccs, WithNumNodes};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{Local, Operand, Place},
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;
use smallvec::SmallVec;

use crate::{
    array_analysis::solve::{solve, SolveSuccess},
    call_graph::{CallGraph, CallSite, Func},
    def_use::IsDefUse,
    ssa::rename::{handler::SSANameSourceMap, SSANameHandler},
    ty_ext::TyExt,
};

pub mod infer;
pub mod solve;
#[cfg(test)]
mod test;

/// This structure should hold info about all struct definitions
/// and local nested pointers in the crate
pub struct CrateSummary<'tcx, DefUse: IsDefUse> {
    pub tcx: TyCtxt<'tcx>,
    pub call_graph: CallGraph,
    pub lambda_ctxt: CrateLambdaCtxt,
    pub globals: Range<usize>,
    func_summaries: IndexVec<Func, FuncSummary>,
    pub constraints: ConstraintSet,
    boundary_constraints: IndexVec<CallSite, Vec<Constraint>>,
    pub ssa_name_source_map: IndexVec<Func, SSANameSourceMap>,
    _marker: PhantomData<*const DefUse>,
}

/// Pairs of start/end pointers into lambda context and constraints
/// for a given function; Function signature
pub struct FuncSummary {
    pub lambda_ctxt: Range<usize>,
    pub constraints: Range<usize>,
    /// func_sig maps function arguments and return to constraint variables. It follows
    /// the convention of MIR, where the first entry represents return place.
    /// func_sig entries are `Some` if and only if its type is pointer type of concern
    pub func_sig: Vec<SmallVec<[Lambda; NESTED_LEVEL_HINT]>>,
}

impl<'tcx, DefUse: IsDefUse> CrateSummary<'tcx, DefUse> {
    pub fn new<Handler: SSANameHandler>(
        tcx: TyCtxt<'tcx>,
        adt_defs: &[DefId],
        call_graph: CallGraph,
        extra_handler: Handler,
    ) -> Self {
        let num_funcs = call_graph.num_nodes();
        let lambda_ctxt = CrateLambdaCtxt::initiate(tcx, adt_defs, &call_graph);
        CrateSummary {
            tcx,
            call_graph,
            globals: Range {
                start: 0,
                end: lambda_ctxt.lambda_data_map.len(),
            },
            lambda_ctxt,
            func_summaries: IndexVec::with_capacity(num_funcs),
            constraints: ConstraintSet::new(),
            boundary_constraints: IndexVec::new(),
            ssa_name_source_map: IndexVec::with_capacity(num_funcs),
            _marker: PhantomData,
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
                            ..
                        } = &self.func_summaries[func];

                        let locals = locals.clone();
                        let constraints_range = constraints_range.clone();

                        match solve(
                            &mut self.lambda_ctxt.lambda_data_map.assumptions,
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

    pub fn lambda_source_data_to_str(&self, src_data: LambdaSourceData) -> String {
        match src_data {
            LambdaSourceData::Local {
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
            LambdaSourceData::FieldDef {
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
                        for (idx, lambda) in z.iter().enumerate() {
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

        for (lambda, &solution) in self
            .lambda_ctxt
            .lambda_data_map
            .assumptions
            .iter_enumerated()
        {
            // log::debug!(
            log::error!(
                "{: <7} = {: <2} at {}",
                &format!("{:?}", lambda),
                solution
                    .map(|fat| { fat.then_some("1").unwrap_or("0") })
                    .unwrap_or("?"),
                // crate_summary.lambda_ctxt.lambda_map.data_map[lambda]
                self.lambda_source_data_to_str(
                    self.lambda_ctxt.lambda_data_map.source_map[lambda].clone()
                )
            )
        }
    }
}

pub const NESTED_LEVEL_HINT: usize = 1;

/// A bidirectional map between constraint variables lambdas and the language constructs
/// we care about
pub struct CrateLambdaCtxt {
    pub lambda_data_map: LambdaDataMap<Option<bool>>, //IndexVec<Lambda, LambdaData>,
    /// did of adt_def -> variant_idx -> field_idx -> nested_level -> lambda
    /// TODO: turn nested_level -> lambda into Range<Lambda>!!!
    pub field_defs:
        FxHashMap<DefId, IndexVec<VariantIdx, Vec<SmallVec<[Lambda; NESTED_LEVEL_HINT]>>>>,
    /// func -> local -> ssa_idx -> nested_level -> lambda
    /// [[_1^0, *_1^0, **_1^0],
    ///  [_1^1, *_1^1, **_1^1],
    ///  [_1^2, *_1^2, **_1^2],
    ///  ..]
    pub locals: IndexVec<Func, IndexVec<Local, Vec<SmallVec<[Lambda; NESTED_LEVEL_HINT]>>>>,
    // pub locals: IndexVec<Func, IndexVec<Local, Vec<Vec<Lambda>>>>,
}

impl CrateLambdaCtxt {
    pub fn initiate(tcx: TyCtxt, adt_defs: &[DefId], call_graph: &CallGraph) -> Self {
        let mut lambda_data_map = LambdaDataMap::new();

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
                                        .take_while(|ty| ty.is_ptr_but_not_fn_ptr())
                                        .enumerate()
                                        .map(|(nested_level, _)| {
                                            lambda_data_map.push(
                                                None,
                                                LambdaSourceData::FieldDef {
                                                    adt_def: did,
                                                    variant_idx,
                                                    field_idx,
                                                    nested_level,
                                                },
                                            )
                                        })
                                        .collect::<SmallVec<_>>()
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<IndexVec<_, _>>(),
                )
            })
            .collect::<FxHashMap<_, _>>();

        CrateLambdaCtxt {
            lambda_data_map,
            field_defs,
            locals: IndexVec::with_capacity(call_graph.num_nodes()),
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
pub enum BoundaryConstraint<'tcx> {
    Argument {
        caller: (Operand<'tcx>, Option<usize>),
        callee: Local,
    },
    Return {
        caller: (Place<'tcx>, usize),
        callee: Local,
    },
}

/// The language constructs that a constraint variable λ corresponds to
#[derive(Clone, Debug)]
pub enum LambdaSourceData {
    /// A SSA variable
    Local {
        func: Func,
        base: Local,
        ssa_idx: usize,
        nested_level: usize,
    },
    /// field definition
    FieldDef {
        adt_def: DefId,
        variant_idx: VariantIdx,
        field_idx: usize,
        nested_level: usize,
    },
}

impl Display for LambdaSourceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LambdaSourceData::Local {
                func: body,
                base,
                ssa_idx,
                nested_level,
            } => f.write_fmt(format_args!(
                "({:?}, {:*<2$}{3:?}^{4})",
                body, "", nested_level, base, ssa_idx
            )),
            LambdaSourceData::FieldDef {
                adt_def,
                variant_idx: _,
                field_idx,
                nested_level,
            } => f.write_fmt(format_args!(
                "{:*<1$}{2:?}.{3}",
                "", nested_level, adt_def, field_idx
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LambdaDataMap<Domain: Clone + Copy> {
    pub assumptions: IndexVec<Lambda, Domain>,
    pub source_map: IndexVec<Lambda, LambdaSourceData>,
}

impl<Domain: Clone + Copy> LambdaDataMap<Domain> {
    pub fn new() -> Self {
        LambdaDataMap {
            assumptions: IndexVec::new(),
            source_map: IndexVec::new(),
        }
    }

    pub fn push(&mut self, domain: Domain, data: LambdaSourceData) -> Lambda {
        let _lambda = self.assumptions.push(domain);
        let lambda = self.source_map.push(data);
        debug_assert!(_lambda == lambda);
        lambda
    }

    pub fn len(&self) -> usize {
        let res = self.assumptions.len();
        debug_assert_eq!(res, self.source_map.len());
        res
    }

    pub fn next_index(&self) -> Lambda {
        let res = self.assumptions.next_index();
        debug_assert_eq!(res, self.source_map.next_index());
        res
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Lambda {
        DEBUG_FORMAT = "λ_({})"
    }
}
