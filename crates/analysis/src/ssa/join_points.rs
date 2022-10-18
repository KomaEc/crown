//! Functions that calculate the join points of phi nodes

use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use derivative::Derivative;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{BasicBlock, Body, Local};
use smallvec::SmallVec;

use super::{
    constants::{NUM_PHI_NODES, SIZE_PHI_NODE},
    dom::DominanceFrontier,
    state::SSAIdx,
};

/// A phi node for a local X: X_i = ϕ(X_j, X_k)
#[derive(Clone, Derivative)]
#[derivative(Default)]
pub struct PhiNode {
    #[derivative(Default(value = "SSAIdx::INIT"))]
    pub lhs: SSAIdx,
    pub rhs: SmallVec<[SSAIdx; SIZE_PHI_NODE]>,
}

/// Property: the set of join points must guarantee that
/// definitions (consumptions) flow into the final return
/// block, for an implicit final use. Therefore, it seems
/// not necessary to construct a semi-pruned SSA
#[derive(Clone, Debug)]
pub struct JoinPoints<Payload> {
    pub(crate) data: IndexVec<BasicBlock, BasicBlockNodes<Payload>>,
}

impl JoinPoints<PhiNode> {
    pub fn new<'tcx>(
        body: &Body<'tcx>,
        dominance_frontier: &DominanceFrontier,
        maybe_consume_sites: &IndexVec<Local, BitSet<BasicBlock>>,
    ) -> Self {
        // let live_range = &definitions.live_range;
        let mut join_points = JoinPoints::from_raw(IndexVec::from_elem(
            BasicBlockNodes::new(),
            &body.basic_blocks,
        ));
        let mut already_added = BitSet::new_empty(body.basic_blocks.len());
        let mut work_list = VecDeque::with_capacity(body.basic_blocks.len());
        for (a, bbs) in maybe_consume_sites.iter_enumerated() {
            // let mut work_list = bbs.iter().collect::<VecDeque<_>>();
            work_list.extend(bbs.iter());
            while let Some(bb) = work_list.pop_front() {
                for &bb_f in &dominance_frontier[bb] {
                    if !already_added.contains(bb_f) {
                        join_points[bb_f].data.push((a, PhiNode::default()));
                        already_added.insert(bb_f);
                        if !maybe_consume_sites[a].contains(bb_f) {
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

    #[inline]
    pub fn reset(&mut self) {
        for (_, phi_node) in self.phi_nodes_mut() {
            phi_node.rhs.clear();
        }
    }

    #[inline]
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
    pub data: SmallVec<[(Local, Node); NUM_PHI_NODES]>,
}

impl<Payload> FromIterator<(Local, Payload)> for BasicBlockNodes<Payload> {
    #[inline]
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

    #[inline]
    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (Local, &mut T)> {
        self.data
            .iter_mut()
            .map(|(local, payload)| (*local, payload))
    }
}

impl<T> Deref for BasicBlockNodes<T> {
    type Target = SmallVec<[(Local, T); NUM_PHI_NODES]>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for BasicBlockNodes<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
