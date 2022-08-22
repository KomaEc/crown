//! State for analysis steps

use std::num::NonZeroU32;

use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::mir::Local;
use smallvec::{smallvec, SmallVec};

use super::{
    constants::NUM_DEFS_PER_BLOCK,
    join_points::{JoinPoints, PhiNode},
};

orc_common::macros::newtype_index! {
    pub struct SSAIdx {
        DEBUG_FORMAT = "{}"
    }
}

impl SSAIdx {
    pub const INIT: Self = SSAIdx::from_u32(0);
}

impl std::ops::AddAssign<usize> for SSAIdx {
    #[inline]
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

    /// Get the newest version for a variable. If `None` is returned,
    /// this variable is uninitialised.
    #[inline]
    pub fn try_get_name(&self, var: T) -> Option<SSAIdx> {
        self.stack[var].last().copied()
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

#[cfg(test)]
mod test {
    use super::*;
    use smallvec::smallvec;

    #[test]
    fn test1() {
        let mut state: NameState<u32> = NameState {
            count: IndexVec::from_raw(vec![12u32.into()]),
            stack: IndexVec::from_raw(vec![vec![4u32.into(), 5u32.into()]]),
            n_defs: vec![
                smallvec![(0u32.into(), 1.try_into().unwrap())],
                smallvec![(0u32.into(), 1.try_into().unwrap())],
            ],
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
