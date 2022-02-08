//! Andersen's pointer analysis algorithm.

pub mod constraint_solving;
mod diagnostics;

use rustc_data_structures::graph::scc::Sccs;
use crate::pointer_analysis::{ctxt::PointerAnalysisCtxt, PointerAnalysisNode};
use core::marker::PhantomData;
use rustc_data_structures::graph::WithNumNodes;
use graph::implementation::sparse_bit_vector::SparseBitVectorGraph;
use rustc_index::vec::IndexVec;
use rustc_index::{bit_set::HybridBitSet, vec::Idx};
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

rustc_index::newtype_index! {
    pub struct PtsGraphSccIndex {
        DEBUG_FORMAT = "PtsGraphSccIndex({})"
    }
}

pub struct PtsGraphAuxData {
    // crate reverse_graph: SparseBitVectorGraph<AndersenNode>,
    crate sccs: Sccs<PointerAnalysisNode, PtsGraphSccIndex>,
    pub aliases: IndexVec<PointerAnalysisNode, Vec<PointerAnalysisNode>>, //: HashSet<(PointerAnalysisNode, PointerAnalysisNode)>,
}

impl PtsGraphAuxData {
    fn new(graph: &SparseBitVectorGraph<PointerAnalysisNode>) -> Self {
        let reverse_graph = graph.reverse();
        let mut aliases: IndexVec<PointerAnalysisNode, Vec<PointerAnalysisNode>> =
            IndexVec::from_elem_n(Vec::new(), reverse_graph.num_nodes());
        for p in (0..reverse_graph.num_nodes()).map(|n| PointerAnalysisNode::new(n)) {
            let qs = reverse_graph.successor_nodes(p).iter().collect::<Vec<_>>();
            for (i, &q) in qs.iter().enumerate() {
                for &r in &qs[i + 1..] {
                    aliases[q].push(r);
                    aliases[r].push(q);
                }
            }
        }
        PtsGraphAuxData {
            // reverse_graph: graph.reverse(),
            sccs: Sccs::new(graph),
            aliases,
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
    aux_data: PtsGraphAuxDataCache<State>,
}

impl PtsGraph<InConstruction> {
    pub fn new(num_nodes: usize) -> Self {
        PtsGraph {
            graph: SparseBitVectorGraph::new(num_nodes, [].into_iter()),
            aux_data: PtsGraphAuxDataCache::new(),
        }
    }

    pub fn finish(self) -> PtsGraph<Finished> {
        let aux_data = PtsGraphAuxData::new(&self.graph);
        PtsGraph {
            graph: self.graph,
            aux_data: self.aux_data.finish(aux_data),
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
        self.aux_data.as_ref().sccs.all_sccs()
    }

    pub fn alias(&self, p: PointerAnalysisNode, q: PointerAnalysisNode) -> bool {
        self.aux_data.as_ref().aliases[p].contains(&q)
        //.alias
        //.contains(&(if p <= q { (p, q) } else { (q, p) }))
    }

    pub fn aliases_for(
        &self,
        p: PointerAnalysisNode,
    ) -> impl Iterator<Item = PointerAnalysisNode> + '_ {
        self.aux_data.as_ref().aliases[p].iter().map(|&q| q)
    }
}

impl<S> PtsGraph<S> {
    #[inline]
    pub fn pts(&self, p: PointerAnalysisNode) -> &HybridBitSet<PointerAnalysisNode> {
        self.graph.successor_nodes(p)
    }

    //pub fn alias(&self, p: PointerAnalysisNode, q: PointerAnalysisNode) -> bool {
    //    self.pts(p).clone().subtract(self.pts(q)) | self.pts(q).clone().subtract(self.pts(p))
    //}
}
