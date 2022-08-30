//! State for analysis steps

use rustc_index::vec::IndexVec;
use rustc_middle::mir::{Local, Location};

use super::{
    def::{ConsumeChain, Consume},
    join_points::{JoinPoints, PhiNode},
};

orc_common::macros::newtype_index! {
    pub(crate) struct SSAIdx {
        DEBUG_FORMAT = "{}"
    }
}

impl SSAIdx {
    pub(crate) const INIT: Self = SSAIdx::from_u32(0);
}

impl std::ops::AddAssign<usize> for SSAIdx {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

pub(crate) struct SSAState {
    pub(crate) name_state: NameState,
    pub(crate) join_points: JoinPoints<PhiNode>,
    /// TODO: initialise
    pub(crate) consume_chain: ConsumeChain,
}

impl SSAState {
    #[inline]
    pub(crate) fn consume_at(&mut self, local: Local, location: Location) -> Option<Consume> {
        let consume = self.consume_chain.consumes[location.block.index()][location.statement_index]
            .iter_mut()
            .find_map(|(this, consume)| (*this == local).then(|| consume))?;
        let old_ssa_idx = self.name_state.get_name(local);
        let new_ssa_idx = self.name_state.generate_fresh_name(local);
        tracing::debug!(
            "consuming {:?} at {:?}, use: {:?}, def: {:?}",
            local,
            location,
            old_ssa_idx,
            new_ssa_idx
        );
        consume.r#use = old_ssa_idx;
        consume.def = new_ssa_idx;
        assert_eq!(new_ssa_idx, self.consume_chain.locs[local].push(location));
        Some(*consume)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NameState {
    count: IndexVec<Local, SSAIdx>,
    stack: IndexVec<Local, Vec<SSAIdx>>,
}

impl NameState {
    #[inline]
    pub(crate) fn generate_fresh_name(&mut self, var: Local) -> SSAIdx {
        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx
    }

    #[inline]
    pub(crate) fn get_name(&self, var: Local) -> SSAIdx {
        self.stack[var].last().copied().unwrap_or_else(|| {
            panic!(
                "internal error: cannot find fresh name supply for {:?}",
                var
            )
        })
    }

    /// Get the newest version for a variable. If `None` is returned,
    /// this variable is uninitialised.
    #[inline]
    pub(crate) fn try_get_name(&self, var: Local) -> Option<SSAIdx> {
        self.stack[var].last().copied()
    }

    #[inline]
    pub(crate) fn pop(&mut self, var: Local) -> SSAIdx {
        tracing::debug!("popping {:?}", var);
        self.stack[var]
            .pop()
            .unwrap_or_else(|| panic!("internal error: poping non existing version for {:?}", var))
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use smallvec::smallvec;

//     #[test]
//     fn test1() {
//         let mut state: NameState<u32> = NameState {
//             count: IndexVec::from_raw(vec![12u32.into()]),
//             stack: IndexVec::from_raw(vec![vec![4u32.into(), 5u32.into()]]),
//             n_defs: vec![
//                 smallvec![(0u32.into(), 1.try_into().unwrap())],
//                 smallvec![(0u32.into(), 1.try_into().unwrap())],
//             ],
//         };

//         assert_eq!(state.get_name(0), 5u32.into());
//         assert_eq!(state.generate_fresh_name(0), 13u32.into());
//         assert_eq!(state.get_name(0), 13u32.into());

//         assert_eq!(&state.n_defs[0][..], [(0u32.into(), 1.try_into().unwrap())]);
//         assert_eq!(&state.n_defs[1][..], [(0u32.into(), 2.try_into().unwrap())]);

//         state.remove_names(1);

//         assert_eq!(state.get_name(0), 4u32.into());
//         assert_eq!(state.n_defs.len(), 1);
//     }

//     #[test]
//     fn test2() {
//         let mut state: NameState<u32> = NameState {
//             count: IndexVec::from_raw(vec![0u32.into()]),
//             stack: IndexVec::from_raw(vec![vec![0u32.into()]]),
//             n_defs: vec![],
//         };

//         state.enter_new_block();
//         assert_eq!(state.get_name(0), SSAIdx::INIT);
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 1u32.into());
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 2u32.into());

//         state.enter_new_block();
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 3u32.into());

//         state.remove_names(1);
//         assert_eq!(state.get_name(0), 2u32.into());

//         state.enter_new_block();
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 4u32.into());
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 5u32.into());

//         state.remove_names(1);
//         assert_eq!(state.get_name(0), 2u32.into());
//     }
// }
