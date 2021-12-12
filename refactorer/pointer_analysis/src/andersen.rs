//! Andersen's pointer analysis algorithm.

mod constraint_generation;
mod constraint_solving;
mod ctxt;
mod diagnostics;

use crate::andersen::ctxt::AndersenAnalysisCtxt;
use core::marker::PhantomData;
use graph::{implementation::sparse_bit_vector::SparseBitVectorGraph, scc::Sccs};
use index::{bit_set::HybridBitSet, vec::IndexVec};
use rustc_hir::def_id::LocalDefId;
use rustc_middle::{
    mir::{Body, PlaceRef},
    ty::TyCtxt,
};
use std::cell::Ref;
use std::convert::AsRef;
use std::mem::MaybeUninit;
use std::ops::Index;

pub struct AndersenAnalysis;

impl<'aa, 'tcx> AndersenAnalysis {
    pub fn new_analysis(
        all_functions: &'aa [Ref<'aa, Body<'tcx>>],
        tcx: TyCtxt<'tcx>,
    ) -> AndersenAnalysisCtxt<'aa, 'tcx> {
        AndersenAnalysisCtxt::new(all_functions, tcx)
    }
}

pub struct AndersenResult<'ar, 'tcx> {
    pub pts_graph: PtsGraph<Finished>,
    pub aa_ctxt: AndersenAnalysisCtxt<'ar, 'tcx>,
}

impl<'ar, 'tcx> AndersenResult<'ar, 'tcx> {
    pub fn new(
        pts_graph: PtsGraph<InConstruction>,
        node_ctxt: AndersenAnalysisCtxt<'ar, 'tcx>,
    ) -> Self {
        AndersenResult {
            pts_graph: pts_graph.finish(),
            aa_ctxt: node_ctxt,
        }
    }

    pub fn enter<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }

    pub fn dump_pts_sets_to_log(&self) {
        log::debug!("Dumping andersen analysis results:");
        for p in self.aa_ctxt.nodes().indices() {
            log::debug!(
                "pts({}) = {}",
                self.aa_ctxt.node_to_str(p),
                self.pts_graph
                    .pts(p)
                    .iter()
                    .map(|q| self.aa_ctxt.node_to_str(q))
                    .reduce(|acc, item| acc + ", " + &item)
                    .map_or("∅".to_owned(), |s| format!("{{ {} }}", s))
            );
        }
    }
}

index::newtype_index! {
    pub struct PtsGraphSccIndex {
        DEBUG_FORMAT = "PtsGraphSccIndex({})"
    }
}

crate struct PtsGraphAuxData {
    // crate reverse_graph: SparseBitVectorGraph<AndersenNode>,
    crate sccs: Sccs<AndersenNode, PtsGraphSccIndex>,
}

impl PtsGraphAuxData {
    crate fn new(graph: &SparseBitVectorGraph<AndersenNode>) -> Self {
        PtsGraphAuxData {
            // reverse_graph: graph.reverse(),
            sccs: Sccs::new(graph),
        }
    }
}

/// transparent layout to enable safe transmute
#[repr(transparent)]
crate struct PtsGraphAuxDataCache<State> {
    cache: MaybeUninit<PtsGraphAuxData>,
    _state: PhantomData<State>,
}

impl PtsGraphAuxDataCache<InConstruction> {
    pub fn new() -> Self {
        PtsGraphAuxDataCache {
            cache: MaybeUninit::uninit(),
            _state: PhantomData,
        }
    }

    #[inline]
    pub fn finish(mut self, data: PtsGraphAuxData) -> PtsGraphAuxDataCache<Finished> {
        self.cache.write(data);
        unsafe { std::mem::transmute(self) }
    }
}

impl AsRef<PtsGraphAuxData> for PtsGraphAuxDataCache<Finished> {
    fn as_ref(&self) -> &PtsGraphAuxData {
        unsafe { self.cache.assume_init_ref() }
    }
}

impl<S> Drop for PtsGraphAuxDataCache<S> {
    /// default drop does nothing
    default fn drop(&mut self) {}
}

impl Drop for PtsGraphAuxDataCache<Finished> {
    fn drop(&mut self) {
        unsafe { self.cache.assume_init_drop() }
    }
}

#[derive(Debug)]
pub struct InConstruction;
#[derive(Debug)]
pub struct Finished;

pub struct PtsGraph<State> {
    graph: SparseBitVectorGraph<AndersenNode>,
    aux_data_cache: PtsGraphAuxDataCache<State>,
}

