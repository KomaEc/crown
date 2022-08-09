use std::num::NonZeroU32;

use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::mir::{BasicBlock, Local};
use smallvec::{smallvec, SmallVec};

use super::constants::{NUM_DEFS_PER_BLOCK, NUM_PHI_NODES, SIZE_PHI_NODE};

crate::macros::newtype_index! {
    pub struct SSAIdx {
        DEBUG_FORMAT = "{}"
    }
}

impl SSAIdx {
    pub const INIT: Self = SSAIdx::from_u32(0);
}

impl std::ops::AddAssign<usize> for SSAIdx {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

pub struct SSAState {
    name_state: NameState<Local>,
    join_points: JoinPoints<PhiNode>,
}

#[derive(Clone, Debug)]
pub struct NameState<T: Idx> {
    count: IndexVec<T, SSAIdx>,
    stack: IndexVec<T, Vec<SSAIdx>>,
    /// number of definitions in current dominance tree path
    /// block in this path -> local -> nonzero number of definitions
    n_defs: Vec<SmallVec<[(T, NonZeroU32); NUM_DEFS_PER_BLOCK]>>,
}

impl<T: Idx> NameState<T> {
    #[inline]
    pub fn enter_new_block(&mut self) {
        self.n_defs.push(smallvec![]);
    }

    #[inline]
    pub fn generate_fresh_name(&mut self, var: T) -> SSAIdx {
        // get number of definitions in current block
        let n_defs = self
            .n_defs
            .last_mut()
            .expect("internal error: empty dominance tree trace");

        n_defs
            .iter_mut()
            .find(|(that, _)| *that == var)
            .map(|(_, num)| *num = NonZeroU32::saturating_add(*num, 1))
            .unwrap_or_else(|| n_defs.push((var, unsafe { NonZeroU32::new_unchecked(1) })));

        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx
    }

    #[inline]
    pub fn get_name(&self, var: T) -> SSAIdx {
        *self.stack[var].last().expect(&format!(
            "internal error: cannot find fresh name supply for {:?}",
            var
        ))
    }

    #[inline]
    /// Remove fresh names generated in current dominance tree paths
    /// the parameter `n_blocks` is the number of basic block nodes along
    /// the tree path
    pub fn remove_names(&mut self, n_blocks: usize) {
        let n_defs = self.n_defs.drain(self.n_defs.len() - n_blocks..);

        for n_defs in n_defs {
            for (local, n_defs) in n_defs {
                let stack_len = self.stack[local].len();
                self.stack[local].truncate(stack_len - n_defs.get() as usize);
            }
        }
    }
}

/// A phi node for a local X: X_i = $\phi$(X_j, X_k)
#[derive(Clone)]
pub struct PhiNode {
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
    raw: IndexVec<BasicBlock, BasicBlockNodes<Payload>>,
}

impl<Payload> JoinPoints<Payload> {
    pub fn from_raw(raw: IndexVec<BasicBlock, BasicBlockNodes<Payload>>) -> Self {
        JoinPoints { raw }
    }

    pub fn into_iter(self) -> impl Iterator<Item = BasicBlockNodes<Payload>> {
        self.raw.into_iter()
    }

    pub fn repack<F, U>(&self, f: F) -> JoinPoints<U>
    where
        F: Fn(Local, &Payload) -> U,
    {
        self.raw
            .iter()
            .map(|bb_nodes| bb_nodes.repack(&f))
            .collect::<IndexVec<_, _>>()
            .into()
    }

    pub fn filter_repack<U, F>(&self, f: F) -> JoinPoints<U>
    where
        F: Fn(Local, &Payload) -> Option<U>,
    {
        self.raw
            .iter()
            .map(|bb_nodes| bb_nodes.filter_repack(&f))
            .collect::<IndexVec<_, _>>()
            .into()
    }
}

impl<Payload> From<IndexVec<BasicBlock, BasicBlockNodes<Payload>>> for JoinPoints<Payload> {
    fn from(raw: IndexVec<BasicBlock, BasicBlockNodes<Payload>>) -> Self {
        Self { raw }
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

#[cfg(test)]
mod test {
    use super::*;
    use smallvec::smallvec;

    #[test]
    fn test1() {
        let mut state: NameState<u32> = NameState {
            count: IndexVec::from_raw(vec![12u32.into()]),
            stack: IndexVec::from_raw(vec![vec![4u32.into(), 5u32.into()]]),
            n_defs: vec![smallvec![(0u32.into(), 1.try_into().unwrap())], smallvec![(0u32.into(), 1.try_into().unwrap())]],
        };

        assert_eq!(state.get_name(0), 5u32.into());
        assert_eq!(state.generate_fresh_name(0), 13u32.into());
        assert_eq!(state.get_name(0), 13u32.into());

        assert_eq!(&state.n_defs[0][..], [(0u32.into(), 1.try_into().unwrap())]);
        assert_eq!(&state.n_defs[1][..], [(0u32.into(), 2.try_into().unwrap())]);

        state.remove_names(1);

        assert_eq!(state.get_name(0), 4u32.into());
        assert_eq!(state.n_defs.len(), 1);
    }

    #[test]
    fn test2() {
        let mut state: NameState<u32> = NameState {
            count: IndexVec::from_raw(vec![0u32.into()]),
            stack: IndexVec::from_raw(vec![vec![0u32.into()]]),
            n_defs: vec![],
        };

        state.enter_new_block();
        assert_eq!(state.get_name(0), SSAIdx::INIT);
        let _ = state.generate_fresh_name(0);
        assert_eq!(state.get_name(0), 1u32.into());
        let _ = state.generate_fresh_name(0);
        assert_eq!(state.get_name(0), 2u32.into());

        state.enter_new_block();
        let _ = state.generate_fresh_name(0);
        assert_eq!(state.get_name(0), 3u32.into());

        state.remove_names(1);
        assert_eq!(state.get_name(0), 2u32.into());

        state.enter_new_block();
        let _ = state.generate_fresh_name(0);
        assert_eq!(state.get_name(0), 4u32.into());
        let _ = state.generate_fresh_name(0);
        assert_eq!(state.get_name(0), 5u32.into());

        state.remove_names(1);
        assert_eq!(state.get_name(0), 2u32.into());
    }
}
