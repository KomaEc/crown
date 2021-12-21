//! Andersen's pointer analysis algorithm.

pub mod constraint_solving;
mod diagnostics;

use crate::{ctxt::PointerAnalysisCtxt, PointerAnalysisNode};
use core::marker::PhantomData;
use graph::{implementation::sparse_bit_vector::SparseBitVectorGraph, scc::Sccs};
use index::bit_set::HybridBitSet;
use std::convert::AsRef;
use std::mem::MaybeUninit;

pub struct AndersenResult<'ar, 'tcx> {
    pub pts_graph: PtsGraph<Finished>,
    pub ptr_ctxt: PointerAnalysisCtxt<'ar, 'tcx>,
}

impl<'ar, 'tcx> AndersenResult<'ar, 'tcx> {
    pub fn new(
        pts_graph: PtsGraph<InConstruction>,
        node_ctxt: PointerAnalysisCtxt<'ar, 'tcx>,
    ) -> Self {
        AndersenResult {
            pts_graph: pts_graph.finish(),
            ptr_ctxt: node_ctxt,
        }
    }

    pub fn enter<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }

    pub fn dump_pts_sets_to_log(&self) {
        log::debug!("Dumping andersen analysis results:");
        for p in self.ptr_ctxt.nodes().indices() {
            log::debug!(
                "pts({}) = {}",
                self.ptr_ctxt.node_to_str(p),
                self.pts_graph
                    .pts(p)
                    .iter()
                    .map(|q| self.ptr_ctxt.node_to_str(q))
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
    crate sccs: Sccs<PointerAnalysisNode, PtsGraphSccIndex>,
}

impl PtsGraphAuxData {
    crate fn new(graph: &SparseBitVectorGraph<PointerAnalysisNode>) -> Self {
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
    graph: SparseBitVectorGraph<PointerAnalysisNode>,
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
    pub fn pts_mut(&mut self, p: PointerAnalysisNode) -> &mut HybridBitSet<PointerAnalysisNode> {
        self.graph.successor_nodes_mut(p)
    }

    #[inline]
    pub fn pick2_pts_mut(
        &mut self,
        p: PointerAnalysisNode,
        q: PointerAnalysisNode,
    ) -> (
        &mut HybridBitSet<PointerAnalysisNode>,
        &mut HybridBitSet<PointerAnalysisNode>,
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
    pub fn pts(&self, p: PointerAnalysisNode) -> &HybridBitSet<PointerAnalysisNode> {
        self.graph.successor_nodes(p)
    }

    pub fn alias(&self, p: PointerAnalysisNode, q: PointerAnalysisNode) -> bool {
        self.pts(p).clone().subtract(self.pts(q)) | self.pts(q).clone().subtract(self.pts(p))
    }
}