impl PtsGraph<InConstruction> {
    pub fn new(num_nodes: usize) -> Self {
        PtsGraph {
            graph: SparseBitVectorGraph::new(num_nodes, [].into_iter()),
            aux_data_cache: PtsGraphAuxDataCache::new(),
        }
    }

    pub fn finish(self) -> PtsGraph<Finished> {
        let aux_data = PtsGraphAuxData::new(&self.graph);
        PtsGraph {
            graph: self.graph,
            aux_data_cache: self.aux_data_cache.finish(aux_data),
        }
    }

    #[inline]
    pub fn pts_mut(&mut self, p: AndersenNode) -> &mut HybridBitSet<AndersenNode> {
        self.graph.successor_nodes_mut(p)
    }

    #[inline]
    pub fn pick2_pts_mut(
        &mut self,
        p: AndersenNode,
        q: AndersenNode,
    ) -> (
        &mut HybridBitSet<AndersenNode>,
        &mut HybridBitSet<AndersenNode>,
    ) {
        self.graph.pick2_successor_nodes_mut(p, q)
    }
}

impl PtsGraph<Finished> {
    pub fn all_cyclic_reference_group_indices(&self) -> impl Iterator<Item = PtsGraphSccIndex> {
        self.aux_data_cache.as_ref().sccs.all_sccs()
    }
}

impl<S> PtsGraph<S> {
    #[inline]
    pub fn pts(&self, p: AndersenNode) -> &HybridBitSet<AndersenNode> {
        self.graph.successor_nodes(p)
    }

    pub fn alias(&self, p: AndersenNode, q: AndersenNode) -> bool {
        self.pts(p).clone().subtract(self.pts(q)) | self.pts(q).clone().subtract(self.pts(p))
    }
}

pub struct ConstraintSet {
    constraints: IndexVec<ConstraintIndex, Constraint>,
}

impl ConstraintSet {
    pub fn new() -> ConstraintSet {
        ConstraintSet {
            constraints: IndexVec::new(),
        }
    }

    #[inline]
    pub fn num_constraints(&self) -> usize {
        self.constraints.len()
    }

    #[inline]
    pub fn push(&mut self, c: Constraint) -> ConstraintIndex {
        self.constraints.push(c)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.iter()
    }

    #[inline]
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (ConstraintIndex, &Constraint)> {
        self.constraints.iter_enumerated()
    }

    #[inline]
    pub fn universe(&self) -> &IndexVec<ConstraintIndex, Constraint> {
        &self.constraints
    }
}

impl Index<ConstraintIndex> for ConstraintSet {
    type Output = Constraint;

    fn index(&self, i: ConstraintIndex) -> &Self::Output {
        &self.constraints[i]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ConstraintKind {
    /// p = &q
    AddressOf,
    /// p = q
    Copy,
    /// p = *q
    Load,
    /// *p = q
    Store,
}

#[derive(Clone, Copy)]
pub struct Constraint {
    pub constraint_kind: ConstraintKind,
    pub left: AndersenNode,
    pub right: AndersenNode,
}

impl Constraint {
    pub fn new(ck: ConstraintKind, l: AndersenNode, r: AndersenNode) -> Self {
        Constraint {
            constraint_kind: ck,
            left: l,
            right: r,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AndersenNodeData<'tcx> {
    Mir(LocalDefId, PlaceRef<'tcx>),
    Temporary(LocalDefId),
}

impl<'tcx> From<(LocalDefId, PlaceRef<'tcx>)> for AndersenNodeData<'tcx> {
    fn from(data: (LocalDefId, PlaceRef<'tcx>)) -> Self {
        AndersenNodeData::Mir(data.0, data.1)
    }
}

impl<'tcx> From<LocalDefId> for AndersenNodeData<'tcx> {
    fn from(did: LocalDefId) -> Self {
        AndersenNodeData::Temporary(did)
    }
}

index::newtype_index! {
    pub struct AndersenNode {
        DEBUG_FORMAT = "AndersenNode({})"
    }
}

impl AndersenNode {
    pub fn kopy(self, other: AndersenNode) -> Constraint {
        Constraint::new(ConstraintKind::Copy, self, other)
    }

    pub fn load(self, other: AndersenNode) -> Constraint {
        Constraint::new(ConstraintKind::Load, self, other)
    }

    pub fn store(self, other: AndersenNode) -> Constraint {
        Constraint::new(ConstraintKind::Store, self, other)
    }

    pub fn get_address_of(self, other: AndersenNode) -> Constraint {
        Constraint::new(ConstraintKind::AddressOf, self, other)
    }
}

index::newtype_index! {
    pub struct ConstraintIndex {
        DEBUG_FORMAT = "AndersenConstraintIndex({})"
    }
}
