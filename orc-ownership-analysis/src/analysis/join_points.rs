//! Functions that calculate the join points of phi nodes

use std::collections::VecDeque;

use derivative::Derivative;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{BasicBlock, Body, Local};
use smallvec::SmallVec;

use super::{
    body_ext::DominanceFrontier,
    constants::{NUM_PHI_NODES, SIZE_PHI_NODE},
    def_sites::Definitions,
    state::SSAIdx,
};

/// `def_sites` contains only locals with long live-range
pub(crate) fn semi_pruned_join_points<'tcx>(
    body: &Body<'tcx>,
    dominance_frontier: &DominanceFrontier,
    // def_sites: &DefSites,
    definitions: &Definitions,
) -> JoinPoints<PhiNode> {
    let def_sites = &definitions.sites;
    let mut join_points = JoinPoints::from_raw(IndexVec::from_elem(
        BasicBlockNodes::new(),
        body.basic_blocks(),
    ));
    let mut already_added = BitSet::new_empty(body.basic_blocks().len());
    let mut work_list = VecDeque::with_capacity(body.basic_blocks.len());
    for (a, bbs) in def_sites.iter_enumerated() {
        // let mut work_list = bbs.iter().collect::<VecDeque<_>>();
        work_list.extend(bbs.iter());
        while let Some(bb) = work_list.pop_front() {
            for &bb_f in &dominance_frontier[bb] {
                if !already_added.contains(bb_f) {
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

/// A phi node for a local X: X_i = $\phi$(X_j, X_k)
#[derive(Clone, Derivative)]
#[derivative(Default)]
pub(crate) struct PhiNode {
    #[derivative(Default(value = "SSAIdx::INIT"))]
    pub(crate) lhs: SSAIdx,
    pub(crate) rhs: SmallVec<[SSAIdx; SIZE_PHI_NODE]>,
}

impl PhiNode {
    pub(crate) fn new(lhs: SSAIdx, rhs: SmallVec<[SSAIdx; SIZE_PHI_NODE]>) -> Self {
        Self { lhs, rhs }
    }
}

/// TODO: use `VecArray<(Local, Payload)>` ??
#[derive(Clone, Debug)]
pub(crate) struct JoinPoints<Payload> {
    data: IndexVec<BasicBlock, BasicBlockNodes<Payload>>,
}

impl<Payload> JoinPoints<Payload> {
    pub(crate) fn from_raw(raw: IndexVec<BasicBlock, BasicBlockNodes<Payload>>) -> Self {
        JoinPoints { data: raw }
    }

    pub(crate) fn into_iter(self) -> impl Iterator<Item = BasicBlockNodes<Payload>> {
        self.data.into_iter()
    }

    pub(crate) fn repack<F, U>(&self, f: F) -> JoinPoints<U>
    where
        F: Fn(Local, &Payload) -> U,
    {
        self.data
            .iter()
            .map(|bb_nodes| bb_nodes.repack(&f))
            .collect::<IndexVec<_, _>>()
            .into()
    }

    pub(crate) fn filter_repack<U, F>(&self, f: F) -> JoinPoints<U>
    where
        F: Fn(Local, &Payload) -> Option<U>,
    {
        self.data
            .iter()
            .map(|bb_nodes| bb_nodes.filter_repack(&f))
            .collect::<IndexVec<_, _>>()
            .into()
    }
}

impl<Payload> From<IndexVec<BasicBlock, BasicBlockNodes<Payload>>> for JoinPoints<Payload> {
    fn from(raw: IndexVec<BasicBlock, BasicBlockNodes<Payload>>) -> Self {
        Self { data: raw }
    }
}

impl<T> std::ops::Index<BasicBlock> for JoinPoints<T> {
    type Output = BasicBlockNodes<T>;

    #[inline]
    fn index(&self, bb: BasicBlock) -> &Self::Output {
        &self.data[bb]
    }
}

impl<T> std::ops::IndexMut<BasicBlock> for JoinPoints<T> {
    #[inline]
    fn index_mut(&mut self, bb: BasicBlock) -> &mut Self::Output {
        &mut self.data[bb]
    }
}

#[derive(Clone, Debug)]
pub(crate) struct BasicBlockNodes<Node> {
    data: SmallVec<[(Local, Node); NUM_PHI_NODES]>,
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
    pub(crate) fn new() -> Self {
        BasicBlockNodes {
            data: SmallVec::new(),
        }
    }

    #[inline]
    pub(crate) fn push(&mut self, local: Local, payload: T) {
        self.data.push((local, payload))
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub(crate) fn locals(&self) -> impl Iterator<Item = Local> + '_ {
        self.data.iter().map(|&(local, _)| local)
    }

    #[inline]
    pub(crate) fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter().map(|(_, payload)| payload)
    }

    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().map(|(_, payload)| payload)
    }

    #[inline]
    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().map(|(_, payload)| payload)
    }

    #[inline]
    pub(crate) fn into_iter_enumerated(self) -> impl Iterator<Item = (Local, T)> {
        self.data.into_iter()
    }

    #[inline]
    pub(crate) fn iter_enumerated(&self) -> impl Iterator<Item = (Local, &T)> {
        self.data.iter().map(|(local, payload)| (*local, payload))
    }

    #[inline]
    pub(crate) fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (Local, &mut T)> {
        self.data
            .iter_mut()
            .map(|(local, payload)| (*local, payload))
    }
}

impl<T> std::ops::Index<Local> for BasicBlockNodes<T> {
    type Output = T;

    fn index(&self, local: Local) -> &Self::Output {
        self.data
            .iter()
            .find_map(|&(this_local, ref t)| (this_local == local).then(|| t))
            .expect(&format!("no phi node for {:?}", local))
    }
}

impl<T> std::ops::IndexMut<Local> for BasicBlockNodes<T> {
    fn index_mut(&mut self, local: Local) -> &mut Self::Output {
        self.data
            .iter_mut()
            .find_map(|&mut (this_local, ref mut t)| (this_local == local).then(|| t))
            .expect(&format!("no phi node for {:?}", local))
    }
}

impl<T> BasicBlockNodes<T> {
    pub(crate) fn repack<F, U>(&self, f: F) -> BasicBlockNodes<U>
    where
        F: Fn(Local, &T) -> U,
    {
        self.iter_enumerated()
            .map(|(local, t)| (local, f(local, t)))
            .collect::<BasicBlockNodes<_>>()
    }

    pub(crate) fn filter_repack<U, F>(&self, f: F) -> BasicBlockNodes<U>
    where
        F: Fn(Local, &T) -> Option<U>,
    {
        self.iter_enumerated()
            .filter_map(|(local, t)| Some((local, f(local, t)?)))
            .collect::<BasicBlockNodes<_>>()
    }
}
