//! Functions that calculate the join points of phi nodes

use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use rustc_index::{bit_set::BitSet, IndexVec};
use rustc_middle::{
    mir::{BasicBlock, Body, Local},
    ty::TyCtxt,
};
use rustc_mir_dataflow::{impls::MaybeLiveLocals, Analysis};
use smallvec::SmallVec;

use super::{dom::DominanceFrontier, SSAIdx};

/// A phi node for a local X: X_i = ϕ(X_j, X_k)
#[derive(Clone, Default)]
pub struct PhiNode {
    pub lhs: SSAIdx,
    pub rhs: SmallVec<[SSAIdx; 3]>,
}

/// Property: the set of join points must guarantee that
/// definitions (consumptions) flow into the final return
/// block, for an implicit final use. Therefore, it seems
/// not necessary to construct a semi-pruned SSA
#[derive(Clone, Debug)]
pub struct JoinPoints<Payload> {
    pub data: IndexVec<BasicBlock, BasicBlockNodes<Payload>>,
}

impl JoinPoints<PhiNode> {
    pub fn new<'tcx>(
        body: &Body<'tcx>,
        dominance_frontier: &DominanceFrontier,
        def_sites: &IndexVec<Local, BitSet<BasicBlock>>,
        tcx: TyCtxt<'tcx>,
    ) -> Self {
        let mut liveness = MaybeLiveLocals
            .into_engine(tcx, body)
            .iterate_to_fixpoint()
            .into_results_cursor(body);
        let mut join_points = JoinPoints::from_raw(IndexVec::from_elem(
            BasicBlockNodes::new(),
            &body.basic_blocks,
        ));
        let mut already_added = BitSet::new_empty(body.basic_blocks.len());
        let mut work_list = VecDeque::with_capacity(body.basic_blocks.len());
        for (a, bbs) in def_sites.iter_enumerated() {
            work_list.extend(bbs.iter());
            while let Some(bb) = work_list.pop_front() {
                for &bb_f in &dominance_frontier[bb] {
                    liveness.seek_to_block_start(bb_f);
                    // The liveness checking here is necessary! The reason is that we have a set of
                    // copy locals during ownership analysis, which are guaranteed to have short live-range.
                    // Since they are copies, they should not be joined with any variables (e.g. entry)!
                    if !already_added.contains(bb_f) && liveness.get().contains(a) {
                        join_points[bb_f].data.push((a, PhiNode::default()));
                        already_added.insert(bb_f);
                        if !def_sites[a].contains(bb_f) {
                            work_list.push_back(bb_f);
                        }
                    }
                }
            }
            work_list.clear();
            already_added.clear();
        }
        join_points
    }

    // pub fn phi_nodes(&self) -> impl Iterator<Item = (Local, &PhiNode)> {
    //     self.data
    //         .iter()
    //         .flat_map(|bb_nodes| bb_nodes.iter_enumerated())
    // }

    pub fn reset(&mut self) {
        for (_, phi_node) in self.phi_nodes_mut() {
            phi_node.rhs.clear();
        }
    }

    pub fn phi_nodes_mut(&mut self) -> impl Iterator<Item = (Local, &mut PhiNode)> {
        self.data
            .iter_mut()
            .flat_map(|bb_nodes| bb_nodes.iter_enumerated_mut())
    }
}

impl<Payload> JoinPoints<Payload> {
    pub fn from_raw(raw: IndexVec<BasicBlock, BasicBlockNodes<Payload>>) -> Self {
        JoinPoints { data: raw }
    }
}

impl<Payload> From<IndexVec<BasicBlock, BasicBlockNodes<Payload>>> for JoinPoints<Payload> {
    fn from(raw: IndexVec<BasicBlock, BasicBlockNodes<Payload>>) -> Self {
        Self { data: raw }
    }
}

impl<T> Deref for JoinPoints<T> {
    type Target = IndexVec<BasicBlock, BasicBlockNodes<T>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for JoinPoints<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[derive(Clone, Debug)]
pub struct BasicBlockNodes<Node> {
    pub data: SmallVec<[(Local, Node); 3]>,
}

impl<Payload> FromIterator<(Local, Payload)> for BasicBlockNodes<Payload> {
    fn from_iter<T: IntoIterator<Item = (Local, Payload)>>(iter: T) -> Self {
        Self {
            data: iter.into_iter().collect::<SmallVec<_>>(),
        }
    }
}

impl<T> BasicBlockNodes<T> {
    pub fn new() -> Self {
        BasicBlockNodes {
            data: SmallVec::new(),
        }
    }

    // #[inline]
    // pub fn iter_enumerated(&self) -> impl Iterator<Item = (Local, &T)> {
    //     self.data.iter().map(|(local, payload)| (*local, payload))
    // }

    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (Local, &mut T)> {
        self.data
            .iter_mut()
            .map(|(local, payload)| (*local, payload))
    }
}

impl<T> Deref for BasicBlockNodes<T> {
    type Target = SmallVec<[(Local, T); 3]>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for BasicBlockNodes<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
