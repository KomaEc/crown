use std::fmt::{Debug, Display};

use rustc_data_structures::graph::WithNumNodes;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::Local,
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;

use crate::{
    call_graph::{CallGraph, CallSite, Func},
    ty_ext::TyExt,
};

pub mod intra;
pub mod solve;
#[cfg(test)]
mod test;

/// This structure should hold info about all struct definitions
/// and local nested pointers in the crate
pub struct CrateSummary<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub call_graph: CallGraph,
    lambda_ctxt: CrateLambdaCtxt,
    constraints: IndexVec<ConstraintIdx, Constraint>,
    boundary_constraints: IndexVec<CallSite, Vec<Constraint>>,
    equalities: Vec<Vec<Lambda>>,
}

impl<'tcx> CrateSummary<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, adt_defs: &[DefId], call_graph: CallGraph) -> Self {
        let lambda_ctxt = CrateLambdaCtxt::initiate(tcx, adt_defs, &call_graph);
        CrateSummary {
            tcx,
            call_graph,
            lambda_ctxt,
            constraints: IndexVec::new(),
            boundary_constraints: IndexVec::new(),
            equalities: vec![],
        }
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
                                        .take_while(|ty| {
                                            ty.is_ptr_of_concerned() && !ty.is_fn_ptr()
                                        })
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
#[derive(Clone, Debug)]
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

/*
#[derive(Clone, Debug)]
pub struct BoundaryConstraint {
    pub parameter: Local,
    /// Local and ssa_idx
    pub argument: (Local, usize),
}

impl Display for BoundaryConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} ≤ {:?}", self.parameter, self.argument))
    }
}
*/

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
}

pub fn assert_fat(lambda_map: &mut LambdaMap<Option<bool>>, lambda: Lambda) {
    log::debug!("assert that {:?} is fat", lambda);
    let assumption = &mut lambda_map.assumptions[lambda];
    if matches!(assumption, Some(false)) {
        panic!("conflict in constraint!")
    } else {
        *assumption = Some(true)
    }
}

pub fn assert_thin(lambda_map: &mut LambdaMap<Option<bool>>, lambda: Lambda) {
    log::debug!("assert that {:?} is thin", lambda);
    let assumption = &mut lambda_map.assumptions[lambda];
    if matches!(assumption, Some(true)) {
        panic!("conflict in constraint!")
    } else {
        *assumption = Some(false)
    }
}

rustc_index::newtype_index! {
    pub struct ConstraintIdx {
        DEBUG_FORMAT = "constraint ({})"
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Lambda {
        DEBUG_FORMAT = "λ_({})"
    }
}
