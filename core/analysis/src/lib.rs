#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(min_specialization)]
#![feature(crate_visibility_modifier)]
#![feature(bool_to_option)]
#![feature(split_array)]
#![feature(generic_associated_types)]
#![feature(associated_type_defaults)]
#![feature(step_trait)]

pub mod boundary_model;
pub mod call_graph;
pub mod def_use;
pub mod fat_thin_analysis;
pub mod lattice;
pub mod libcall_model;
pub mod liveness_analysis;
pub mod null_analysis;
pub mod ownership_analysis;
pub mod pointer_analysis;
pub mod ssa;
#[cfg(test)]
pub mod test;
pub mod toy_analysis;
pub mod ty_ext;

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate tracing;

use boundary_model::BoundaryModel;
use call_graph::{CallGraph, Func};
use def_use::IsDefUse;
use graph::implementation::forward_star::Graph;
use lattice::JoinLattice;
use libcall_model::LibCallModel;
use range_ext::{IsRustcIndexDefinedCV, RangeExt};
use rustc_data_structures::graph::WithNumNodes;
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::{
    mir::{BasicBlock, Body, Local, Location},
    ty::{subst::GenericArgKind, TyCtxt},
};
use rustc_target::abi::VariantIdx;
use ssa::rename::{SSAIdx, SSANameHandler, SSARename};
use std::{
    fmt::Display,
    iter::Step,
    ops::{Index, IndexMut, Range},
};
use ty_ext::TyExt;

/// This is a marker trait marking analysis that uses the same variant
/// of the SSA rename algorithm to infer constraints. Note that the infer
/// engine `Infer` is required to use the same `IsDefUse` as the whole
/// analysis
pub trait Analysis<'tcx> {
    const NAME: &'static str;
    type DefUse: IsDefUse;
    type Infer<'a, E>: SSARename<'tcx, DefUse = Self::DefUse>
        + LibCallModel<'tcx>
        + BoundaryModel<'tcx>
    where
        'tcx: 'a,
        E: SSANameHandler;
}

pub struct CrateAnalysisCtxt<CV, Domain>
where
    CV: IsRustcIndexDefinedCV,
    Domain: Clone + Copy + JoinLattice,
{
    pub assumptions: IndexVec<CV, Domain>,
    pub source_map: IndexVec<CV, CVSourceData>,
    /// did of adt_def -> variant_idx -> field_idx -> nested_level -> constraint variables
    pub field_defs: FxHashMap<DefId, IndexVec<VariantIdx, Vec<Range<CV>>>>,
    /// func -> local -> ssa_idx -> nested_level -> constraint variables
    /// [[_1^0, *_1^0, **_1^0],
    ///  [_1^1, *_1^1, **_1^1],
    ///  [_1^2, *_1^2, **_1^2],
    ///  ..]
    pub locals: IndexVec<Func, IndexVec<Local, IndexVec<SSAIdx, Range<CV>>>>,
}

pub struct CrateAnalysisCtxtIntraView<'intra, CV, Domain>
where
    CV: IsRustcIndexDefinedCV,
    Domain: Clone + Copy + JoinLattice,
{
    func: Func,
    assumptions: &'intra mut IndexVec<CV, Domain>,
    source_map: &'intra mut IndexVec<CV, CVSourceData>,
    field_defs: &'intra FxHashMap<DefId, IndexVec<VariantIdx, Vec<Range<CV>>>>,
    locals: &'intra mut IndexVec<Local, IndexVec<SSAIdx, Range<CV>>>,
}

