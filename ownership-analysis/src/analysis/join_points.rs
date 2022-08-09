//! Functions that calculate the join points of phi nodes

use derivative::Derivative;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Local};
use smallvec::SmallVec;

use super::{
    body_ext::DominanceFrontier,
    constants::{NUM_PHI_NODES, SIZE_PHI_NODE},
    state::SSAIdx,
};

pub fn compute_join_points<'tcx>(
    body: &Body<'tcx>,
    dominance_frontier: &DominanceFrontier,
) -> JoinPoints<PhiNode> {
    todo!()
}

/// A phi node for a local X: X_i = $\phi$(X_j, X_k)
#[derive(Clone, Derivative)]
#[derivative(Default)]
pub struct PhiNode {
    #[derivative(Default(value = "SSAIdx::INIT"))]
    pub lhs: SSAIdx,
    pub rhs: SmallVec<[SSAIdx; SIZE_PHI_NODE]>,
}

impl PhiNode {
    pub fn new(lhs: SSAIdx, rhs: SmallVec<[SSAIdx; SIZE_PHI_NODE]>) -> Self {
        Self { lhs, rhs }
    }
}

#[derive(Clone, Debug)]
pub struct JoinPoints<Payload> {
    data: IndexVec<BasicBlock, BasicBlockNodes<Payload>>,
}

impl<Payload> JoinPoints<Payload> {
    pub fn from_raw(raw: IndexVec<BasicBlock, BasicBlockNodes<Payload>>) -> Self {
        JoinPoints { data: raw }
    }

    pub fn into_iter(self) -> impl Iterator<Item = BasicBlockNodes<Payload>> {
        self.data.into_iter()
    }

    pub fn repack<F, U>(&self, f: F) -> JoinPoints<U>
    where
        F: Fn(Local, &Payload) -> U,
    {
        self.data
            .iter()
            .map(|bb_nodes| bb_nodes.repack(&f))
            .collect::<IndexVec<_, _>>()
            .into()
    }

    pub fn filter_repack<U, F>(&self, f: F) -> JoinPoints<U>
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

// /// `PhiNodeInsertionPoints<Payload>` should act completely the same as
// /// `IndexVec<BasicBlock, BasicBlockInsersionPoints<Payload>>`, so we implement
// /// `Deref`
// impl<Payload> std::ops::Deref for JoinPoints<Payload> {
//     type Target = IndexVec<BasicBlock, BasicBlockNodes<Payload>>;

//     fn deref(&self) -> &Self::Target {
//         &self.raw
//     }
// }

// impl<Payload> std::ops::DerefMut for JoinPoints<Payload> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.raw
//     }
// }

#[derive(Clone, Debug)]
pub struct BasicBlockNodes<Node> {
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
    pub fn new() -> Self {
        BasicBlockNodes {
            data: SmallVec::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, local: Local, payload: T) {
        self.data.push((local, payload))
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn locals(&self) -> impl Iterator<Item = Local> + '_ {
        self.data.iter().map(|&(local, _)| local)
    }

    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter().map(|(_, payload)| payload)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().map(|(_, payload)| payload)
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().map(|(_, payload)| payload)
    }

    #[inline]
    pub fn into_iter_enumerated(self) -> impl Iterator<Item = (Local, T)> {
        self.data.into_iter()
    }

    #[inline]
    pub fn iter_enumerated(&self) -> impl Iterator<Item = (Local, &T)> {
        self.data.iter().map(|(local, payload)| (*local, payload))
    }

    #[inline]
    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (Local, &mut T)> {
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
    pub fn repack<F, U>(&self, f: F) -> BasicBlockNodes<U>
    where
        F: Fn(Local, &T) -> U,
    {
        self.iter_enumerated()
            .map(|(local, t)| (local, f(local, t)))
            .collect::<BasicBlockNodes<_>>()
    }

    pub fn filter_repack<U, F>(&self, f: F) -> BasicBlockNodes<U>
    where
        F: Fn(Local, &T) -> Option<U>,
    {
        self.iter_enumerated()
            .filter_map(|(local, t)| Some((local, f(local, t)?)))
            .collect::<BasicBlockNodes<_>>()
    }
}