impl<CV, Domain> CrateAnalysisCtxt<CV, Domain>
where
    CV: IsRustcIndexDefinedCV,
    Domain: Clone + Copy + JoinLattice,
{
    pub fn initiate(tcx: TyCtxt, adt_defs: &[DefId], call_graph: &CallGraph) -> Self {
        // let mut lambda_data_map = LambdaDataMap::new();
        let mut assumptions = IndexVec::new();
        let mut source_map = IndexVec::new();

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

                                    let start = assumptions.next_index();

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
                                            assumptions.push(Domain::bottom());
                                            source_map.push(CVSourceData::FieldDef {
                                                adt_def: did,
                                                variant_idx,
                                                field_idx,
                                                nested_level,
                                            });
                                        });

                                    let end = assumptions.next_index();

                                    Range { start, end }
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<IndexVec<_, _>>(),
                )
            })
            .collect::<FxHashMap<_, _>>();

        CrateAnalysisCtxt {
            assumptions,
            source_map,
            field_defs,
            locals: IndexVec::with_capacity(call_graph.num_nodes()),
        }
    }

    pub fn push_cv(&mut self, domain: Domain, source: CVSourceData) -> CV {
        let cv = self.assumptions.push(domain);
        assert_eq!(cv, self.source_map.push(source));
        cv
    }

    pub fn next_index(&self) -> CV {
        let cv = self.assumptions.next_index();
        assert_eq!(cv, self.source_map.next_index());
        cv
    }

    pub fn intra_view(
        &mut self,
        body: &Body,
        func: Func,
    ) -> CrateAnalysisCtxtIntraView<'_, CV, Domain> {
        let locals = body
            .local_decls
            .iter_enumerated()
            .map(|(local, local_decl)| {
                let start = self.assumptions.next_index();
                local_decl
                    .ty
                    .walk()
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
                        self.assumptions.push(Domain::bottom());
                        self.source_map.push(CVSourceData::Local {
                            func,
                            base: local,
                            ssa_idx: 0usize.into(),
                            nested_level,
                        });
                    });
                let end = self.assumptions.next_index();
                IndexVec::from_raw(vec![Range { start, end }])
            })
            .collect::<IndexVec<_, _>>();

        assert_eq!(func, self.locals.push(locals));

        CrateAnalysisCtxtIntraView {
            func,
            assumptions: &mut self.assumptions,
            source_map: &mut self.source_map,
            field_defs: &self.field_defs,
            locals: &mut self.locals[func],
        }
    }
}

impl<'intra, CV, Domain> CrateAnalysisCtxtIntraView<'intra, CV, Domain>
where
    CV: IsRustcIndexDefinedCV,
    Domain: Clone + Copy + JoinLattice,
{
    pub fn generate_local(&mut self, base: Local, ssa_idx: SSAIdx) -> Range<CV> {
        let cvs = &self.locals[base];
        let entry_fact = &cvs[0usize.into()];
        let nested_level = entry_fact.len();
        assert!(nested_level > 0);

        let n_facts = cvs.len();
        assert_eq!(n_facts, ssa_idx.index());
        // let new_fact = (0..nested_level)

        let start = self.assumptions.next_index();
        (0..nested_level).for_each(|nested_level| {
            let cv = self.assumptions.push(Domain::bottom());
            assert_eq!(
                cv,
                self.source_map.push(CVSourceData::Local {
                    func: self.func,
                    base,
                    ssa_idx,
                    nested_level,
                })
            );
            log::debug!(
                "generate {:?} for {:*<2$}{base:?}^{ssa_idx}",
                cv,
                "",
                nested_level
            );
            // lambda
        });
        let end = self.assumptions.next_index();

        let cvs = Range { start, end };
        // .collect::<SmallVec<_>>();
        self.locals[base].push(cvs.clone());
        // n_facts

        cvs
    }
}

impl<'intra, CV: IsRustcIndexDefinedCV> CrateAnalysisCtxtIntraView<'intra, CV, Option<bool>> {
    pub fn assume(&mut self, cv: CV, value: bool) {
        let assumption = &mut self.assumptions[cv];
        match assumption {
            &mut Some(val) if val ^ value => panic!("conflict in constraint!"),
            _ => *assumption = Some(value),
        }
        log::debug!(
            "generate constraint {:?} = {}",
            cv,
            value.then_some(1).unwrap_or(0)
        )
    }
}

#[derive(Clone)]
pub enum CVSourceData {
    /// A SSA variable
    Local {
        func: Func,
        base: Local,
        ssa_idx: SSAIdx,
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

impl Display for CVSourceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CVSourceData::Local {
                func: body,
                base,
                ssa_idx,
                nested_level,
            } => f.write_fmt(format_args!(
                "({:?}, {:*<2$}{3:?}^{4})",
                body, "", nested_level, base, ssa_idx
            )),
            CVSourceData::FieldDef {
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

pub struct Boundary<Return, Argument> {
    r#return: Return,
    arguments: Vec<Argument>,
}

pub type BoundaryE<T> = Boundary<T, T>;

pub struct FnSigVal<Return, Parameter> {
    r#return: Return,
    parameters: Vec<Parameter>,
}

pub type FnSigValE<T> = FnSigVal<T, T>;

pub trait BoundariesInstantiable {
    type FnReturn;
    type Parameter;
    fn instantiate(&self, fn_sig: &FnSigVal<Self::FnReturn, Self::Parameter>);
}

/// Unit Less or Equal constraint graph per function
/// Node indexing: `[0, 1, globals.start..globals.end, locals.start..locals.end]`
struct ULEConstraintGraph<CV: IsRustcIndexDefinedCV> {
    globals: Range<CV>,
    locals: Range<CV>,
    /// Invariant: `graph.num_nodes() == locals.len() + globals.len() + 2`
    /// TODO!!!!!!!!!!!!!!!!!!!!!
    /// use SparseBitVector Graph instead. forward star allows multi-graph!!!!!!
    graph: Graph<usize, usize>,
}

impl<CV: IsRustcIndexDefinedCV> ULEConstraintGraph<CV> {
    const ZERO_IDX: usize = 0;
    const ONE_IDX: usize = 1;

    pub fn new(globals: Range<CV>, locals: Range<CV>) -> Self {
        let graph = graph::implementation::forward_star::Graph::new(
            2 + globals.len() + locals.len(),
            (globals.clone().chain(locals.clone()).enumerate()).flat_map(|(idx, _)| {
                let idx = idx + 2;
                [(idx, Self::ONE_IDX), (Self::ZERO_IDX, idx)].into_iter()
            }),
        );
        ULEConstraintGraph {
            globals,
            locals,
            graph,
        }
    }

    pub fn add_local(&mut self, x: CV) {
        assert_eq!(self.locals.end, x);
        self.graph.add_node();
        self.locals.end = self.locals.end + 1;
    }

    #[inline]
    pub fn query_cv(&self, x: CV) -> usize {
        self
            .globals
            .contains(&x)
            .then(|| x.index() - self.globals.start.index() + 2)
            .unwrap_or_else(|| x.index() - self.locals.start.index() + 2 + self.globals.len())
    }

    pub fn add_fact(&mut self, x: CV, y: CV) {
        let x_idx = self.query_cv(x);
        let y_idx = self.query_cv(y);
        self.graph.add_edge(x_idx, y_idx);
    }

    pub fn reachable(&self, x: usize, y: usize) -> bool {
        todo!()
    }

    pub fn is_consistent(&self) -> bool {
        !self.reachable(Self::ONE_IDX, Self::ZERO_IDX)
    }
}

#[derive(Debug, Clone)]
pub struct LocationMap<T> {
    /// Location-indexed (BasicBlock for outer index, index within BB
    /// for inner index) map.
    pub(crate) map: IndexVec<BasicBlock, Vec<T>>,
}

impl<T> Index<Location> for LocationMap<T> {
    type Output = T;
    fn index(&self, index: Location) -> &Self::Output {
        &self.map[index.block][index.statement_index]
    }
}

impl<T> IndexMut<Location> for LocationMap<T> {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        &mut self.map[index.block][index.statement_index]
    }
}

impl<T> LocationMap<T>
where
    T: Default + Clone,
{
    crate fn new(body: &Body<'_>) -> Self {
        LocationMap {
            map: body
                .basic_blocks()
                .iter()
                .map(|block| vec![T::default(); block.statements.len() + 1])
                .collect(),
        }
    }
}

rustc_index::newtype_index! {
    pub struct FieldDefIdx {
        DEBUG_FORMAT = "field_def ({})"
    }
}
